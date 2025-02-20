fn main() {
    let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string;
    
    use std::io;
    let mut line = String::new();
    while io::stdin().read_line(&mut line).unwrap() > 0 {
        if line.contains(pattern){
            print!("{line}");
        }

        line.clear();
    }
}
