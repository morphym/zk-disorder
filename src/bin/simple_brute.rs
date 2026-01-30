use rand::Rng;
use std::time::Instant;
use zk_disorder::FractCipher;

fn main() {
    println!("=== Classical Hyperchaotic Cipher Attack (Brute Force) ===");
    println!("Target: Recover 128-bit Secret Key (Capacity) from 128-bit Rate Output");

    println!(
        "NOTE: This would fail as it have already failure, classically breaking it is impossible, this serve as demo for classical cryptoanalysis."
    );

    println!(
        "This is previous result:
    {{
    [Target Configuration]
      IV (Public):     1111111111111111 2222222222222222
      Plaintext:       aabbccddeeff0011 2233445566778899
      Secret Key:      ???????????????? ????????????????
      Ciphertext:      27b23492553ed5d3 57e0eb8daf93e1c4

    [Attack] Launching 50000000 brute-force attempts...

    [Result]
      Status:    FAILED
      Attempts:  50000000
      Time:      124.37s
      Speed:     0.40 Million keys/sec
      Est. Time: 2.68e25 Years to exhaust key space
     Expected: The non-linear chaotic mixing prevents algebraic shortcuts, classically it would fail wth possibility 2^256 needed to break- impossible. ]
    }}
     "
    );

    // --- 1. Setup the Target (The "Real" User) ---
    // Secret Key (The Capacity of the Sponge)
    let real_key = [0xDEADBEEFCAFEBABE, 0x123456789ABCDEF0];

    // Public IV (The Initial Rate)
    let iv = [0x1111111111111111, 0x2222222222222222];

    // Known Plaintext (128 bits)
    let plaintext = [0xAA_BB_CC_DD_EE_FF_00_11, 0x22_33_44_55_66_77_88_99];

    println!("\n[Target Configuration]");
    println!("  IV (Public):     {:016x} {:016x}", iv[0], iv[1]);
    println!(
        "  Plaintext:       {:016x} {:016x}",
        plaintext[0], plaintext[1]
    );
    println!("  Secret Key:      ???????????????? ????????????????");

    // Encrypt to get the Target Ciphertext
    let mut target_cipher = FractCipher::new(real_key, iv);
    let target_ciphertext = target_cipher.encrypt(plaintext);

    println!(
        "  Ciphertext:      {:016x} {:016x}",
        target_ciphertext[0], target_ciphertext[1]
    );

    // --- 2. The Attack ---
    let attempts: u64 = 50_000_000; // 50 Million Attempts
    println!("Launching 50M attemps");
    println!("\n[Attack] Launching {} brute-force attempts...", attempts);

    let start = Instant::now();
    let mut rng = rand::thread_rng();
    let mut success = false;

    // Hot loop optimization
    for i in 0..attempts {
        // 1. Generate Candidate Key (Random Guess)
        let guess_key = [rng.random::<u64>(), rng.random::<u64>()];

        // 2. Initialize Cipher with Guess
        let mut attacker = FractCipher::new(guess_key, iv);

        // 3. Encrypt Known Plaintext
        let output = attacker.encrypt(plaintext);

        // 4. Compare with Target Ciphertext
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

    // --- 3. Results Analysis ---
    println!("\n[Result]");
    if !success {
        println!("  Status:    FAILED");
        println!("  Attempts:  {}", attempts);
        println!("  Time:      {:.2?}", duration);

        let rate = attempts as f64 / duration.as_secs_f64();
        println!("  Speed:     {:.2} Million keys/sec", rate / 1_000_000.0);

        // Security Estimation
        // Total Keys = 2^128 ~= 3.4e38
        let total_keys: f64 = 3.4028e38;
        let seconds_in_year = 31_536_000.0;
        let years_to_crack = total_keys / rate / seconds_in_year;

        println!(
            "  Est. Time: {:.2e} Years to exhaust key space",
            years_to_crack
        );
        println!(
            "  Expected: The non-linear chaotic mixing prevents algebraic shortcuts, classically it would fail wth possibility 2^256 needed to break- impossible."
        );
    }
}
