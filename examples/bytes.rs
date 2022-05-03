fn main() {
    let v = "hello";
    println!("v: {}", v);
    println!("&: {:?}", v.as_bytes());
    println!("p: {:?}", v.as_ptr());
}
