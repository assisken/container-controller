use std::collections::HashSet;

use reqwest;

use crate::containers::Container;

pub struct Smiap {
    url: &'static str,
}

impl Smiap {
    pub fn new() -> Self {
        let url = "https://smiap.ru/api/v1";

        Smiap { url }
    }

    pub fn fetch_containers(&self) -> Result<HashSet<Container>, reqwest::Error> {
        let url = format!("{}/{}", self.url, "containers/");

        reqwest::blocking::get(url)?.json::<HashSet<Container>>()
    }
}
