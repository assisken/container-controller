use std::io::{Error, ErrorKind, Result};
use std::{collections::HashSet, process::Command};

use crate::containers::Container;

pub struct LXD;

fn lxc(args: &[&str]) -> Result<()> {
    let mut cmd = Command::new("lxc");
    for arg in args.iter() {
        cmd.arg(arg);
    }

    let status = cmd.spawn()?.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("LXD {:?} failed with {}", args, status),
        ))
    }
}

impl LXD {
    pub fn get_containers() -> HashSet<Container> {
        let mut containers = HashSet::new();

        for info in lxd::Info::all(lxd::Location::Local).unwrap() {
            containers.insert(Container::from_lxd_info(info));
        }

        containers
    }

    pub fn create_local<'a>(container: &'a Container) -> Result<&'a Container> {
        Self::create(lxd::Location::Local, container)
    }

    fn create<'a>(location: lxd::Location, container: &'a Container) -> Result<&'a Container> {
        let full_name = match location {
            lxd::Location::Local => format!("{}", container.name),
            lxd::Location::Remote(remote) => format!("{}:{}", remote, container.name),
        };

        lxc(&["launch", &container.image, &full_name, "-e", "-n", "lxdbr0"])?;

        // Hack to wait for network up and running
        lxc(&[
            "exec",
            &full_name,
            "--mode=non-interactive",
            "-n",
            "--",
            "dhclient",
        ])?;

        Ok(container)
    }

    pub fn delete(container: &Container) {
        if container.do_not_remove == Some(true) {
            return;
        }
        let _ = lxc(&["stop", &container.name]);
    }
}
