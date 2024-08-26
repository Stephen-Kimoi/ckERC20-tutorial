use std::str::FromStr;

use b3_utils::api::{CallCycles, InterCall};
use candid::{CandidType, Nat, Principal};
use ic_cdk::api::call::{CallResult, RejectionCode};
use serde::Deserialize;

const CK_SEPOLIA_ERC20_LEDGER_SUITE_ORCHESTRATOR_CANISTER: &str = "2s5qh-7aaaa-aaaar-qadya-cai";

// Define constants for chain IDs and token addresses
const SEPOLIA_CHAIN_ID: &str = "11155111";
const ETHEREUM_CHAIN_ID: &str = "1";
const SEPOLIA_USDC_ADDRESS: &str = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238";
const ETHEREUM_USDC_ADDRESS: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";

#[derive(CandidType, Deserialize)]
struct CanisterIdsArgs {
    chain_id: Nat,
    address: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct CanisterIdsResult {
    ledger: Option<Principal>,
    index: Option<Principal>,
    archives: Vec<Principal>,
}

#[derive(Clone, Copy, CandidType, Deserialize)]
pub enum Token {
    CkSepoliaUSDC,
    CkUSDC,
}

impl Token {
    fn get_chain_id(&self) -> Nat {
        match self {
            Token::CkSepoliaUSDC => Nat::from_str(SEPOLIA_CHAIN_ID).unwrap(),
            Token::CkUSDC => Nat::from_str(ETHEREUM_CHAIN_ID).unwrap(),
        }
    }

    fn get_address(&self) -> String {
        match self {
            Token::CkSepoliaUSDC => SEPOLIA_USDC_ADDRESS.to_string(),
            Token::CkUSDC => ETHEREUM_USDC_ADDRESS.to_string(),
        }
    }
}

#[ic_cdk::update]
async fn get_canister_ids(token: Token) -> Result<Option<CanisterIdsResult>, String> {
    let args = CanisterIdsArgs {
        chain_id: token.get_chain_id(),
        address: token.get_address(),
    };

    match InterCall::from(CK_SEPOLIA_ERC20_LEDGER_SUITE_ORCHESTRATOR_CANISTER).call::<CanisterIdsArgs, Result<Option<CanisterIdsResult>, (RejectionCode, String)>>(
        "canister_ids",
        args,
        CallCycles::NoPay
    )
    .await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err((code, message))) => Err(format!("Canister error: {:?} - {}", code, message)),
        Err(e) => Err(format!("InterCall error: {:?}", e)),
    }
}

// Example usage
#[ic_cdk::update]
async fn get_cksepoliausdc_canister_ids() -> Result<Option<CanisterIdsResult>, String> {
    get_canister_ids(Token::CkSepoliaUSDC).await
}

#[ic_cdk::update]
async fn get_ckusdc_canister_ids() -> Result<Option<CanisterIdsResult>, String> {
    get_canister_ids(Token::CkUSDC).await
}