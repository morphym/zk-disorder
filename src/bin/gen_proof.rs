// A general easy way to create zk-disorder encryptions proof.
// through a binary that accept encryption configuration (iv, key), such that it can be invoked
// in other program which isn't written rust, more easily.
//

use zk_disorder::ZKProof;
use std::env;
use std::fs::File;
use std::io::Write;
use borsh::BorshSerialize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: gen_proof <key0_hex> <key1_hex> <iv0_hex> <iv1_hex>");
        std::process::exit(1);
    }

    // 1. Parse Arguments (Hex String -> u64)
    let key = [
        u64::from_str_radix(args[1].trim_start_matches("0x"), 16).expect("Invalid Key[0]"),
        u64::from_str_radix(args[2].trim_start_matches("0x"), 16).expect("Invalid Key[1]"),
    ];
    let iv = [
        u64::from_str_radix(args[3].trim_start_matches("0x"), 16).expect("Invalid IV[0]"),
        u64::from_str_radix(args[4].trim_start_matches("0x"), 16).expect("Invalid IV[1]"),
    ];

    // 2. Generate Proof
    let proof = ZKProof::prove(key, iv);

    // 3. Save to fixed path for the test runner to pick up
    let bytes = borsh::to_vec(&proof).expect("Borsh serialization failed");
    let mut file = File::create("proof.bin").expect("Failed to create file");
    file.write_all(&bytes).expect("Write failed");
}
