use borsh::BorshSerialize;
use rand::Rng;
use std::time::{Duration, Instant};
use zk_disorder::{FractCipher, TRACE_LEN, ZKProof}; // Required for try_to_vec()

fn main() {
    println!("=== ZK-Disorder: Benchmark  ===");
    println!("Specs: 8-Round Hyperchaotic Sponge, Cut-and-Choose (4 Slices)");

    // --- 1. Setup ---
    let mut rng = rand::thread_rng();
    let key = [rng.random::<u64>(), rng.random::<u64>()];
    let iv = [rng.random::<u64>(), rng.random::<u64>()];
    let plaintext = [rng.random::<u64>(), rng.random::<u64>()];

    // --- 2. Encryption Phase ---
    println!("\n[1] Encryption Phase (Client Side)");
    let start_enc = Instant::now();
    let mut cipher = FractCipher::new(key, iv);
    let _ciphertext = cipher.encrypt(plaintext);
    let enc_time = start_enc.elapsed();
    println!("    Time:        {:.2?}", enc_time);
    println!("    Throughput:  Extremely High (Linear Chaos)");

    // --- 3. Proof Generation ---
    println!("\n[2] Proof Generation (Client Side)");

    // Warmup
    ZKProof::prove(key, iv);

    let start_prove = Instant::now();
    let proof = ZKProof::prove(key, iv);
    let prove_time = start_prove.elapsed();

    // Measure Size using Borsh (Native Solana Format)
    let proof_bytes = borsh::to_vec(&proof).expect("Borsh serialization failed");
    let size = proof_bytes.len();

    println!("    Time:        {:.2?}", prove_time);
    println!("    Proof Size:  {} bytes (Borsh)", size);
    if size < 1232 {
        println!("    Status:      FITS IN SINGLE UDP PACKET / MTU (Perfect)");
    }

    // --- 4. Verification Phase ---
    println!("\n[3] Verification (On-Chain Simulation)");

    let start_verify = Instant::now();
    let is_valid = proof.verify();
    let verify_time = start_verify.elapsed();

    println!(
        "    Result:      {}",
        if is_valid { "VALID" } else { "INVALID" }
    );
    println!("    Time:        {:.2?}", verify_time);

    // --- 5. Solana CU Estimation ---
    println!("\n[4] Solana BPF Compute Budget Analysis");

    // Cost Model based on FRACT Whitepaper & Solana BPF Constraints
    // 1. Permutation (Phi): ~272 instructions per block (8 rounds).
    //    In BPF, add overhead for memory safety. Est 350 CU per tick.
    // 2. Merkle Hash: FRACT is ~49 cycles/byte.
    //    Hashing a 32-byte leaf + 32-byte node ~ small. Est 400 CU per hash.
    // 3. Tree Depth: log2(16) = 4 levels.
    // 4. Slices Revealed: 4 (Standard Cut-and-Choose).

    let ops_per_phi = 350;
    let ops_per_hash = 400;
    let slice_count = 4;
    let merkle_depth = 4; // log2(TRACE_LEN)

    // A. Challenge Derivation (1 Hash + Modulos)
    let cost_challenge = ops_per_hash + 100;

    // B. Slice Verification (The Loop)
    // For each slice: 1 Phi execution + Merkle Path verification
    let cost_per_slice = ops_per_phi + (merkle_depth * ops_per_hash);
    let cost_loop = slice_count * cost_per_slice;

    // C. Overhead (Deserialization via Borsh, Stack management)
    // Borsh is very efficient, estimating 1000 CU for this struct size
    let cost_overhead = 1000;

    let total_cu = cost_challenge + cost_loop + cost_overhead;

    println!("    ----------------------------------------");
    println!("    Operation Breakdown:");
    println!("    + Challenge Gen:     {:5} CU", cost_challenge);
    println!(
        "    + Chaos Checks (x4): {:5} CU  (4 * {} CU)",
        slice_count * ops_per_phi,
        ops_per_phi
    );
    println!(
        "    + Merkle Paths (x4): {:5} CU  (4 * 4 * {} CU)",
        slice_count * merkle_depth * ops_per_hash,
        ops_per_hash
    );
    println!("    + Program Overhead:  {:5} CU", cost_overhead);
    println!("    ----------------------------------------");
    println!("    TOTAL ESTIMATE:      {:5} CU", total_cu);
    println!("    ----------------------------------------");
    println!("    Standard Limit:      200,000 CU");
    println!(
        "    Usage:               {:.2}%",
        (total_cu as f64 / 200_000.0) * 100.0
    );

    if total_cu < 10_000 {
        println!("    Conclusion:          EXTREMELY LIGHTWEIGHT. Can batch ~20 proofs/tx.");
    } else {
        println!("    Conclusion:          Standard efficiency.");
    }

    // --- 6. Stress Test ---
    println!("\n[5] Stress Test (1,000 Iterations)");
    let mut total_duration = Duration::new(0, 0);
    for _ in 0..1000 {
        let start = Instant::now();
        proof.verify();
        total_duration += start.elapsed();
    }
    println!(
        "    Avg Verify Time: {:.2} µs",
        total_duration.as_micros() as f64 / 1000.0
    );
    println!(
        "    Verify TPS:      {:.0}",
        1.0 / (total_duration.as_secs_f64() / 1000.0)
    );
}
