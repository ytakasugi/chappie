use argon2::{
    password_hash::{
        self, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Algorithm, Argon2, Params, Version,
};

pub fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

pub fn hash(password: &str, salt: &SaltString) -> Result<String, password_hash::Error> {
    let password = password.as_bytes();

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    );

    let password_hash = argon2.hash_password(password, salt)?.to_string();

    Ok(password_hash)
}

pub fn verify(password: &str, hash: &str) -> Result<bool, password_hash::Error> {
    let password = password.as_bytes();
    let hash = PasswordHash::new(hash)?;

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    );
    
    Ok(argon2.verify_password(password, &hash).is_ok())
}
