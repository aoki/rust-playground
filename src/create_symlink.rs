use std::fs::{self, remove_file};
use std::os::unix::fs as unix_fs;
use std::path::Path;
fn main() -> std::io::Result<()> {
    let link = Path::new("./foo.link");

    unix_fs::symlink(Path::new("./Cargo.toml"), &link)?;

    let read = fs::read_link(link)?;
    println!("Read link: {:?}", read);

    remove_file(&link)?;

    Ok(())
}
