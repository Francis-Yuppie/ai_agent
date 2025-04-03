use candid::{CandidType, Decode, Encode};
use ic_cdk::api::caller;
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{borrow::Cow, cell::RefCell};


type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(CandidType, Clone, Serialize, Deserialize, Default)]
pub struct CanisterInfo {
    id: u64,
    user_id: String,
    canister_id: String,
    name: String,
    created_at: u64,
    updated_at: u64,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub errors: std::collections::HashMap<String, String>,
}

impl ErrorResponse {
    pub fn new(field: &str, message: &str) -> Self {
        let mut errors = HashMap::new();
        errors.insert(field.to_string(), message.to_string());
        Self { errors }
    }
}

// impl Storable for CanisterInfo {
//     fn to_bytes(&self) -> Cow<[u8]> {
//         Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }
// }
impl Storable for CanisterInfo {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|_| {
            CanisterInfo {
                id: 0,
                user_id: "".to_string(),
                canister_id: "".to_string(),
                name: "Unknown".to_string(), // Prevents text decoding errors
                created_at: 0,
                updated_at: 0,
            }
        })
    }
}

impl BoundedStorable for CanisterInfo {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STORAGE: RefCell<StableBTreeMap<u64, CanisterInfo, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );
}

#[ic_cdk::update]
pub fn register_canister(canister_id: String, name: String) -> Result<CanisterInfo, ErrorResponse> {
    if canister_id.is_empty() {
        return Err(ErrorResponse::new("canister_id", "Canister ID is required"));
    }
    if name.is_empty() {
        return Err(ErrorResponse::new("name", "Name is required"));
    }

    let user_id = caller().to_text();

    let exists = STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .any(|(_, c)| c.canister_id == canister_id)
    });

    if exists {
        return Err(ErrorResponse::new("canister_id", "Canister already exists"));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment id counter");

    let canister_info = CanisterInfo {
        id,
        user_id,
        canister_id,
        name,
        created_at: time(),
        updated_at: time(),
    };

    do_insert(id, &canister_info);
    Ok(canister_info)
}

#[ic_cdk::query]
pub fn list_canisters() -> Vec<CanisterInfo> {
    let user_id = caller().to_text();

    STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter_map(|(_, c)| {
                if c.user_id == user_id {
                    Some(CanisterInfo {
                        id: c.id,
                        user_id: c.user_id.clone(),
                        canister_id: c.canister_id.clone(),
                        name: c.name.clone(),
                        created_at: c.created_at,
                        updated_at: c.updated_at,
                    })
                } else {
                    None
                }
            })
            .collect()
    })
}

// pub fn list_canisters() -> Vec<CanisterInfo> {
//     STORAGE.with(|service| {
//         service
//             .borrow()
//             .iter()
//             .map(|(_, c)| CanisterInfo {
//                 id: c.id,
//                 user_id: c.user_id.clone(),
//                 canister_id: c.canister_id.clone(),
//                 name: c.name.clone_or_default(), // Handle missing values
//                 created_at: c.created_at,
//                 updated_at: c.updated_at,
//             })
//             .collect()
//     })
// }

#[ic_cdk::query]
pub fn view_canister(canister_id: String) -> Result<CanisterInfo, ErrorResponse> {
    if canister_id.is_empty() {
        return Err(ErrorResponse::new("canister_id", "Canister ID is required"));
    }

    STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .find(|(_, c)| c.canister_id == canister_id)
            .map(|(_, c)| Ok(c.clone()))
            .unwrap_or_else(|| Err(ErrorResponse::new("canister_id", "Canister does not exist")))
    })
}

#[ic_cdk::update]
pub fn update_canister(canister_id: String, new_name: String) -> Result<bool, ErrorResponse> {
    if canister_id.is_empty() {
        return Err(ErrorResponse::new("canister_id", "Canister ID is required"));
    }
    if new_name.is_empty() {
        return Err(ErrorResponse::new("name", "New name is required"));
    }

    let timestamp = time();
    STORAGE.with(|service| {
        let storage = service.borrow();

        let key_to_update = storage
            .iter()
            .find(|(_, c)| c.canister_id == canister_id)
            .map(|(id, _)| id);

        drop(storage);

        if let Some(id) = key_to_update {
            let mut storage = service.borrow_mut();
            if let Some(mut canister) = storage.get(&id) {
                canister.name = new_name;
                canister.updated_at = timestamp;
                storage.insert(id, canister);
                return Ok(true);
            }
        }
        Err(ErrorResponse::new("canister_id", "Canister does not exist"))
    })
}

#[ic_cdk::update]
pub fn delete_canister(canister_id: String) -> Result<bool, ErrorResponse> {
    if canister_id.trim().is_empty() {
        return Err(ErrorResponse::new("canister_id", "Canister ID is required"));
    }

    let keys_to_remove: Vec<u64> = STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, c)| c.canister_id == canister_id)
            .map(|(id, _)| id)
            .collect()
    });

    if keys_to_remove.is_empty() {
        return Err(ErrorResponse::new("canister_id", "Canister does not exist"));
    }

    STORAGE.with(|service| {
        let mut storage = service.borrow_mut();
        for key in &keys_to_remove {
            storage.remove(key);
        }
    });

    Ok(true)
}

fn do_insert(id: u64, canister: &CanisterInfo) {
    STORAGE.with(|service| service.borrow_mut().insert(id, canister.clone()));
}

#[ic_cdk::update]
pub fn clear_storage() {
    STORAGE.with(|service| {
        let keys: Vec<u64> = service.borrow().iter().map(|(key, _)| key).collect();
        let mut storage = service.borrow_mut();
        for key in keys {
            storage.remove(&key);
        }
    });
}

// ic_cdk::export_candid!();
