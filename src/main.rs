mod control_flow;
mod core_data_types;
mod stack_and_heap;
mod combination_lock;
mod structs;
mod enumerations;
mod union;
mod option;
mod array_slice_tuples;
mod generics;
mod data_structures;
mod iterators;
mod types;
mod string;

use crate::array_slice_tuples::{array_demo, slice_demo, tuples_demo};
use crate::control_flow::*;
use crate::core_data_types::core_data_types;
use crate::stack_and_heap::stack_and_heap;
use crate::combination_lock::combination_lock;
use crate::data_structures::{hashmap_demo, vec_demo};
use crate::enumerations::enum_demo;
use crate::option::option_demo;
use crate::structs::struct_demo;
use crate::union::union_demo;
use crate::generics::generics_demo;
use crate::iterators::iter_demo;
use crate::string::string_demo;

const MEANING_OF_LIFE: u8 = 42;
static VALOR: u8 = 10;

// functions
fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) // start with i32
{
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 // return value
{
    let z = x * y;
    z // no semicolons
}

pub fn functions_demo() {
    print_value(123);

    let mut z = 1;
    increase(&mut z); // lend z
    println!("z = {}", z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p);
}

fn main() {
    // println!("Meaning of life: {}", MEANING_OF_LIFE);
    // println!("Value of valor plus 10: {}", VALOR + 10);

    // functions_demo();
    // core_data_types();
    // stack_and_heap();
    // temperature_with_if(22);
    // while_demo();
    // for_demo();
    // match_demo();
    // combination_lock();
    // struct_demo();
    // enum_demo();
    // union_demo();
    // option_demo();
    // array_demo();
    // slice_demo();
    // tuples_demo();
    // generics_demo();
    // vec_demo();
    // hashmap_demo();
    // iter_demo();
    string_demo();
}
