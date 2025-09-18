use std::io;

fn make_money(s: &mut String) {
    s.push_str("$$$");
}

fn trim_and_capitalize(s: &str) -> String {
    s.trim().to_uppercase()
}

fn elements(s: &str) -> Vec<&str> {
    s.split('!').collect()
}

fn get_identity() -> String {
    let mut first_name = String::new();
    let mut last_name = String::new();
    let input = io::stdin();

    println!("Please enter your first name: ");
    input
        .read_line(&mut first_name)
        .expect("Failed to collect first name");

    println!("Please enter your last name: ");
    input
        .read_line(&mut last_name)
        .expect("Failed to collect last name");

    format!("{} {}", first_name.trim(), last_name.trim())
}

fn main() {
    let mut my_string = String::from("I want to make money");
    make_money(&mut my_string);
    println!("{}", my_string);

    let trimmed_and_capitalized = trim_and_capitalize("  hello world  ");
    println!("{}", trimmed_and_capitalized);

    let elements = elements("Gold!Silver!Platinum");
    println!("{:?}", elements);

    let name = get_identity();
    println!("{}", name);
}
