fn default_reverse(input: &String) -> String {
    input.chars().rev().collect()
}

/// simple reverse
fn simple_reverse(input: &String) -> String {
    let mut output = String::new();
    let size = input.len();
    for i in 0..size {
        output.push_str(&input[(size - i - 1)..(size - i)]);
    }
    output
}

fn main() {
    let string = String::from("The quick brown fox jumps over the lazy dog.");
    println!("{}", default_reverse(&string));
    println!("{}", simple_reverse(&string));

    let x = "hello";
    println!("{:?}", char::from(x.as_bytes()[0]));
    // input.chars().nth(i)
}
