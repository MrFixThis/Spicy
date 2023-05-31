mod common;

use entity::user_profile;
use pretty_assertions::assert_eq;
use sea_orm::{prelude::Date, ActiveValue, MockExecResult};
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
        let new_user_profile = user_profile::Model {
            id: 0,
            user_id: 1,
            phone_number: Some("9817453026".to_owned()),
            birth_date: Some("1990-01-01".parse::<Date>().unwrap()),
            bio: Some("I love cooking and experimenting with new recipes.".to_owned()),
        };

        assert_eq!(
            UserProfileService::create(&db, new_user_profile)
                .await
                .unwrap(),
            user_profile::ActiveModel {
                id: ActiveValue::Unchanged(1),
                user_id: ActiveValue::Unchanged(1),
                phone_number: ActiveValue::Unchanged(Some("9817453026".to_owned())),
                birth_date: ActiveValue::Unchanged(Some("1990-01-01".parse::<Date>().unwrap())),
                bio: ActiveValue::Unchanged(Some(
                    "I love cooking and experimenting with new recipes.".to_owned()
                ))
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
                user_id: 2,
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
                    user_id: 2,
                    phone_number: Some("5128739640".to_owned()),
                    birth_date: Some("1995-02-15".parse::<Date>().unwrap()),
                    bio: Some("Food enthusiast and aspiring chef.".to_owned()),
                },
                user_profile::Model {
                    id: 3,
                    user_id: 3,
                    phone_number: Some("6394872150".to_owned()),
                    birth_date: Some("1988-07-10".parse::<Date>().unwrap()),
                    bio: Some("Passionate about creating delicious meals.".to_owned()),
                }
            ]
        );
    }

    // update
    {
        let updated_user_profile = user_profile::Model {
            id: 1,
            user_id: 1,
            phone_number: Some("2375069814".to_owned()),
            birth_date: Some("1992-04-30".parse::<Date>().unwrap()),
            bio: Some("Cooking is my therapy.".to_owned()),
        };

        assert_eq!(
            UserProfileService::update_by(&db, user_profile::Column::Id, 1, updated_user_profile)
                .await
                .unwrap(),
            user_profile::ActiveModel {
                id: ActiveValue::Unchanged(1),
                user_id: ActiveValue::Unchanged(1),
                phone_number: ActiveValue::Unchanged(Some("2375069814".to_owned())),
                birth_date: ActiveValue::Unchanged(Some("1992-04-30".parse::<Date>().unwrap())),
                bio: ActiveValue::Unchanged(Some("Cooking is my therapy.".to_owned()))
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
