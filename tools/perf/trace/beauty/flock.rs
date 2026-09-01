// SPDX-License-Identifier: LGPL-2.1

use std::os::raw::{c_char, c_int};

// C dependencies removed from executable Rust:
// #include "trace/beauty/beauty.h"
// #include <linux/kernel.h>
// #include <linux/fcntl.h>

const LOCK_SH: c_int = 1;
const LOCK_EX: c_int = 2;
const LOCK_NB: c_int = 4;
const LOCK_UN: c_int = 8;

const LOCK_MAND: c_int = 32;
const LOCK_READ: c_int = 64;
const LOCK_WRITE: c_int = 128;
const LOCK_RW: c_int = 192;

#[repr(C)]
pub struct syscall_arg {
    pub val: c_int,
    pub show_string_prefix: bool,
}

unsafe extern "C" {
    fn scnprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
}

pub unsafe extern "C" fn syscall_arg__scnprintf_flock(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let show_prefix: bool = unsafe { (*arg).show_string_prefix };
    let prefix: *const c_char = c"LOCK_".as_ptr();
    let mut printed: c_int = 0;
    let mut op: c_int = unsafe { (*arg).val };

    if op == 0 {
        return unsafe { scnprintf(bf, size, c"NONE".as_ptr()) as usize };
    }

    macro_rules! P_CMD {
        ($cmd:ident, $lock:ident) => {
            if (op & $lock) == $lock {
                printed += unsafe {
                    scnprintf(
                        bf.offset(printed as isize),
                        size.wrapping_sub(printed as usize),
                        c"%s%s%s".as_ptr(),
                        if printed != 0 {
                            c"|".as_ptr()
                        } else {
                            c"".as_ptr()
                        },
                        if show_prefix { prefix } else { c"".as_ptr() },
                        concat!(stringify!($cmd), "\0").as_ptr() as *const c_char,
                    )
                };
                op &= !$lock;
            }
        };
    }

    P_CMD!(SH, LOCK_SH);
    P_CMD!(EX, LOCK_EX);
    P_CMD!(NB, LOCK_NB);
    P_CMD!(UN, LOCK_UN);
    P_CMD!(MAND, LOCK_MAND);
    P_CMD!(RW, LOCK_RW);
    P_CMD!(READ, LOCK_READ);
    P_CMD!(WRITE, LOCK_WRITE);

    if op != 0 {
        printed += unsafe {
            scnprintf(
                bf.offset(printed as isize),
                size.wrapping_sub(printed as usize),
                c"%s%#x".as_ptr(),
                if printed != 0 {
                    c"|".as_ptr()
                } else {
                    c"".as_ptr()
                },
                op,
            )
        };
    }

    printed as usize
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
