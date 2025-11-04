use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[cfg(feature = "evm")]
use ethers::{abi::Abi, prelude::*};

#[cfg(feature = "solana")]
use {
    borsh::{BorshDeserialize, BorshSerialize},
    bs58,
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        system_program,
        transaction::Transaction,
    },
    tokio::task,
};

#[cfg(all(feature = "evm", feature = "solana"))]
compile_error!(
    "The 'evm' and 'solana' features cannot be enabled at the same time due to upstream zeroize version conflicts."
);

/// Represents the OTP entry structure from the Solidity contract
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OtpEntry {
    pub hash: [u8; 32],
    pub expiry: u64,
    pub used: bool,
}

/// Represents the OTP verifier contract
#[cfg(feature = "evm")]
#[derive(Debug, Clone)]
pub struct OtpVerifierContract<M> {
    #[allow(dead_code)]
    client: Arc<M>,
    contract: Contract<M>,
}

#[cfg(feature = "evm")]
impl<M: Middleware> OtpVerifierContract<M> {
    /// Creates a new instance of the OTP verifier contract
    pub fn new(address: Address, client: Arc<M>) -> Self {
        // ABI for the OtpVerifier contract
        // In a real implementation, this would be generated from the Solidity contract
        let abi = r#"[{"inputs":[{"internalType":"address","name":"_issuer","type":"address"}],"stateMutability":"nonpayable","type":"constructor"},{"anonymous":"false","inputs":[{"indexed":"true","internalType":"address","name":"oldIssuer","type":"address"},{"indexed":"true","internalType":"address","name":"newIssuer","type":"address"}],"name":"IssuerChanged","type":"event"},{"anonymous":"false","inputs":[{"indexed":"true","internalType":"bool","name":"paused","type":"bool"}],"name":"Paused","type":"event"},{"anonymous":"false","inputs":[{"indexed":"true","internalType":"bytes32","name":"requestId","type":"bytes32"},{"indexed":"false","internalType":"uint64","name":"expiry","type":"uint64"}],"name":"OtpSet","type":"event"},{"anonymous":"false","inputs":[{"indexed":"true","internalType":"bytes32","name":"requestId","type":"bytes32"},{"indexed":"true","internalType":"address","name":"by","type":"address"}],"name":"OtpVerified","type":"event"},{"inputs":[{"internalType":"bytes32","name":"requestId","type":"bytes32"},{"internalType":"bytes32","name":"otpHash","type":"bytes32"},{"internalType":"uint64","name":"expiry","type":"uint64"}],"name":"setOtp","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes32","name":"requestId","type":"bytes32"},{"internalType":"string","name":"otp","type":"string"}],"name":"verify","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bool","name":"_paused","type":"bool"}],"name":"pause","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newIssuer","type":"address"}],"name":"setIssuer","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes32","name":"","type":"bytes32"}],"name":"entries","outputs":[{"internalType":"bytes32","name":"hash","type":"bytes32"},{"internalType":"uint64","name":"expiry","type":"uint64"},{"internalType":"bool","name":"used","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"issuer","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"paused","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"}]"#;

        let contract = Contract::new(
            address,
            serde_json::from_str::<Abi>(abi).expect("Invalid ABI"),
            client.clone(),
        );

        Self { client, contract }
    }

    /// Returns a clone of the underlying middleware client.
    pub fn client(&self) -> Arc<M> {
        self.client.clone()
    }

    /// Builds a transaction for `setOtp`
    pub fn set_otp(
        &self,
        request_id: [u8; 32],
        otp_hash: [u8; 32],
        expiry: u64,
    ) -> Result<ContractCall<M, ()>, ContractError<M>> {
        self.contract
            .method::<_, ()>("setOtp", (request_id, otp_hash, expiry))
            .map_err(ContractError::AbiError)
    }

    /// Builds a call for `verify`
    pub fn verify(
        &self,
        request_id: [u8; 32],
        otp: &str,
    ) -> Result<ContractCall<M, bool>, ContractError<M>> {
        self.contract
            .method::<_, bool>("verify", (request_id, otp.to_owned()))
            .map_err(ContractError::AbiError)
    }

    /// Builds a transaction for `pause`
    pub fn pause(&self, paused: bool) -> Result<ContractCall<M, ()>, ContractError<M>> {
        self.contract
            .method::<_, ()>("pause", (paused,))
            .map_err(ContractError::AbiError)
    }

    /// Builds a transaction for `setIssuer`
    pub fn set_issuer(&self, new_issuer: Address) -> Result<ContractCall<M, ()>, ContractError<M>> {
        self.contract
            .method::<_, ()>("setIssuer", (new_issuer,))
            .map_err(ContractError::AbiError)
    }

    /// Creates a call to read the raw `entries` mapping
    pub fn entries(
        &self,
        request_id: [u8; 32],
    ) -> Result<ContractCall<M, (Vec<u8>, u64, bool)>, ContractError<M>> {
        self.contract
            .method::<_, (Vec<u8>, u64, bool)>("entries", (request_id,))
            .map_err(ContractError::AbiError)
    }

