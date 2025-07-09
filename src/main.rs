pub mod closures;
pub mod control_flow;
pub mod helpers;
pub mod match_expression;
pub mod rust_enum;
pub mod rust_struct_generic_trait;
pub mod test_traits;
pub mod vec_types;
// use crate::helpers::get_full_name;
// use crate::control_flow::{test_if, test_while, test_loop, test_for};
// use crate::closures::test_closures;
// use crate::match_expression::*;
//use crate::rust_enum::*;
// use crate::rust_struct_generic_trait::*;
// use crate::test_traits::*;
use crate::vec_types::*;

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

    // test_match_string();

    // let result = test_option_type();
    // println!("{:?}", result);

    //let result = test_option_chartype();
    //println!("{:?}", result.unwrap());

    // test_create_new_person();

    // test_create_new_vehicle();

    // create_new_vehicle_tuple();

    // test_create_static_new_vehicle();

    // create_person();
    
    // test_vec_int();
    
    test_vec_string();

}
