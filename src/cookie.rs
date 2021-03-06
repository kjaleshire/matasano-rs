use rand;
use serde_urlencoded;

use std::collections::HashMap;

use aes;
use utility::error::{Result, ResultExt};

pub struct Cookie {
    pub key: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
pub struct Profile {
    pub email: String,
    pub uid: usize,
    pub role: String,
}

impl Cookie {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        Cookie {
            key: aes::generate_random_aes_key(&mut rng, 16),
        }
    }

    pub fn new_with_key(key: &str) -> Self {
        Cookie {
            key: Vec::from(key),
        }
    }

    pub fn block_size(&self) -> usize {
        16
    }

    pub fn deserialize_cookie(serialized_cookie: &str) -> Result<HashMap<String, String>> {
        serde_urlencoded::from_str(serialized_cookie).chain_err(|| "could not deserialize cookie")
    }

    pub fn serialize_cookie(profile: HashMap<&str, &str>) -> Result<String> {
        serde_urlencoded::to_string(profile).chain_err(|| "could not serialize cookie")
    }

    pub fn deserialize_profile(serialized_profile: &str) -> Result<Profile> {
        serde_urlencoded::from_str(serialized_profile).chain_err(|| "could not deserialize profile")
    }

    pub fn profile_for(&self, email: &str) -> String {
        let sanitized_email = email.replace("=", "").replace("&", "");
        let profile = Profile {
            email: sanitized_email,
            uid: 10,
            role: String::from("user"),
        };

        format!(
            "email={}&uid={}&role={}",
            profile.email, profile.uid, profile.role
        )

        // serde_urlencoded::to_string(profile).chain_err(|| "could not serialize profile")
    }

    pub fn encrypted_profile_for(&self, email: &str) -> Result<Vec<u8>> {
        let profile = self.profile_for(email);
        self.encrypt_cookie(&profile)
    }

    pub fn decrypted_profile_for(&self, encrypted_profile: &[u8]) -> Result<Profile> {
        let serialized_profile = self
            .decrypt_cookie(encrypted_profile)
            .chain_err(|| "could not decrypt cookie")?;
        let string_profile =
            String::from_utf8(serialized_profile).chain_err(|| "could not stringify profile vec");
        Self::deserialize_profile(&string_profile?.clone())
    }

    pub fn decrypt_cookie(&self, cookie: &[u8]) -> Result<Vec<u8>> {
        aes::decrypt_ecb_text(cookie, &self.key)
    }

    pub fn encrypt_cookie(&self, cookie: &str) -> Result<Vec<u8>> {
        aes::encrypt_ecb_text(cookie.as_bytes(), &self.key)
    }
}
