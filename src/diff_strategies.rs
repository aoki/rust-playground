fn compare(text1: &str, text2: &str) -> bool {
    text1 == text2
}

fn main() {
    let text1 = "Apples are a fruit.";
    let text2 = "Bananas are also fruit.";
    let diff = compare(text1, text2);

    println!("Text 1: {}", &text1);
    println!("Text 2: {}", &text2);
    println!("Diff  : {}", &diff);
}

#[cfg(test)]
mod tests {
    #[test]
    fn equality_check() {
        let text1 = "Apples are a fruit.";
        let text2 = "Bananas are also fruit.";
        let text3 = "林檎は果物です。";
        let text4 = "バナナも果物です。";
        let actural = compare(text1, text2);
        assert_eq!(compare, true);
    }
}
