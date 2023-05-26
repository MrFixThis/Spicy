use entity::{prelude::UserProfile, user_profile};

use crate::{pk_ty, MutationRepository, QueryRepository};

#[derive(Debug)]
pub struct UserProfileService;
pk_ty!(user_profile::PrimaryKey);

#[async_trait::async_trait]
impl QueryRepository<UserProfile, PrimaryKey> for UserProfileService {}

#[async_trait::async_trait]
impl MutationRepository<UserProfile, user_profile::ActiveModel, PrimaryKey> for UserProfileService {}
