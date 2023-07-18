use anyhow::Result;
use async_trait::async_trait;
use chrono::prelude::*;

use crate::comment::data::{CommentData, CommentTarget};
use crate::comment::db_entry::TargetType;
use crate::infra::result::IntoResult;
use crate::permission::PermissionLv;
use crate::Id;

#[async_trait]
pub trait CommentSetPipeline: Send + Sync {
    async fn set_data(&self, new: CommentData) -> Result<()> {
        let id = new.id;

        self.set_body(id, new.body)
            .await?;

        self.set_create_time(id, new.create_time)
            .await?;
        self.set_access_time(id, new.access_time)
            .await?;
        self.set_modify_time(id, new.modify_time)
            .await?;

        let (target_type, target_id) = match new.target {
            CommentTarget::Post(id) => (TargetType::Post, id),
            CommentTarget::Comment(id) => (TargetType::Comment, id)
        };
        self.set_target_type(id, target_type)
            .await?;
        self.set_target_id(id, target_id)
            .await?;

        self.set_user_id(id, new.user_id)
            .await?;

        self.set_read_lv(id, new.permission.read_lv)
            .await?;
        self.set_write_lv(id, new.permission.write_lv)
            .await?;
        self.set_comment_lv(id, new.permission.comment_lv)
            .await?;

        ().into_ok()
    }

    async fn set_body(&self, id: Id, new: String) -> Result<()>;

    async fn set_create_time(&self, id: Id, new: DateTime<Utc>) -> Result<()>;
    async fn set_access_time(&self, id: Id, new: DateTime<Utc>) -> Result<()>;
    async fn set_modify_time(&self, id: Id, new: DateTime<Utc>) -> Result<()>;

    async fn set_target_type(&self, id: Id, new: TargetType) -> Result<()>;
    async fn set_target_id(&self, id: Id, new: Id) -> Result<()>;

    async fn set_user_id(&self, id: Id, new: Id) -> Result<()>;

    async fn set_read_lv(&self, id: Id, new: PermissionLv) -> Result<()>;
    async fn set_write_lv(&self, id: Id, new: PermissionLv) -> Result<()>;
    async fn set_comment_lv(&self, id: Id, new: PermissionLv) -> Result<()>;
}
