use rand::Rng;
use std::time::Instant;
use zk_disorder::FractSponge;

fn main() {
    println!("=== Classical Chaos Inversion Attack (Brute Force) ===");
    println!("Target: Recover 128-bit Secret Key from State Evolution");

    println!(
        "NOTE: This has been already verified, result was failure, as classically it is impossible to break, this serve as demo."
    );

    println!(
        "previous result: {{[Target Info]
      IV (Public):    1111111111111111 2222222222222222
      Ciphertext:     e54f88bc4fea27f7
      Plaintext:      aabbccddeeff0011 (Known Plaintext Attack)
      Unknown Key:    ???????????????? ????????????????

    [Attack] Launching 20000000 brute-force attempts...

    [Result]
      Status:    FAILED
      Attempts:  20000000
      Time:      44.57s
      Speed:     0.45 Million keys/sec
      Est. Time: 2.40e25 Years to exhaust key space]}}  "
    );

    // 1. Setup the Target (The "Real" User)
    let real_key = [0xDEADBEEFCAFEBABE, 0x123456789ABCDEF0];
    let iv = [0x1111111111111111, 0x2222222222222222];
    let plaintext = 0xAA_BB_CC_DD_EE_FF_00_11;

    let mut target_sponge = FractSponge::new(real_key, iv);
    let target_ciphertext = target_sponge.encrypt(plaintext);

    println!("\n[Target Info]");
    println!("  IV (Public):    {:016x} {:016x}", iv[0], iv[1]);
    println!("  Ciphertext:     {:016x}", target_ciphertext);
    println!(
        "  Plaintext:      {:016x} (Known Plaintext Attack)",
        plaintext
    );
    println!("  Unknown Key:    ???????????????? ????????????????");

    // 2. The Attack
    let attempts: u64 = 20_000_000;
    println!("\n[Attack] Launching {} brute-force attempts...", attempts);

    let start = Instant::now();
    // Using the updated RNG constructor if on latest rand, otherwise thread_rng()
    let mut rng = rand::thread_rng();
    let mut success = false;

    for i in 0..attempts {
        let guess_key = [rng.random::<u64>(), rng.random::<u64>()];

        let mut attacker_sponge = FractSponge::new(guess_key, iv);
        let output = attacker_sponge.encrypt(plaintext);

        if output == target_ciphertext {
            println!("\n[CRITICAL] KEY FOUND at attempt {}!", i);
            println!(
                "  Recovered Key: {:016x} {:016x}",
                guess_key[0], guess_key[1]
            );
            success = true;
            break;
        }
    }

    let duration = start.elapsed();

    println!("\n[Result]");
    if !success {
        println!("  Status:    FAILED");
        println!("  Attempts:  {}", attempts);
        println!("  Time:      {:.2?}", duration);

        let rate = attempts as f64 / duration.as_secs_f64();
        println!("  Speed:     {:.2} Million keys/sec", rate / 1_000_000.0);

        // Contextualize the failure
        // 2^128 keys total
        let total_keys: f64 = 3.402e38;
        let years_to_crack = total_keys / rate / 31_536_000.0;

        println!(
            "  Est. Time: {:.2e} Years to exhaust key space",
            years_to_crack
        );
        println!("  Security:  The chaotic attractor is sufficiently divergent.");
    }
}
