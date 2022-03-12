use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};
use std::{fs, io::Cursor};
fn main() -> std::io::Result<()> {
    let dir = fs::read_dir(".")?;
    let file_list: Vec<String> = dir
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    println!("{:?}", file_list);

    let skim_options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(file_list.join("\n")));

    let selected_items = Skim::run_with(&skim_options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        print!("{}\n", item.output());
    }

    Ok(())
}
