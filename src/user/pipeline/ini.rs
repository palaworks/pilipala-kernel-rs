use anyhow::Result;
use async_trait::async_trait;

use crate::user::data::UserData;
use crate::Id;

#[async_trait]
pub trait UserIniPipeline: Send + Sync {
    async fn call(&self, data: UserData) -> Result<Id>;
}
