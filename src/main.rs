// Este projeto tem vários dead code apenas para
// demonstração, então ignoramos os warnings de dead
// code no projeto (veja o !)
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

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
mod closures;
mod higher_order_functions;
mod traits;
mod into_drop;
mod lifetime;

use crate::array_slice_tuples::{array_demo, slice_demo, tuples_demo};
use crate::closures::closure_demo;
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
use crate::into_drop::into_drop_demo;
use crate::iterators::iter_demo;
use crate::lifetime::lifetime_demo;
use crate::string::string_demo;
use crate::traits::trait_demo;

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
    println!("=> Constants demo");
    println!("Meaning of life: {}", MEANING_OF_LIFE);
    println!("Value of valor plus 10: {}", VALOR + 10);

    println!("\n => Functions demo");
    functions_demo();
    println!("\n => Core data types");
    core_data_types();
    println!("\n => Stack and heap");
    stack_and_heap();
    println!("\n => Temperature with if");
    temperature_with_if(22);
    println!("\n => While demo");
    while_demo();
    println!("\n => For demo");
    for_demo();
    println!("\n => Match demo");
    match_demo();
    // println!("\n => Combination lock");
    // combination_lock();
    println!("\n => Struct demo");
    struct_demo();
    println!("\n => Enum demo");
    enum_demo();
    println!("\n => Union demo");
    union_demo();
    println!("\n => Option demo");
    option_demo();
    println!("\n => Array demo");
    array_demo();
    println!("\n => Slice demo");
    slice_demo();
    println!("\n => Tuples demo");
    tuples_demo();
    println!("\n => Generics demo");
    generics_demo();
    println!("\n => Vec demo");
    vec_demo();
    println!("\n => Hashmap demo");
    hashmap_demo();
    println!("\n => Iter demo");
    iter_demo();
    println!("\n => String demo");
    string_demo();
    println!("\n => Closure demo");
    closure_demo();
    println!("\n => Trait demo");
    trait_demo();
    println!("\n => Into and Debug");
    into_drop_demo();
    println!("\n => Lifetime");
    lifetime_demo();
}

