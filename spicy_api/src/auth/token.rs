use core::convert::TryFrom;
use pasetors::{
    claims::{Claims, ClaimsValidationRules},
    errors,
    keys::SymmetricKey,
    local,
    token::UntrustedToken,
    version4::V4,
    Local,
};
use uuid::Uuid;

/// The **issuer** of the `Paseto` tokens.
const TOKEN_ISSUER: &str = "spicy.com";

/// The **subject** for the `refresh` token.
const REFRESH_SUBJECT: &str = "spicy_refresh_token";

/// The **subject** for the `access` token.
const ACCESS_SUBJECT: &str = "spicy_access_token";

/// The kind of a `Paseto` token being **issued** or **verified**.
///
/// In the case in which a new token needs to be issued, the value attached to the variant refers
/// to the user's [`Uuid`] in string format.
///
/// On the other hand, in the case in which a token `verification` process is required,
/// the value attached to the variant refers to the token being *verified*.
pub enum TokenKind {
    Access(String),
    Refresh(String),
}

/// Issues a new `Paseto` token considering a given `user`'s id as a custom claim.
pub fn issue_token(tk: TokenKind) -> Result<String, errors::Error> {
    let app_settings = &crate::settings::AppSettings::get();
    let mut claims = Claims::new()?;
    let (user_id, topts) = match tk {
        TokenKind::Refresh(id) => {
            claims.subject(REFRESH_SUBJECT)?;
            claims.token_identifier(&Uuid::new_v4().to_string())?; // HACK: For redis blacklisting
            (id, &app_settings.token[1])
        }
        TokenKind::Access(id) => {
            claims.subject(ACCESS_SUBJECT)?;
            (id, &app_settings.token[0])
        }
    };

    claims.add_additional("user_id", user_id)?;
    claims.issuer(TOKEN_ISSUER)?;
    claims.audience(&app_settings.frontend_url)?;
    claims.expiration(
        &(chrono::Local::now() + chrono::Duration::minutes(topts.exp_time)).to_rfc3339(),
    )?;

    local::encrypt(
        &SymmetricKey::<V4>::from(topts.secret_key.as_bytes())?,
        &claims,
        None,
        Some(topts.implicit_assert.as_bytes()),
    )
}

/// Verifies a given `Paseto` token and returns its *identifier* (if the token being verified is a
/// **refresh** token) and the custom claim ***user_id*** if it was successfully validated.
pub fn verify_token(tk: TokenKind) -> Result<(Option<Uuid>, Uuid), errors::Error> {
    let app_settings = &crate::settings::AppSettings::get();
    let mut validation_rules = ClaimsValidationRules::new();
    let (token, topts) = match tk {
        TokenKind::Refresh(token) => {
            validation_rules.validate_subject_with(REFRESH_SUBJECT);
            (token, &app_settings.token[1])
        }
        TokenKind::Access(token) => {
            validation_rules.validate_subject_with(ACCESS_SUBJECT);
            (token, &app_settings.token[0])
        }
    };
    validation_rules.validate_issuer_with(TOKEN_ISSUER);
    validation_rules.validate_audience_with(&app_settings.frontend_url);

    let untrusted_token = UntrustedToken::<Local, V4>::try_from(&token)?;
    let trusted_token = local::decrypt(
        &SymmetricKey::<V4>::from(topts.secret_key.as_bytes())?,
        &untrusted_token,
        &validation_rules,
        None,
        Some(topts.implicit_assert.as_bytes()),
    )?;
    let claims = trusted_token.payload_claims().unwrap();

    match (claims.get_claim("jti"), claims.get_claim("user_id")) {
        (Some(ti), Some(ui)) => {
            match (
                Uuid::try_parse(&ti.to_string()),
                Uuid::try_parse(&ui.to_string()),
            ) {
                (Ok(ti), Ok(ui)) => Ok((Some(ti), ui)),
                _ => Err(errors::Error::InvalidClaim),
            }
        }
        (None, Some(ui)) => match Uuid::try_parse(ui.as_str().unwrap()) {
            Ok(ui) => Ok((None, ui)),
            _ => Err(errors::Error::InvalidClaim),
        },
        _ => Err(errors::Error::TokenValidation),
    }
}
