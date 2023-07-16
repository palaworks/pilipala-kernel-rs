use std::future::Future;

use anyhow::Result;

use crate::comment::data::CommentData;
use crate::Id;

pub trait CommentGetPipeline {
    fn get(&self, id: Id) -> dyn Future<Output = Result<CommentData>>;
}
