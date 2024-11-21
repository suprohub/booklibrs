use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

pub mod old_book;
pub mod old_poem;

#[derive(Encode, Decode, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub date: u32,
    pub lang: [u8; 4],
    pub title: String,
    pub publisher: String,
    pub pub_type: PubType,
    pub desc: String,
    pub authors: Vec<String>,
}

#[derive(Encode, Decode, Serialize, Deserialize, Clone, Copy)]
pub enum PubType {
    Book,
    Poem,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum Data {
    TextOnly(Vec<String>)
}

impl Meta {
    pub fn new(
        date: u32,
        lang: [u8; 4],
        title: String,
        publisher: String,
        pub_type: PubType,
        desc: String,
        authors: Vec<String>,
    ) -> Self {
        Self {
            date,
            lang,
            title,
            publisher,
            pub_type,
            desc,
            authors,
        }
    }
}
