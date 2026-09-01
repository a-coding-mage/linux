// SPDX-License-Identifier: GPL-2.0
// C dependencies: <fcntl.h>, <linux/fs.h>, <stdio.h>, <sys/ioctl.h>

use std::env;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_ulong, c_void};

const O_RDONLY: c_int = 0;
const FS_IOC_GETLBMD_CAP: c_ulong = 0x4008667f;

#[repr(C)]
#[derive(Default)]
struct logical_block_metadata_cap {
    lbmd_size: u32,
    lbmd_pi_offset: u32,
    lbmd_pi_size: u32,
}

extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cap = logical_block_metadata_cap::default();
    let filename: CString;
    let fd: c_int;
    let result: c_int;

    if args.len() != 2 {
        eprintln!("Usage: {} BLOCK_DEVICE", args[0]);
        std::process::exit(1);
    }

    filename = CString::new(args[1].as_str()).unwrap();
    fd = unsafe { open(filename.as_ptr(), O_RDONLY) };
    if fd < 0 {
        unsafe {
            perror(filename.as_ptr());
        }
        std::process::exit(1);
    }

    result = unsafe {
        ioctl(
            fd,
            FS_IOC_GETLBMD_CAP,
            &mut cap as *mut logical_block_metadata_cap as *mut c_void,
        )
    };
    if result < 0 {
        let ioctl_name = CString::new("ioctl").unwrap();
        unsafe {
            perror(ioctl_name.as_ptr());
        }
        std::process::exit(1);
    }

    println!("metadata_size: {}", cap.lbmd_size);
    println!("pi_offset: {}", cap.lbmd_pi_offset);
    println!("pi_tuple_size: {}", cap.lbmd_pi_size);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
