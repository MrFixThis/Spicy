pub mod password;
pub mod token;

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::settings;

    use super::password::*;
    use super::token::*;

    #[test]
    fn test_password_is_hashed_and_verified_propertly() -> anyhow::Result<()> {
        let password = b"53cur3_P455w0rd";
        let hash = hash_password(password)?;
        verify_password(password, &hash)?;

        Ok(())
    }

    #[test]
    fn test_malformed_hash_is_checked_and_warned() {
        let password = b"53cur3_P455w0rd";
        let hash = "invalid_hash";

        assert!(verify_password(password, hash).is_err());
    }

    #[test]
    fn test_token_is_issued_and_verified_propertly() -> anyhow::Result<()> {
        settings::parse_app_settings()?;
        let user_id: Uuid = Uuid::new_v4();
        let token = issue_token(TokenKind::Access(user_id.to_string()))?;

        _ = verify_token(TokenKind::Access(token))?;

        Ok(())
    }

    #[test]
    fn test_malformed_token_is_checked_and_warned() {
        settings::parse_app_settings().unwrap();
        let token = "v5.local.malformed_token".to_owned();
        assert!(verify_token(TokenKind::Access(token)).is_err());
    }

    // NOTE: Add more tests realted with the refresh tokens
}
