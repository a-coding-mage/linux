// SPDX-License-Identifier: LGPL-2.1

use std::ffi::{c_char, c_int};

// From trace/beauty/beauty.h and libc-style C interfaces.
#[repr(C)]
pub struct syscall_arg {
    pub show_string_prefix: bool,
    pub val: c_int,
}

unsafe extern "C" {
    fn scnprintf(bf: *mut c_char, size: usize, fmt: *const c_char, ...) -> usize;
}

/*
 * Not defined anywhere else, probably, just to make sure we
 * catch future flags
 */
const SCHED_POLICY_MASK: c_int = 0xff;

// Fallback definitions used when the C build environment does not provide them.
const SCHED_DEADLINE: c_int = 6;
const SCHED_RESET_ON_FORK: c_int = 0x40000000;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_sched_policy(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let show_prefix: bool = unsafe { (*arg).show_string_prefix };
    let prefix: *const c_char = c"SCHED_".as_ptr();
    let policies: [*const c_char; 7] = [
        c"NORMAL".as_ptr(),
        c"FIFO".as_ptr(),
        c"RR".as_ptr(),
        c"BATCH".as_ptr(),
        c"ISO".as_ptr(),
        c"IDLE".as_ptr(),
        c"DEADLINE".as_ptr(),
    ];
    let mut printed: usize;
    let mut policy: c_int = unsafe { (*arg).val };
    let mut flags: c_int = policy & !SCHED_POLICY_MASK;

    policy &= SCHED_POLICY_MASK;
    if policy <= SCHED_DEADLINE {
        printed = unsafe {
            scnprintf(
                bf,
                size,
                c"%s%s".as_ptr(),
                if show_prefix { prefix } else { c"".as_ptr() },
                policies[policy as usize],
            )
        };
    } else {
        printed = unsafe { scnprintf(bf, size, c"%#x".as_ptr(), policy) };
    }

    if flags & SCHED_RESET_ON_FORK != 0 {
        printed += unsafe {
            scnprintf(
                bf.add(printed),
                size.wrapping_sub(printed),
                c"|%s%s".as_ptr(),
                if show_prefix { prefix } else { c"".as_ptr() },
                c"RESET_ON_FORK".as_ptr(),
            )
        };
        flags &= !SCHED_RESET_ON_FORK;
    }

    if flags != 0 {
        printed += unsafe {
            scnprintf(
                bf.add(printed),
                size.wrapping_sub(printed),
                c"|%#x".as_ptr(),
                flags,
            )
        };
    }

    printed
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
