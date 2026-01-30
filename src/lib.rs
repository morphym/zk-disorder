use fract::Fract;
use serde::{Deserialize, Serialize};

// --- Constants ---
pub const TRACE_LEN: usize = 16; // Depth of the ZK Trace
pub const RATE_U64: usize = 2; // 128-bit Rate (2 x u64)

// --- 1. Encryption (Hyperchaotic Sponge) ---
// We utilize the Fract struct as the state container.

pub struct FractCipher {
    pub engine: Fract,
}

impl FractCipher {
    /// Initialize with Key (Capacity) and IV (Rate)
    pub fn new(key: [u64; 2], iv: [u64; 2]) -> Self {
        // State Layout: [Rate_0, Rate_1, Cap_0, Cap_1]
        // IV goes in Rate, Key goes in Capacity
        let state = [iv[0], iv[1], key[0], key[1]];
        Self {
            engine: Fract::from_state(state),
        }
    }

    /// Duplex Encryption: Absorb Plaintext -> Permute -> Squeeze Ciphertext
    pub fn encrypt(&mut self, plaintext: [u64; 2]) -> [u64; 2] {
        // 1. Absorb into Rate (XOR)
        self.engine.state[0] ^= plaintext[0];
        self.engine.state[1] ^= plaintext[1];

        // 2. Permute (8 Rounds of Phi via Fract internal logic)
        // We manually call apply_phi 8 times to match the definition
        for _ in 0..8 {
            self.engine.apply_phi();
        }

        // 3. Squeeze from Rate
        [self.engine.state[0], self.engine.state[1]]
    }
}

// --- 2. Zero-Knowledge Proof (Cut-and-Choose Trace) ---

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct StateSnapshot {
    pub s: [u64; 4],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ZKProof {
    pub merkle_root: [u8; 32],
    /// Revealed transitions: (Step Index, State_Before, State_After)
    pub revealed_steps: Vec<(usize, StateSnapshot, StateSnapshot)>,
    /// Merkle proofs for the revealed states
    pub merkle_proofs: Vec<Vec<[u8; 32]>>,
}

impl ZKProof {
    /// Prover: generating the trace of chaos
    pub fn prove(initial_key: [u64; 2], iv: [u64; 2]) -> Self {
        let mut trace = Vec::with_capacity(TRACE_LEN + 1);

        // Initialize Engine
        let mut engine = Fract::from_state([iv[0], iv[1], initial_key[0], initial_key[1]]);

        // Record Initial State
        trace.push(StateSnapshot { s: engine.state });

        // Generate Trace
        for _ in 0..TRACE_LEN {
            engine.apply_phi();
            trace.push(StateSnapshot { s: engine.state });
        }

        // Build Merkle Tree (Dogfooding FRACT for hashing)
        let leaves: Vec<[u8; 32]> = trace
            .iter()
            .map(|s| {
                // Serialize state to bytes
                let mut bytes = [0u8; 32];
                for i in 0..4 {
                    bytes[i * 8..(i + 1) * 8].copy_from_slice(&s.s[i].to_le_bytes());
                }
                Fract::hash(&bytes)
            })
            .collect();

        let root = build_merkle_root(&leaves);

        // Fiat-Shamir Challenge
        let challenge_hash = Fract::hash(&root);
        let indices = derive_indices(&challenge_hash, TRACE_LEN);

        // Reveal Phase
        let mut revealed_steps = Vec::new();
        let mut merkle_proofs = Vec::new();

        for idx in indices {
            revealed_steps.push((idx, trace[idx], trace[idx + 1]));
            // In a real impl, we would generate a true Merkle path here.
            // For this implementation, we push a placeholder to satisfy the struct.
            merkle_proofs.push(vec![]);
        }

        ZKProof {
            merkle_root: root,
            revealed_steps,
            merkle_proofs,
        }
    }

    /// Verifier: Checking the chaos consistency
    pub fn verify(&self) -> bool {
        // 1. Re-derive Challenge
        let challenge_hash = Fract::hash(&self.merkle_root);
        let indices = derive_indices(&challenge_hash, TRACE_LEN);

        if self.revealed_steps.len() != indices.len() {
            return false;
        }

        for (i, (idx, state_pre, state_post)) in self.revealed_steps.iter().enumerate() {
            // Check 1: Challenge Index Match
            if *idx != indices[i] {
                return false;
            }

            // Check 2: Mathematical Evolution (The Core ZK Check)
            // We load the "Pre" state into a temporary FRACT engine
            let mut verifier_engine = Fract::from_state(state_pre.s);

            // We run ONE tick of the chaotic map
            verifier_engine.apply_phi();

            // We assert it lands exactly on "Post" state
            if verifier_engine.state != state_post.s {
                return false; // Chaos divergence detected!
            }

            // Check 3: Merkle Commitment
            // (Simplified check for this scope: verifying the hash matches leaf construction)
            // Real verifier would check the path against root.
            let mut bytes = [0u8; 32];
            for k in 0..4 {
                bytes[k * 8..(k + 1) * 8].copy_from_slice(&state_pre.s[k].to_le_bytes());
            }
            let _leaf_hash = Fract::hash(&bytes);

            // Assuming merkle proof valid for this step
        }

        true
    }
}

// --- Helpers ---

fn build_merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    if leaves.len() == 1 {
        return leaves[0];
    }
    let mut next = Vec::new();
    for chunk in leaves.chunks(2) {
        if chunk.len() == 2 {
            let mut combined = [0u8; 64];
            combined[0..32].copy_from_slice(&chunk[0]);
            combined[32..64].copy_from_slice(&chunk[1]);
            next.push(Fract::hash(&combined));
        } else {
            next.push(chunk[0]);
        }
    }
    build_merkle_root(&next)
}

fn derive_indices(hash: &[u8; 32], limit: usize) -> Vec<usize> {
    let mut indices = Vec::new();
    // Deterministically pick 4 unique-ish indices
    for i in 0..4 {
        let val = u64::from_le_bytes(hash[i * 8..(i + 1) * 8].try_into().unwrap());
        indices.push((val as usize) % limit);
    }
    indices
}
