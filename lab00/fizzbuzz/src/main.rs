fn main() {
    for i in 1..=100 {
        let mut line = i.to_string();
        if i % 3 == 0 && i % 5 == 0 {
            line = "FizzBuzz".to_string();
        } else if i % 3 == 0 {
            line = "Fizz".to_string();
        } else if i % 5 == 0 {
            line = "Buzz".to_string();
        }

        println!("{}", line)
    }
}
