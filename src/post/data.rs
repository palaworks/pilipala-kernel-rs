use chrono::prelude::*;

use crate::permission::PermissionLv;
use crate::post::db_entry::PostDbEntry;
use crate::Id;

pub struct PostPermission {
    pub read_lv: PermissionLv,
    pub write_lv: PermissionLv,
    pub comment_lv: PermissionLv
}

pub struct PostData {
    pub title: String,
    pub body: String,

    pub create_time: DateTime<Utc>,
    pub access_time: DateTime<Utc>,
    pub modify_time: DateTime<Utc>,

    pub id: Id,
    pub user_id: Id,

    pub permission: PostPermission
}

impl From<PostDbEntry> for PostData {
    fn from(entry: PostDbEntry) -> Self {
        PostData {
            title: entry.title,
            body: entry.body,

            create_time: entry.create_time,
            access_time: entry.access_time,
            modify_time: entry.modify_time,

            id: entry.id,
            user_id: entry.user_id,

            permission: PostPermission {
                read_lv: entry.read_lv,
                write_lv: entry.write_lv,
                comment_lv: entry.comment_lv
            }
        }
    }
}
