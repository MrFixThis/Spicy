use actix_web::{
    body::MessageBody,
    cookie::Cookie,
    dev::{ServiceRequest, ServiceResponse},
    http::header,
    HttpMessage,
};
use actix_web_lab::middleware::Next;
use pasetors::errors;

use crate::{
    auth::token::{self, TokenKind},
    error,
};

pub async fn auth_validator(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    if let Some(token) = req.headers().get(header::AUTHORIZATION) {
        if let Some(token) = token.to_str().ok() {
            match token::verify_token(TokenKind::Access(token.to_owned())) {
                Ok((_, user_id)) => {
                    req.extensions_mut().insert(user_id);
                    return next.call(req).await;
                }
                Err(err) => match err {
                    errors::Error::TokenValidation => return check_refresh_token(req, next).await,
                    _ => {
                        return Err(
                            error::Error::AuthError("invalid access token".to_string()).into()
                        )
                    }
                },
            }
        }
    }

    Err(error::Error::AuthError("no access token".to_string()).into())
}

async fn check_refresh_token<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<B>, actix_web::Error>
where
    B: MessageBody,
{
    if let Some(cookie) = req.cookie("refresh_token") {
        if let Some((_, user_id)) =
            token::verify_token(TokenKind::Refresh(cookie.value().to_string())).ok()
        {
            req.extensions_mut().insert(user_id);
            let mut res = next.call(req).await;
            res.iter_mut().for_each(|sr| {
                _ = sr.response_mut().add_removal_cookie(&Cookie::new(
                    "access_token",
                    token::issue_token(TokenKind::Access(user_id.to_string())).unwrap(),
                ));
            });

            return res;
        }
    }

    Err(error::Error::AuthError("session expired".to_string()).into())
}
