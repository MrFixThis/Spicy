use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    http::header,
    HttpMessage,
};
use actix_web_lab::middleware::Next;

use crate::auth;

pub async fn token_validator(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    if let Some(token) = req.headers().get(header::AUTHORIZATION) {
        let token = token.to_str().unwrap().to_owned();
        match auth::token::verify_token(token) {
            Ok(cl) => {
                let user_id = cl
                    .unwrap()
                    .get_claim("user_id")
                    .and_then(|v| v.as_i64())
                    .unwrap();

                req.extensions_mut().insert(user_id as i32);
                next.call(req).await
            }
            Err(_) => Err(ErrorUnauthorized("invalid token")),
        }
    } else {
        Err(ErrorUnauthorized("not allowed to access the resource"))
    }
}
