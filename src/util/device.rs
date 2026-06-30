use sha2::{Digest, Sha256};

pub fn get_device_id() -> String {
    hex::encode(Sha256::digest(machine_uid::get().unwrap()))
}
