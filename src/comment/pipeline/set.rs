use anyhow::Result;
use async_trait::async_trait;

use crate::comment::data::CommentData;

#[async_trait]
pub trait CommentSetPipeline: Send + Sync {
    async fn call(&self, data: CommentData) -> Result<()>;
}
