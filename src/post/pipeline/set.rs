use anyhow::Result;
use async_trait::async_trait;
use chrono::prelude::*;

use crate::infra::result::IntoResult;
use crate::permission::PermissionLv;
use crate::post::data::PostData;
use crate::Id;

#[async_trait]
pub trait PostSetPipeline: Send + Sync {
    async fn set_data(&self, id: Id, new: PostData) -> Result<()> {
        let id = new.id;

        self.set_title(id, new.title)
            .await?;
        self.set_body(id, new.body)
            .await?;

        self.set_create_time(id, new.create_time)
            .await?;
        self.set_access_time(id, new.access_time)
            .await?;
        self.set_modify_time(id, new.modify_time)
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

    async fn set_title(&self, id: Id, new: String) -> Result<()>;
    async fn set_body(&self, id: Id, new: String) -> Result<()>;

    async fn set_create_time(&self, id: Id, new: DateTime<Utc>) -> Result<()>;
    async fn set_access_time(&self, id: Id, new: DateTime<Utc>) -> Result<()>;
    async fn set_modify_time(&self, id: Id, new: DateTime<Utc>) -> Result<()>;

    async fn set_user_id(&self, id: Id, new: Id) -> Result<()>;

    async fn set_read_lv(&self, id: Id, new: PermissionLv) -> Result<()>;
    async fn set_write_lv(&self, id: Id, new: PermissionLv) -> Result<()>;
    async fn set_comment_lv(&self, id: Id, new: PermissionLv) -> Result<()>;
}
