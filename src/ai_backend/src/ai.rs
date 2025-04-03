use serde::{Deserialize, Serialize};
use candid::CandidType; // 👈 Import CandidType

#[derive(Serialize, Deserialize, CandidType)] // 👈 Add CandidType here
pub struct AnalysisResult {
    pub risks: Vec<String>,
    pub score: f32,
}

pub fn analyze_wasm(wasm_code: Vec<u8>) -> AnalysisResult {
    let risk_level = if wasm_code.len() > 100000 { "High" } else { "Low" };

    AnalysisResult {
        risks: vec![format!("Detected {} risk", risk_level)],
        score: if risk_level == "High" { 0.3 } else { 0.8 },
    }
}
