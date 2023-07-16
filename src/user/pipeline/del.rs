use anyhow::Result;
use async_trait::async_trait;

use crate::Id;

#[async_trait]
pub trait UserDelPipeline: Send + Sync {
    async fn call(&self, id: Id) -> Result<()>;
}
