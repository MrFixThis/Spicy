mod common;

use entity::auditing;
use pretty_assertions::assert_eq;
use sea_orm::{prelude::DateTimeUtc, MockExecResult};
use service::{AuditingService, QueryRepository};

#[tokio::test]
async fn auditing_test() {
    let db = common::build_mock_db::<auditing::Model, _>(
        "auditing_data.json",
        [MockExecResult {
            last_insert_id: 0,
            rows_affected: 1,
        }],
    )
    .unwrap();

    // select
    {
        let retr_auditing = AuditingService::find_by_pk(&db, 1).await.unwrap().unwrap();

        assert_eq!(
            retr_auditing,
            auditing::Model {
                id: 1,
                table_name: "user".to_owned(),
                operation_kind: auditing::OperationKind::Create,
                user_id: 1,
                timestamp: "2023-01-01 09:00:00Z".parse::<DateTimeUtc>().unwrap()
            }
        );
    }

    // select all
    {
        let retrv_auditings = AuditingService::find_all(&db).await.unwrap();

        assert_eq!(
            retrv_auditings,
            vec![
                auditing::Model {
                    id: 2,
                    table_name: "user".to_owned(),
                    operation_kind: auditing::OperationKind::Update,
                    user_id: 2,
                    timestamp: "2023-01-02 11:30:00Z".parse::<DateTimeUtc>().unwrap()
                },
                auditing::Model {
                    id: 3,
                    table_name: "user".to_owned(),
                    operation_kind: auditing::OperationKind::Delete,
                    user_id: 3,
                    timestamp: "2023-01-03 11:45:00Z".parse::<DateTimeUtc>().unwrap()
                },
                auditing::Model {
                    id: 4,
                    table_name: "recipe".to_owned(),
                    operation_kind: auditing::OperationKind::Create,
                    user_id: 4,
                    timestamp: "2023-01-04 08:15:00Z".parse::<DateTimeUtc>().unwrap()
                }
            ]
        );
    }
}
