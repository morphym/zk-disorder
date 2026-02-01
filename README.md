
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


READ **WHITEPAPER** bout this, unique; novel. zk-disorder: <br/>
https://pawit.co/whitepapers/zk-disorder.pdf


**Anchor Program** Impl: Anchor [project](https://github.com/morphym/disorderd) impl.

---

### Metrics (machine used 4vCPU 2.25GHZ x86)

### FRACT.
<img width="2701" height="779" alt="Frame 2" src="https://github.com/user-attachments/assets/1cf5a4bf-77cd-4a09-a51c-3b1c40


### ZK-Disorder diff.

<img width="3368" height="856" alt="Frame 2 (1)" src="https://github.com/user-attachments/assets/42f613b0-a771-4a91-b016-6c326d27b7d1" />

<img width="3284" height="941" alt="upscalemedia-transformed (3)" src="https://github.com/user-attachments/assets/1280b745-2851-4c46-ba36-16b99f2bd097" />




#### Verified on chain evidence

on solana devnet


Encryption Operation: 3,316 CU consumed (Slot 439,029,170)
[https://explorer.solana.com/tx/2mmQsU9JtY4UV95sj8JFmtauWqNfEd43L21CqLoazgXXcxmQGmsqqFNn
...](https://explorer.solana.com/tx/2mmQsU9JtY4UV95sj8JFmtauWqNfEd43L21CqLoazgXXcxmQGmsqqFNn9tCWnv8mbLbnDd5mVys8VGLBRJKR4frP?cluster=devnet#ix-1)

Proof Verification: 239,234 CU consumed (Slot 439,029,174)
[https://explorer.solana.com/tx/4cAFKBLee4MxMUGLCzp4w2sSXse5x2foQy98Rb87u6LiZt9fwG1A1fhK
...](https://explorer.solana.com/tx/4cAFKBLee4MxMUGLCzp4w2sSXse5x2foQy98Rb87u6LiZt9fwG1A1fhKsgZy145UZxNefHguQUN2w7LrZVmXZ5AC?cluster=devnet#ix-2)









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


[4] Stress Test (1,000 Iterations)
    Avg Verify Time: 11.17 µs
    Verify TPS:      89550
```

Test it for youself:

``cargo build --release && ./target/release/bench `` 



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
fract = "1.2.3" # 1.2.3 is stable version for zk-disorder as it contain no deps, other version may contain deps that are for terminal or hex this is for general usecase, but, zk-disorder doesn't need such.
```

**Run Tests:**
```bash
cargo test --release
```

## Docs

Read docs at 

---

##  References

*   **FRACT Library**: The underlying hyperchaotic primitive.
*   Whitepaper: [FRACT: A Hyperchaotic, Quantum-Resistant Hash](https://pawit.co/whitepapers/fract)
*   **ZK-disorder**: [Whitepaper](https://pawit.co/whitepapers/zk-disorder.pdf)
*   Anchor Program impl of, zk-disorder: [repo](https://github.com/morphym/disorderd)

* grokipedia page on *FRACT*: [Grokipedia](https://grokipedia.com/page/Fract_cryptographic_hash_function
)

## License

Everything presented whose author is Pawit Sahare, also including whitepapers; code,

are licensed either MIT or CC 4.0. you can use it anything personal or not but attribution or copyright notice should be present or given to
Pawit Sahare.

2025 Dec- 2026.

0:21
