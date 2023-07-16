use std::future::Future;

use anyhow::Result;

use crate::Id;

pub trait PostDelPipeline {
    fn del(&self, id: Id) -> dyn Future<Output = Result<()>>;
}
