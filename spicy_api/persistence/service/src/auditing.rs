use entity::{auditing, prelude::Auditing};
use sea_orm::{DbConn, EntityTrait, QueryOrder};

use crate::{pk_ty, QueryRepository};

#[derive(Debug)]
pub struct AuditingService;
pk_ty!(auditing::PrimaryKey);

#[async_trait::async_trait]
impl QueryRepository<Auditing, PrimaryKey> for AuditingService {
    async fn find_all(db: &DbConn) -> anyhow::Result<Vec<auditing::Model>> {
        Auditing::find()
            .order_by_desc(auditing::Column::Timestamp)
            .all(db)
            .await
            .map_err(anyhow::Error::msg)
    }
}
