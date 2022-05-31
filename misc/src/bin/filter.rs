fn main() -> std::io::Result<()> {
    let list: Vec<Result<String, String>> = vec![
        Ok(String::from("foo")),
        Ok(String::from("bar")),
        Err(String::from("Error")),
        Ok(String::from("buz")),
        Err(String::from("Error")),
    ];
    println!("List: {:?}", list);

    let filterd_ok: Vec<&Result<String, String>> = list.iter().filter(|f| f.is_ok()).collect();
    println!("Filtered: {:?}", filterd_ok);

    let filterd_err: Vec<&Result<String, String>> = list.iter().filter(|f| f.is_err()).collect();
    println!("Filtered: {:?}", filterd_err);

    Ok(())
}
