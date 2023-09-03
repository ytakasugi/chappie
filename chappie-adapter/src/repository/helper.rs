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

pub fn verify(password: &str, password_hash: String) -> Result<(), password_hash::Error> {
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    );

    let parsed_hash = PasswordHash::new(&password_hash)?;
    let _veryfy = argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(())
}
