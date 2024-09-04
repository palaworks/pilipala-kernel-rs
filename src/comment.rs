use anyhow::Result;
use async_trait::async_trait;
use chrono::prelude::*;

use crate::comment::data::{CommentPermission, CommentTarget};
use crate::permission::PermissionLv;
use crate::Id;

pub mod data;
pub mod db_entry;
pub mod mapped;
pub mod pipeline;
pub mod provider;

#[async_trait]
pub trait Comment {
    async fn get_body(&self) -> String;
    async fn update_body(&self, new: impl Into<String>) -> Result<()>;

    async fn get_create_time(&self) -> DateTime<Utc>;
    async fn get_access_time(&self) -> DateTime<Utc>;
    async fn get_modify_time(&self) -> DateTime<Utc>;

    async fn get_target(&self) -> CommentTarget;

    fn get_id(&self) -> Id;
    async fn get_user_id(&self) -> Id;

    async fn get_permission(&self) -> CommentPermission;
    async fn update_permission(&self, new: CommentPermission) -> Result<()>;
    async fn update_read_permission(&self, new: PermissionLv) -> Result<()>;
    async fn update_write_permission(&self, new: PermissionLv) -> Result<()>;
    async fn update_comment_permission(&self, new: PermissionLv) -> Result<()>;

    async fn new_comment(body: impl Into<String>) -> Result<Id>;
}
