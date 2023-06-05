pub mod password;
pub mod token;

#[cfg(test)]
mod tests {
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
        const USER_ID: i32 = 5;
        let token = issue_token(USER_ID)?;

        _ = verify_token(token)?;

        Ok(())
    }

    #[test]
    fn test_malformed_token_is_checked_and_warned() {
        let token = "v5.local.malformed_token".to_owned();
        assert!(verify_token(token).is_err());
    }
}
