use anyhow::Result;
use async_trait::async_trait;

use crate::user::data::UserData;
use crate::user::mapped::MappedUserData;
use crate::user::pipeline::del::UserDelPipeline;
use crate::user::pipeline::get::UserGetPipeline;
use crate::user::pipeline::ini::UserIniPipeline;
use crate::user::pipeline::set::UserSetPipeline;
use crate::Id;

#[async_trait]
pub trait UserProvider {
    async fn get_user(&self, id: Id) -> Result<MappedUserData>;
    async fn ini_user(&self, data: UserData) -> Result<Id>;
    async fn del_user(&self, id: Id) -> Result<()>;
}

pub struct DefaultUserProvider {
    ini: Box<dyn UserIniPipeline>,
    get: Box<dyn UserGetPipeline>,
    set: Box<dyn UserSetPipeline>,
    del: Box<dyn UserDelPipeline>
}

impl DefaultUserProvider {
    pub fn new(
        ini: Box<dyn UserIniPipeline>,
        get: Box<dyn UserGetPipeline>,
        set: Box<dyn UserSetPipeline>,
        del: Box<dyn UserDelPipeline>
    ) -> Self {
        DefaultUserProvider { ini, get, set, del }
    }
}

#[async_trait]
impl UserProvider for DefaultUserProvider {
    async fn get_user(&self, id: Id) -> Result<MappedUserData> {
        //self.get.get_data(id).await
        todo!()
    }

    async fn ini_user(&self, data: UserData) -> Result<Id> {
        self.ini.call(data).await
    }

    async fn del_user(&self, id: Id) -> Result<()> { self.del.call(id).await }
}
