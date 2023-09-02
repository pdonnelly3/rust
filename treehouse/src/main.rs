use std::io::stdin;

fn main() {
    println!("Hello, {:?}", what_is_your_name());
}

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}
