// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021, Microsoft Corporation.
 *
 * Authors:
 *   Beau Belgrave <beaub@linux.microsoft.com>
 */

use std::ffi::{c_char, c_int, c_ulong, c_void};

const DATA_FILE: *const c_char = b"/sys/kernel/tracing/user_events_data\0".as_ptr() as *const c_char;
const O_RDWR: c_int = 0x0002;
const DIAG_IOCSREG: c_ulong = 0x4008_7520;

#[repr(C)]
struct Iovec {
    iov_base: *mut c_void,
    iov_len: usize,
}

#[repr(C)]
struct UserReg {
    size: u32,
    enable_bit: u8,
    enable_size: u8,
    flags: u16,
    enable_addr: u64,
    name_args: u64,
    field_count: u32,
    field_size: u32,
    status_bit: u32,
    write_index: c_int,
}

unsafe extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn writev(fd: c_int, iov: *const Iovec, iovcnt: c_int) -> isize;
    fn printf(format: *const c_char, ...) -> c_int;
    fn getchar() -> c_int;
    static mut errno: c_int;
}

static mut ENABLED: c_int = 0;

unsafe fn event_reg(fd: c_int, command: *const c_char, write: *mut c_int, enabled: *mut c_int) -> c_int {
    let mut reg: UserReg = std::mem::zeroed();

    reg.size = std::mem::size_of::<UserReg>() as u32;
    reg.enable_bit = 31;
    reg.enable_size = std::mem::size_of::<c_int>() as u8;
    reg.enable_addr = enabled as u64;
    reg.name_args = command as u64;

    if ioctl(fd, DIAG_IOCSREG, &mut reg) == -1 {
        return -1;
    }

    *write = reg.write_index;

    0
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let data_fd: c_int;
    let mut write: c_int = 0;
    let mut io: [Iovec; 2];
    let mut count: u32 = 0;

    data_fd = open(DATA_FILE, O_RDWR);

    if event_reg(data_fd, b"test u32 count\0".as_ptr() as *const c_char, &mut write, &raw mut ENABLED) == -1 {
        return errno;
    }

    /* Setup iovec */
    io = [
        Iovec {
            iov_base: &mut write as *mut c_int as *mut c_void,
            iov_len: std::mem::size_of::<c_int>(),
        },
        Iovec {
            iov_base: &mut count as *mut u32 as *mut c_void,
            iov_len: std::mem::size_of::<u32>(),
        },
    ];

    loop {
        printf(b"Press enter to check status...\n\0".as_ptr() as *const c_char);
        getchar();

        /* Check if anyone is listening */
        if ENABLED != 0 {
            /* Yep, trace out our data */
            writev(data_fd, io.as_ptr(), 2);

            /* Increase the count */
            count = count.wrapping_add(1);

            printf(b"Something was attached, wrote data\n\0".as_ptr() as *const c_char);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
