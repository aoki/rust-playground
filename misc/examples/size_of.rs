use libc::malloc;
use std::{ffi::c_void, mem::size_of_val};

fn main() {
    println!("usize: {}", std::mem::size_of::<usize>());

    let p: *mut c_void;
    // let m = "hello\n".as_ptr() as c_void;

    // size_t size = 24;
    unsafe {
        p = malloc(std::mem::size_of::<usize>() * 10);
        println!("S: {}", size_of_val(&*p));
        // p.write_bytes("hello\n".as_ptr() as u8, 10);
        for i in 0..10 {
            print!("{:x}", p.offset(i) as u16);
        }
        println!();
        p.write_bytes(0, 10);

        for i in 0..10 {
            print!("{:x}", p.offset(i) as u16);
        }
        println!();
    }

    unsafe {
        libc::free(p as *mut libc::c_void);
    }
}
