use chrono::prelude::*;

use crate::comment::data::{CommentPermission, CommentTarget};
use crate::Id;

pub struct MappedCommentData {
    body: Option<String>,

    create_time: Option<DateTime<Utc>>,
    access_time: Option<DateTime<Utc>>,
    modify_time: Option<DateTime<Utc>>,

    target: Option<CommentTarget>,

    id: Id,
    user_id: Option<Id>,

    permission: Option<CommentPermission>
}
