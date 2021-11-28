pub fn string_demo() {
    // a slice that always points to a valid UTF-8 sequence
    // a view into a String
    let s: &'static str = "hi there!"; // &'static str -->
    // statically allocated (part of the program)

    // s = "bar"; // cannot reassign immutable

    //let a = s[0]; // cannot index

    for c in s.chars().rev()
    // reversed! also as_bytes()
    {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char)
    }

    // heap allocated construct
    // Vec<u8>, guaranteed to be valid UTF-8

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(","); // note the _str
        a = a + 1;
    }

    println!("{}", letters);

    // str from String
    let u: &str = &letters; // deref conversion
    // there are situations when the coercion does NOT happen

    // concatenation
    // String + str
    // String + &String

    // String from str
    //let mut abc = String::from("hello world");
    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}, {}", abc, abc.replace("ello", "goodbye"));
}
