use anyhow::Result;
use async_trait::async_trait;

use crate::post::data::PostData;
use crate::Id;

#[async_trait]
pub trait PostGetPipeline: Send + Sync {
    async fn call(&self, id: Id) -> Result<PostData>;
}
