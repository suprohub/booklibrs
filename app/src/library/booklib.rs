use std::fmt::Display;

use async_trait::async_trait;
use projcore::{
    data_types::{Data, Meta},
    AsyncError,
};
use reqwest::{Client, IntoUrl, Url};

use super::Library;

pub struct BookLibRs {
    pub client: Client,
    pub server: Url,
}

unsafe impl Send for BookLibRs {}
unsafe impl Sync for BookLibRs {}

impl BookLibRs {
    pub fn new(server: impl IntoUrl) -> Result<Self, AsyncError> {
        Ok(Self {
            client: Client::new(),
            server: server.into_url()?,
        })
    }
}

#[async_trait(?Send)]
impl Library for BookLibRs {
    async fn get_data(&mut self, id: impl Display + Send + Sync) -> Result<Data, AsyncError> {
        todo!()
    }

    async fn data_info(&mut self, id: impl Display + Send + Sync) -> Result<Meta, AsyncError> {
        todo!()
    }

    async fn search(&mut self, query: String) {}
}
