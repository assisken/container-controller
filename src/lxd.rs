use std::collections::HashSet;

use crate::containers::Container;

pub struct LXD;

impl LXD {
    pub fn new() -> Self {
        LXD {}
    }

    pub fn get_containers(&self) -> HashSet<Container> {
        let mut containers = HashSet::new();

        for info in lxd::Info::all(lxd::Location::Local).unwrap() {
            containers.insert(Container::from_lxd_info(info));
        }

        containers
    }
}
