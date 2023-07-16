use chrono::prelude::*;

use crate::comment::db_entry::{CommentDbEntry, TargetType};
use crate::permission::PermissionLv;
use crate::Id;

pub enum CommentTarget {
    Post(Id),
    Comment(Id)
}

pub struct CommentPermission {
    pub read_lv: PermissionLv,
    pub write_lv: PermissionLv,
    pub comment_lv: PermissionLv
}

pub struct CommentData {
    pub body: String,

    pub create_time: DateTime<Utc>,
    pub access_time: DateTime<Utc>,
    pub modify_time: DateTime<Utc>,

    pub target: CommentTarget,

    pub id: Id,
    pub user_id: Id,

    pub permission: CommentPermission
}

impl From<CommentDbEntry> for CommentData {
    fn from(entry: CommentDbEntry) -> Self {
        let target = match entry.target_type {
            TargetType::Post => CommentTarget::Post(entry.target_id),
            TargetType::Comment => CommentTarget::Comment(entry.target_id)
        };

        CommentData {
            body: entry.body,

            create_time: entry.create_time,
            access_time: entry.access_time,
            modify_time: entry.modify_time,

            target,

            id: entry.id,
            user_id: entry.user_id,

            permission: CommentPermission {
                read_lv: entry.read_lv,
                write_lv: entry.write_lv,
                comment_lv: entry.comment_lv
            }
        }
    }
}
