
#  ZK-Disorder
### Hyperchaotic Zero-Knowledge Privacy Protocol

**ZK-Disorder** is a novel privacy primitive that abandons heavy arithmetic circuits (SNARKs/STARKs) in favor of **Chaotic Dynamical Systems**. 
By utilizing the `fract` hyperchaotic hash function, it achieves encryption and zero-knowledge proofs that are **50x orders of magnitude faster** and lighter than elliptic-curve alternatives; 

providing classically impossible to break while current\nisq era quantum machine are also impossible, one of reason is because of 'fract' new highly novel design that leverage hyperchaotic dynamical system, leaving no vulunerability or leverage to current quantum computers.

> **Status**: Experimental / Research Grade.
> **Primitive**: [FRACT-256](https://crates.io/crates/fract) (Hyperchaotic Sponge)

-> *FRACT*: [Github repo](https://github.com/morphym/) <br/>
[Whitepaper](https://pawit.co/whitepapers/fract)
<br> </br>

you can also read comphresive information on grokipedia; including it's post-quantum security, everythin else more easily: 

[Grokipedia;](https://grokipedia.com/page/Fract_cryptographic_hash_function)


READ WHITEPAPER bout this, unique; novel. zk-disorder: <br/>
https://pawit.co/whitepapers/zk-disorder.pdf

---

### Metrics (machine used 4vCPU 2.25GHZ x86)

### FRACT.
<img width="2701" height="779" alt="Frame 2" src="https://github.com/user-attachments/assets/1cf5a4bf-77cd-4a09-a51c-3b1c40587bf1" />


### ZK-Disorder difference.

<img width="2854" height="1902" alt="upscalemedia-transformed (2)" src="https://github.com/user-attachments/assets/1ef87000-048a-4463-a351-0411e849b4cc" />


### ZK-disorder solana perf.

<img width="5464" height="3072" alt="image" src="https://github.com/user-attachments/assets/90b80b73-d5fa-4e80-95bd-c77f98a54efe" />




**Encryption & Proof Gen**
```text
princee@princee:~/projects/codename/sylix/sylix/zk_disorder$ ./target/release/bench
=== ZK-Disorder: Benchmark  ===
Specs: 8-Round Hyperchaotic Sponge, Cut-and-Choose (4 Slices)

[1] Encryption Phase (Client Side)
    Time:        747.00ns
    Throughput:  Extremely High (Linear Chaos)

[2] Proof Generation (Client Side)
    Time:        47.21µs
    Proof Size:  968 bytes (Borsh)
    Status:      FITS IN SINGLE UDP PACKET / MTU (Perfect)

[3] Verification (On-Chain Simulation)
    Result:      VALID
    Time:        25.16µs

[4] Solana BPF Compute Budget Analysis
    ----------------------------------------
    Operation Breakdown:
    + Challenge Gen:       500 CU
    + Chaos Checks (x4):  1400 CU  (4 * 350 CU)
    + Merkle Paths (x4):  6400 CU  (4 * 4 * 400 CU)
    + Program Overhead:   1000 CU
    ----------------------------------------
    TOTAL ESTIMATE:       9300 CU
    ----------------------------------------
    Standard Limit:      200,000 CU
    Usage:               4.65%
    Conclusion:          EXTREMELY LIGHTWEIGHT. Can batch ~20 proofs/tx.

[5] Stress Test (1,000 Iterations)
    Avg Verify Time: 11.17 µs
    Verify TPS:      89550
```

Test it for youself:

``cargo build --release && ./target/release/bench `` 

adding rust benches would limit it only to rust users, it was also redudant;


## The Core Mechanic: Chaos as a Commitment

Traditional ZK proves knowledge of a path through an algebraic circuit. ZK-FRACT proves knowledge of a **trajectory through a chaotic attractor**.

### 1. The Lattice (Encryption)
The internal state is a 256-bit lattice evolving under the **Hybrid Logistic-Tent Map ($\Phi$)**- This is through 'FRACT' 
*   **Encryption**: A Duplex Sponge. The secret key is the *Capacity*. The message is absorbed into the *Rate*.
*   **Security**: Mathematical chaos ensures that without the initial capacity, predicting the trajectory (decrypting) requires inverting a system with 4 positive Lyapunov exponents.

### 2. The Trace (Proof)
Instead of building a R1CS constraint system, the Prover records the "physics" of the encryption:
1.  **Commitment**: The Prover Merkle-hashes the entire execution trace (state at every round).
2.  **Challenge**: The Verifier (Fiat-Shamir) asks to see random slices of time (e.g., Round 3 to 4).
3.  **Response**: The Prover reveals *only* those specific state transitions.

### 3. The Check (Verification)
The Verifier runs the chaotic map $\Phi$ on `State[i]` and asserts it equals `State[i+1]`.
*   If the physics holds, the trace is valid.
*   If the Merkle proofs hold, the trace was committed before the challenge.
*   **Result**: Valid proof of key ownership without revealing the key.

---

## Performance Benchmarks

**NOTE**: Always run benchmarks in `--release`. Debug builds include overflow checks and lack vectorization, skewing chaotic map performance by 10-100x.

```bash
cargo run --bin bench --release
```



---

## Security Analysis

Security relies on the hardness of the **Chaotic Inversion Problem**. Unlike RSA (factoring) or EC (discrete log), breaking ZK-FRACT requires finding a preimage in a non-linear system that expands entropy exponentially.

### Brute Force Simulation
We attempted to recover a 128-bit key from a known plaintext/ciphertext pair.

```bash
cargo run --bin simple_brute --release
```

**Results:**
```text
[Attack] Launching 50,000,000 brute-force attempts...
Status:    FAILED
Time:      124.37s
Speed:     0.40 Million keys/sec
Est. Time: 2.68e25 Years to exhaust key space
```

**Classical Analysis**: Recovering the key requires solving a system of coupled modular equations of degree $\ge 2$. The Jacobian is non-invertible in $\mathbb{Z}_{2^{64}}$, rendering algebraic attacks (Gröbner basis) **computationally infeasible** ($> 2^{192}$ ops).

---

## Usage

Add it directly (prefered):

``cargo add zk-disorder``

**Add manually Dependency:**
```toml
[dependencies]
zk-disorder = "0.1.1" # check latest version.
fract = "0.1.2" # 1.2 version is only suited for zk-disorder  
```

**Run Tests:**
```bash
cargo test --release
```

---

##  References

*   **FRACT Library**: The underlying hyperchaotic primitive.
*   Whitepaper: [FRACT: A Hyperchaotic, Quantum-Resistant Hash](https://pawit.co/whitepapers/fract)
*   **ZK-disorder**: [Whitepaper](https://pawit.co/whitepapers/zk-disorder.pdf)

* Intensive information on *FRACT*: [Grokipedia](https://grokipedia.com/page/Fract_cryptographic_hash_function
)

## License

Everything presented whose author is Pawit Sahare, also including whitepapers; code,

are licensed either MIT or CC 4.0. you can use it anything personal or not but attribution or copyright notice should be present or given to
Pawit Sahare.

2025 Dec- 2026.

0:21
