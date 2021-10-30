use core::hash::Hash;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Container {
    pub name: String,

    #[serde(default = "default_image")]
    pub image: String,
    pub cores: Option<i32>,
    pub memory_gb: Option<i32>,
    pub partition_size_gb: Option<i32>,
    pub do_not_remove: Option<bool>,
    pub group: Option<i32>,
}

fn default_image() -> String {
    String::from("ubuntu:20.04")
}

impl Container {
    fn from_name(name: String) -> Self {
        Container {
            name,
            image: default_image(),
            cores: None,
            memory_gb: None,
            partition_size_gb: None,
            do_not_remove: None,
            group: None,
        }
    }

    pub fn to_lxd(&self) -> lxd::Container {
        lxd::Container::new(
            lxd::Location::Local,
            self.name.as_str(),
            self.image.as_str(),
        )
        .unwrap()
    }

    pub fn from_lxd_info(info: lxd::Info) -> Self {
        Self::from_name(info.name)
    }
}

impl PartialEq for Container {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Container {}
impl Hash for Container {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
