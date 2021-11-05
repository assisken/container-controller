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

    pub fn set_limits<'a>(container: &'a Container) -> Result<&Container> {
        Self::set_cores(container)?;
        Self::set_memory(container)
    }

    fn set_cores<'a>(container: &'a Container) -> Result<&Container> {
        let cores = match container.cores {
            Some(memory) => memory,
            None => return Ok(container),
        };

        match lxc(&[
            "config",
            "set",
            &container.name,
            "limits.cpu",
            &cores.to_string(),
        ]) {
            Ok(_) => Ok(container),
            Err(err) => Err(err),
        }
    }

    fn set_memory<'a>(container: &'a Container) -> Result<&Container> {
        let memory_gb = match container.memory_gb {
            Some(memory) => memory,
            None => return Ok(container),
        };
        let memory = format!("{}GB", memory_gb);

        match lxc(&["config", "set", &container.name, "limits.memory", &memory]) {
            Ok(_) => Ok(container),
            Err(err) => Err(err),
        }
    }

    pub fn delete(container: &Container) {
        if container.do_not_remove == Some(true) {
            return;
        }
        let _ = lxc(&["stop", &container.name]);
    }
}
