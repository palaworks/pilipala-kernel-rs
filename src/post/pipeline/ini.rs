use anyhow::Result;
use async_trait::async_trait;

use crate::post::data::PostData;
use crate::Id;

#[async_trait]
pub trait PostIniPipeline: Send + Sync {
    async fn call(&self, data: PostData) -> Result<Id>;
}
