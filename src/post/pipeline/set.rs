use std::future::Future;

use anyhow::Result;

use crate::post::data::PostData;

pub trait PostSetPipeline {
    fn set(&self, data: PostData) -> dyn Future<Output = Result<()>>;
}
