use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString, PasswordVerifier},
    Pbkdf2,
};
use rand_core::OsRng;

pub struct LocalHasher;

impl LocalHasher {
    // this should be refactored to allow dependency injection such that the method of hash is chosen by the caller
    pub fn generate_hash<'a>(password: &'a str) -> String {
        let pass = password.as_bytes();

        let salt = SaltString::generate(&mut OsRng);

        Pbkdf2
            .hash_password_simple(pass, salt.as_ref())
            .unwrap()
            .to_string()
    }

    // this should be refactored to allow dependency injection such that the method of hash is chosen by the caller
    pub fn verify_hash(hashed_password: String, password: String) -> bool {
        let parsed_hash = PasswordHash::new(hashed_password.as_str()).unwrap();
        let pass = password.as_bytes();

        Pbkdf2.verify_password(pass, &parsed_hash).is_ok()
    }
}

