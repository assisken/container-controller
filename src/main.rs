mod containers;
mod lxd;
mod smiap;

use crate::lxd::LXD;

fn main() {
    let smiap = smiap::Smiap::new();
    let desired = smiap.fetch_containers().unwrap();

    let actual = LXD::get_containers();

    println!("Desired:\n{:?}", desired);
    println!("Actual:\n{:?}", actual);
    println!("---");

    let to_create = desired.difference(&actual);
    let to_delete = actual.difference(&desired);

    println!("To create:\n{:?}", to_create);
    println!("To delete:\n{:?}", to_delete);

    for container in to_create {
        LXD::create_local(container).unwrap();
    }
    for container in to_delete {
        LXD::delete(container);
    }
}
