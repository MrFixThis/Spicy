mod common;

use entity::user;
use pretty_assertions::assert_eq;
use sea_orm::{prelude::Date, ActiveValue};
use service::{MutationRepository, QueryRepository, UserService};

// #[cfg(feature = "mock")]
#[tokio::test]
async fn main() {
    let db = common::build_mock_db().await.unwrap();

    // create
    {
        let new_user = user::Model {
            id: 0,
            email: "user1@example.com".to_owned(),
            password: "password1".to_owned(),
            name: "John".to_owned(),
            surname: "Doe".to_owned(),
            date_joined: "2023-01-01".parse::<Date>().unwrap(),
            is_active: true,
            thumbnail: Some("thumbnail1".to_owned()),
        };

        assert_eq!(
            UserService::create(&db, new_user).await.unwrap(),
            user::ActiveModel {
                id: ActiveValue::Unchanged(1),
                email: ActiveValue::Unchanged("user1@example.com".to_owned()),
                password: ActiveValue::Unchanged("password1".to_owned()),
                name: ActiveValue::Unchanged("John".to_owned()),
                surname: ActiveValue::Unchanged("Doe".to_owned()),
                date_joined: ActiveValue::Unchanged("2023-01-01".parse::<Date>().unwrap()),
                is_active: ActiveValue::Unchanged(true),
                thumbnail: ActiveValue::Unchanged(Some("thumbnail1".to_owned())),
            }
        );
    }

    // select
    {
        let retr_user = UserService::find_by_pk(&db, 2).await.unwrap().unwrap();

        assert_eq!(
            retr_user,
            user::Model {
                id: 2,
                email: "user2@example.com".to_owned(),
                password: "password2".to_owned(),
                name: "Jane".to_owned(),
                surname: "Smith".to_owned(),
                date_joined: "2023-01-02".parse::<Date>().unwrap(),
                is_active: true,
                thumbnail: Some("thumbnail2".to_owned()),
            }
        );
    }

    // select all
    {
        let users = UserService::find_all(&db).await.unwrap();

        assert_eq!(
            users,
            vec![
                user::Model {
                    id: 3,
                    email: "user3@example.com".to_owned(),
                    password: "password3".to_owned(),
                    name: "Alice".to_owned(),
                    surname: "Johnson".to_owned(),
                    date_joined: "2023-01-03".parse::<Date>().unwrap(),
                    is_active: true,
                    thumbnail: Some("thumbnail3".to_owned()),
                }
            ]
        )
    }

    // update
    {
        let updated_user = user::Model {
            id: 4,
            email: "user3@example.com".to_owned(),
            password: "password3".to_owned(),
            name: "John".to_owned(),
            surname: "Doe".to_owned(),
            date_joined: "2023-01-01".parse::<Date>().unwrap(),
            is_active: true,
            thumbnail: Some("thumbnail1".to_owned()),
        };

        assert_eq!(
            UserService::update_by(&db, user::Column::Id, 4, updated_user).await.unwrap(),
            user::ActiveModel {
                id: ActiveValue::Unchanged(4),
                email: ActiveValue::Unchanged("user4@example.com".to_owned()),
                password: ActiveValue::Unchanged("password4".to_owned()),
                name: ActiveValue::Unchanged("Bob".to_owned()),
                surname: ActiveValue::Unchanged("Williams".to_owned()),
                date_joined: ActiveValue::Unchanged("2023-01-04".parse::<Date>().unwrap()),
                is_active: ActiveValue::Unchanged(true),
                thumbnail: ActiveValue::Unchanged(Some("thumbnail4".to_owned())),
            }
        )
    }

    // delete
    {
        assert_eq!(UserService::delete_by_pk(&db, 5).await.unwrap().rows_affected, 1)
    }

    // delete all
    {
        assert_eq!(UserService::delete_all(&db).await.unwrap().rows_affected, 4)
    }

    //TODO: relation ops
}
