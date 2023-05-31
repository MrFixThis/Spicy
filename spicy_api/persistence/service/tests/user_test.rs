mod common;

use entity::user;
use pretty_assertions::assert_eq;
use sea_orm::{prelude::Date, ActiveValue, ColumnTrait, MockExecResult};
use service::{MutationRepository, QueryRepository, UserService};

#[tokio::test]
async fn user_test() {
    let db = common::build_mock_db::<user::Model, _>(
        "user_data.json",
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

    // select for login
    {
        let payload = user::UserPayload {
            email: "user1@example.com".to_owned(),
            password: "password1".to_owned(),
        };

        assert_eq!(
            UserService::find_for_login(&db, &payload.email)
                .await
                .unwrap()
                .unwrap(),
            user::Model {
                id: 1,
                email: "user1@example.com".to_owned(),
                password: "password1".to_owned(),
                name: "John".to_owned(),
                surname: "Doe".to_owned(),
                date_joined: "2023-01-01".parse::<Date>().unwrap(),
                is_active: true,
                thumbnail: Some("thumbnail1".to_owned()),
            }
        )
    }

    // select all
    {
        let retr_users = UserService::find_all(&db).await.unwrap();

        assert_eq!(
            retr_users,
            vec![
                user::Model {
                    id: 1,
                    email: "user1@example.com".to_owned(),
                    password: "password1".to_owned(),
                    name: "John".to_owned(),
                    surname: "Doe".to_owned(),
                    date_joined: "2023-01-01".parse::<Date>().unwrap(),
                    is_active: true,
                    thumbnail: Some("thumbnail1".to_owned()),
                },
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
            ]
        )
    }

    // select all inactive
    {
        let retr_inactive_users = UserService::find_all_by(&db, user::Column::IsActive.eq(false))
            .await
            .unwrap();

        assert_eq!(
            retr_inactive_users,
            vec![user::Model {
                id: 1,
                email: "user1@example.com".to_owned(),
                password: "password1".to_owned(),
                name: "John".to_owned(),
                surname: "Doe".to_owned(),
                date_joined: "2023-01-01".parse().unwrap(),
                is_active: false,
                thumbnail: Some("thumbnail1".to_owned())
            }]
        );
    }

    // update
    {
        let updated_user = user::Model {
            id: 3,
            email: "user3@example.com".to_owned(),
            password: "password3".to_owned(),
            name: "Bob".to_owned(),
            surname: "Williams".to_owned(),
            date_joined: "2023-01-03".parse::<Date>().unwrap(),
            is_active: true,
            thumbnail: Some("thumbnail3".to_owned()),
        };

        assert_eq!(
            UserService::update_by(&db, user::Column::Id, 3, updated_user)
                .await
                .unwrap(),
            user::ActiveModel {
                id: ActiveValue::Unchanged(3),
                email: ActiveValue::Unchanged("user3@example.com".to_owned()),
                password: ActiveValue::Unchanged("password3".to_owned()),
                name: ActiveValue::Unchanged("Bob".to_owned()),
                surname: ActiveValue::Unchanged("Williams".to_owned()),
                date_joined: ActiveValue::Unchanged("2023-01-03".parse::<Date>().unwrap()),
                is_active: ActiveValue::Unchanged(true),
                thumbnail: ActiveValue::Unchanged(Some("thumbnail3".to_owned())),
            }
        )
    }

    // delete
    {
        assert_eq!(
            UserService::delete_by_pk(&db, 4)
                .await
                .unwrap()
                .rows_affected,
            1
        )
    }

    // delete all
    {
        assert_eq!(UserService::delete_all(&db).await.unwrap().rows_affected, 2)
    }
}
