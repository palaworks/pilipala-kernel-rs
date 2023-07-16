use anyhow::Result;
use async_trait::async_trait;

use crate::user::data::UserData;

#[async_trait]
pub trait UserSetPipeline: Send + Sync {
    async fn set(&self, data: UserData) -> Result<()>;
}
