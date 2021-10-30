mod smiap;

use smiap::Smiap;

fn main() {
    let smiap = Smiap::new();
    let containers = smiap.get_containers().unwrap();
    println!("{:?}", containers);
}
