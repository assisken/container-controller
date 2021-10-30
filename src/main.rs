mod containers;
mod lxd;
mod smiap;

fn main() {
    let smiap = smiap::Smiap::new();
    let desired = smiap.fetch_containers().unwrap();

    let lxd = lxd::LXD::new();
    let actual = lxd.get_containers();

    println!("Desired:\n{:?}", desired);
    println!("Actual:\n{:?}", actual);
    println!("---");

    let to_create = desired.difference(&actual);
    let to_delete = actual.difference(&desired);

    println!("To create:\n{:?}", to_create);
    println!("To delete:\n{:?}", to_delete);

    for container in to_create {
        container.to_lxd();
    }
    for container in to_delete {
        drop(container.to_lxd());
    }
}
