use std::io::{self, Write};

fn main() {
    print!("What is your name?: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    let _b1 = io::stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();

    if _b1 == 0 {
        print!("No name entered :(, goodbye.\n");
        std::process::exit(1);
    } else {
        print!("Hello, {}, nice to meet you!\n", name);
    }
}
