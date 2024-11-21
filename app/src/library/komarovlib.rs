use std::fmt::Display;

use async_trait::async_trait;
use projcore::{
    data_types::{Data, Meta, PubType},
    AsyncError,
};
use quick_xml::{events::Event, Reader};
use reqwest::Client;

use super::Library;

pub struct KomarovLib {
    pub client: Client,
}

unsafe impl Send for KomarovLib {}
unsafe impl Sync for KomarovLib {}

impl Default for KomarovLib {
    fn default() -> Self {
        Self::new()
    }
}

impl KomarovLib {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait(?Send)]
impl Library for KomarovLib {
    async fn get_data(&mut self, id: impl Display + Send + Sync) -> Result<Data, AsyncError> {
        let data = reqwest::get(format!("https://www.ilibrary.ru/text/{id}/p.1/index.html"))
            .await?
            .text()
            .await?;

        let mut reader = Reader::from_str(&data);
        reader.config_mut().check_end_names = false;

        let mut text_section = false;
        let mut info = vec![String::new()];

        while let Ok(event) = reader.read_event() {
            match event {
                Event::Start(e) => {
                    if e.name().as_ref() == b"z" {
                        text_section = true
                    }
                }
                Event::End(e) => {
                    if e.name().as_ref() == b"z" {
                        text_section = false;
                        info.push(String::new());
                    }
                }
                Event::Text(e) => {
                    if text_section && e.as_ref() != &[10] {
                        if let Some(last) = info.last_mut() {
                            if last.is_empty() {
                                last.push_str(&e.unescape()?);
                            } else {
                                last.push('\n');
                                last.push_str(&e.unescape()?);
                            }
                        }
                    }
                }
                Event::Eof => {
                    break;
                }
                _ => (),
            }
        }

        Ok(Data::TextOnly(info))
    }

    async fn data_info(&mut self, id: impl Display + Send + Sync) -> Result<Meta, AsyncError> {
        let data = reqwest::get(format!("https://www.ilibrary.ru/text/{id}/index.html"))
            .await?
            .text()
            .await?;
        let mut reader = Reader::from_str(&data);
        reader.config_mut().check_end_names = false;

        let mut text_section = false;
        let mut after_title = false;
        let mut info = Vec::new();
        let mut pubtype = PubType::Poem;
        let mut desc = "(Пусто)".to_string();

        while let Ok(event) = reader.read_event() {
            match event {
                Event::Start(e) => {
                    if after_title {
                        if e.name().as_ref() == b"div" && e.attributes_raw() == b"" {
                            text_section = true;
                        }
                    } else if e.name().as_ref() == b"div" && e.attributes_raw() == b" class=\"tabout\""
                        || e.name().as_ref() == b"h1"
                            && e.attributes_raw() == b" style=\"padding-bottom: 0em;padding: 0em\""
                    {
                        text_section = true;
                    } else if e.name().as_ref() == b"a" && e.attributes_raw() == b" name=\"toc\"" {
                        pubtype = PubType::Book;
                    }
                }
                Event::Text(e) => {
                    if after_title && text_section {
                        desc = e.unescape()?.to_string();
                    } else if text_section {
                        info.push(e.unescape()?);
                    }
                }
                Event::End(e) => {
                    if after_title {
                        text_section = false;
                        after_title = false;
                    } else if e.name().as_ref() == b"div" && text_section {
                        text_section = false;
                    } else if e.name().as_ref() == b"h1" && text_section {
                        text_section = false;
                        after_title = true;
                    }
                }
                Event::Eof => {
                    break;
                }
                _ => (),
            }
        }
        
        let pubinfo: Vec<&str> = info[4].splitn(4, '.').collect();

        Ok(Meta::new(
            info[2].replace(".", "").replace("г", "").trim().parse()?,
            *b"ruRU",
            info[0].to_string(),
            pubinfo[3].to_string(),
            pubtype,
            desc,
            vec![pubinfo[0].to_string() + "." + pubinfo[1] + "." + pubinfo[2] + "."],
        ))
    }

    async fn search(&mut self, query: String) {}
}
