use std::io;

fn main() {
    let mut name = String::new();

    println!("Please, input your name");

    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            print!("Hello, {}", name)
        }
        Err(error_text) => {
            print!("Input error. Please, input your name {}", error_text)
        }
    }
}
