use candid::Principal;
use ic_cdk::api::management_canister::http_request::HttpResponse;

mod canister_info;
mod canister_manager;
mod github_manager;
// mod ai_agent;

pub use canister_manager::{
    delete_canister, list_canisters, register_canister, update_canister, view_canister,
    CanisterInfo,
};

pub use canister_info::{canister_info, canister_metadata, canister_transactions, ErrorResponse};

pub use github_manager::{link_repository,  analyze_repository, RepoAnalysisResult, Vulnerability, CodeQualityMetrics};

// pub use ai_agent::analyze_github_repo;

ic_cdk::export_candid!();
