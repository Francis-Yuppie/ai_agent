use anyhow::{anyhow, Result};
use candid::CandidType;
use candid::Encode;
use candid::Principal as CandidPrincipal;
use candid::{Deserialize, Principal};
// use ic_agent::export::Principal as IcPrincipal;
// use ic_agent::Agent;
use ic_cdk::api::call::call;
use ic_cdk::api::management_canister::main;
use ic_cdk::api::management_canister::main::{CanisterIdRecord, CanisterStatusResponse};
use serde::Serialize;
 
#[derive(CandidType, Serialize, Deserialize)]
pub struct CanisterInfo {
    pub canister_id: Principal,
    pub wasm_code: Option<Vec<u8>>,
    pub metadata: Option<String>,
    pub transaction_history: Option<Vec<String>>,
    pub timestamp: u64,
}



pub async fn scan_canister(canister_id: Principal) -> Result<CanisterInfo, String> {
    let (wasm_module,) = main::raw_rand()
        .await
        .map_err(|e| format!("Failed to fetch canister WASM: {:?}", e))?;

    let (metadata_response,): (CanisterStatusResponse,) =
        main::canister_status(CanisterIdRecord { canister_id })
            .await
            .map_err(|e| format!("Failed to fetch metadata: {:?}", e))?;

    let transaction_history = fetch_transaction_history(canister_id)
        .await
        .map_err(|e| format!("Failed to fetch transaction history: {:?}", e))?;

    Ok(CanisterInfo {
        canister_id,
        wasm_code: Some(wasm_module),
        metadata: Some(format!("{:?}", metadata_response.status)),
        transaction_history: Some(transaction_history),
        timestamp: ic_cdk::api::time(),
    })
}

// pub async fn create_agent(url: &str, is_mainnet: bool) -> Result<Agent> {
//     let agent = Agent::builder().with_url(url).build()?;
//     if !is_mainnet {
//         agent.fetch_root_key().await?;
//     }
//     Ok(agent)
// }

// async fn get_canister_wasm(agent: &Agent, canister_id: CandidPrincipal) -> Result<Vec<u8>> {
//     let ic_canister_id = IcPrincipal::from_text(canister_id.to_text())
//         .map_err(|e| anyhow!("Failed to convert canister_id: {}", e))?;

//     let arg = Encode!().map_err(|e| anyhow!("Failed to encode empty argument: {}", e))?;

//     let response = agent
//         .query(&ic_canister_id, "get_wasm") // Corrected Principal type
//         .with_arg(&arg)
//         .call()
//         .await
//         .map_err(|e| anyhow!("Failed to query get_wasm: {}", e))?;

//     if !response.is_empty() {
//         let wasm_bytes = response.clone();
//         let wasm_size = wasm_bytes.len();

//         let max_wasm_size = 10 * 1024 * 1024; // 10 MB

//         if wasm_size > max_wasm_size {
//             return Err(anyhow!("WASM size exceeds limit: {} bytes", wasm_size));
//         }

//         Ok(wasm_bytes)
//     } else {
//         Err(anyhow!("Unexpected empty response from get_wasm"))
//     }
// }

// pub async fn wasm_code(
//     url: &str,
//     is_mainnet: bool,
//     canister_id: CandidPrincipal,
// ) -> Result<Vec<u8>, String> {
    
//     let agent = create_agent(url, is_mainnet)
//         .await
//         .map_err(|e| format!("Failed to create agent: {}", e))?;

//     get_canister_wasm(&agent, canister_id)
//         .await
//         .map_err(|e| format!("Error fetching WASM code: {}", e))
// }

pub async fn fetch_metadata(canister_id: Principal) -> Result<String, String> {
    let (metadata_response,): (CanisterStatusResponse,) =
        main::canister_status(CanisterIdRecord { canister_id })
            .await
            .map_err(|e| format!("Failed to fetch metadata: {:?}", e))?;

    Ok(format!("{:?}", metadata_response.status))
}

pub async fn fetch_transaction_history(canister_id: Principal) -> Result<Vec<String>, String> {
    let (transaction_log_query,): (Vec<u8>,) = call(canister_id, "get_transaction_history", ())
        .await
        .map_err(|e| format!("Failed to query transaction history: {:?}", e))?;

    let transaction_history: Vec<String> = serde_json::from_slice(&transaction_log_query)
        .map_err(|e| format!("Failed to parse transaction history: {:?}", e))?;

    Ok(transaction_history)
}
