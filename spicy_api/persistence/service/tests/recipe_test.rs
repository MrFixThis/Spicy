mod common;

use entity::recipe;
use pretty_assertions::assert_eq;
use sea_orm::{
    prelude::{Date, Time},
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
        let new_recipe = recipe::Model {
            id: 0,
            title: "Pasta Carbonara".to_owned(),
            date_created: "2023-01-01".parse::<Date>().unwrap(),
            description: "Classic Italian pasta dish".to_owned(),
            preparation_steps: "1. Cook pasta. \
                2. Fry bacon. \
                3. Beat eggs and mix with cheese. \
                4. Combine pasta, bacon, and egg mixture. \
                5. Serve hot."
                .to_owned(),
            cooking_time: "00:30:00".parse::<Time>().unwrap(),
            is_visible: true,
            user_id: 1,
        };

        assert_eq!(
            RecipeService::create(&db, new_recipe).await.unwrap(),
            recipe::ActiveModel {
                id: ActiveValue::Unchanged(1),
                title: ActiveValue::Unchanged("Pasta Carbonara".to_owned()),
                date_created: ActiveValue::Unchanged("2023-01-01".parse::<Date>().unwrap()),
                description: ActiveValue::Unchanged("Classic Italian pasta dish".to_owned()),
                preparation_steps: ActiveValue::Unchanged(
                    "1. Cook pasta. \
                    2. Fry bacon. \
                    3. Beat eggs and mix with cheese. \
                    4. Combine pasta, bacon, and egg mixture. \
                    5. Serve hot."
                        .to_owned()
                ),
                cooking_time: ActiveValue::Unchanged("00:30:00".parse::<Time>().unwrap()),
                is_visible: ActiveValue::Unchanged(true),
                user_id: ActiveValue::Unchanged(1),
            }
        );
    }

    // select
    {
        let retr_recipe = RecipeService::find_by_pk(&db, 2).await.unwrap().unwrap();

        assert_eq!(
            retr_recipe,
            recipe::Model {
                id: 2,
                title: "Chicken Curry".to_owned(),
                date_created: "2023-01-02".parse::<Date>().unwrap(),
                description: "Spicy and flavorful chicken curry".to_owned(),
                preparation_steps: "1. Heat oil and sauté onions. \
                    2. Add spices and chicken. \
                    3. Cook until chicken is done. \
                    4. Garnish with cilantro. \
                    5. Serve with rice."
                    .to_owned(),
                cooking_time: "01:00:00".parse::<Time>().unwrap(),
                is_visible: false,
                user_id: 2
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
                    id: 2,
                    title: "Chicken Curry".to_owned(),
                    date_created: "2023-01-02".parse::<Date>().unwrap(),
                    description: "Spicy and flavorful chicken curry".to_owned(),
                    preparation_steps: "1. Heat oil and sauté onions. \
                        2. Add spices and chicken. \
                        3. Cook until chicken is done. \
                        4. Garnish with cilantro. \
                        5. Serve with rice."
                        .to_owned(),
                    cooking_time: "01:00:00".parse::<Time>().unwrap(),
                    is_visible: false,
                    user_id: 2
                },
                recipe::Model {
                    id: 3,
                    title: "Chocolate Brownies".to_owned(),
                    date_created: "2023-01-03".parse::<Date>().unwrap(),
                    description: "Rich and fudgy chocolate brownies".to_owned(),
                    preparation_steps: "1. Melt butter and chocolate. \
                        2. Mix in sugar and eggs. \
                        3. Add flour and mix well. \
                        4. Bake in preheated oven. \
                        5. Let cool and cut into squares."
                        .to_owned(),
                    cooking_time: "00:45:00".parse::<Time>().unwrap(),
                    is_visible: true,
                    user_id: 3
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
                id: 3,
                title: "Chocolate Brownies".to_owned(),
                date_created: "2023-01-03".parse::<Date>().unwrap(),
                description: "Rich and fudgy chocolate brownies".to_owned(),
                preparation_steps: "1. Melt butter and chocolate. \
                        2. Mix in sugar and eggs. \
                        3. Add flour and mix well. \
                        4. Bake in preheated oven. \
                        5. Let cool and cut into squares."
                    .to_owned(),
                cooking_time: "00:45:00".parse::<Time>().unwrap(),
                is_visible: false,
                user_id: 3
            }]
        );
    }

    // update
    {
        let updated_recipe = recipe::Model {
            id: 3,
            title: "Chocolate Brownies with Nuts".to_owned(),
            date_created: "2023-01-03".parse::<Date>().unwrap(),
            description: "Rich and fudgy chocolate brownies nuts".to_owned(),
            preparation_steps: "1. Melt butter and chocolate. \
                2. Mix in sugar and eggs. \
                3. Add flour and nut, then mix well. \
                4. Bake in preheated oven. \
                5. Let cool and cut into squares."
                .to_owned(),
            cooking_time: "00:45:00".parse::<Time>().unwrap(),
            is_visible: true,
            user_id: 3,
        };

        assert_eq!(
            RecipeService::update_by(&db, recipe::Column::Id, 3, updated_recipe)
                .await
                .unwrap(),
            recipe::ActiveModel {
                id: ActiveValue::Unchanged(3),
                title: ActiveValue::Unchanged("Chocolate Brownies with Nuts".to_owned()),
                date_created: ActiveValue::Unchanged("2023-01-03".parse::<Date>().unwrap()),
                description: ActiveValue::Unchanged(
                    "Rich and fudgy chocolate brownies nuts".to_owned()
                ),
                preparation_steps: ActiveValue::Unchanged(
                    "1. Melt butter and chocolate. \
                    2. Mix in sugar and eggs. \
                    3. Add flour and nut, then mix well. \
                    4. Bake in preheated oven. \
                    5. Let cool and cut into squares."
                        .to_owned()
                ),
                cooking_time: ActiveValue::Unchanged("00:45:00".parse::<Time>().unwrap()),
                is_visible: ActiveValue::Unchanged(true),
                user_id: ActiveValue::Unchanged(3),
            }
        );
    }

    // delete
    {
        assert_eq!(
            RecipeService::delete_by_pk(&db, 4)
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
