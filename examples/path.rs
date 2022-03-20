use billboard::{Alignment, Billboard};
use colored::Colorize;
use std::{fs, path::PathBuf};

fn main() {
    Billboard::builder()
        .text_alignment(Alignment::Left)
        .box_alignment(Alignment::Left)
        .build()
        .display(&format!("{}", "fs::canonicalize".bold().red()));

    println!("{}", "[..]".bold().green());
    let path: PathBuf = ["..", "dots"].iter().collect();
    let full_path = fs::canonicalize::<PathBuf>(path.clone());
    println!("{:?}", &path);
    println!("{:?}\n", &full_path);

    println!("{}", "[.]".bold().green());
    let path: PathBuf = [".", "src"].iter().collect();
    let full_path = fs::canonicalize::<PathBuf>(path.clone());
    println!("{:?}", &path);
    println!("{:?}\n", &full_path);

    println!("{}", "[~]".bold().green());
    let path: PathBuf = ["~", "work", "src", "github.com", "aoki", "rust-playground"]
        .iter()
        .collect();
    let full_path = fs::canonicalize::<PathBuf>(path.clone());
    println!("{:?}", &path);
    println!("{:?}\n", &full_path);

    Billboard::builder()
        .text_alignment(Alignment::Left)
        .box_alignment(Alignment::Left)
        .build()
        .display(&format!("{}", "shellexpand::tilde".bold().red()));

    println!("{}", "[..]".bold().green());
    let path: PathBuf = ["..", "rust-playground"].iter().collect();
    let s = path.to_string_lossy();
    let full_path = shellexpand::tilde(&s);
    println!("{:?}", &path);
    println!("{:?}", &full_path);

    println!("{}", "[.]".bold().green());
    let path: PathBuf = [".", "src"].iter().collect();
    let s = path.to_string_lossy();
    let full_path = shellexpand::tilde(&s);
    println!("{:?}", &path);
    println!("{:?}", &full_path);

    println!("{}", "[~]".bold().green());
    let path: PathBuf = ["~", "work", "src", "github.com", "aoki", "rust-playground"]
        .iter()
        .collect();
    let s = path.to_string_lossy();
    let full_path = shellexpand::tilde(&s);
    println!("{:?}", &path);
    println!("{:?}", &full_path);
}
