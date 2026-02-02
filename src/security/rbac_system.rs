use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use ring::digest::{Context, SHA256};
use ring::pbkdf2;
use ring::rand::{SecureRandom, SystemRandom};
use ring::aead::{AeadInPlace, LessSafeKey, NONCE_LEN, OpeningKey, UnboundKey, AES_256_GCM, BoundKey, Nonce, NonceSequence, Tag};
use std::num::NonZeroU32;

struct NonceSeq {
    counter: Mutex<u64>,
}

impl NonceSeq {
    fn new() -> Self {
        NonceSeq { counter: Mutex::new(0) }
    }
}

impl NonceSequence for NonceSeq {
    fn advance(&self) -> Result<Nonce, ring::error::Unspecified> {
        let mut counter = self.counter.lock().unwrap();
        let nonce_bytes: [u8; NONCE_LEN] = (*counter).to_be_bytes()[..NONCE_LEN].try_into().unwrap();
        *counter += 1;
        Ok(Nonce::assume_unique_for_key(nonce_bytes))
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Role {
    Host,
    Stakeholder,
    Team,
    Public,
}

struct RBAC {
    roles: HashMap<String, Role>, // user_id -> Role
    data: Arc<Mutex<HashMap<String, Vec<u8>>>>, // key -> encrypted data
    log_file: Arc<Mutex<File>>,
    key: LessSafeKey,
}

impl RBAC {
    fn new(password: &str) -> io::Result<Self> {
        let mut salt = [0u8; 16];
        let rng = SystemRandom::new();
        rng.fill(&mut salt).unwrap();

        let mut pbkdf2_key = [0u8; 32];
        pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA256, NonZeroU32::new(100_000).unwrap(), &salt, password.as_bytes(), &mut pbkdf2_key);

        let unbound_key = UnboundKey::new(&AES_256_GCM, &pbkdf2_key).unwrap();
        let nonce_seq = NonceSeq::new();
        let key = LessSafeKey::new(unbound_key);

        let log_file = OpenOptions::new().create(true).append(true).open("rbac_audit.log")?;

        Ok(RBAC {
            roles: HashMap::new(),
            data: Arc::new(Mutex::new(HashMap::new())),
            log_file: Arc::new(Mutex::new(log_file)),
            key,
        })
    }

    fn assign_role(&mut self, user_id: String, role: Role) {
        self.roles.insert(user_id, role);
        self.log("Assigned role".to_string(), user_id);
    }

    fn store_data(&self, key: String, plaintext: Vec<u8>) {
        let mut data = plaintext.clone();
        let nonce = self.key.algorithm().nonce_len();
        let tag = self.key.seal_in_place_separate_tag(Nonce::assume_unique_for_key([0; NONCE_LEN]), b"", &mut data).unwrap();
        let mut encrypted = data;
        encrypted.extend_from_slice(tag.as_ref());

        let mut store = self.data.lock().unwrap();
        store.insert(key, encrypted);
    }

    fn access_data(&self, user_id: &str, key: &str) -> Option<Vec<u8>> {
        if let Some(role) = self.roles.get(user_id) {
            let access_level = match role {
                Role::Host => 4,
                Role::Stakeholder => 3,
                Role::Team => 2,
                Role::Public => 1,
            };
            if access_level >= Self::required_level(key) {
                let store = self.data.lock().unwrap();
                if let Some(encrypted) = store.get(key) {
                    let mut data = encrypted.clone();
                    let plaintext = self.key.open_in_place(Nonce::assume_unique_for_key([0; NONCE_LEN]), b"", &mut data[AES_256_GCM.tag_len()..]).unwrap();
                    self.log("Accessed data".to_string(), user_id.to_string());
                    return Some(plaintext.to_vec());
                }
            }
        }
        self.log("Access denied".to_string(), user_id.to_string());
        None
    }

    fn required_level(key: &str) -> u8 {
        // Example: based on key prefix
        if key.starts_with("ground") { 4 } else if key.starts_with("stake") { 3 } else if key.starts_with("team") { 2 } else { 1 }
    }

    fn log(&self, action: String, user_id: String) {
        let mut log = self.log_file.lock().unwrap();
        writeln!(log, "{}: {} by {}", chrono::Utc::now(), action, user_id).unwrap();
    }
}

fn main() -> io::Result<()> {
    let mut rbac = RBAC::new("securepass")?;
    rbac.assign_role("user1".to_string(), Role::Host);
    rbac.store_data("ground_truth".to_string(), b"full data".to_vec());
    let data = rbac.access_data("user1", "ground_truth");
    println!("{:?}", data);
    Ok(())
}
