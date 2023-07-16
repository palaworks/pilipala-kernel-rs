use anyhow::Result;
use async_trait::async_trait;

use crate::comment::data::CommentData;
use crate::Id;

#[async_trait]
pub trait CommentGetPipeline: Send + Sync {
    async fn call(&self, id: Id) -> Result<CommentData>;
}
