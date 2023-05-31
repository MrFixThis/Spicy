mod common;

use entity::{likes, recipe, recipe_image, user};
use pretty_assertions::assert_eq;
use sea_orm::{
    prelude::Date, DatabaseBackend, DatabaseConnection, MockDatabase, MockExecResult, ModelTrait,
};
use service::RelationService;

#[tokio::test]
async fn recipe_relations_test() {
    let db = common::build_mock_db_from(|| {
        Ok(MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(common::parse_query_result::<user::Model>(
                "relations/recipe_user_data.json",
            )?)
            // .append_query_results(common::parse_query_result::<recipe_image::Model>(
            //     "relations/recipe_image_data.json",
            // )?)
            // .append_query_results(common::parse_query_result::<likes::Model>(
            //     "relations/likes_data.json",
            // )?)
            .into_connection())
    })
    .unwrap();

    // load one-to-one [user]
    {
        let user = RelationService::load_one_by_pk::<user::Entity, recipe::Entity, _, _>(
            &db,
            user::Entity,
            1,
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(
            user,
            user::Model {
                id: 1,
                email: "user1@example.com".to_owned(),
                password: "password1".to_owned(),
                name: "John".to_owned(),
                surname: "Doe".to_owned(),
                date_joined: "2023-01-01".parse::<Date>().unwrap(),
                is_active: true,
                thumbnail: Some("thumbnail1".to_owned())
            }
        );
    }

    // load one-to-many [image]
    {}

    // load one-to-many [likes]
    {}
}
