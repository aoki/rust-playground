use std::{fs, path::PathBuf};

fn main() {
    let path: PathBuf = ["..", "dots"].iter().collect();
    println!("{:?}", &path);

    let full_path = fs::canonicalize::<PathBuf>(path).unwrap();
    println!("{:?}", &full_path);

    let path: PathBuf = ["~", "work", "src", "github.com", "aoki", "rust-playground"]
        .iter()
        .collect();
    println!("{:?}", &path);

    let s = path.to_string_lossy();
    let full_path = shellexpand::tilde(&s);
    println!("{:?}", &full_path);
}
