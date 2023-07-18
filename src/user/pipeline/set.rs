use anyhow::Result;
use async_trait::async_trait;
use chrono::prelude::*;

use crate::infra::result::IntoResult;
use crate::permission::PermissionLv;
use crate::user::data::UserData;
use crate::Id;

#[async_trait]
pub trait UserSetPipeline: Send + Sync {
    async fn set_data(&self, id: Id, new: UserData) -> Result<()> {
        let id = new.id;

        self.set_name(id, new.name)
            .await?;
        self.set_pwd_hash(id, new.pwd_hash)
            .await?;

        self.set_create_time(id, new.create_time)
            .await?;
        self.set_access_time(id, new.access_time)
            .await?;

        self.set_user_read_lv(id, new.permission.user_read_lv)
            .await?;
        self.set_user_write_lv(id, new.permission.user_write_lv)
            .await?;

        let post_permission = new.permission.post_permission;
        self.set_post_read_lv(id, post_permission.read_lv)
            .await?;
        self.set_post_write_lv(id, post_permission.write_lv)
            .await?;
        self.set_post_comment_lv(id, post_permission.comment_lv)
            .await?;

        let comment_permission = new
            .permission
            .comment_permission;
        self.set_comment_read_lv(id, comment_permission.read_lv)
            .await?;
        self.set_comment_write_lv(id, comment_permission.write_lv)
            .await?;
        self.set_comment_comment_lv(id, comment_permission.comment_lv)
            .await?;

        ().into_ok()
    }

    async fn set_name(&self, id: Id, new: String) -> Result<()>;
    async fn set_pwd_hash(&self, id: Id, new: String) -> Result<()>;

    async fn set_create_time(&self, id: Id, new: DateTime<Utc>) -> Result<()>;
    async fn set_access_time(&self, id: Id, new: DateTime<Utc>) -> Result<()>;

    async fn set_user_read_lv(&self, id: Id, new: PermissionLv) -> Result<()>;
    async fn set_user_write_lv(&self, id: Id, new: PermissionLv) -> Result<()>;

    async fn set_post_read_lv(&self, id: Id, new: PermissionLv) -> Result<()>;
    async fn set_post_write_lv(&self, id: Id, new: PermissionLv) -> Result<()>;
    async fn set_post_comment_lv(
        &self,
        id: Id,
        new: PermissionLv
    ) -> Result<()>;

    async fn set_comment_read_lv(
        &self,
        id: Id,
        new: PermissionLv
    ) -> Result<()>;
    async fn set_comment_write_lv(
        &self,
        id: Id,
        new: PermissionLv
    ) -> Result<()>;
    async fn set_comment_comment_lv(
        &self,
        id: Id,
        new: PermissionLv
    ) -> Result<()>;
}
