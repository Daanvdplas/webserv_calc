use std::io;

fn main() {
    loop {
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input);
        input.pop();
        calc::run(input);
    }
}
