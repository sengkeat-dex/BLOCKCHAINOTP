Extend Coverage
                                                                                 
  - contracts/test/OtpVerifier.t.sol:22 already exercises happy path; add fuzz   
    cases for near-expiry, mixed uppercase/lowercase OTP strings, and pause      
    toggles mid-flight, then gate with forge coverage for branch stats.          
  - tests/otp_contract_integration.rs:77 mirrors on-chain behaviour; duplica• Extend Coverage

  - contracts/test/OtpVerifier.t.sol:22 already exercises happy path; add fuzz
    cases for near-expiry, mixed uppercase/lowercase OTP strings, and pause 
    toggles mid-flight, then gate with forge coverage for branch stats.     
  - tests/otp_contract_integration.rs:77 mirrors on-chain behaviour; duplicate
    the flow under #[tokio::test] using a mocked rate-limit window or multi-user
    scenario so AttemptsLocked and cleanup paths are all asserted off-chain.
  - frontend/src/utils/mod.rs:33 now has unit coverage; add component-level tests
    (e.g., request/verify forms) using wasm-bindgen-test to exercise DOM events
    and API stubs.

  Locate Off-Chain Handling

  - Rust contract client: crates/otp-contract/src/lib.rs:179 defines the OtpChain                                                                       
    trait; its verify method now returns the bool that upstream callers must
    check.
  - Backend integration: tests/otp_contract_integration.rs:77 shows the simulator
    consuming that return value; use it as the canonical example for other  
    services.
  - Docs/code references: connection.md:119 and layout.md:55 still describe 
    verify(requestId, otp); update them if you want the new semantics spelled out
    for integrators.