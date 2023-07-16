use std::future::Future;

use anyhow::Result;

use crate::user::data::UserData;
use crate::Id;

pub trait UserIniPipeline {
    fn ini(&self, data: UserData) -> dyn Future<Output = Result<Id>>;
}
