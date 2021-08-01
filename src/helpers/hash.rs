use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString, PasswordVerifier},
    Pbkdf2,
};
use rand_core::OsRng;

pub struct LocalHasher<'a> {
    pub password: &'a str
}

impl<'a> LocalHasher<'a> {
    pub fn new(password: &'a str) -> Self {
        Self {password}
    }

    // this should be refactored to allow dependency injection such that the method of hash is chosen by the caller
    pub fn generate_hash(&self) -> String {
        let pass = self.password.as_bytes();

        let salt = SaltString::generate(&mut OsRng);

        Pbkdf2
            .hash_password_simple(pass, salt.as_ref())
            .unwrap()
            .to_string()
    }

    // this should be refactored to allow dependency injection such that the method of hash is chosen by the caller
    pub fn verify_hash(&self, hash: String) -> bool {
        let parsed_hash = PasswordHash::new(&hash).unwrap();
        let pass = &self.password.as_bytes();

        Pbkdf2.verify_password(pass, &parsed_hash).is_ok()
    }
}

