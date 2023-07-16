use std::future::Future;

use anyhow::Result;

use crate::post::data::PostData;
use crate::Id;

pub trait CommentIniPipeline {
    fn ini(&self, data: PostData) -> dyn Future<Output = Result<Id>>;
}
