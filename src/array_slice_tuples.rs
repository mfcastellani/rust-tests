use std::mem;

pub(crate) fn array_demo() {
    let mut a/*:[i32;5]*/ = [1, 2, 3, 4, 5];

    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("first value of a is {}", a[0]);

    assert_eq!(a, [321, 2, 3, 4, 5]);

    if a != [1, 2, 3, 5, 6]
    // size must match
    {
        println!("arrays not equal!");
    }

    // fill an array with 1s
    let b = [1u16; 10]; // try changing to 5
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    // just print the entire array
    println!("{:?}", b);
    println!("b took up {} bytes", mem::size_of_val(&b));

    // multidimensional array
    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    println!("{:?}", mtx);

    // print all the diagonal values
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn use_slice(slice: &mut [i32]) {
    println!("first elem is {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;

    // will crash
    //let z = slice[10];
}

pub(crate) fn slice_demo() {
    // a slice is part of an array
    // its size is not known at compile time
    let mut data = [1, 2, 3, 4, 5];

    // start w/o mut, borrow as a slice
    use_slice(&mut data[1..4]);
    use_slice(&mut data); // entire array

    println!("data after slice use = {:?}", data);
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

pub(crate) fn tuples_demo() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(3, 4);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1); // try sp.5

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    // tuple of tuples
    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last element is {}", (combined.1).1);

    let ((c, d), (e, f)) = combined;

    // tuple of different elements
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    // tuple of a single element
    let meanings = (42,); // start w/o comma
    println!("{:?}", meanings);
}

