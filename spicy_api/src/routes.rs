pub mod health_check;
mod recipes;
pub mod session;
mod users;

use users::{users_likes::*, users_recipes::*, *};

use crate::middleware::auth_validator;

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

// Admin-level related services over users
route_config! {
    name = admins_config,
    scope_pfx = "/admins",
    services = [
        fetch_all_users,
        toggle_user_state
    ],
    middleware = [ auth_validator ]
}

// Individual users' related services
route_config! {
    name = users_config,
    scope_pfx = "/users",
    services = [
        fetch_user,
        update_user
        // fetch_user_recipes,
        // fetch_user_likes
    ],
    middleware = [ auth_validator ]
}
