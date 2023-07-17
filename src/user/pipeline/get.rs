use anyhow::Result;
use async_trait::async_trait;
use chrono::prelude::*;

use crate::comment::data::CommentPermission;
use crate::infra::result::IntoResult;
use crate::permission::PermissionLv;
use crate::post::data::PostPermission;
use crate::user::data::{UserData, UserPermission};
use crate::Id;

#[async_trait]
pub trait UserGetPipeline: Send + Sync {
    async fn get_data(&self, id: Id) -> Result<UserData> {
        UserData {
            id,
            name: self.get_name(id).await?,
            pwd_hash: self.get_pwd_hash(id).await?,
            create_time: self
                .get_create_time(id)
                .await?,
            access_time: self
                .get_access_time(id)
                .await?,
            permission: {
                UserPermission {
                    user_read_lv: self
                        .get_user_read_lv(id)
                        .await?,
                    user_write_lv: self
                        .get_user_write_lv(id)
                        .await?,
                    post_permission: PostPermission {
                        read_lv: self
                            .get_post_read_lv(id)
                            .await?,
                        write_lv: self
                            .get_post_write_lv(id)
                            .await?,
                        comment_lv: self
                            .get_post_comment_lv(id)
                            .await?
                    },
                    comment_permission: CommentPermission {
                        read_lv: self
                            .get_comment_read_lv(id)
                            .await?,
                        write_lv: self
                            .get_comment_write_lv(id)
                            .await?,
                        comment_lv: self
                            .get_comment_comment_lv(id)
                            .await?
                    }
                }
            }
        }
        .into_ok()
    }

    async fn get_name(&self, id: Id) -> Result<String> {
        self.get_data(id)
            .await?
            .name
            .into_ok()
    }
    async fn get_pwd_hash(&self, id: Id) -> Result<String> {
        self.get_data(id)
            .await?
            .pwd_hash
            .into_ok()
    }

    async fn get_create_time(&self, id: Id) -> Result<DateTime<Utc>> {
        self.get_data(id)
            .await?
            .create_time
            .into_ok()
    }
    async fn get_access_time(&self, id: Id) -> Result<DateTime<Utc>> {
        self.get_data(id)
            .await?
            .access_time
            .into_ok()
    }

    async fn get_user_read_lv(&self, id: Id) -> Result<PermissionLv> {
        self.get_data(id)
            .await?
            .permission
            .user_read_lv
            .into_ok()
    }
    async fn get_user_write_lv(&self, id: Id) -> Result<PermissionLv> {
        self.get_data(id)
            .await?
            .permission
            .user_write_lv
            .into_ok()
    }

    async fn get_post_read_lv(&self, id: Id) -> Result<PermissionLv> {
        self.get_data(id)
            .await?
            .permission
            .post_permission
            .read_lv
            .into_ok()
    }
    async fn get_post_write_lv(&self, id: Id) -> Result<PermissionLv> {
        self.get_data(id)
            .await?
            .permission
            .post_permission
            .write_lv
            .into_ok()
    }
    async fn get_post_comment_lv(&self, id: Id) -> Result<PermissionLv> {
        self.get_data(id)
            .await?
            .permission
            .post_permission
            .comment_lv
            .into_ok()
    }

    async fn get_comment_read_lv(&self, id: Id) -> Result<PermissionLv> {
        self.get_data(id)
            .await?
            .permission
            .comment_permission
            .read_lv
            .into_ok()
    }
    async fn get_comment_write_lv(&self, id: Id) -> Result<PermissionLv> {
        self.get_data(id)
            .await?
            .permission
            .comment_permission
            .write_lv
            .into_ok()
    }
    async fn get_comment_comment_lv(&self, id: Id) -> Result<PermissionLv> {
        self.get_data(id)
            .await?
            .permission
            .comment_permission
            .comment_lv
            .into_ok()
    }
}
