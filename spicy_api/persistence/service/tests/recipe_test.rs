mod common;

use entity::recipe;
use pretty_assertions::assert_eq;
use sea_orm::{
    prelude::{DateTimeUtc, Time, Uuid},
    ActiveValue, ColumnTrait, MockExecResult,
};
use service::{MutationRepository, QueryRepository, RecipeService};

#[tokio::test]
async fn recipe_test() {
    let db = common::build_mock_db::<recipe::Model, _>(
        "recipe_data.json",
        [
            MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 0,
                rows_affected: 2,
            },
            MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            },
        ],
    )
    .unwrap();

    // create
    {
        let new_recipe = recipe::ActiveModel {
            id: ActiveValue::Set(Uuid::parse_str("f67e7a21-c2d0-4aef-9e1d-8e7e1aa6f4a7").unwrap()),
            user_id: ActiveValue::Set(
                Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
            ),
            title: ActiveValue::Set("Pasta Carbonara".to_owned()),
            description: ActiveValue::Set("Classic Italian pasta dish".to_owned()),
            preparation_steps: ActiveValue::Set(
                "1. Cook pasta. \
                2. Fry bacon. \
                3. Beat eggs and mix with cheese. \
                4. Combine pasta, bacon, and egg mixture. \
                5. Serve hot."
                    .to_owned(),
            ),
            cooking_time: ActiveValue::Set("00:30:00".parse::<Time>().unwrap()),
            is_visible: ActiveValue::Set(true),
            created_at: ActiveValue::Set("2023-06-21 12:34:56Z".parse::<DateTimeUtc>().unwrap()),
            updated_at: ActiveValue::Set(None),
        };

        assert_eq!(
            RecipeService::create(&db, new_recipe).await.unwrap(),
            recipe::Model {
                id: Uuid::parse_str("f67e7a21-c2d0-4aef-9e1d-8e7e1aa6f4a7").unwrap(),
                user_id: Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
                title: "Pasta Carbonara".to_owned(),
                description: "Classic Italian pasta dish".to_owned(),
                preparation_steps: "1. Cook pasta. \
                    2. Fry bacon. \
                    3. Beat eggs and mix with cheese. \
                    4. Combine pasta, bacon, and egg mixture. \
                    5. Serve hot."
                    .to_owned(),
                cooking_time: "00:30:00".parse::<Time>().unwrap(),
                is_visible: true,
                created_at: "2023-06-21 12:34:56Z".parse::<DateTimeUtc>().unwrap(),
                updated_at: None,
            }
        );
    }

    // select
    {
        let retr_recipe = RecipeService::find_by_pk(
            &db,
            Uuid::parse_str("1b678ae7-1e12-4e9e-92bc-45a080f2e2e1").unwrap(),
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(
            retr_recipe,
            recipe::Model {
                id: Uuid::parse_str("1b678ae7-1e12-4e9e-92bc-45a080f2e2e1").unwrap(),
                user_id: Uuid::parse_str("7f8e51cd-26e1-4f55-9eab-d7a9dd5b9d24").unwrap(),
                title: "Chicken Curry".to_owned(),
                description: "Spicy and flavorful chicken curry".to_owned(),
                preparation_steps: "1. Heat oil and sauté onions. \
                    2. Add spices and chicken. \
                    3. Cook until chicken is done. \
                    4. Garnish with cilantro. \
                    5. Serve with rice."
                    .to_owned(),
                cooking_time: "01:00:00".parse::<Time>().unwrap(),
                is_visible: false,
                created_at: "2023-06-21 12:34:57Z".parse::<DateTimeUtc>().unwrap(),
                updated_at: None,
            }
        )
    }

    // select all
    {
        let retr_recipes = RecipeService::find_all(&db).await.unwrap();

        assert_eq!(
            retr_recipes,
            vec![
                recipe::Model {
                    id: Uuid::parse_str("1b678ae7-1e12-4e9e-92bc-45a080f2e2e1").unwrap(),
                    user_id: Uuid::parse_str("7f8e51cd-26e1-4f55-9eab-d7a9dd5b9d24").unwrap(),
                    title: "Chicken Curry".to_owned(),
                    description: "Spicy and flavorful chicken curry".to_owned(),
                    preparation_steps: "1. Heat oil and sauté onions. \
                        2. Add spices and chicken. \
                        3. Cook until chicken is done. \
                        4. Garnish with cilantro. \
                        5. Serve with rice."
                        .to_owned(),
                    cooking_time: "01:00:00".parse::<Time>().unwrap(),
                    is_visible: false,
                    created_at: "2023-06-21 12:34:57Z".parse::<DateTimeUtc>().unwrap(),
                    updated_at: None
                },
                recipe::Model {
                    id: Uuid::parse_str("e65e70e1-462d-4a6c-961a-4fcab6d82762").unwrap(),
                    user_id: Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
                    title: "Chocolate Brownies".to_owned(),
                    description: "Rich and fudgy chocolate brownies".to_owned(),
                    preparation_steps: "1. Melt butter and chocolate. \
                        2. Mix in sugar and eggs. \
                        3. Add flour and mix well. \
                        4. Bake in preheated oven. \
                        5. Let cool and cut into squares."
                        .to_owned(),
                    cooking_time: "00:45:00".parse::<Time>().unwrap(),
                    is_visible: true,
                    created_at: "2023-06-21 12:34:58Z".parse::<DateTimeUtc>().unwrap(),
                    updated_at: None
                }
            ]
        );
    }

    // select all not visible
    {
        let retr_not_visible_recipes =
            RecipeService::find_all_by(&db, recipe::Column::IsVisible.eq(false))
                .await
                .unwrap();

        assert_eq!(
            retr_not_visible_recipes,
            vec![recipe::Model {
                id: Uuid::parse_str("e65e70e1-462d-4a6c-961a-4fcab6d82762").unwrap(),
                user_id: Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
                title: "Chocolate Brownies".to_owned(),
                description: "Rich and fudgy chocolate brownies".to_owned(),
                preparation_steps: "1. Melt butter and chocolate. \
                    2. Mix in sugar and eggs. \
                    3. Add flour and mix well. \
                    4. Bake in preheated oven. \
                    5. Let cool and cut into squares."
                    .to_owned(),
                cooking_time: "00:45:00".parse::<Time>().unwrap(),
                is_visible: false,
                created_at: "2023-06-21 12:34:58Z".parse::<DateTimeUtc>().unwrap(),
                updated_at: None
            }]
        );
    }

    // update
    {
        let updated_recipe = recipe::ActiveModel {
            id: ActiveValue::Unchanged(
                Uuid::parse_str("e65e70e1-462d-4a6c-961a-4fcab6d82762").unwrap(),
            ),
            user_id: ActiveValue::Unchanged(
                Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
            ),
            title: ActiveValue::Unchanged("Chocolate Brownies".to_owned()),
            description: ActiveValue::Unchanged("Rich and fudgy chocolate brownies".to_owned()),
            preparation_steps: ActiveValue::Unchanged(
                "1. Melt butter and chocolate. \
                    2. Mix in sugar and eggs. \
                    3. Add flour and mix well. \
                    4. Bake in preheated oven. \
                    5. Let cool and cut into squares."
                    .to_owned(),
            ),
            cooking_time: ActiveValue::Unchanged("00:45:00".parse::<Time>().unwrap()),
            is_visible: ActiveValue::Unchanged(true),
            created_at: ActiveValue::Unchanged(
                "2023-06-21 12:34:58Z".parse::<DateTimeUtc>().unwrap(),
            ),
            updated_at: ActiveValue::Unchanged(None),
        };

        assert_eq!(
            RecipeService::update(&db, updated_recipe).await.unwrap(),
            recipe::Model {
                id: Uuid::parse_str("e65e70e1-462d-4a6c-961a-4fcab6d82762").unwrap(),
                user_id: Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
                title: "Chocolate Brownies".to_owned(),
                description: "Rich and fudgy chocolate brownies".to_owned(),
                preparation_steps: "1. Melt butter and chocolate. \
                    2. Mix in sugar and eggs. \
                    3. Add flour and mix well. \
                    4. Bake in preheated oven. \
                    5. Let cool and cut into squares."
                    .to_owned(),
                cooking_time: "00:45:00".parse::<Time>().unwrap(),
                is_visible: true,
                created_at: "2023-06-21 12:34:58Z".parse::<DateTimeUtc>().unwrap(),
                updated_at: Some("2023-06-21 12:40:00Z".parse::<DateTimeUtc>().unwrap()),
            }
        );
    }

    // delete
    {
        assert_eq!(
            RecipeService::delete_by_pk(
                &db,
                Uuid::parse_str("9861c2b1-71f1-4c68-bd03-59e0d8c0e369").unwrap()
            )
            .await
            .unwrap()
            .rows_affected,
            1
        );
    }

    // delete all
    {
        assert_eq!(
            RecipeService::delete_all(&db).await.unwrap().rows_affected,
            2
        );
    }

    // delete all not visible
    {
        assert_eq!(
            RecipeService::delete_all_by(&db, recipe::Column::IsVisible.eq(false))
                .await
                .unwrap()
                .rows_affected,
            1
        );
    }
}
