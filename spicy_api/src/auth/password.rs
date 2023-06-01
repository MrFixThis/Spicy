use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

/// Computes the `hash` from a given password using [`Argon2`] with the default configuration
/// and a randomly generated base-64 encoded salt string.
pub fn hash_password(raw_password: &[u8]) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(raw_password, &salt)
        .map(|p| p.to_string())
        .map_err(anyhow::Error::msg)
}

/// Verifies the validness af a given password against an existent password `hash` using
/// [`Argon2`] with the default configuration.
pub fn verify_password(password: &[u8], hash: &str) -> anyhow::Result<()> {
    let parsed_hash = PasswordHash::new(hash).map_err(anyhow::Error::msg)?;
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .map_err(anyhow::Error::msg)
}
