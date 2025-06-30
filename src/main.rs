pub mod helpers;

use crate::helpers::get_full_name;

fn main() {
    println!("Hello, world!");
    let my_result = get_full_name("Ramachandran", "Ramakrishnan");
    println!("{}", my_result);
}
