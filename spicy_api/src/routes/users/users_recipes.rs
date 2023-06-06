use actix_multipart::form::MultipartForm;
use actix_web::{http, web, HttpMessage, HttpRequest, HttpResponse};
use entity::{recipe, recipe_image, user};
use service::sea_orm::{self, EntityTrait, TryIntoModel};
use service::{
    sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, TransactionTrait},
    RelationService,
};
use service::{MutationRepository, RecipeService};

use crate::payload::{recipe_payload::NewRecipe, ErrorResponse};

#[actix_web::post("/recipes/")]
async fn save_user_recipe(
    db: web::Data<DatabaseConnection>,
    recipe: MultipartForm<NewRecipe>,
    req: HttpRequest,
) -> HttpResponse {
    let user_id = req.extensions().get::<i32>().unwrap().to_owned();
    let res = db
        .transaction::<_, recipe::Model, DbErr>(move |tnx| {
            Box::pin(async move {
                let (recipe, images) = recipe.into_inner().into_user_recipe(user_id).await;
                match recipe.save(tnx).await {
                    Ok(usr) => {
                        if let Some(images) = images {
                            let assests = images
                                .into_iter()
                                .map(|path| recipe_image::ActiveModel {
                                    id: sea_orm::NotSet,
                                    path: sea_orm::Set(path),
                                    recipe_id: usr.id.to_owned(),
                                })
                                .collect::<Vec<recipe_image::ActiveModel>>();

                            _ = recipe_image::Entity::insert_many(assests).exec(tnx).await?
                        };

                        Ok(usr.try_into_model().unwrap())
                    }
                    Err(err) => Err(err),
                }
            })
        })
        .await;

    match res {
        Ok(rp) => HttpResponse::Ok().json(rp),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error saving recipe",
            Some(err.to_string()),
        )),
    }
}

// TODO: implement recipe updating

#[actix_web::get("/recipes/")]
async fn fetch_user_recipes(db: web::Data<DatabaseConnection>, req: HttpRequest) -> HttpResponse {
    let user_id = req.extensions().get::<i32>().unwrap().to_owned();
    let recipes = RelationService::load_many_by_pk::<user::Entity, _, _>(
        db.as_ref(),
        recipe::Entity,
        user_id,
    )
    .await;

    match recipes {
        Ok(pair) => match pair {
            Some((_, col)) => HttpResponse::Ok().json(col),
            None => HttpResponse::NotFound().json(ErrorResponse::new(
                http::StatusCode::NOT_FOUND,
                "the user does not have related recipes",
                None,
            )),
        },
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error retrieving user's related recipes",
            Some(err.to_string()),
        )),
    }
}

#[actix_web::delete("/recipes/{recipe_id}")]
async fn delete_user_recipe(
    db: web::Data<DatabaseConnection>,
    recipe_id: web::Path<i32>,
) -> HttpResponse {
    match RecipeService::delete_by_pk(db.as_ref(), recipe_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "error deleting user's recipe",
            Some(err.to_string()),
        )),
    }
} // TODO: delete the related persisted images
