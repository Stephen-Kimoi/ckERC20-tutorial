use b3_utils::{vec_to_hex_string_with_0x, Subaccount};

#[ic_cdk::query]
fn canister_deposit_principal() -> String {
    let subaccount = Subaccount::from(ic_cdk::id());

    let bytes32 = subaccount.to_bytes32().unwrap();

    vec_to_hex_string_with_0x(bytes32)
}

ic_cdk::export_candid!();