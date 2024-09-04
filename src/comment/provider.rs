use anyhow::Result;
use async_trait::async_trait;

use crate::comment::data::CommentData;
use crate::comment::mapped::MappedCommentData;
use crate::comment::pipeline::del::CommentDelPipeline;
use crate::comment::pipeline::get::CommentGetPipeline;
use crate::comment::pipeline::ini::CommentIniPipeline;
use crate::comment::pipeline::set::CommentSetPipeline;
use crate::Id;

#[async_trait]
pub trait CommentProvider {
    async fn get_comment(&self, id: Id) -> Result<MappedCommentData>;
    async fn ini_comment(&self, data: CommentData) -> Result<Id>;
    async fn del_comment(&self, id: Id) -> Result<()>;
}

pub struct DefaultCommentProvider {
    ini: Box<dyn CommentIniPipeline>,
    get: Box<dyn CommentGetPipeline>,
    set: Box<dyn CommentSetPipeline>,
    del: Box<dyn CommentDelPipeline>
}

impl DefaultCommentProvider {
    pub fn new(
        ini: Box<dyn CommentIniPipeline>,
        get: Box<dyn CommentGetPipeline>,
        set: Box<dyn CommentSetPipeline>,
        del: Box<dyn CommentDelPipeline>
    ) -> Self {
        DefaultCommentProvider { ini, get, set, del }
    }
}

#[async_trait]
impl CommentProvider for DefaultCommentProvider {
    async fn get_comment(&self, id: Id) -> Result<MappedCommentData> {
        //self.get.get_data(id).await
        todo!()
    }

    async fn ini_comment(&self, data: CommentData) -> Result<Id> {
        self.ini.call(data).await
    }

    async fn del_comment(&self, id: Id) -> Result<()> {
        self.del.call(id).await
    }
}
