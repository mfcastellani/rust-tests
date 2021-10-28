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

use crate::array_slice_tuples::{array_demo, slice_demo, tuples_demo};
use crate::control_flow::*;
use crate::core_data_types::core_data_types;
use crate::stack_and_heap::stack_and_heap;
use crate::combination_lock::combination_lock;
use crate::enumerations::enum_demo;
use crate::option::option_demo;
use crate::structs::struct_demo;
use crate::union::union_demo;
use crate::generics::generics_demo;

const MEANING_OF_LIFE: u8 = 42;
static VALOR: u8 = 10; // melhor usar const neste caso

fn main() {
    println!("Meaning of life: {}", MEANING_OF_LIFE);
    println!("Value of valor plus 10: {}", VALOR + 10);

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
    generics_demo();
}
