// SPDX-License-Identifier: GPL-2.0-only
//! PID 1 fixture for boot-testing images produced by the migrated build tools.

use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::MetadataExt;
use std::time::Duration;

#[cfg(not(target_arch = "x86_64"))]
compile_error!("the boot fixture uses the x86-64 Linux syscall ABI");

fn load_module(bytes: &[u8]) -> io::Result<()> {
    unsafe extern "C" {
        fn syscall(number: std::ffi::c_long, ...) -> std::ffi::c_long;
    }
    // x86-64 syscall_64.tbl: init_module = 175. The module buffer and empty
    // NUL-terminated parameters remain valid for the synchronous system call.
    let result = unsafe { syscall(175, bytes.as_ptr(), bytes.len(), c"".as_ptr()) };
    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

fn main() {
    assert_eq!(std::process::id(), 1);
    let expected = b"Rust-generated initramfs fixture\n\0with binary data\xff";
    assert_eq!(fs::read("/fixture").unwrap(), expected);
    assert_eq!(fs::read("/hardlink").unwrap(), expected);
    assert_eq!(fs::read("/symlink").unwrap(), expected);
    assert_eq!(
        fs::read_link("/symlink").unwrap().to_str(),
        Some("/fixture")
    );
    let file = fs::metadata("/fixture").unwrap();
    let link = fs::metadata("/hardlink").unwrap();
    assert_eq!(file.ino(), link.ino());
    assert_eq!(file.nlink(), 2);
    assert_eq!(file.uid(), 123);
    assert_eq!(file.gid(), 456);
    assert_eq!(file.mode() & 0o777, 0o640);
    // These optional fixtures only exist in the isolated test VM. Check
    // rejected signatures before the valid load, so EEXIST cannot mask them.
    for index in 0.. {
        let path = format!("/reject-module.{index}");
        if !fs::exists(&path).unwrap() {
            break;
        }
        let result = load_module(&fs::read(path).unwrap());
        assert_eq!(
            result.unwrap_err().raw_os_error(),
            Some(129),
            "expected EKEYREJECTED"
        );
        println!("LUPOS_RUST_MODULE_REJECT_OK {index}");
    }
    if fs::exists("/test-module.ko").unwrap() {
        load_module(&fs::read("/test-module.ko").unwrap()).unwrap();
        println!("LUPOS_RUST_MODULE_LOAD_OK");
    }
    println!("LUPOS_RUST_BUILD_BOOT_OK");
    io::stdout().flush().unwrap();
    // The runner stops the VM after receiving the marker. Keeping PID 1 alive
    // avoids an intentional kernel panic being mistaken for a boot failure.
    loop {
        std::thread::sleep(Duration::from_secs(60));
    }
}
