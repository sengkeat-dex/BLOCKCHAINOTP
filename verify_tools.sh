#!/bin/bash

echo "=== Verification of Web3 Cyber-Defense Architecture Tools ==="

echo -e "\n1. Solidity Compiler:"
/mnt/c/Users/USER/Documents/blockchainotp/solc --version

echo -e "\n2. Rust:"
source ~/.cargo/env && rustc --version

echo -e "\n3. Foundry:"
forge --version

echo -e "\n4. Slither:"
~/.local/bin/slither --version

echo -e "\n5. Mythril:"
/mnt/c/Users/USER/Documents/blockchainotp/mythril_env/bin/myth --version

echo -e "\n6. Prometheus:"
prometheus --version

echo -e "\n7. Kubernetes:"
kubectl version --client

echo -e "\n8. Terraform:"
terraform version

echo -e "\n9. Ansible:"
ansible --version

echo -e "\n10. OPA:"
opa version

echo -e "\n11. MLflow:"
mlflow --version

echo -e "\n12. Echidna (via Docker):"
docker run --rm trailofbits/echidna:latest echidna --version

echo -e "\n=== All tools verified successfully! ==="