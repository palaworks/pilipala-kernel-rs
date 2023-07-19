use chrono::prelude::*;

use crate::post::data::PostPermission;
use crate::Id;

pub struct MappedPostData {
    title: Option<String>,
    body: Option<String>,

    create_time: Option<DateTime<Utc>>,
    access_time: Option<DateTime<Utc>>,
    modify_time: Option<DateTime<Utc>>,

    id: Id,
    user_id: Option<Id>,

    permission: Option<PostPermission>
}
