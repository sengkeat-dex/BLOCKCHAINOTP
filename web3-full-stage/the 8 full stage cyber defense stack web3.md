| Stage | Name                       | Goal                                 |
| ----- | -------------------------- | ------------------------------------ |
| **1** | Web3 Architecture          | Smart contracts, bridges, wallets    |
| **2** | Security Layers (22)       | Prevent & design controls            |
| **3** | Testing Layers (100+)      | Verify & prove safety                |
| **4** | Protection Layers (15)     | Contain & mitigate runtime attacks   |
| **5** | Detection & Response       | Observe & alert in real time         |
| **6** | Resilience & Recovery      | Restore & stay operational           |
| **7** | Governance & Compliance    | Policy & accountability              |
| **8** | Intelligence & Improvement | Learn & evolve faster than attackers |


layer,type,subtype,features,goal,tools,metrics,evidence
1,Core Infrastructure,Network Layer,"P2P mesh, node discovery, gossip relay","Reliable peer-to-peer connectivity","libp2p, DevP2P, Waku","peer uptime %, packet loss","Network topology map, node logs"
1,Core Infrastructure,Consensus Layer,"PoS/PoW/BFT algorithms, validator election, finality","Ensure block consensus and chain integrity","Tendermint, HotStuff, Casper","finality time, validator participation","Consensus metrics, signed blocks"
1,Core Infrastructure,Execution Layer,"Virtual machine, runtime sandbox, gas metering","Execute contracts deterministically","EVM, WASM","tx success rate, gas avg","Execution traces, state diffs"
1,Core Infrastructure,Storage Layer,"State trie, block DB, archival nodes","Persist and retrieve state safely","LevelDB, RocksDB, Arweave","state sync time, data integrity","Merkle proofs, checksum logs"
2,Smart Contract,Logic Layer,"Business rules, AMM math, vault logic","Define protocol behavior","Solidity, Vyper, CosmWasm","unit tests pass %, code coverage","Audit reports, test coverage reports"
2,Smart Contract,Access Layer,"RBAC, Ownable, pausability","Enforce least-privilege control","OpenZeppelin AccessControl","privilege escalation incidents","Role matrix, AccessControl logs"
2,Smart Contract,Upgrade Layer,"Proxy, UUPS, Beacon pattern","Allow safe upgrades","OpenZeppelin Upgrades","successful migration %, layout diff=0","Proxy storage map, migration checklist"
3,Bridge,Messaging Layer,"Relayers, proof submitters","Cross-chain message transport","LayerZero, Wormhole","message success %, reorg tolerance","Bridge tx logs, proof receipts"
3,Bridge,Validation Layer,"Light client, Merkle/zk verification","Verify source chain proofs","IBC, zkSNARK, zkLightClient","proof verification time","Merkle proof record, validator quorum"
3,Bridge,Liquidity Layer,"Pools, bonded liquidity, lock-mint","Maintain transfer liquidity","Synapse, Stargate","TVL uptime, slippage %","Liquidity audit, reserve proofs"
4,Wallet,Key Management,"Seed, MPC, HSM","Protect user keys","BIP32/39/44, Fireblocks","key loss incidents","Key rotation logs, MPC quorum proof"
4,Wallet,Signing Layer,"EIP-712 typed data signing, nonce mgmt","Generate valid on-chain signatures","ethers.js, WalletConnect","signature failure rate","Signed tx archive"
4,Wallet,Recovery Layer,"Social/guardian recovery","Enable account recovery","Safe recovery, Argent","recovery success %","Guardian signature log"
5,Detection & Response,Telemetry Collection,"Logs, metrics, traces, blockchain telemetry","Observe system health","OpenTelemetry, Loki, Prometheus","MTTD <5m, coverage >90%","Centralized telemetry store"
5,Detection & Response,Threat Detection,"Rule, anomaly, ML detection, on-chain alerts","Identify abnormal or malicious activity","Forta, ELK, Suricata","true positive rate, FPR <10%","Alert logs, detection rule IDs"
5,Detection & Response,Incident Automation,"Auto-pause, playbooks, forensics","Contain threats automatically","GitHub Actions, OPA, Sentinel","MTTR <15m","Runbook results, paused contract IDs"
6,Resilience & Recovery,Redundancy & HA,"Multi-node replicas, load balancers, storage sync","Eliminate single points of failure","K8s replicas, Linkerd, HAProxy","uptime ≥99.9%","Replication logs, SLO reports"
6,Resilience & Recovery,Backup & Snapshot,"Hot/cold backups, blockchain state dumps","Preserve recoverable state","pgBackRest, Velero, Arweave","RPO ≤5m","Snapshot hashes, restore test log"
6,Resilience & Recovery,Chaos & Fault Testing,"Fault injection, node kill drills, network partition","Validate fault tolerance","Chaos Mesh, Gremlin","chaos pass rate ≥95%","Chaos test report, metrics"
6,Resilience & Recovery,Disaster Recovery Plan,"Runbooks, RTO/RPO metrics, tabletop drills","Structured recovery under outage","Ansible, Terraform restore","RTO ≤15m","DR test evidence, RCA docs"
7,Governance & Compliance,Policy Framework,"CODEOWNERS, SECURITY.md, versioned rules","Define decision authority","GitHub CODEOWNERS, OPA","policy coverage ≥95%","Signed policy file, change log"
7,Governance & Compliance,Access Governance,"RBAC/ABAC policies, key custody, separation of duties","Control privileges and key usage","OPA Rego, Safe Multisig","unauthorized change=0","Access audit trail"
7,Governance & Compliance,Compliance Mapping,"NIST/ISO/SOC2 controls, KYC/AML alignment","Meet external regulatory standards","Drata, Vanta, TRM Labs","audit pass rate","External audit certificate"
7,Governance & Compliance,Audit & Evidence,"Logs, SBOM, attestations","Prove compliance and accountability","immudb, cosign","evidence completeness 100%","Signed SBOM, immutability hash"
8,Intelligence & Improvement,Threat Intelligence,"External feeds, OSINT, ML clustering","Discover emerging threats early","Forta Intel, MISP, Chainalysis","new patterns ≥10/mo","Intel reports, feed IDs"
8,Intelligence & Improvement,Adversarial Simulation,"Red/Blue/Purple teaming, fuzz bots","Test defense vs real-world attacks","Foundry fuzz, MythX, Halborn","exploit coverage %, issues closed","Attack simulation results"
8,Intelligence & Improvement,Knowledge Management,"RCA database, defense pattern repo, training portal","Retain and share lessons learned","Notion, Confluence, GitBook","incident repeat rate=0","Postmortem documents"
8,Intelligence & Improvement,Automation & Foresight,"ML feedback loops, policy auto-tuning, tech radar","Self-improving adaptive defense","MLflow, OPA, Prometheus","automation coverage ≥80%","Model registry, forecast report"
