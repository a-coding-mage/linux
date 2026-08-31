// SPDX-License-Identifier: LGPL-2.1

use std::os::raw::{c_char, c_int};

// Dependencies from:
// - trace/beauty/beauty.h
// - linux/futex.h

#[repr(C)]
pub struct syscall_arg {
    pub val: c_int,
    pub mask: c_int,
    pub show_string_prefix: bool,
}

unsafe extern "C" {
    fn scnprintf(bf: *mut c_char, size: usize, fmt: *const c_char, ...) -> usize;
}

pub const FUTEX_WAIT: c_int = 0;
pub const FUTEX_WAKE: c_int = 1;
pub const FUTEX_FD: c_int = 2;
pub const FUTEX_REQUEUE: c_int = 3;
pub const FUTEX_CMP_REQUEUE: c_int = 4;
pub const FUTEX_WAKE_OP: c_int = 5;
pub const FUTEX_LOCK_PI: c_int = 6;
pub const FUTEX_UNLOCK_PI: c_int = 7;
pub const FUTEX_TRYLOCK_PI: c_int = 8;

// #ifndef FUTEX_WAIT_BITSET
pub const FUTEX_WAIT_BITSET: c_int = 9;
// #endif
// #ifndef FUTEX_WAKE_BITSET
pub const FUTEX_WAKE_BITSET: c_int = 10;
// #endif
// #ifndef FUTEX_WAIT_REQUEUE_PI
pub const FUTEX_WAIT_REQUEUE_PI: c_int = 11;
// #endif
// #ifndef FUTEX_CMP_REQUEUE_PI
pub const FUTEX_CMP_REQUEUE_PI: c_int = 12;
// #endif

pub const FUTEX_PRIVATE_FLAG: c_int = 128;
// #ifndef FUTEX_CLOCK_REALTIME
pub const FUTEX_CLOCK_REALTIME: c_int = 256;
// #endif
pub const FUTEX_CMD_MASK: c_int = !(FUTEX_PRIVATE_FLAG | FUTEX_CLOCK_REALTIME);

pub const SCF_UADDR: c_int = 1 << 0;
pub const SCF_OP: c_int = 1 << 1;
pub const SCF_VAL: c_int = 1 << 2;
pub const SCF_TIMEOUT: c_int = 1 << 3;
pub const SCF_UADDR2: c_int = 1 << 4;
pub const SCF_VAL3: c_int = 1 << 5;

