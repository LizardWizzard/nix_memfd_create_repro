use std::ffi::CString;

use nix::sys::memfd;

fn main() {
    memfd::memfd_create(
        &CString::new("foo").unwrap(),
        memfd::MemFdCreateFlag::MFD_CLOEXEC,
    )
    .expect("create");

    println!("Hello, world!");
}
