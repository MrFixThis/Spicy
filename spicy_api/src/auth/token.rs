use core::convert::TryFrom;
use pasetors::{
    claims::{Claims, ClaimsValidationRules},
    keys::SymmetricKey,
    local,
    token::{TrustedToken, UntrustedToken},
    version4::V4,
    Local,
};
use uuid::Uuid;

/// The length of the `secret key` required to encrypt/decrypt the tokens for the **local** purpose.
/// see [Algorithm Lucidity](https://github.com/paseto-standard/paseto-spec/blob/master/docs/02-Implementation-Guide/03-Algorithm-Lucidity.md).
const SECRET_KEY_LENGTH: usize = 32;

/// Issues a new `Passeto` considering a given `user`'s id as a custom claim.
pub fn issue_token(user_id: u32) -> anyhow::Result<String> {
    let token_settings = crate::settings::load_app_settings()?.token;
    let sk = secret_key_checked_build(token_settings.secret_key.as_bytes())?;
    let mut claims = Claims::new()?;
    claims.non_expiring(); // NOTE: Temporal setting. Make the tokens expiring-tokens
    claims.token_identifier(&Uuid::new_v4().to_string())?;
    claims.add_additional("user_id", user_id)?;
    // let exp_time = chrono::Local::now() + chrono::Duration::minutes(token_settings.exp_time);
    // claims.expiration(&exp_time.to_rfc3339())?;

    local::encrypt(
        &sk,
        &claims,
        None,
        Some(token_settings.implicit_assert.as_bytes()),
    )
    .map_err(|e| anyhow::Error::msg(format!("TokenIssue: {e}")))
}

/// Verifies a given `Paseto` token and returns its [`TrustedToken`]
/// representation if it was successfully validated.
pub fn verify_token(token: String) -> anyhow::Result<TrustedToken> {
    let token_settings = crate::settings::load_app_settings()?.token;
    let sk = secret_key_checked_build(token_settings.secret_key.as_bytes())?;
    let mut validator_rules = ClaimsValidationRules::new();
    validator_rules.allow_non_expiring();

    let untrusted_token = UntrustedToken::<Local, V4>::try_from(&token)?;
    local::decrypt(
        &sk,
        &untrusted_token,
        &validator_rules,
        None,
        Some(token_settings.implicit_assert.as_bytes()),
    )
    .map_err(|e| anyhow::Error::msg(format!("TokenVerification: {e}")))
}

fn secret_key_checked_build(sk: &[u8]) -> anyhow::Result<SymmetricKey<V4>> {
    match sk.len() {
        SECRET_KEY_LENGTH => Ok(SymmetricKey::<V4>::from(sk)?),
        _ => Err(anyhow::Error::msg(
            "invalid secret-key. length must be 256 bits or 32 bytes",
        )),
    }
}
