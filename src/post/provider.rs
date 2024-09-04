use anyhow::Result;
use async_trait::async_trait;

use crate::post::data::PostData;
use crate::post::mapped::MappedPostData;
use crate::post::pipeline::del::PostDelPipeline;
use crate::post::pipeline::get::PostGetPipeline;
use crate::post::pipeline::ini::PostIniPipeline;
use crate::post::pipeline::set::PostSetPipeline;
use crate::Id;

#[async_trait]
pub trait PostProvider {
    async fn get_post(&self, id: Id) -> Result<MappedPostData>;
    async fn ini_post(&self, data: PostData) -> Result<Id>;
    async fn del_post(&self, id: Id) -> Result<()>;
}

pub struct DefaultPostProvider {
    ini: Box<dyn PostIniPipeline>,
    get: Box<dyn PostGetPipeline>,
    set: Box<dyn PostSetPipeline>,
    del: Box<dyn PostDelPipeline>
}

impl DefaultPostProvider {
    pub fn new(
        ini: Box<dyn PostIniPipeline>,
        get: Box<dyn PostGetPipeline>,
        set: Box<dyn PostSetPipeline>,
        del: Box<dyn PostDelPipeline>
    ) -> Self {
        DefaultPostProvider { ini, get, set, del }
    }
}

#[async_trait]
impl PostProvider for DefaultPostProvider {
    async fn get_post(&self, id: Id) -> Result<MappedPostData> {
        //self.get.get_data(id).await
        todo!()
    }

    async fn ini_post(&self, data: PostData) -> Result<Id> {
        self.ini.call(data).await
    }

    async fn del_post(&self, id: Id) -> Result<()> { self.del.call(id).await }
}