unsafe fn p_futex_op(
    bf: *mut c_char,
    size: usize,
    show_prefix: bool,
    prefix: *const c_char,
    name: *const c_char,
) -> usize {
    unsafe {
        scnprintf(
            bf,
            size,
            b"%s%s\0".as_ptr() as *const c_char,
            if show_prefix {
                prefix
            } else {
                b"\0".as_ptr() as *const c_char
            },
            name,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_futex_op(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let show_prefix = unsafe { (*arg).show_string_prefix };
    let prefix = b"FUTEX_\0".as_ptr() as *const c_char;
    let op = unsafe { (*arg).val };
    let cmd = op & FUTEX_CMD_MASK;
    let mut printed: usize = 0;

    match cmd {
        FUTEX_WAIT => {
            printed = unsafe { p_futex_op(bf, size, show_prefix, prefix, b"WAIT\0".as_ptr() as *const c_char) };
            unsafe { (*arg).mask |= SCF_VAL3 | SCF_UADDR2 };
        }
        FUTEX_WAKE => {
            printed = unsafe { p_futex_op(bf, size, show_prefix, prefix, b"WAKE\0".as_ptr() as *const c_char) };
            unsafe { (*arg).mask |= SCF_VAL3 | SCF_UADDR2 | SCF_TIMEOUT };
        }
        FUTEX_FD => {
            printed = unsafe { p_futex_op(bf, size, show_prefix, prefix, b"FD\0".as_ptr() as *const c_char) };
            unsafe { (*arg).mask |= SCF_VAL3 | SCF_UADDR2 | SCF_TIMEOUT };
        }
        FUTEX_REQUEUE => {
            printed = unsafe { p_futex_op(bf, size, show_prefix, prefix, b"REQUEUE\0".as_ptr() as *const c_char) };
            unsafe { (*arg).mask |= SCF_VAL3 | SCF_TIMEOUT };
        }
        FUTEX_CMP_REQUEUE => {
            printed = unsafe { p_futex_op(bf, size, show_prefix, prefix, b"CMP_REQUEUE\0".as_ptr() as *const c_char) };
            unsafe { (*arg).mask |= SCF_TIMEOUT };
        }
        FUTEX_CMP_REQUEUE_PI => {
            printed = unsafe { p_futex_op(bf, size, show_prefix, prefix, b"CMP_REQUEUE_PI\0".as_ptr() as *const c_char) };
            unsafe { (*arg).mask |= SCF_TIMEOUT };
        }
        FUTEX_WAKE_OP => {
            printed = unsafe { p_futex_op(bf, size, show_prefix, prefix, b"WAKE_OP\0".as_ptr() as *const c_char) };
        }
        FUTEX_LOCK_PI => {
            printed = unsafe { p_futex_op(bf, size, show_prefix, prefix, b"LOCK_PI\0".as_ptr() as *const c_char) };
            unsafe { (*arg).mask |= SCF_VAL3 | SCF_UADDR2 | SCF_TIMEOUT };
        }
        FUTEX_UNLOCK_PI => {
            printed = unsafe { p_futex_op(bf, size, show_prefix, prefix, b"UNLOCK_PI\0".as_ptr() as *const c_char) };
            unsafe { (*arg).mask |= SCF_VAL3 | SCF_UADDR2 | SCF_TIMEOUT };
        }
        FUTEX_TRYLOCK_PI => {
            printed = unsafe { p_futex_op(bf, size, show_prefix, prefix, b"TRYLOCK_PI\0".as_ptr() as *const c_char) };
            unsafe { (*arg).mask |= SCF_VAL3 | SCF_UADDR2 };
        }
        FUTEX_WAIT_BITSET => {
            printed = unsafe { p_futex_op(bf, size, show_prefix, prefix, b"WAIT_BITSET\0".as_ptr() as *const c_char) };
            unsafe { (*arg).mask |= SCF_UADDR2 };
        }
        FUTEX_WAKE_BITSET => {
            printed = unsafe { p_futex_op(bf, size, show_prefix, prefix, b"WAKE_BITSET\0".as_ptr() as *const c_char) };
            unsafe { (*arg).mask |= SCF_UADDR2 };
        }
        FUTEX_WAIT_REQUEUE_PI => {
            printed = unsafe { p_futex_op(bf, size, show_prefix, prefix, b"WAIT_REQUEUE_PI\0".as_ptr() as *const c_char) };
        }
        _ => {
            printed = unsafe {
                scnprintf(
                    bf,
                    size,
                    b"%#x\0".as_ptr() as *const c_char,
                    cmd,
                )
            };
        }
    }

    if (op & FUTEX_PRIVATE_FLAG) != 0 {
        printed += unsafe {
            scnprintf(
                bf.add(printed),
                size.wrapping_sub(printed),
                b"|%s%s\0".as_ptr() as *const c_char,
                if show_prefix {
                    prefix
                } else {
                    b"\0".as_ptr() as *const c_char
                },
                b"PRIVATE_FLAG\0".as_ptr() as *const c_char,
            )
        };
    }

    if (op & FUTEX_CLOCK_REALTIME) != 0 {
        printed += unsafe {
            scnprintf(
                bf.add(printed),
                size.wrapping_sub(printed),
                b"|%s%s\0".as_ptr() as *const c_char,
                if show_prefix {
                    prefix
                } else {
                    b"\0".as_ptr() as *const c_char
                },
                b"CLOCK_REALTIME\0".as_ptr() as *const c_char,
            )
        };
    }

    printed
}
