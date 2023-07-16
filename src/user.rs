use anyhow::Result;
use chrono::prelude::*;

use crate::comment::data::CommentPermission;
use crate::permission::PermissionLv;
use crate::post::data::PostPermission;
use crate::user::data::UserPermission;
use crate::Id;

pub mod data;
pub mod db_entry;
pub mod pipeline;
pub mod provider;

pub trait User {
    fn get_id(&self) -> Id;

    async fn get_name(&self) -> String;
    async fn update_name(&self, new: impl Into<String>) -> Result<()>;

    async fn validate_pwd(&self, pwd: impl Into<String>) -> bool;
    async fn update_pwd(&self, new: impl Into<String>) -> Result<()>;

    async fn get_create_time(&self) -> DateTime<Utc>;
    async fn get_access_time(&self) -> DateTime<Utc>;

    async fn get_permission(&self) -> UserPermission;

    async fn update_user_permission(&self, new: UserPermission) -> Result<()>;
    async fn update_user_read_permission(
        &self,
        new: PermissionLv
    ) -> Result<()>;
    async fn update_user_write_permission(
        &self,
        new: PermissionLv
    ) -> Result<()>;

    async fn update_post_permission(&self, new: PostPermission) -> Result<()>;
    async fn update_post_read_permission(
        &self,
        new: PermissionLv
    ) -> Result<()>;
    async fn update_post_write_permission(
        &self,
        new: PermissionLv
    ) -> Result<()>;
    async fn update_post_comment_permission(
        &self,
        new: PermissionLv
    ) -> Result<()>;

    async fn update_comment_permission(
        &self,
        new: CommentPermission
    ) -> Result<()>;
    async fn update_comment_read_permission(
        &self,
        new: PermissionLv
    ) -> Result<()>;
    async fn update_comment_write_permission(
        &self,
        new: PermissionLv
    ) -> Result<()>;
    async fn update_comment_comment_permission(
        &self,
        new: PermissionLv
    ) -> Result<()>;

    async fn new_post<S: Into<String>>(title: S, body: S) -> Result<Id>;
}
