// SPDX-License-Identifier: GPL-2.0
/*
 * Provide diagnose information via misc device /dev/diag.
 *
 * Copyright IBM Corp. 2024
 */

// Dependencies supplied by the kernel and by the corresponding diagnostic
// headers are intentionally left as external Rust symbols.

use core::ffi::{c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn diag324_piblen(arg: c_ulong) -> c_long;
    fn diag324_pibbuf(arg: c_ulong) -> c_long;
    fn diag310_memtop_stride(arg: c_ulong) -> c_long;
    fn diag310_memtop_len(arg: c_ulong) -> c_long;
    fn diag310_memtop_buf(arg: c_ulong) -> c_long;
    fn misc_register(dev: *mut miscdevice) -> c_int;
    fn nonseekable_open(inode: *mut inode, file: *mut file) -> c_int;
}

type c_long = isize;

#[repr(C)]
struct inode;

#[repr(C)]
struct file;

#[repr(C)]
struct file_operations {
    owner: *mut c_void,
    open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    unlocked_ioctl:
        Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
}

#[repr(C)]
struct miscdevice {
    minor: c_int,
    name: *const u8,
    fops: *const file_operations,
    mode: u16,
}

extern "C" {
    static mut THIS_MODULE: c_void;
}

const MISC_DYNAMIC_MINOR: c_int = 255;
const ENOIOCTLCMD: c_long = 515;

// Values are supplied by <uapi/asm/diag.h> and "diag_ioctl.h".
extern "C" {
    static DIAG324_GET_PIBLEN: c_uint;
    static DIAG324_GET_PIBBUF: c_uint;
    static DIAG310_GET_STRIDE: c_uint;
    static DIAG310_GET_MEMTOPLEN: c_uint;
    static DIAG310_GET_MEMTOPBUF: c_uint;
}

unsafe extern "C" fn diag_ioctl(
    _filp: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_long {
    let rc: c_long;

    match cmd {
        DIAG324_GET_PIBLEN => {
            rc = diag324_piblen(arg);
        }
        DIAG324_GET_PIBBUF => {
            rc = diag324_pibbuf(arg);
        }
        DIAG310_GET_STRIDE => {
            rc = diag310_memtop_stride(arg);
        }
        DIAG310_GET_MEMTOPLEN => {
            rc = diag310_memtop_len(arg);
        }
        DIAG310_GET_MEMTOPBUF => {
            rc = diag310_memtop_buf(arg);
        }
        _ => {
            rc = -ENOIOCTLCMD;
        }
    }
    rc
}

static FOPS: file_operations = file_operations {
    owner: unsafe { &raw mut THIS_MODULE },
    open: Some(nonseekable_open),
    unlocked_ioctl: Some(diag_ioctl),
};

static mut DIAGDEV: miscdevice = miscdevice {
    name: b"diag\0".as_ptr(),
    minor: MISC_DYNAMIC_MINOR,
    fops: &FOPS,
    mode: 0o444,
};

unsafe extern "C" fn diag_init() -> c_int {
    misc_register(&raw mut DIAGDEV)
}

// device_initcall(diag_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
