use std::mem;

pub(crate) fn core_data_types() {
    let mut a: u8 = 123; // unsigned, 8 bits (0 to 255)
    println!("Value of a: {:?}", a);

    a = 45;
    println!("Value of a: {:?}", a);

    let b: i8 = -123; // signed, 8 bits (-128 to 127)
    println!("Value of b: {}", b);

    let c: i8 = -123;
    println!("Value of c: {}, takes up {} bytes", c, mem::size_of_val(&c));

    let d: isize = 120; // usize and isize uses the size of processor
    println!("This is a {}-bit OS", mem::size_of_val(&d) * 8);

    // exponenciação
    let a_cubed = i32::pow(d as i32, 3);
    println!("{} ^ 3 = {}", d, a_cubed);

    // PI
    println!("PI is {}", std::f64::consts::PI);
}
