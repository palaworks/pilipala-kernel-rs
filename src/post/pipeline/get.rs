use anyhow::Result;
use async_trait::async_trait;
use chrono::prelude::*;

use crate::infra::result::IntoResult;
use crate::permission::PermissionLv;
use crate::post::data::{PostData, PostPermission};
use crate::Id;

#[async_trait]
pub trait PostGetPipeline: Send + Sync {
    async fn get_data(&self, id: Id) -> Result<PostData> {
        PostData {
            title: self.get_title(id).await?,
            body: self.get_body(id).await?,
            create_time: self
                .get_create_time(id)
                .await?,
            access_time: self
                .get_access_time(id)
                .await?,
            modify_time: self
                .get_modify_time(id)
                .await?,
            id,
            user_id: self.get_user_id(id).await?,
            permission: PostPermission {
                read_lv: self.get_read_lv(id).await?,
                write_lv: self.get_write_lv(id).await?,
                comment_lv: self
                    .get_comment_lv(id)
                    .await?
            }
        }
        .into_ok()
    }

    async fn get_title(&self, id: Id) -> Result<String> {
        self.get_data(id)
            .await?
            .title
            .into_ok()
    }
    async fn get_body(&self, id: Id) -> Result<String> {
        self.get_data(id)
            .await?
            .title
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
    async fn get_modify_time(&self, id: Id) -> Result<DateTime<Utc>> {
        self.get_data(id)
            .await?
            .modify_time
            .into_ok()
    }

    async fn get_user_id(&self, id: Id) -> Result<Id> {
        self.get_data(id)
            .await?
            .user_id
            .into_ok()
    }

    async fn get_read_lv(&self, id: Id) -> Result<PermissionLv> {
        self.get_data(id)
            .await?
            .permission
            .read_lv
            .into_ok()
    }
    async fn get_write_lv(&self, id: Id) -> Result<PermissionLv> {
        self.get_data(id)
            .await?
            .permission
            .write_lv
            .into_ok()
    }
    async fn get_comment_lv(&self, id: Id) -> Result<PermissionLv> {
        self.get_data(id)
            .await?
            .permission
            .comment_lv
            .into_ok()
    }
}
