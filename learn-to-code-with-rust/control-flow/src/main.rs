fn main() {
    let color = "blue";
    let number_if = color_to_number(color);
    let number_match = color_to_number_match(color);
    let number_match_true = color_to_number_match_true(color);
    println!(
        "Color: {}, Number (if): {}, Number (match): {}, Number (match true): {}",
        color, number_if, number_match, number_match_true
    );

    let n = 4;
    let fact_non_recursive = factorial_non_recursive(n);
    let fact_recursive = factorial_recursive(n);
    println!("Factorial of {} (non-recursive): {}", n, fact_non_recursive);
    println!("Factorial of {} (recursive): {}", n, fact_recursive);
}

fn color_to_number(color: &str) -> u32 {
    if color == "red" {
        1
    } else if color == "green" {
        2
    } else if color == "blue" {
        3
    } else {
        0
    }
}

fn color_to_number_match(color: &str) -> u32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn color_to_number_match_true(color: &str) -> u32 {
    match true {
        _ if color == "red" => 1,
        _ if color == "green" => 2,
        _ if color == "blue" => 3,
        _ => 0,
    }
}

fn factorial_non_recursive(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

fn factorial_recursive(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}
