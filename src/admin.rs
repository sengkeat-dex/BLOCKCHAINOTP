use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, TransactionReceipt, U64},
};
use otp_contract::OtpVerifierContract;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs, path::Path, str::FromStr, sync::Arc, time::Duration};
use thiserror::Error;

type EvmClient = SignerMiddleware<Provider<Http>, LocalWallet>;

#[derive(Debug, Clone)]
pub struct AdminController {
    wallet: LocalWallet,
    deployments: HashMap<String, EvmDeployment>,
}

#[derive(Debug, Clone)]
struct EvmDeployment {
    key: String,
    label: String,
    rpc_url: String,
    contract: Address,
}

#[derive(Debug, Deserialize)]
struct DeploymentEntry {
    network: String,
    rpc_url: String,
    #[serde(default)]
    otp_verifier: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ActionOutcome {
    pub action: String,
    pub network_key: String,
    pub network_label: String,
    pub tx_hash: String,
    pub status: Option<String>,
    pub block_number: Option<u64>,
}

#[derive(Debug, Error)]
pub enum AdminError {
    #[error("admin private key is not configured")]
    MissingAdminKey,
    #[error("invalid admin private key: {0}")]
    InvalidAdminKey(String),
    #[error("deployment config '{0}' not found")]
    MissingConfig(String),
    #[error("failed to read deployment config: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse deployment config: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("network '{0}' is not configured for OTP verifier automation")]
    UnknownNetwork(String),
    #[error("invalid address string: {0}")]
    InvalidAddress(String),
    #[error("rpc provider error: {0}")]
    Rpc(String),
    #[error("contract call error: {0}")]
    Contract(String),
    #[error("transaction receipt was not produced")]
    MissingReceipt,
}

impl AdminController {
    pub fn network_count(&self) -> usize {
        self.deployments.len()
    }

    pub fn maybe_from_env() -> Result<Option<Self>, AdminError> {
        let key = match env::var("ADMIN_PRIVATE_KEY") {
            Ok(raw) if raw.trim().is_empty() => return Err(AdminError::MissingAdminKey),
            Ok(raw) => raw,
            Err(env::VarError::NotPresent) => return Ok(None),
            Err(env::VarError::NotUnicode(_)) => {
                return Err(AdminError::InvalidAdminKey(
                    "value is not valid UTF-8".into(),
                ))
            }
        };

        let wallet = LocalWallet::from_str(&key)
            .map_err(|err| AdminError::InvalidAdminKey(err.to_string()))?;

        let (config_path, explicit_path) = match env::var("DEPLOYMENTS_CONFIG") {
            Ok(value) => (value, true),
            Err(_) => ("config/deployments.toml".into(), false),
        };

        let contents = match read_config_file(&config_path) {
            Ok(text) => text,
            Err(ConfigReadError::Missing) => {
                if explicit_path {
                    return Err(AdminError::MissingConfig(config_path));
                } else {
                    return Ok(None);
                }
            }
            Err(ConfigReadError::Io(err)) => return Err(AdminError::Io(err)),
        };

        let raw_entries: HashMap<String, DeploymentEntry> = toml::from_str(&contents)?;
        let mut deployments = HashMap::new();

        for (key, entry) in raw_entries {
            let contract_address = match entry.otp_verifier {
                Some(addr) => addr,
                None => continue,
            };

            let contract = Address::from_str(contract_address.trim())
                .map_err(|_| AdminError::InvalidAddress(contract_address.clone()))?;

            deployments.insert(
                key.clone(),
                EvmDeployment {
                    key,
                    label: entry.network,
                    rpc_url: entry.rpc_url,
                    contract,
                },
            );
        }

        if deployments.is_empty() {
            return Ok(None);
        }

        Ok(Some(Self {
            wallet,
            deployments,
        }))
    }

    pub async fn pause(&self, network: &str, paused: bool) -> Result<ActionOutcome, AdminError> {
        let deployment = self.get_deployment(network)?;
        self.execute(deployment, "pause", |contract| contract.pause(paused))
            .await
    }

    pub async fn rotate_issuer(
        &self,
        network: &str,
        new_issuer: &str,
    ) -> Result<ActionOutcome, AdminError> {
        let issuer = Address::from_str(new_issuer)
            .map_err(|_| AdminError::InvalidAddress(new_issuer.to_owned()))?;
        let deployment = self.get_deployment(network)?;
        self.execute(deployment, "rotate_issuer", |contract| {
            contract.set_issuer(issuer)
        })
        .await
    }

    pub async fn rotate_admin(
        &self,
        network: &str,
        new_admin: &str,
    ) -> Result<ActionOutcome, AdminError> {
        let admin = Address::from_str(new_admin)
            .map_err(|_| AdminError::InvalidAddress(new_admin.to_owned()))?;
        let deployment = self.get_deployment(network)?;
        self.execute(deployment, "rotate_admin", |contract| {
            contract.set_admin(admin)
        })
        .await
    }

    fn get_deployment(&self, network: &str) -> Result<&EvmDeployment, AdminError> {
        self.deployments
            .get(network)
            .ok_or_else(|| AdminError::UnknownNetwork(network.to_owned()))
    }

    async fn execute<F>(
        &self,
        deployment: &EvmDeployment,
        action: &'static str,
        builder: F,
    ) -> Result<ActionOutcome, AdminError>
    where
        F: FnOnce(
            &OtpVerifierContract<EvmClient>,
        ) -> Result<
            ethers::contract::builders::ContractCall<EvmClient, ()>,
            ethers::contract::ContractError<EvmClient>,
        >,
    {
        let provider = Provider::<Http>::try_from(deployment.rpc_url.as_str())
            .map_err(|err| AdminError::Rpc(err.to_string()))?
            .interval(Duration::from_millis(1200));

        let chain_id = provider
            .get_chainid()
            .await
            .map_err(|err| AdminError::Rpc(err.to_string()))?;

        let wallet = self.wallet.clone().with_chain_id(chain_id.as_u64());
        let client = Arc::new(SignerMiddleware::new(provider, wallet));
        let contract = OtpVerifierContract::new(deployment.contract, Arc::clone(&client));

        let tx = builder(&contract).map_err(|err| AdminError::Contract(err.to_string()))?;
        let pending = tx
            .send()
            .await
            .map_err(|err| AdminError::Contract(err.to_string()))?;
        let receipt = pending
            .await
            .map_err(|err| AdminError::Contract(err.to_string()))?
            .ok_or(AdminError::MissingReceipt)?;

        Ok(ActionOutcome::from_receipt(action, deployment, receipt))
    }
}

impl ActionOutcome {
    fn from_receipt(action: &str, deployment: &EvmDeployment, receipt: TransactionReceipt) -> Self {
        let status = receipt.status.map(status_to_string);
        let block_number = receipt.block_number.map(|num| num.as_u64());

        Self {
            action: action.to_owned(),
            network_key: deployment.key.clone(),
            network_label: deployment.label.clone(),
            tx_hash: format!("{:#x}", receipt.transaction_hash),
            status,
            block_number,
        }
    }
}

fn status_to_string(status: U64) -> String {
    if status == U64::one() {
        "success".into()
    } else {
        "failed".into()
    }
}

enum ConfigReadError {
    Missing,
    Io(std::io::Error),
}

fn read_config_file(path: &str) -> Result<String, ConfigReadError> {
    let file_path = Path::new(path);
    match fs::read_to_string(file_path) {
        Ok(contents) => Ok(contents),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Err(ConfigReadError::Missing),
        Err(err) => Err(ConfigReadError::Io(err)),
    }
}
