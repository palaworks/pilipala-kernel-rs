use chrono::prelude::*;

use crate::user::data::UserPermission;
use crate::Id;

pub struct MappedUserData {
    id: Id,

    name: Option<String>,
    pwd_hash: Option<String>,

    create_time: Option<DateTime<Utc>>,
    access_time: Option<DateTime<Utc>>,

    permission: Option<UserPermission>
}
