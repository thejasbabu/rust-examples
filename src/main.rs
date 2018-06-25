use std::io;

fn main() {
    println!("Please enter your name");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read the name");

    println!("The name is {}", name);
}
