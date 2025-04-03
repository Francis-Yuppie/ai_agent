use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{query, update};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
struct UserProfile {
    principal: Principal,
    name: String,
    registered_canisters: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
enum AuthError {
    Unauthorized,
    UserAlreadyExists,
}

struct UserStorage {
    users: HashMap<Principal, UserProfile>,
}

impl UserStorage {
    fn new() -> Self {
        UserStorage {
            users: HashMap::new(),
        }
    }

    fn register_user(&mut self, principal: Principal, name: String) -> Result<(), AuthError> {
        if self.users.contains_key(&principal) {
            return Err(AuthError::UserAlreadyExists);
        }
        
        let profile = UserProfile {
            principal,
            name,
            registered_canisters: Vec::new(),
        };
        
        self.users.insert(principal, profile);
        Ok(())
    }

    fn add_canister(&mut self, principal: Principal, canister_id: Principal) -> Option<()> {
        self.users.get_mut(&principal)?.registered_canisters.push(canister_id);
        Some(())
    }
}

static mut USER_STORAGE: Option<UserStorage> = None;

#[update]
fn register_user(name: String) -> Result<(), AuthError> {
    let caller = ic_cdk::caller();
    unsafe {
        if USER_STORAGE.is_none() {
            USER_STORAGE = Some(UserStorage::new());
        }
        USER_STORAGE.as_mut().unwrap().register_user(caller, name)
    }
}

#[update]
fn register_canister(canister_id: Principal) -> Result<(), String> {
    let caller = ic_cdk::caller();
    unsafe {
        if USER_STORAGE.is_none() {
            return Err("User storage not initialized".to_string());
        }
        USER_STORAGE.as_mut().unwrap().add_canister(caller, canister_id)
            .ok_or("User not found".to_string())
    }
}

#[query]
fn get_my_canisters() -> Result<Vec<Principal>, String> {
    let caller = ic_cdk::caller();
    unsafe {
        USER_STORAGE.as_ref()
            .and_then(|storage| storage.users.get(&caller))
            .map(|profile| profile.registered_canisters.clone())
            .ok_or("User not found".to_string())
    }
}