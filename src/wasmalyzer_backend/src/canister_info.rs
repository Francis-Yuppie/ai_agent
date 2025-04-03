#![allow(unused_imports)]
use candid::{CandidType, Deserialize, Principal};

mod canister_scanner;
pub use canister_scanner::{scan_canister, CanisterInfo};

#[derive(CandidType, Deserialize)]
pub struct ErrorResponse {
    field: String,
    message: String,
}

// impl ErrorResponse {
//     fn new(field: &str, message: &str) -> Self {
//         ErrorResponse {
//             field: field.to_string(),
//             message: message.to_string(),
//         }
//     }
// }

#[ic_cdk::update]
pub async fn canister_info(canister_id: Principal) -> Result<CanisterInfo, String> {
    canister_scanner::scan_canister(canister_id)
        .await
        .map_err(|e| format!("Error retrieving full canister info: {}", e))
}

#[ic_cdk::update]
pub async fn canister_metadata(canister_id: Principal) -> Result<String, String> {
    let metadata = canister_scanner::fetch_metadata(canister_id)
        .await
        .map_err(|e| format!("Error retrieving metadata: {}", e))?;

    Ok(metadata)
}

#[ic_cdk::update]
pub async fn canister_transactions(canister_id: Principal) -> Result<Vec<String>, String> {
    let canister_info = canister_scanner::scan_canister(canister_id)
        .await
        .map_err(|e| format!("Error retrieving canister info: {}", e))?;

    Ok(canister_info.transaction_history.unwrap_or_default())
}

// #[ic_cdk::update]
// pub async fn fetch_wasm_code(canister_id: Principal) -> Result<Vec<u8>, String> {
//     let is_mainnet = true;
//     let url = if is_mainnet {
//         "https://ic0.app"
//     } else {
//         "http://127.0.0.1:4943"
//     };

//     let wasm_data = canister_scanner::wasm_code(url, is_mainnet, canister_id)
//         .await
//         .map_err(|e| format!("Error retrieving WASM code: {}", e))?;

//     Ok(wasm_data)
// }
