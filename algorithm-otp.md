Awesome—here’s a **complete, practical catalog of core algorithms** you can use to build a blockchain-backed OTP, plus **exact primitives, recommended params, and reference pseudocode** for each style.

---

# 0) Goals & Threat Model (what the algorithm must satisfy)

* **Freshness** (expires fast), **Uniqueness** (one-time use), **Unforgeability** (can’t guess), **Bind to user/action**, **Auditability** (on-chain proof), **No secret leakage on-chain**.

---

# 1) Algorithm Families (choose 1–2 that match your UX & gas budget)

## A) HOTP (counter-based)

* **Idea:** `otp = Truncate(HMAC(secret, counter)) mod 10^digits`
* **When to use:** Offline devices, strict step counters, no clock requirement.
* **Chain fit:** Store only `keccak256(otp)` + `expiry`; off-chain increments counter; verify OTP on-chain by preimage (see §4.2).
* **Pros:** No time sync; **Cons:** counter drift handling.

## B) TOTP (time-based, RFC 6238)

* **Idea:** `counter = floor((t - t0)/step)`, then HOTP with this counter.
* **When to use:** Standard 30–60s codes (Google Authenticator style).
* **Chain fit (hybrid):** On-chain stores `keccak256(otp)` + expiry. Off-chain generates TOTP; user submits OTP; contract checks hash + time window.
* **Pros:** Familiar UX; **Cons:** time drift needs windowing (±1 step if needed).

## C) VRF-OTP (Verifiable Random Function)

* **Idea:** OTP = function of a **public, verifiably random** value, e.g. `otp = f(VRF(seed, context))`.
* **Chain fit:** Use Chainlink/Substrate VRF to produce randomness proof; on-chain verifies the VRF proof or trusts oracle; OTP derived deterministically.
* **Pros:** Public verifiability; **Cons:** oracle/VRF fee, latency.

## D) Commit–Reveal OTP

* **Idea:** Issuer commits `C = H(otp || salt)` on-chain. User later reveals `(otp, salt)`, contract checks `H(otp||salt)==C` and expiry.
* **When to use:** Purely on-chain workflows; batching.
* **Pros:** No secrets on-chain, simple; **Cons:** Two-phase UX, frontrunning mitigated by salt.

## E) ZK-OTP (Zero-Knowledge proof of knowledge)

* **Idea:** User proves knowledge of a valid OTP (HOTP/TOTP) **without revealing it** using a SNARK/PLONK circuit.
* **Chain fit:** Contract verifies zk-proof against committed public parameters (hash of secret seed / counter / window).
* **Pros:** Privacy; **Cons:** prover latency + setup complexity.

## F) Oracle-Signed OTP

* **Idea:** OTP or its hash is signed by a whitelisted oracle key: `sig = Sign(issuer_sk, requestId || otpHash || expiry)`.
* **Chain fit:** Contract checks signature; optionally still stores hash+expiry.
* **Pros:** Cheap single tx set-and-verify; **Cons:** trust in oracle keys (use multisig/MPC).

## G) Merkle-Batch OTP

* **Idea:** Many `(requestId, otpHash, expiry)` entries in a tree; post only `merkleRoot`. User provides Merkle proof at verify.
* **When to use:** High volume issuance, L1 gas savings.
* **Pros:** Amortized cost; **Cons:** Proof size/complexity.

## H) Threshold/MPC-Verified OTP

* **Idea:** Multiple nodes verify HOTP/TOTP and co-sign a quorum attestation; contract checks threshold signature.
* **Pros:** No single issuer; **Cons:** infra complexity.

---

# 2) Crypto Primitives (production-grade picks)

