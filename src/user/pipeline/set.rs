use std::future::Future;

use anyhow::Result;

use crate::user::data::UserData;

pub trait UserSetPipeline {
    fn set(&self, data: UserData) -> dyn Future<Output = Result<()>>;
}
