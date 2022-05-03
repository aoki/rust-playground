use nix::sys::mman::{mmap, MapFlags, ProtFlags};

fn main() {
    let size = 1024;
    let buffer = unsafe {
        mmap(
            std::ptr::null_mut(),
            size,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_PRIVATE | MapFlags::MAP_ANONYMOUS,
            -1,
            0,
        )
    }
    .unwrap();

    for i in 0..size {
        unsafe {
            buffer.offset(i as isize).write_bytes(0, 1);
        }
    }
}
