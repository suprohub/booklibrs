pub mod booklib;
use std::fmt::Display;

pub use booklib::*;
pub mod komarovlib;
pub use komarovlib::*;

use async_trait::async_trait;
use projcore::{
    data_types::{Data, Meta},
    AsyncError,
};

#[async_trait(?Send)]
pub trait Library: Send + Sync {
    async fn get_data(&mut self, id: impl Display + Send + Sync) -> Result<Data, AsyncError>;
    async fn data_info(&mut self, id: impl Display + Send + Sync) -> Result<Meta, AsyncError>;
    async fn search(&mut self, query: String);
}
