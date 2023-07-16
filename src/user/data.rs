use chrono::prelude::*;

use crate::comment::data::CommentPermission;
use crate::permission::PermissionLv;
use crate::post::data::PostPermission;
use crate::user::db_entry::UserDbEntry;
use crate::Id;

pub struct UserPermission {
    pub user_read_lv: PermissionLv,
    pub user_write_lv: PermissionLv,

    pub post_permission: PostPermission,
    pub comment_permission: CommentPermission
}

pub struct UserData {
    pub id: Id,

    pub name: String,
    pub pwd_hash: String,

    pub create_time: DateTime<Utc>,
    pub access_time: DateTime<Utc>,

    pub permission: UserPermission
}

impl From<UserDbEntry> for UserData {
    fn from(entry: UserDbEntry) -> Self {
        UserData {
            id: entry.id,

            name: entry.name,
            pwd_hash: entry.pwd_hash,

            create_time: entry.create_time,
            access_time: entry.access_time,

            permission: UserPermission {
                user_read_lv: entry.user_read_lv,
                user_write_lv: entry.user_write_lv,

                post_permission: PostPermission {
                    read_lv: entry.post_read_lv,
                    write_lv: entry.post_write_lv,
                    comment_lv: entry.post_comment_lv
                },

                comment_permission: CommentPermission {
                    read_lv: entry.comment_read_lv,
                    write_lv: entry.comment_write_lv,
                    comment_lv: entry.comment_comment_lv
                }
            }
        }
    }
}
