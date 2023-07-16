use std::future::Future;

use anyhow::Result;

use crate::post::data::PostData;

pub trait CommentSetPipeline {
    fn set(&self, data: PostData) -> dyn Future<Output = Result<()>>;
}
