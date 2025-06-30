pub mod closures;
pub mod control_flow;
pub mod helpers;
pub mod match_expression;

// use crate::helpers::get_full_name;
// use crate::control_flow::{test_if, test_while, test_loop, test_for};
// use crate::closures::test_closures;
use crate::match_expression::*;

fn main() {
    println!("Hello, world!");
    // let my_result = get_full_name("Ramachandran", "Ramakrishnan");
    // println!("{}", my_result);

    // let result_if =  test_if();
    // println!("{:?}", result_if);

    // test_while()

    // test_loop()

    // test_for()

    // test_closures();

    // test_match_int();

    test_match_string();
}
