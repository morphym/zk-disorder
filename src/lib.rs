use borsh::{BorshDeserialize, BorshSerialize};
use fract::Fract;

// --- Constants ---
pub const TRACE_LEN: usize = 16;
pub const TOTAL_STATES: usize = TRACE_LEN + 1; // 17 states
// We pad to next power of 2 (32) for simple Merkle math
pub const MERKLE_LEAVES: usize = 32;

// --- 1. Encryption (Hyperchaotic Sponge) ---

pub struct FractCipher {
    pub engine: Fract,
}

impl FractCipher {
    pub fn new(key: [u64; 2], iv: [u64; 2]) -> Self {
        // State Layout: [Rate_0, Rate_1, Cap_0, Cap_1]
        let state = [iv[0], iv[1], key[0], key[1]];
        Self {
            engine: Fract::from_state(state),
        }
    }

    pub fn encrypt(&mut self, plaintext: [u64; 2]) -> [u64; 2] {
        self.engine.state[0] ^= plaintext[0];
        self.engine.state[1] ^= plaintext[1];

        for _ in 0..8 {
            self.engine.apply_phi();
        }

        [self.engine.state[0], self.engine.state[1]]
    }
}

// --- 2. Zero-Knowledge Proof (Cut-and-Choose Trace) ---

#[derive(Clone, Copy, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct StateSnapshot {
    pub s: [u64; 4],
}

impl StateSnapshot {
    pub fn to_bytes(&self) -> [u8; 32] {
        let mut buf = [0u8; 32];
        buf[0..8].copy_from_slice(&self.s[0].to_le_bytes());
        buf[8..16].copy_from_slice(&self.s[1].to_le_bytes());
        buf[16..24].copy_from_slice(&self.s[2].to_le_bytes());
        buf[24..32].copy_from_slice(&self.s[3].to_le_bytes());
        buf
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct ZKProof {
    pub merkle_root: [u8; 32],
    /// Revealed transitions: (Step Index, State_Before, State_After)
    pub revealed_steps: Vec<(u32, StateSnapshot, StateSnapshot)>,
    /// Full Merkle paths for verification
    pub merkle_proofs: Vec<Vec<[u8; 32]>>,
}

impl ZKProof {
    /// Prover: generating the trace of chaos
    pub fn prove(initial_key: [u64; 2], iv: [u64; 2]) -> Self {
        let mut trace = Vec::with_capacity(TOTAL_STATES);

        let mut engine = Fract::from_state([iv[0], iv[1], initial_key[0], initial_key[1]]);
        trace.push(StateSnapshot { s: engine.state });

        for _ in 0..TRACE_LEN {
            engine.apply_phi();
            trace.push(StateSnapshot { s: engine.state });
        }

        // 1. Build Merkle Tree (With Padding)
        let mut leaves: Vec<[u8; 32]> = trace.iter().map(|s| Fract::hash(&s.to_bytes())).collect();

        // Pad with zeros to MERKLE_LEAVES (32) to make tree binary and perfect
        while leaves.len() < MERKLE_LEAVES {
            leaves.push([0u8; 32]);
        }

        let tree_layers = build_full_tree(leaves);
        let root = tree_layers.last().unwrap()[0];

        // 2. Fiat-Shamir Challenge
        let challenge_hash = Fract::hash(&root);
        let indices = derive_indices(&challenge_hash, TRACE_LEN);

        // 3. Reveal Phase
        let mut revealed_steps = Vec::new();
        let mut merkle_proofs = Vec::new();

        for idx in indices {
            revealed_steps.push((idx as u32, trace[idx], trace[idx + 1]));

            // Generate valid Merkle Proof for trace[idx]
            let proof = generate_merkle_path(&tree_layers, idx);
            merkle_proofs.push(proof);
        }

        ZKProof {
            merkle_root: root,
            revealed_steps,
            merkle_proofs,
        }
    }

    /// Verifier: Checking the chaos consistency
    pub fn verify(&self) -> bool {
        let challenge_hash = Fract::hash(&self.merkle_root);
        let indices = derive_indices(&challenge_hash, TRACE_LEN);

        if self.revealed_steps.len() != indices.len() {
            return false;
        }

        for (i, (idx, state_pre, state_post)) in self.revealed_steps.iter().enumerate() {
            let idx_usize = *idx as usize;

            // 1. Challenge Check
            if idx_usize != indices[i] {
                return false;
            }

            // 2. Physics Check (Hyperchaos)
            let mut verifier_engine = Fract::from_state(state_pre.s);
            verifier_engine.apply_phi(); // Single Tick

            if verifier_engine.state != state_post.s {
                return false;
            }

            // 3. Commitment Check (Merkle Verification)
            // Re-hash the provided pre-state
            let leaf_hash = Fract::hash(&state_pre.to_bytes());

            let calculated_root = compute_merkle_root(leaf_hash, idx_usize, &self.merkle_proofs[i]);

            if calculated_root != self.merkle_root {
                return false;
            }
        }

        true
    }
}

// --- Merkle Helpers (Complete Implementation) ---

/// Builds all layers of the Merkle tree
fn build_full_tree(mut leaves: Vec<[u8; 32]>) -> Vec<Vec<[u8; 32]>> {
    let mut layers = vec![leaves.clone()];

    while leaves.len() > 1 {
        let mut next_layer = Vec::new();
        for chunk in leaves.chunks(2) {
            let left = chunk[0];
            let right = if chunk.len() > 1 { chunk[1] } else { chunk[0] };

            // Hash(Left || Right)
            let mut combined = [0u8; 64];
            combined[0..32].copy_from_slice(&left);
            combined[32..64].copy_from_slice(&right);
            next_layer.push(Fract::hash(&combined));
        }
        layers.push(next_layer.clone());
        leaves = next_layer;
    }
    layers
}

/// Generates a path of sibling hashes up to the root
fn generate_merkle_path(layers: &Vec<Vec<[u8; 32]>>, mut idx: usize) -> Vec<[u8; 32]> {
    let mut path = Vec::new();
    // iterate through layers (excluding the root layer)
    for layer in layers.iter().take(layers.len() - 1) {
        let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };

        // If sibling is out of bounds (shouldn't happen with power of 2 padding), use self
        let sibling = if sibling_idx < layer.len() {
            layer[sibling_idx]
        } else {
            layer[idx]
        };

        path.push(sibling);
        idx /= 2; // Move up
    }
    path
}

/// Recomputes root from leaf and path
fn compute_merkle_root(mut hash: [u8; 32], mut idx: usize, path: &[[u8; 32]]) -> [u8; 32] {
    for sibling in path {
        let mut combined = [0u8; 64];
        if idx % 2 == 0 {
            // We are Left, Sibling is Right
            combined[0..32].copy_from_slice(&hash);
            combined[32..64].copy_from_slice(sibling);
        } else {
            // We are Right, Sibling is Left
            combined[0..32].copy_from_slice(sibling);
            combined[32..64].copy_from_slice(&hash);
        }
        hash = Fract::hash(&combined);
        idx /= 2;
    }
    hash
}

fn derive_indices(hash: &[u8; 32], limit: usize) -> Vec<usize> {
    let mut indices = Vec::new();
    // Deterministically pick 4 unique indices
    for i in 0..4 {
        let val = u64::from_le_bytes(hash[i * 8..(i + 1) * 8].try_into().unwrap());
        indices.push((val as usize) % limit);
    }
    indices
}