| Purpose        | Recommended                                                                             |
| -------------- | --------------------------------------------------------------------------------------- |
| **Hash**       | Keccak-256 (native on EVM), SHA-256 (has precompile); use **domain separation** strings |
| **HMAC**       | HMAC-SHA256 (or SHA1 for RFC compat; prefer SHA256)                                     |
| **KDF**        | HKDF-SHA256 for deriving per-user/per-action seeds from a master key                    |
| **CSPRNG**     | OS RNG + DRBG (e.g., ChaCha20-DRBG) for OTP & salts                                     |
| **AEAD**       | AES-GCM or ChaCha20-Poly1305 for seed storage & transport                               |
| **Sigs**       | ECDSA/secp256k1 (EVM), Ed25519 off-chain; threshold BLS for MPC                         |
| **Rate-limit** | Token Bucket or Leaky Bucket (see §5)                                                   |
| **ZK**         | PLONK/Groth16 circuit with Poseidon/Keccak gadget                                       |

---

# 3) Recommended Parameters (sane defaults)

| Setting                     | Value                                   |
| --------------------------- | --------------------------------------- |
| OTP digits                  | 6 (or 8 for higher security)            |
| TOTP step                   | 60s (window ±0 optional)                |
| HOTP counter                | 64-bit, monotonic per user              |
| Hash domain tag             | `"OTP:v1:<chainId>:<app>"`              |
| Expiry                      | ≤ 60s on-chain, enforce server-side too |
| Max attempts                | 3 per requestId                         |
| Salt length (commit-reveal) | 16–32 bytes random                      |
| KDF context                 | `HKDF(info="otp/<purpose>/<user>")`     |

---

# 4) Reference Pseudocode (drop-in blueprints)

## 4.1 TOTP/HOTP Generation (off-chain)

```python
# HMAC-SHA256 TOTP/HOTP, returns 6-digit string (RFC-style dynamic truncation)
def hotp(secret_bytes, counter, digits=6):
    mac = HMAC_SHA256(key=secret_bytes, msg=to_be64(counter))
    offset = mac[-1] & 0x0F
    code = ((mac[offset]   & 0x7f) << 24) | \
           ((mac[offset+1] & 0xff) << 16) | \
           ((mac[offset+2] & 0xff) <<  8) | \
           ((mac[offset+3] & 0xff))
    return str(code % (10 ** digits)).zfill(digits)

def totp(secret_bytes, unix_time, step=60, digits=6, t0=0):
    counter = (unix_time - t0) // step
    return hotp(secret_bytes, counter, digits)
```

**On-chain usage (hybrid):**

* Issuer stores `otpHash = keccak256(otp)` + `expiry`.
* User submits `otp` → contract compares hash + time.

## 4.2 Commit–Reveal with Salt (pure on-chain)

```text
commit phase:  C = keccak256( DOMAIN || requestId || otp || salt )
verify phase:  recompute and compare; then mark used=true and check expiry
```

* **Why salt?** Prevents an attacker from dictionary-searching the 6-digit space using the on-chain commitment.

## 4.3 Oracle-Signed OTP (cheap verify)

```text
issuer signs: sig = Sign(sk, keccak256(DOMAIN || requestId || otpHash || expiry))
contract checks: ecrecover(sig) ∈ allowlist && now <= expiry && keccak256(otp) == otpHash
```

## 4.4 Merkle-Batch OTP

* Build leaves: `Li = keccak256(DOMAIN || requestId_i || otpHash_i || expiry_i)`
* Compute `root = Merkle(L1..Ln)`
* Post `root` once; verify with `{leaf, proof}` + regular OTP check.

## 4.5 ZK-OTP (idea sketch)

* Circuit public inputs: `requestId, expiry, hashSeed, now`
* Witness: `seed, counter/time, otp`
* Constraints: `otp == HOTP/TOTP(seed, counter/time)` and `keccak256(otp) == otpHash` and `now ≤ expiry`
* Contract: verifies SNARK proof; sets `used=true`.

---

# 5) Rate-Limiting Algorithms (to make guessing infeasible)

## Token Bucket (per user / IP / device)

