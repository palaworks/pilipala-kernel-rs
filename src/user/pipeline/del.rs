use std::future::Future;

use anyhow::Result;

use crate::Id;

pub trait UserDelPipeline {
    fn del(&self, id: Id) -> dyn Future<Output = Result<()>>;
}
