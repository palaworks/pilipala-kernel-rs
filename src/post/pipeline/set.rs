use anyhow::Result;
use async_trait::async_trait;

use crate::post::data::PostData;

#[async_trait]
pub trait PostSetPipeline: Send + Sync {
    async fn call(&self, data: PostData) -> Result<()>;
}