    /// Creates a call to read the issuer address
    pub fn issuer(&self) -> Result<ContractCall<M, Address>, ContractError<M>> {
        self.contract
            .method::<_, Address>("issuer", ())
            .map_err(ContractError::AbiError)
    }

    /// Creates a call to read the paused flag
    pub fn paused(&self) -> Result<ContractCall<M, bool>, ContractError<M>> {
        self.contract
            .method::<_, bool>("paused", ())
            .map_err(ContractError::AbiError)
    }
}

/// Normalized error channel for chain-agnostic clients.
#[derive(Debug, Error)]
pub enum OtpChainError {
    #[error("evm contract error: {0}")]
    Evm(String),
    #[error("solana error: {0}")]
    Solana(String),
    #[error("unsupported chain: {0}")]
    Unsupported(String),
}

#[async_trait]
pub trait OtpChain: Send + Sync {
    async fn set_otp(
        &self,
        request_id: [u8; 32],
        otp_hash: [u8; 32],
        expiry: u64,
    ) -> Result<(), OtpChainError>;

    async fn verify(&self, request_id: [u8; 32], otp: &str) -> Result<bool, OtpChainError>;

    async fn get_entry(&self, request_id: [u8; 32]) -> Result<OtpEntry, OtpChainError>;
}

#[cfg(feature = "evm")]
fn evm_err<M: Middleware>(err: ContractError<M>) -> OtpChainError {
    OtpChainError::Evm(err.to_string())
}

#[cfg(feature = "evm")]
fn evm_middleware_err<E: std::fmt::Display>(err: E) -> OtpChainError {
    OtpChainError::Evm(err.to_string())
}

#[cfg(feature = "solana")]
fn solana_err<E: std::fmt::Display>(err: E) -> OtpChainError {
    OtpChainError::Solana(err.to_string())
}

#[cfg(feature = "evm")]
#[async_trait]
impl<M> OtpChain for OtpVerifierContract<M>
where
    M: Middleware + Send + Sync + 'static,
{
    async fn set_otp(
        &self,
        request_id: [u8; 32],
        otp_hash: [u8; 32],
        expiry: u64,
    ) -> Result<(), OtpChainError> {
        let call = OtpVerifierContract::set_otp(self, request_id, otp_hash, expiry).map_err(evm_err)?;
        let pending = call.send().await.map_err(evm_middleware_err)?;
        pending.await.map_err(evm_middleware_err)?;
        Ok(())
    }

    async fn verify(&self, request_id: [u8; 32], otp: &str) -> Result<bool, OtpChainError> {
        let call = OtpVerifierContract::verify(self, request_id, otp).map_err(evm_err)?;
        call.call().await.map_err(evm_middleware_err)
    }

    async fn get_entry(&self, request_id: [u8; 32]) -> Result<OtpEntry, OtpChainError> {
        let (hash_vec, expiry, used) = self
            .entries(request_id)
            .map_err(evm_err)?
            .call()
            .await
            .map_err(evm_middleware_err)?;

        let mut hash = [0u8; 32];
        let copy_len = hash_vec.len().min(32);
        hash[..copy_len].copy_from_slice(&hash_vec[..copy_len]);

        Ok(OtpEntry { hash, expiry, used })
    }
}

/// Solana program client implementing the same trait.
#[cfg(feature = "solana")]
pub struct SolanaOtpClient {
    rpc_url: String,
    program_id: Pubkey,
    payer: Arc<Keypair>,
}

#[cfg(feature = "solana")]
impl SolanaOtpClient {
    pub fn try_new(rpc_url: impl Into<String>, program_id: &str, payer_base58: &str) -> Result<Self, OtpChainError> {
        let program = program_id
            .parse::<Pubkey>()
            .map_err(|e| OtpChainError::Solana(format!("invalid program id: {e}")))?;
        let secret = bs58::decode(payer_base58)
            .into_vec()
            .map_err(|e| OtpChainError::Solana(format!("invalid payer key: {e}")))?;
        let payer = Keypair::from_bytes(&secret)
            .map_err(|e| OtpChainError::Solana(format!("keypair decode failed: {e}")))?;
        Ok(Self {
            rpc_url: rpc_url.into(),
            program_id: program,
            payer: Arc::new(payer),
        })
    }

    fn entry_pda(&self, request_id: &[u8; 32]) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[OTP_PDA_SEED, request_id], &self.program_id)
    }

    async fn with_rpc<F, R>(&self, f: F) -> Result<R, OtpChainError>
    where
        F: FnOnce(SolanaContext) -> Result<R, OtpChainError> + Send + 'static,
        R: Send + 'static,
    {
        let rpc_url = self.rpc_url.clone();
        let program_id = self.program_id;
        let payer_bytes = self.payer.to_bytes();

        task::spawn_blocking(move || {
            let rpc = RpcClient::new(rpc_url);
            let payer = Keypair::from_bytes(&payer_bytes)
                .map_err(|e| OtpChainError::Solana(format!("keypair restoration failed: {e}")))?;
            let ctx = SolanaContext {
                rpc,
                program_id,
                payer: Arc::new(payer),
            };
            f(ctx)
        })
        .await
        .map_err(|e| OtpChainError::Solana(format!("solana task join error: {e}")))?
    }
}

