use std::fs;

use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase, MockExecResult, ModelTrait};
use serde::Deserialize;

pub fn build_mock_db<M, I>(path: &str, exec_res: I) -> anyhow::Result<DatabaseConnection>
where
    M: ModelTrait,
    for<'de> M: Deserialize<'de>,
    I: IntoIterator<Item = MockExecResult>,
{
    Ok(MockDatabase::new(DatabaseBackend::Sqlite)
        .append_query_results(parse_query_result::<M>(path)?)
        .append_exec_results(exec_res)
        .into_connection())
}

pub fn _build_mock_db_from<F>(f: F) -> anyhow::Result<DatabaseConnection>
where
    F: FnOnce() -> anyhow::Result<DatabaseConnection>,
{
    f()
}

pub fn parse_query_result<M>(path: &str) -> anyhow::Result<Vec<Vec<M>>>
where
    M: ModelTrait,
    for<'de> M: Deserialize<'de>,
{
    let cont = fs::read_to_string(format!("testdata/{path}"))?;
    serde_json::from_str(cont.as_str()).map_err(anyhow::Error::msg)
}
