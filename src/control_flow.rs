pub(crate) fn temperature_with_if(temp: i32) {
    if temp > 30 {
        // curly braces are mandatory!
        println!("really hot outside!");
    } else if temp < 10 {
        println!("really cold, don't go out!");
    } else {
        println!("temperature is OK");
    }

    // if is an expression!
    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("today is {}", day);

    // 20+ hot, <20 cold
    println!(
        "it is {}",
        if temp > 20 {
            "hot"
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    );

    println!(
        "it is {}",
        if temp > 20 {
            if temp > 30 {
                "very hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    );
}

pub(crate) fn while_demo() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;

        if x == 64 {
            continue;
        }

        println!("x = {}", x);
    }

    let mut y = 1;

    loop
    // while true
    {
        y *= 2;
        println!("y = {}", y);

        // stop at 2^10
        if y == 1 << 10 {
            break;
        }
    }
}

pub(crate) fn for_demo() {
    for x in 1..11
    // 1 to 10; 11..1 won't work
    {
        // skip 3
        if x == 3 {
            continue;
        }

        // stop at 7
        if x == 8 {
            break;
        }

        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

pub(crate) fn match_demo() {
    let country_code = 999;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown", // inclusive range ... is deprecated
        _ => "invalid",        // try commenting this out - must cover all cases!
    };

    println!("the country with code {} is {}", country_code, country);

    let x = false;

    let _s = match x {
        true => "yes",
        false => "no",
    };
}
