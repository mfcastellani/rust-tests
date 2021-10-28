enum Color {
    Red, // unit-like struct
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple struct
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

pub(crate) fn enum_demo() {
    let c = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };
    //Color::RgbColor(0,0,0);

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),

        Color::RgbColor(0, 0, 0)
        | Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("black"),

        Color::RgbColor(r, g, b) => println!("rgb({},{},{}", r, g, b),
        _ => (),
    }
}
