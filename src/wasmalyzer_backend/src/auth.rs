use ic_cdk::api::caller;
use ic_cdk_macros::query;

#[query]
fn get_user_identity() -> String {
    caller().to_text()
}
