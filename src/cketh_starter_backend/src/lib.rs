use b3_utils::{vec_to_hex_string_with_0x, Subaccount};
use b3_utils::ledger::{ICRCAccount, ICRC1, ICRC2ApproveArgs, ICRC2ApproveResult};
use b3_utils::api::{InterCall, CallCycles}; 
use candid::{Nat, Principal, CandidType, Deserialize};
use ic_cdk::api::call::CallResult;
use canister_ids::{CanisterIdsResult, Token}; 

use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens, TransferArg, TransferError};
use serde::Serialize;

mod canister_ids;

const CK_SEPOLIA_ETH_LEDGER: &str = "apia6-jaaaa-aaaar-qabma-cai";
const CK_SEPOLIA_USDC_LEDGER_CANISTER: &str = "yfumr-cyaaa-aaaar-qaela-cai"; 
const CK_SEPOLIA_USDC_INDEX_CANISTER: &str = "ycvkf-paaaa-aaaar-qaelq-cai"; 
const CK_SEPOLIA_ETH_MINTER_CANISTER: &str = "jzenf-aiaaa-aaaar-qaa7q-cai"; 
pub const CK_SEPOLIA_ERC20_LEDGER_SUITE_ORCHESTRATOR_CANISTER: &str = "2s5qh-7aaaa-aaaar-qadya-cai"; 

const SEPOLIA_CHAIN_ID: &str = "11155111"; 
const SEPOLIA_USDC_ADDRESS: &str = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238"; 

// Withdraw structs
#[derive(candid::CandidType, serde::Deserialize)]
struct WithdrawErc20Args {
    ckerc20_ledger_id: Principal,
    recipient: String,
    amount: Nat,
}

#[derive(candid::CandidType, serde::Deserialize)]
struct WithdrawErc20Result {
    block_index: Nat,
}

// Transfer structs
#[derive(CandidType, Deserialize, Serialize)]
pub struct TransferArgs {
    amount: NumTokens,
    to_account: Account,
}

#[ic_cdk::query]
fn canister_deposit_principal() -> String {
    let subaccount = Subaccount::from(ic_cdk::id());

    let bytes32 = subaccount.to_bytes32().unwrap();

    vec_to_hex_string_with_0x(bytes32)
}

#[ic_cdk::update]
async fn check_ckusdc_balance(principal_id: Principal) -> Nat {
  let account = ICRCAccount::new(principal_id, None);

  ICRC1::from(CK_SEPOLIA_USDC_LEDGER_CANISTER).balance_of(account).await.unwrap()
}

#[ic_cdk::update] 
async fn approve_cketh_burning(user_principal: Principal, amount: Nat) -> ICRC2ApproveResult {
    let from_subaccount = Subaccount::from(user_principal);
    
    // Use the ckETH minter as the spender
    let minter_principal = Principal::from_text(CK_SEPOLIA_ETH_MINTER_CANISTER).expect("Invalid minter principal");
    let spender = ICRCAccount::new(minter_principal, None);

    let approve_args = ICRC2ApproveArgs {
        from_subaccount: Some(from_subaccount), 
        spender, 
        amount, 
        expected_allowance: None,
        expires_at: None,
        fee: None, 
        created_at_time: None, 
        memo: None 
    }; 

    InterCall::from(CK_SEPOLIA_ETH_LEDGER).call(
        "icrc2_approve", 
        approve_args, 
        CallCycles::NoPay
    )
    .await 
    .unwrap()
}
 
#[ic_cdk::update]
async fn approve_usdc_burning(user_principal: Principal, amount: Nat) -> ICRC2ApproveResult {
    let from_subaccount = Subaccount::from(user_principal);
    
    // Convert minter Principal to ICRCAccount
    let minter_principal = Principal::from_text(CK_SEPOLIA_ETH_MINTER_CANISTER).expect("Invalid minter principal");
    let spender = ICRCAccount::new(minter_principal, None);

    let approve_args = ICRC2ApproveArgs {
        from_subaccount: Some(from_subaccount),
        spender,
        amount,
        expected_allowance: None,
        expires_at: None,
        fee: None,
        created_at_time: None,
        memo: None
    };

    InterCall::from(CK_SEPOLIA_USDC_LEDGER_CANISTER).call(
        "icrc2_approve",
        approve_args,
        CallCycles::NoPay
    )
    .await
    .unwrap()
}

#[ic_cdk::update]
async fn withdraw_ckusdc_to_ethereum(amount: Nat, eth_address: String) -> CallResult<WithdrawErc20Result> {
    let args = WithdrawErc20Args {
        ckerc20_ledger_id: Principal::from_text(CK_SEPOLIA_USDC_LEDGER_CANISTER).expect("Invalid USDC ledger principal"),
        recipient: eth_address,
        amount,
    };

    InterCall::from(CK_SEPOLIA_ETH_MINTER_CANISTER).call(
        "withdraw_erc20",
        args,
        CallCycles::NoPay
    )
    .await
    .unwrap()
}

#[ic_cdk::update] 
async fn transfer_ckusdc_to_principal(amount: Nat, to: Principal) -> CallResult<BlockIndex> {
    
    let transfer_args = TransferArgs {
        amount,
        to_account: Account {
            owner: to,
            subaccount: None,
        },
    };
    
    InterCall::from(CK_SEPOLIA_USDC_LEDGER_CANISTER).call(
        "icrc1_transfer",
        transfer_args,
        CallCycles::NoPay
    )
    .await
    .unwrap()
}

ic_cdk::export_candid!();