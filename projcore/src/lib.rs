use std::error::Error;

pub type AsyncError = Box<dyn Error + Sync + Send>;

pub mod data_types;
