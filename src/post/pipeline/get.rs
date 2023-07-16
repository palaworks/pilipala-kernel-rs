use std::future::Future;

use anyhow::Result;

use crate::post::data::PostData;
use crate::Id;

pub trait PostGetPipeline {
    fn get(&self, id: Id) -> dyn Future<Output = Result<PostData>>;
}
