use chrono::prelude::*;

use crate::permission::PermissionLv;
use crate::user::data::UserData;
use crate::Id;

pub struct UserDbEntry {
    pub id: Id,

    pub name: String,
    pub pwd_hash: String,

    pub create_time: DateTime<Utc>,
    pub access_time: DateTime<Utc>,

    pub user_read_lv: PermissionLv,
    pub user_write_lv: PermissionLv,

    pub post_read_lv: PermissionLv,
    pub post_write_lv: PermissionLv,
    pub post_comment_lv: PermissionLv,

    pub comment_read_lv: PermissionLv,
    pub comment_write_lv: PermissionLv,
    pub comment_comment_lv: PermissionLv
}

impl From<UserData> for UserDbEntry {
    fn from(data: UserData) -> Self {
        UserDbEntry {
            id: data.id,

            name: data.name,
            pwd_hash: data.pwd_hash,

            create_time: data.create_time,
            access_time: data.access_time,

            user_read_lv: data.permission.user_read_lv,
            user_write_lv: data.permission.user_write_lv,

            post_read_lv: data
                .permission
                .post_permission
                .read_lv,
            post_write_lv: data
                .permission
                .post_permission
                .write_lv,
            post_comment_lv: data
                .permission
                .post_permission
                .comment_lv,

            comment_read_lv: data
                .permission
                .comment_permission
                .read_lv,
            comment_write_lv: data
                .permission
                .comment_permission
                .write_lv,
            comment_comment_lv: data
                .permission
                .comment_permission
                .comment_lv
        }
    }
}
