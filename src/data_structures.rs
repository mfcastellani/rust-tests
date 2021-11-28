use std::collections::HashMap;

pub fn hashmap_demo() {
    let mut shapes = HashMap::new();

    // types that implement the Copy trait are ok
    // types that don't are moved
    let triangle = String::from("triangle");
    shapes.insert(triangle, 3);
    shapes.insert("square".into(), 4);

    //let t = triangle;

    println!("a square has {} sides", shapes["square".into()]);

    // iterate the entire thing
    for (key, value) in &shapes {
        println!("{} : {}", key, value);
    }

    // can overwrite values
    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);

    let e = shapes.entry("square".into());

    // upsert (only insert if it has no value)
    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);
}

pub fn vec_demo() {
    /*
    let mut a = Vec::new();

    a.push(1);
    a.push(2);
    a.push(3);
    */

    let mut a = vec![1, 2, 3]; // [1;10]
    println!("a = {:?}", a);

    let idx/*:i32*/ = 0; // will not work with :i32
    // you need usize
    println!("a[0] = {}", a[idx]);

    // unsafe access
    //println!("a[5] = {}", a[5]);

    match a.get(5) {
        Some(x) => println!("a[5] = {}", x),
        None => println!("error, no such element"),
    }

    // iterate
    for x in &a {
        println!("{}", x);
    }

    // adding/removing
    a.push(44);
    println!("{:?}", a);

    let last_elem = a.pop(); // can easily yield nothing
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    // explain why this doesn't work
    //let Some(last_value) = a.pop();

    // print the elements in reverse order
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}
