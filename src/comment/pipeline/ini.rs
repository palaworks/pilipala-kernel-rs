use anyhow::Result;
use async_trait::async_trait;

use crate::comment::data::CommentData;
use crate::Id;

#[async_trait]
pub trait CommentIniPipeline: Send + Sync {
    async fn call(&self, data: CommentData) -> Result<Id>;
}
