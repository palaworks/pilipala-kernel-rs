use std::future::Future;

use anyhow::Result;

use crate::user::data::UserData;
use crate::Id;

pub trait UserGetPipeline {
    fn get(&self, id: Id) -> dyn Future<Output = Result<UserData>>;
}
