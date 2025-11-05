#!/bin/bash

echo "=== Consolidated Web3 Toolchain Verification ==="

echo -e "\n1. EVM Tools:"
echo "   Solidity Compiler:" $(./solc --version | head -1)
echo "   Foundry:" $(forge --version | head -1)
echo "   Slither:" $(~/.local/bin/slither --version)
echo "   Mythril:" $(/mnt/c/Users/USER/Documents/blockchainotp/mythril_env/bin/myth --version)
echo "   Echidna:" $(docker run --rm trailofbits/echidna:latest echidna --version)

echo -e "\n2. Rust/WASM Tools:"
echo "   Rust Compiler:" $(rustc --version)
echo "   Cargo:" $(cargo --version)
echo "   Clippy:" $(cargo clippy --version)
echo "   Rustfmt:" $(cargo fmt --version)
echo "   Cargo Audit:" $(cargo audit --version)
echo "   Cargo Deny:" $(cargo deny --version)
echo "   Cargo Fuzz:" $(cargo fuzz --version)
echo "   Kani:" $(cargo kani --version)
echo "   Crev:" $(cargo crev --version)
echo "   SBOM:" $(cargo sbom --version)

echo -e "\n3. Infrastructure Tools:"
echo "   Docker:" $(docker --version)
echo "   Kubernetes:" $(kubectl version --client --short)
echo "   Terraform:" $(terraform version | head -1)
echo "   Ansible:" $(ansible --version | head -1)
echo "   Prometheus:" $(prometheus --version | head -1)

echo -e "\n4. Policy & Analytics Tools:"
echo "   OPA:" $(opa version | head -1)
echo "   MLflow:" $(mlflow --version | head -1)

echo -e "\n=== Verification Complete ==="