#[cfg(feature = "solana")]
impl Clone for SolanaOtpClient {
    fn clone(&self) -> Self {
        let payer = Keypair::from_bytes(&self.payer.to_bytes()).expect("keypair clone");
        Self {
            rpc_url: self.rpc_url.clone(),
            program_id: self.program_id,
            payer: Arc::new(payer),
        }
    }
}

#[cfg(feature = "solana")]
const OTP_PDA_SEED: &[u8] = b"otp";

#[cfg(feature = "solana")]
#[derive(BorshSerialize, BorshDeserialize)]
enum SolanaOtpInstruction {
    Set {
        request_id: [u8; 32],
        otp_hash: [u8; 32],
        expiry: u64,
    },
    Verify {
        request_id: [u8; 32],
        otp: String,
    },
}

#[cfg(feature = "solana")]
#[derive(BorshDeserialize)]
struct SolanaOtpAccount {
    hash: [u8; 32],
    expiry: u64,
    used: bool,
}

#[cfg(feature = "solana")]
struct SolanaContext {
    rpc: RpcClient,
    program_id: Pubkey,
    payer: Arc<Keypair>,
}

#[cfg(feature = "solana")]
impl SolanaContext {
    fn send_instruction(&self, instruction: Instruction) -> Result<(), OtpChainError> {
        let blockhash = self.rpc.get_latest_blockhash().map_err(solana_err)?;
        let payer = self.payer.as_ref();
        let tx = Transaction::new_signed_with_payer(&[instruction], Some(&payer.pubkey()), &[payer], blockhash);
        self.rpc.send_and_confirm_transaction(&tx).map_err(solana_err)?;
        Ok(())
    }

    fn set_otp(&self, request_id: [u8; 32], otp_hash: [u8; 32], expiry: u64) -> Result<(), OtpChainError> {
        let (entry_pda, _) = Pubkey::find_program_address(&[OTP_PDA_SEED, &request_id], &self.program_id);
        let ix_data = SolanaOtpInstruction::Set {
            request_id,
            otp_hash,
            expiry,
        }
        .try_to_vec()
        .map_err(solana_err)?;

        let accounts = vec![
            AccountMeta::new(self.payer.pubkey(), true),
            AccountMeta::new(entry_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ];
        let ix = Instruction::new_with_bytes(self.program_id, &ix_data, accounts);
        self.send_instruction(ix)
    }

    fn verify(&self, request_id: [u8; 32], otp: String) -> Result<(), OtpChainError> {
        let (entry_pda, _) = Pubkey::find_program_address(&[OTP_PDA_SEED, &request_id], &self.program_id);
        let ix_data = SolanaOtpInstruction::Verify { request_id, otp }
            .try_to_vec()
            .map_err(solana_err)?;
        let accounts = vec![
            AccountMeta::new(self.payer.pubkey(), true),
            AccountMeta::new(entry_pda, false),
        ];
        let ix = Instruction::new_with_bytes(self.program_id, &ix_data, accounts);
        self.send_instruction(ix)
    }

    fn get_entry(&self, request_id: [u8; 32]) -> Result<OtpEntry, OtpChainError> {
        let (entry_pda, _) = Pubkey::find_program_address(&[OTP_PDA_SEED, &request_id], &self.program_id);
        let data = self.rpc.get_account_data(&entry_pda).map_err(solana_err)?;
        let account = SolanaOtpAccount::try_from_slice(&data).map_err(solana_err)?;
        Ok(OtpEntry {
            hash: account.hash,
            expiry: account.expiry,
            used: account.used,
        })
    }
}

#[async_trait]
#[cfg(feature = "solana")]
#[async_trait]
impl OtpChain for SolanaOtpClient {
    async fn set_otp(
        &self,
        _request_id: [u8; 32],
        _otp_hash: [u8; 32],
        _expiry: u64,
    ) -> Result<(), OtpChainError> {
        self.with_rpc(move |ctx| ctx.set_otp(request_id, otp_hash, expiry))
            .await
    }

    async fn verify(&self, request_id: [u8; 32], otp: &str) -> Result<bool, OtpChainError> {
        let otp_owned = otp.to_owned();
        self.with_rpc(move |ctx| {
            ctx.verify(request_id, otp_owned)?;
            Ok(true)
        })
        .await
    }

    async fn get_entry(&self, request_id: [u8; 32]) -> Result<OtpEntry, OtpChainError> {
        self.with_rpc(move |ctx| ctx.get_entry(request_id)).await
    }
}
