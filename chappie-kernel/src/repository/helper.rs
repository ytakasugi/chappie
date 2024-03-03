use argon2::{
    password_hash::{
        self, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Algorithm, Argon2, Params, Version,
};

pub fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

pub fn hash(password: &str, salt: &SaltString) -> anyhow::Result<String, password_hash::Error> {
    let password = password.as_bytes();

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    );

    let password_hash = argon2.hash_password(password, salt)?.to_string();

    Ok(password_hash)
}

pub fn verify(password: &str, hash: &str) -> anyhow::Result<bool, password_hash::Error> {
    let password = password.as_bytes();
    let hash = PasswordHash::new(hash)?;

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    );

    Ok(argon2.verify_password(password, &hash).is_ok())
}

#[cfg(test)]
mod test {
    use super::{generate_salt, hash, verify};

    #[test]
    fn test_password_hash() {
        let password = "password123";
        let salt = generate_salt();
        let password_hash = hash(password, &salt);

        assert!(password_hash.is_ok());
    }

    #[test]
    fn test_password_verify() {
        let password = "password123";
        let salt = generate_salt();
        let password_hash = hash(password, &salt).unwrap();

        let veryfied_password = verify(password, &password_hash);
        assert_eq!(veryfied_password.unwrap(), true);
    }
}
