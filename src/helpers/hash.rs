use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString, PasswordVerifier},
    Pbkdf2,
};
use rand_core::OsRng;



pub fn generate_hash<'a>(password: &'a str) -> String {
    let pass = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    Pbkdf2
        .hash_password_simple(pass, salt.as_ref())
        .unwrap()
        .to_string()
}

pub fn verify_hash<'a>(password: &'a str, hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&hash).unwrap();
    let pass = password.as_bytes();

    Pbkdf2.verify_password(pass, &parsed_hash).is_ok()
}
