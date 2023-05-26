use entity::{likes, prelude::Likes};

use crate::{pk_ty, QueryRepository};

#[derive(Debug)]
pub struct LikesService;
pk_ty!(likes::PrimaryKey);

#[async_trait::async_trait]
impl QueryRepository<Likes, PrimaryKey> for LikesService {}
