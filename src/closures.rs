fn say_hello() {
    println!("hello");
}

pub(crate) fn closure_demo() {
    let sh = say_hello;
    sh();

    let plus_one = |x: i32| -> i32 { x + 1 };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    {
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    }

    let borrow_two = &mut two;

    // T: by value
    // T&
    // &mut &
    let plus_three = |mut x: i32| x += 3;

    let mut f = 12;
    plus_three(f);
    println!("f = {}", f);
}
