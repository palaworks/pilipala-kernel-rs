use chrono::prelude::*;

use crate::permission::PermissionLv;
use crate::post::data::PostData;
use crate::Id;

pub struct PostDbEntry {
    pub title: String,
    pub body: String,

    pub create_time: DateTime<Utc>,
    pub access_time: DateTime<Utc>,
    pub modify_time: DateTime<Utc>,

    pub id: Id,
    pub user_id: Id,

    pub read_lv: PermissionLv,
    pub write_lv: PermissionLv,
    pub comment_lv: PermissionLv
}

impl From<PostData> for PostDbEntry {
    fn from(data: PostData) -> Self {
        PostDbEntry {
            title: data.title,
            body: data.body,

            create_time: data.create_time,
            access_time: data.access_time,
            modify_time: data.modify_time,

            id: data.id,
            user_id: data.user_id,

            read_lv: data.permission.read_lv,
            write_lv: data.permission.write_lv,
            comment_lv: data.permission.comment_lv
        }
    }
}
