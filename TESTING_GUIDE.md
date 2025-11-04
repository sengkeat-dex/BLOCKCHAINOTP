# Blockchain OTP Testing Guide

This guide explains how to run tests for the Blockchain OTP system to verify all security layers and protection mechanisms.

## Prerequisites

1. Install Foundry: https://getfoundry.sh/
2. Navigate to the contracts directory

## Running Tests

### Run All Tests

```bash
forge test
```

This will run all test files in the [test](file:///c%3A/Users/USER/Documents/blockchainotp/contracts/test) directory, including:
- Unit tests for the OtpVerifier contract
- Security tests for various attack vectors
- Deployment tests for the enhanced deployment script
- Comprehensive integration tests

### Run Specific Test Files

```bash
# Run only the OtpVerifier contract tests
forge test --match-path test/OtpVerifier.t.sol

# Run only the security layer tests
forge test --match-path test/SecurityLayer.t.sol

# Run only the deployment tests
forge test --match-path test/DeployOtpVerifier.t.sol

# Run only the comprehensive tests
forge test --match-path test/ComprehensiveTest.t.sol
```

### Run Tests with Verbose Output

```bash
# Run tests with detailed output
forge test -vvv

# Run tests with the most verbose output
forge test -vvvv
```

### Run Tests with Gas Reports

```bash
# Run tests with gas usage reports
forge test --gas-report
```

## Test Categories

### 1. Unit Tests ([OtpVerifier.t.sol](file:///c%3A/Users/USER/Documents/blockchainotp/contracts/test/OtpVerifier.t.sol))

These tests verify the basic functionality of the OtpVerifier contract:

- Role-based access control
- OTP creation and verification
- Attempt limiting
- Expiration handling
- Cleanup functionality
- Pause mechanism
- Role rotation

### 2. Security Layer Tests ([SecurityLayer.t.sol](file:///c%3A/Users/USER/Documents/blockchainotp/contracts/test/SecurityLayer.t.sol))

These tests verify protection against common attack vectors:

- Brute force attack prevention
- Replay attack prevention
- Expiration attack prevention
- Unauthorized access prevention
- Emergency pause functionality
- Unauthorized admin functions
- Cleanup security
- Hash collision resistance
- Zero hash prevention

### 3. Deployment Tests ([DeployOtpVerifier.t.sol](file:///c%3A/Users/USER/Documents/blockchainotp/contracts/test/DeployOtpVerifier.t.sol))

These tests verify the enhanced deployment script:

- Successful deployment with valid parameters
- Failure handling with invalid parameters
- Security checks during deployment
- Basic functionality testing after deployment
- Event emission verification

### 4. Comprehensive Tests ([ComprehensiveTest.t.sol](file:///c%3A/Users/USER/Documents/blockchainotp/contracts/test/ComprehensiveTest.t.sol))

These tests verify end-to-end functionality:

- Complete deployment flow
- Full OTP lifecycle
- Role rotation
- Emergency procedures
- Edge cases and boundary conditions
- Concurrent requests
- Expired entry cleanup

## Test Coverage

To generate a coverage report:

```bash
forge coverage
```

To generate an HTML coverage report:

```bash
forge coverage --report debug
```

## Continuous Integration

The test suite is designed to be run in CI/CD pipelines. All tests should pass before any deployment to ensure the security layers are functioning correctly.

## Test Descriptions

### Security Test Details

1. **Brute Force Attack Prevention**
   - Verifies that after 3 failed attempts, an OTP is locked
   - Ensures no information is leaked about correct digits

2. **Replay Attack Prevention**
   - Ensures OTPs can only be used once
   - Verifies that replayed OTPs are rejected

3. **Expiration Attack Prevention**
   - Ensures expired OTPs cannot be verified
   - Verifies proper expiration time handling

4. **Unauthorized Access Prevention**
   - Ensures only authorized roles can perform specific actions
   - Verifies proper error messages for unauthorized access

5. **Emergency Pause Functionality**
   - Tests that all operations can be paused in an emergency
   - Verifies that operations resume correctly after unpausing

6. **Unauthorized Admin Functions**
   - Ensures only admins can perform administrative functions
   - Verifies proper error handling for unauthorized admin actions

7. **Cleanup Security**
   - Ensures only authorized roles can perform cleanup
   - Verifies that active entries cannot be cleaned up
   - Tests cleanup of expired and used entries

8. **Hash Collision Resistance**
   - Ensures different request IDs with same OTPs work independently
   - Verifies proper hash handling

9. **Zero Hash Prevention**
   - Ensures zero hashes cannot be stored
   - Verifies proper hash validation

### Deployment Test Details

1. **Successful Deployment**
   - Verifies contract deployment with valid parameters
   - Ensures proper initialization of contract state

2. **Address Validation**
   - Tests that zero addresses are rejected
   - Verifies proper error handling for invalid addresses

3. **Role Separation**
   - Ensures issuer and admin must be different addresses
   - Tests proper error handling for same addresses

4. **Post-Deployment Verification**
   - Verifies contract parameters are set correctly
   - Tests basic functionality after deployment

5. **Event Emission**
   - Verifies that proper events are emitted during deployment
   - Ensures security check events are emitted

### Comprehensive Test Details

1. **Complete Deployment Flow**
   - Tests the entire deployment process
   - Verifies all contract parameters

2. **Full OTP Lifecycle**
   - Tests creating, verifying, and cleaning up an OTP
   - Verifies proper state transitions

3. **Role Rotation**
   - Tests changing issuer and admin addresses
   - Verifies proper permission handling after rotation

4. **Emergency Procedures**
   - Tests pausing and unpausing the contract
   - Verifies proper operation after emergency procedures

5. **Edge Cases**
   - Tests minimal and maximal expiry times
   - Verifies proper handling of boundary conditions

6. **Concurrent Requests**
   - Tests multiple simultaneous OTP requests
   - Verifies proper handling of concurrent operations

7. **Expired Entry Cleanup**
   - Tests cleanup of expired OTP entries
   - Verifies proper state after cleanup

## Troubleshooting Tests

### Common Test Issues

1. **Tests fail with "Unauthorized" errors**
   - Ensure proper addresses are used for prank calls
   - Verify role-based access control is working correctly

2. **Tests fail with "Paused" errors**
   - Check if the contract is paused during testing
   - Ensure proper pause/unpause operations

3. **Tests fail with "AttemptsExceeded" errors**
   - Verify attempt counting is working correctly
   - Check that OTPs are properly locked after 3 attempts

### Debugging Tips

1. **Use verbose output**
   - Run tests with `-vvv` or `-vvvv` for detailed output
   - Look for console.log messages in the test output

2. **Run specific tests**
   - Use `--match-test` to run specific test functions
   - Use `--match-contract` to run specific test contracts

3. **Check gas usage**
   - Use `--gas-report` to analyze gas consumption
   - Optimize expensive operations

## Best Practices

### 1. Test Coverage
- Aim for 100% test coverage
- Test both positive and negative cases
- Include edge cases and boundary conditions

### 2. Security Testing
- Regularly update tests for new attack vectors
- Include fuzz testing for input validation
- Test integration with external systems

### 3. Performance Testing
- Monitor gas usage for expensive operations
- Optimize contract code based on test results
- Test with realistic data sets

### 4. Continuous Testing
- Run tests automatically in CI/CD pipelines
- Monitor test results for regressions
- Update tests when contract functionality changes

## Conclusion

The comprehensive test suite ensures that all security layers and protection mechanisms are functioning correctly. By following this guide, you can verify that the Blockchain OTP system is secure and robust against common attack vectors.