use std::fs;

use entity::{auditing, likes, recipe, recipe_image, role, user, user_profile, user_role};
use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase, MockExecResult, ModelTrait};
use serde::Deserialize;

pub async fn build_mock_db() -> anyhow::Result<DatabaseConnection> {
    let db = MockDatabase::new(DatabaseBackend::Sqlite)
        .append_query_results(parse_query_result::<user::Model>("user_data").await?)
        .append_query_results(parse_query_result::<user_profile::Model>("user_profile_data").await?)
        .append_query_results(parse_query_result::<role::Model>("role_data").await?)
        .append_query_results(parse_query_result::<user_role::Model>("user_role_data").await?)
        .append_query_results(parse_query_result::<recipe::Model>("recipe_data").await?)
        .append_query_results(parse_query_result::<recipe_image::Model>("recipe_image_data").await?)
        .append_query_results(parse_query_result::<likes::Model>("likes_data").await?)
        .append_query_results(parse_query_result::<auditing::Model>("auditing_data").await?)
        .append_exec_results([
            MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 2,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 5,
                rows_affected: 4,
            },
        ])
        .into_connection();

    Ok(db)
}

async fn parse_query_result<T>(source_name: &str) -> anyhow::Result<Vec<Vec<T>>>
where
    T: ModelTrait,
    for<'de> T: Deserialize<'de>,
{
    let cont = fs::read_to_string(format!("testdata/{source_name}.json"))?;
    serde_json::from_str::<Vec<T>>(cont.as_str())
        .and_then(|v| Ok(v.into_iter().map(|t| vec![t]).collect::<Vec<Vec<T>>>()))
        .map_err(anyhow::Error::msg)
}
