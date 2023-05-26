use entity::{prelude::Role, role};

use crate::{pk_ty, MutationRepository, QueryRepository};

#[derive(Debug)]
pub struct RoleService;
pk_ty!(role::PrimaryKey);

#[async_trait::async_trait]
impl QueryRepository<Role, PrimaryKey> for RoleService {}

#[async_trait::async_trait]
impl MutationRepository<Role, role::ActiveModel, PrimaryKey> for RoleService {}
