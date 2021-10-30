use reqwest;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Container {
    pub id: i32,
    pub name: String,
    pub do_not_remove: bool,
    pub cores: i32,
    pub memory_gb: i32,
    pub partition_size_gb: i32,
    pub group: Option<i32>,
}

pub struct Smiap {
    url: &'static str
}

impl Smiap {
    pub fn new() -> Smiap {
        let url = "https://smiap.ru/api/v1";

        Smiap{ url }
    }

    pub fn get_containers(&self) -> Result<Vec<Container>, reqwest::Error> {
        let url = format!("{}/{}", self.url, "containers/");

        reqwest::blocking::get(url)?.json::<Vec<Container>>()
    }
}
