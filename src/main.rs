use std::io;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    loop {
        stdin.read_line(&mut input);
        input.pop();
        if input == *"stop" {
            return;
        }
        match calc::run(input.clone()) {
            Ok(res) => println!("Result: {res}"),
            Err(error) => println!("{error}"),
        }
    }
}
