mod common;

use entity::user_profile;
use pretty_assertions::assert_eq;
use sea_orm::{
    prelude::{Date, Uuid},
    ActiveValue, MockExecResult,
};
use service::{MutationRepository, QueryRepository, UserProfileService};

#[tokio::test]
async fn user_profile_test() {
    let db = common::build_mock_db::<user_profile::Model, _>(
        "user_profile_data.json",
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
        ],
    )
    .unwrap();

    // create
    {
        let new_user_profile = user_profile::ActiveModel {
            id: ActiveValue::NotSet,
            user_id: ActiveValue::Set(
                Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
            ),
            phone_number: ActiveValue::Set(Some("9817453026".to_owned())),
            birth_date: ActiveValue::Set(Some("1990-01-01".parse::<Date>().unwrap())),
            bio: ActiveValue::Set(Some(
                "I love cooking and experimenting with new recipes.".to_owned(),
            )),
        };

        assert_eq!(
            UserProfileService::create(&db, new_user_profile)
                .await
                .unwrap(),
            user_profile::Model {
                id: 1,
                user_id: Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
                phone_number: Some("9817453026".to_owned()),
                birth_date: Some("1990-01-01".parse::<Date>().unwrap()),
                bio: Some("I love cooking and experimenting with new recipes.".to_owned()),
            }
        );
    }

    // select
    {
        let retr_user_profile = UserProfileService::find_by_pk(&db, 2)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            retr_user_profile,
            user_profile::Model {
                id: 2,
                user_id: Uuid::parse_str("7f8e51cd-26e1-4f55-9eab-d7a9dd5b9d24").unwrap(),
                phone_number: Some("5128739640".to_owned()),
                birth_date: Some("1995-02-15".parse::<Date>().unwrap()),
                bio: Some("Food enthusiast and aspiring chef.".to_owned()),
            }
        )
    }

    // select all
    {
        let retrv_user_prfiles = UserProfileService::find_all(&db).await.unwrap();

        assert_eq!(
            retrv_user_prfiles,
            vec![
                user_profile::Model {
                    id: 2,
                    user_id: Uuid::parse_str("7f8e51cd-26e1-4f55-9eab-d7a9dd5b9d24").unwrap(),
                    phone_number: Some("5128739640".to_owned()),
                    birth_date: Some("1995-02-15".parse::<Date>().unwrap()),
                    bio: Some("Food enthusiast and aspiring chef.".to_owned()),
                },
                user_profile::Model {
                    id: 3,
                    user_id: Uuid::parse_str("9861c2b1-71f1-4c68-bd03-59e0d8c0e369").unwrap(),
                    phone_number: Some("6394872150".to_owned()),
                    birth_date: Some("1988-07-10".parse::<Date>().unwrap()),
                    bio: Some("Passionate about creating delicious meals.".to_owned()),
                }
            ]
        );
    }

    // update
    {
        let updated_user_profile = user_profile::ActiveModel {
            id: ActiveValue::Unchanged(1),
            user_id: ActiveValue::Unchanged(
                Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
            ),
            phone_number: ActiveValue::Unchanged(Some("9817453026".to_owned())),
            birth_date: ActiveValue::Unchanged(Some("1990-01-01".parse::<Date>().unwrap())),
            bio: ActiveValue::Unchanged(Some(
                "I love cooking and experimenting with new recipes.".to_owned(),
            )),
        };

        assert_eq!(
            UserProfileService::update(&db, updated_user_profile)
                .await
                .unwrap(),
            user_profile::Model {
                id: 1,
                user_id: Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
                phone_number: Some("9817453026".to_owned()),
                birth_date: Some("1990-01-01".parse::<Date>().unwrap()),
                bio: Some("I love cooking and experimenting with new recipes.".to_owned()),
            }
        )
    }

    // delete
    {
        assert_eq!(
            UserProfileService::delete_by_pk(&db, 5)
                .await
                .unwrap()
                .rows_affected,
            1
        );
    }

    // delete all
    {
        assert_eq!(
            UserProfileService::delete_all(&db)
                .await
                .unwrap()
                .rows_affected,
            2
        );
    }
}