* **State:** `tokens`, `last_refill_ts`
* **Refill:** `tokens = min(capacity, tokens + rate * Δt)`
* **Consume:** if `tokens ≥ 1`, then `tokens -= 1` else reject (429)
* **Suggested:** capacity=3, rate=1/60s (≈ 1 try per 60s, burst 3)

## Attempt Counter per `requestId`

* **State:** `tries[requestId]` with TTL=120s; if `>3`, block verify.

---

# 6) Binding & Anti-Replay (don’t let codes bleed across users)

* **Bind** `requestId = keccak256(userId || random32)` and store `(requestId → userIdHash)` off-chain for audit.
* **One-time use:** `used=true` on success; reject replays.
* **Time guard:** require `block.timestamp ≤ expiry`.

---

# 7) Gas-Aware On-Chain Data Layout

* Pack `expiry` into `uint64`, `used` into a single `bool` in the same slot as a `bytes32 hash` (2 storage slots).
* Expose `view` for entries, and emit `OtpSet(requestId, expiry)` + `OtpVerified(requestId, by)` events for audit.
* Consider **Merkle-batch** if issuing many OTPs per block.

---

# 8) Key Derivation Patterns (per-user/per-purpose)

```text
master_key  --HKDF(info="otp/seed/<app>")--> issuer_seed
issuer_seed --HKDF(info="otp/user/<userId>")--> user_seed
user_seed   --HKDF(info="otp/action/<scope>")--> action_seed
```

* This keeps isolation across users and actions even if one seed leaks.

---

# 9) Attack Surfaces & Algorithmic Mitigations

| Attack               | Mitigation (algorithmic)                                      |
| -------------------- | ------------------------------------------------------------- |
| Brute force          | Token bucket + attempt counter; 6–8 digits; short TTL         |
| Replay               | `used=true` + expiry check                                    |
| Mix-up (wrong user)  | `requestId` derived from `userId`; store binding              |
| Commit rainbow       | Salt in commit; domain separation                             |
| Front-running reveal | Salt + user-bound requestId; or ZK-OTP                        |
| Oracle key theft     | Threshold/MPC or multisig rotation; issuer allowlist on-chain |
| Time drift (TOTP)    | Keep small window (±1 step max) or switch to HOTP             |

---

# 10) “Pick Two” Playbook (what I recommend you actually ship)

**Default (simple & strong):**

1. **TOTP/HOTP off-chain → `keccak256(otp)` on-chain** (Hybrid).
2. **Token bucket + attempt counter**.
3. **Optional:** switch to **Merkle-batch** if issuance volume grows.

**Privacy-sensitive upgrade:**

* Replace plaintext OTP reveal with **ZK-OTP** (keeps OTP secret forever).

**Trust-minimizing upgrade:**

* Add **oracle-signed** or **threshold-signed** attestations and check them on-chain.

---

# 11) Tiny Solidity verify helper (constant-time-ish compare)

```solidity
function _hashOtp(string memory otp) internal pure returns (bytes32) {
    // domain separation: avoids cross-app hash reuse
    return keccak256(abi.encodePacked("OTP:v1", otp));
}
```

*Store `_hashOtp(otp)` as `otpHash`.*

---

# 12) Ready-to-use Parameter Block (drop in config)

```json
{
  "otp": { "digits": 6, "scheme": "TOTP", "step": 60, "expiry": 60 },
  "rate_limit": { "burst": 3, "refill_per_sec": 0.0167 },
  "attempts": 3,
  "hash_domain": "OTP:v1:<chainId>:<app>",
  "kdf": "HKDF-SHA256",
  "aead": "AES-256-GCM",
  "vrf": { "enabled": false },
  "zk": { "enabled": false }
}
```

---

If you want, I can:

* generate a **CSV mapping** (Algorithm → Primitives → Params → Gas impact → Security properties → Tests),
* add a **Foundry test** with HOTP/TOTP vectors and the commit-reveal path,
* or wire a **Merkle-batch** variant for high throughput.
