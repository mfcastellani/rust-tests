use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked
}

pub(crate) fn combination_lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();
    println!("Starting the Combination Lock game!");

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                println!("Current code is {}. Please type: ", entry);
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                    continue;
                }
            }

            State::Failed => {
                println!("You failed!!!!");
                entry.clear();
                state = State::Locked;
                continue;
            }

            State::Unlocked => {
                println!("Great work!!!");
                break;
            }
        }
    }
}