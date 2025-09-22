// not returning a reference, so no lifetime needed
fn double_the_length(vec: &Vec<i32>) -> usize {
    vec.len() * 2
}

// returning a slice that references the input slice, so no explicit lifetime needed
fn last_two(slice: &[i32]) -> &[i32] {
    let len = slice.len();
    &slice[len - 2..]
}

// returning a slice that references the input text, so explicit lifetime needed
// to show which input the output slice is tied to
fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("{}", announcement);
    &text[..5]
}

// returning a reference that could be from either input, so explicit lifetime needed
// to ensure both inputs live long enough for the output reference
fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &str) -> &'a str {
    if first.contains(target) {
        first
    } else {
        second
    }
}

fn main() {
    double_the_length(&vec![1, 2, 3]);
    double_the_length(&vec![1, 2, 3, 4]);
    last_two(&vec![1, 2, 3, 4, 5]);
    first_five("Hello, world!", "Announcing:");
    find_string_that_has_content("I have some content", "I have nothing", "content");
}
