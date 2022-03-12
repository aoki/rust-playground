use std::collections::HashSet;

/// https://stackoverflow.com/questions/39803237/build-hashset-from-a-vector-in-rust
/// https://stackoverflow.com/questions/64226562/check-if-vec-contains-all-elements-from-another-vec
fn main() {
    let list = "foo.txt,bar.txt,buz.txt";
    let ignore = "bar.txt,buz.txt";

    let file_list: Vec<String> = list.split(",").map(|e| e.to_string()).collect();
    let ignore_list: HashSet<String> = HashSet::from_iter(ignore.split(",").map(|e| e.to_string()));

    println!("File list: {:?}", file_list);
    println!("Ignore list: {:?}", &ignore_list);

    println!("\nContains test");
    println!("{:?}", file_list.contains(&"foo.txt".to_string()));
    println!("{:?}", ignore_list.contains(&"foo.txt".to_string()));

    println!("\nFilter test");
    let res = file_list.iter().all(|f| ignore_list.contains(f));
    println!("{:?}", res);

    println!("\nIgnore filter");
    let filtered_list: Vec<&String> = file_list
        .iter()
        .filter(|&s| !ignore_list.contains(s))
        .collect();
    println!("FILTERED LIST: {:?}", filtered_list);
}
