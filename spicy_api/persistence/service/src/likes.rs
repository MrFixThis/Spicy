use entity::{likes, prelude::Likes};

use crate::{pk_ty, QueryRepository, MutationRepository};

#[derive(Debug)]
pub struct LikesService;
pk_ty!(likes::PrimaryKey);

#[async_trait::async_trait]
impl QueryRepository<Likes, PrimaryKey> for LikesService {}

#[async_trait::async_trait]
impl MutationRepository<Likes, likes::ActiveModel, PrimaryKey> for LikesService {}
