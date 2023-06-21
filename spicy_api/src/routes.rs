mod auditing;
pub mod health_check;
mod recipes;
pub mod sessions;
mod users;

use users::{users_likes::*, users_recipes::*, *};

use crate::middleware::token_validator;

macro_rules! route_config {
    {
        name=$name:ident,
        scope_pfx=$pfx:expr,
        services=[ $( $srv:ident ),+ ]
        $( ,configs=[ $( $conf:ident ),+ ] )?
        $( ,middleware=[ $( $mw:ident ),+ ] )?
    } => {
        pub fn $name(cfg: &mut ::actix_web::web::ServiceConfig) {
            cfg.service(
                ::actix_web::web::scope($pfx)
                    $( .service($srv) )+
                    $( $( .configure($conf) )+ )?
                    $( $( .wrap(::actix_web_lab::middleware::from_fn($mw)) )+ )?
            );
        }
    };
}

route_config! {
    name = users_config,
    scope_pfx = "/users",
    services = [
        fetch_user,
        fetch_all_users,
        update_user,
        delete_user,
        fetch_user_recipes,
        fetch_user_likes
    ],
    middleware = [ token_validator ]
}
