use chrono::prelude::*;

use crate::comment::data::{CommentData, CommentTarget};
use crate::permission::PermissionLv;
use crate::Id;

pub enum TargetType {
    Post = 0,
    Comment = 1
}

pub struct CommentDbEntry {
    pub body: String,

    pub create_time: DateTime<Utc>,
    pub access_time: DateTime<Utc>,
    pub modify_time: DateTime<Utc>,

    pub target_type: TargetType,
    pub target_id: Id,

    pub id: Id,
    pub user_id: Id,

    pub read_lv: PermissionLv,
    pub write_lv: PermissionLv,
    pub comment_lv: PermissionLv
}

impl From<CommentData> for CommentDbEntry {
    fn from(data: CommentData) -> Self {
        let (target_type, target_id) = match data.target {
            CommentTarget::Post(id) => (TargetType::Post, id),
            CommentTarget::Comment(id) => (TargetType::Comment, id)
        };

        CommentDbEntry {
            body: data.body,

            create_time: data.create_time,
            access_time: data.access_time,
            modify_time: data.modify_time,

            target_type,
            target_id,

            id: data.id,
            user_id: data.user_id,

            read_lv: data.permission.read_lv,
            write_lv: data.permission.write_lv,
            comment_lv: data.permission.comment_lv
        }
    }
}
