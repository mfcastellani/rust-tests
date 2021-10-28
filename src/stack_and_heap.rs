pub(crate) fn stack_and_heap() {
    // stack
    let x: i32 = 12345;
    println!("X is in stack and has {} inside", x);

    // heap
    let y = Box::new(123);
    println!("Y is in heap and has {} inside", y);
}
