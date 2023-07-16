use std::future::Future;

use anyhow::Result;

use crate::Id;

pub trait CommentDelPipeline {
    fn del(&self, id: Id) -> dyn Future<Output = Result<()>>;
}
