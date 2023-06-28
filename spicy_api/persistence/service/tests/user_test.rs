mod common;

use entity::user;
use pretty_assertions::assert_eq;
use sea_orm::{
    prelude::{DateTimeUtc, Uuid},
    ActiveValue, ColumnTrait, MockExecResult,
};
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
        let new_user = user::ActiveModel {
            id: ActiveValue::Unchanged(
                Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
            ),
            email: ActiveValue::Set("user1@example.com".to_owned()),
            password: ActiveValue::Set("password1".to_owned()),
            name: ActiveValue::Set("John".to_owned()),
            surname: ActiveValue::Set("Doe".to_owned()),
            is_admin: ActiveValue::Set(false),
            is_active: ActiveValue::Set(true),
            thumbnail: ActiveValue::Set(Some("thumbnail1".to_owned())),
            created_at: ActiveValue::Set("2023-06-21 12:34:56Z".parse::<DateTimeUtc>().unwrap()),
            updated_at: ActiveValue::Set(None),
        };

        assert_eq!(
            UserService::create(&db, new_user).await.unwrap(),
            user::Model {
                id: Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
                email: "user1@example.com".to_owned(),
                password: "password1".to_owned(),
                name: "John".to_owned(),
                surname: "Doe".to_owned(),
                is_admin: false,
                is_active: true,
                thumbnail: Some("thumbnail1".to_owned()),
                created_at: "2023-06-21 12:34:56Z".parse::<DateTimeUtc>().unwrap(),
                updated_at: None,
            }
        );
    }

    // select
    {
        let retr_user = UserService::find_by_pk(
            &db,
            Uuid::parse_str("7f8e51cd-26e1-4f55-9eab-d7a9dd5b9d24").unwrap(),
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(
            retr_user,
            user::Model {
                id: Uuid::parse_str("7f8e51cd-26e1-4f55-9eab-d7a9dd5b9d24").unwrap(),
                email: "user2@example.com".to_owned(),
                password: "password2".to_owned(),
                name: "Jane".to_owned(),
                surname: "Smith".to_owned(),
                is_admin: false,
                is_active: true,
                thumbnail: Some("thumbnail2".to_owned()),
                created_at: "2023-06-21 12:34:57Z".parse::<DateTimeUtc>().unwrap(),
                updated_at: None,
            }
        );
    }

    // select all
    {
        let retr_users = UserService::find_all(&db).await.unwrap();

        assert_eq!(
            retr_users,
            vec![
                user::Model {
                    id: Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
                    email: "user1@example.com".to_owned(),
                    password: "password1".to_owned(),
                    name: "John".to_owned(),
                    surname: "Doe".to_owned(),
                    is_admin: false,
                    is_active: true,
                    thumbnail: Some("thumbnail1".to_owned()),
                    created_at: "2023-06-21 12:34:56Z".parse::<DateTimeUtc>().unwrap(),
                    updated_at: None,
                },
                user::Model {
                    id: Uuid::parse_str("7f8e51cd-26e1-4f55-9eab-d7a9dd5b9d24").unwrap(),
                    email: "user2@example.com".to_owned(),
                    password: "password2".to_owned(),
                    name: "Jane".to_owned(),
                    surname: "Smith".to_owned(),
                    is_admin: false,
                    is_active: true,
                    thumbnail: Some("thumbnail2".to_owned()),
                    created_at: "2023-06-21 12:34:57Z".parse::<DateTimeUtc>().unwrap(),
                    updated_at: None,
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
                id: Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap(),
                email: "user1@example.com".to_owned(),
                password: "password1".to_owned(),
                name: "John".to_owned(),
                surname: "Doe".to_owned(),
                is_admin: false,
                is_active: false,
                thumbnail: Some("thumbnail1".to_owned()),
                created_at: "2023-06-21 12:34:56Z".parse::<DateTimeUtc>().unwrap(),
                updated_at: None,
            }]
        );
    }

    // update
    {
        let updated_user = user::ActiveModel {
            id: ActiveValue::Unchanged(
                Uuid::parse_str("9861c2b1-71f1-4c68-bd03-59e0d8c0e369").unwrap(),
            ),
            email: ActiveValue::Unchanged("user3@example.com".to_owned()),
            password: ActiveValue::Unchanged("password3".to_owned()),
            name: ActiveValue::Unchanged("Bob".to_owned()),
            surname: ActiveValue::Unchanged("Williams".to_owned()),
            is_admin: ActiveValue::Set(true),
            is_active: ActiveValue::Set(true),
            thumbnail: ActiveValue::Unchanged(Some("thumbnail3".to_owned())),
            created_at: ActiveValue::Unchanged(
                "2023-06-21 12:34:59Z".parse::<DateTimeUtc>().unwrap(),
            ),
            updated_at: ActiveValue::Unchanged(None),
        };

        assert_eq!(
            UserService::update(&db, updated_user).await.unwrap(),
            user::Model {
                id: Uuid::parse_str("9861c2b1-71f1-4c68-bd03-59e0d8c0e369").unwrap(),
                email: "user3@example.com".to_owned(),
                password: "password3".to_owned(),
                name: "Bob".to_owned(),
                surname: "Williams".to_owned(),
                is_admin: true,
                is_active: true,
                thumbnail: Some("thumbnail3".to_owned()),
                created_at: "2023-06-21 12:34:59Z".parse::<DateTimeUtc>().unwrap(),
                updated_at: Some("2023-06-21 12:55:01Z".parse::<DateTimeUtc>().unwrap()),
            }
        )
    }

    // delete
    {
        assert_eq!(
            UserService::delete_by_pk(
                &db,
                Uuid::parse_str("d6a4155c-33a2-487b-a80d-6d9468e7141e").unwrap()
            )
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
