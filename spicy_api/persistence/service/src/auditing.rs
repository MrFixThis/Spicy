use entity::{auditing, prelude::Auditing};
use sea_orm::{DbConn, DbErr, EntityTrait, QueryOrder};

use crate::{pk_ty, QueryRepository};

#[derive(Debug)]
pub struct AuditingService;
pk_ty!(auditing::PrimaryKey);

#[async_trait::async_trait]
impl QueryRepository<Auditing, PrimaryKey> for AuditingService {
    async fn find_all(db: &DbConn) -> Result<Vec<auditing::Model>, DbErr> {
        Auditing::find()
            .order_by_desc(auditing::Column::Timestamp)
            .all(db)
            .await
    }
}
