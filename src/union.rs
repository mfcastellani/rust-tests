// we don't know whether this contains an int
// or a float
union IntOrFloat {
    i: i32,
    f: f32,
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value");
            }

            //           ↓↓↓ we get to treat it as f32
            IntOrFloat { f } => {
                // we don't know if it's int or float
                println!("got some value which could be a float {}", f);
            }
        }
    }
}

pub(crate) fn union_demo() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    // cannot access member without an unsafe block
    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    let iof2 = IntOrFloat { f: 42.0 };

    process_value(iof2);

    // this will interpret an int as a float
    process_value(IntOrFloat { i: 123456 });
}
