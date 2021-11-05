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
    pub fn from_lxd_info(info: lxd::Info) -> Self {
        let cores: Option<i32> = match info.config.get("limits.cpu") {
            Some(cpu) => Some(cpu.parse().unwrap()),
            None => None,
        };
        let memory_gb: Option<i32> = match info.config.get("limits.memory") {
            Some(memory) => Some(memory.replace("GB", "").parse().unwrap()),
            None => None,
        };

        Container {
            cores,
            memory_gb,
            name: info.name,
            image: default_image(),
            partition_size_gb: None,
            do_not_remove: None,
            group: None,
        }
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
