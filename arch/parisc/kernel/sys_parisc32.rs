// SPDX-License-Identifier: GPL-2.0
/*
 * sys_parisc32.c: Conversion between 32bit and 64bit native syscalls.
 *
 * Copyright (C) 2000-2001 Hewlett Packard Company
 * Copyright (C) 2000 John Marvin
 * Copyright (C) 2001 Matthew Wilcox
 * Copyright (C) 2014 Helge Deller <deller@gmx.de>
 *
 * These routines maintain argument size conversion between 32bit and 64bit
 * environment. Based heavily on sys_ia32.c and sys_sparc32.c.
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_char;

#[repr(C)]
pub struct task_struct {
    pub comm: [c_char; 16],
    pub pid: i32,
}

extern "C" {
    pub static mut current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> i32;
}

pub const KERN_ERR: &[u8] = b"<3>\0";
pub const ENOSYS: i64 = 38;

/// Conversion between 32-bit and 64-bit native syscalls.
#[no_mangle]
pub unsafe extern "C" fn sys32_unimplemented(
    r26: i32,
    r25: i32,
    r24: i32,
    r23: i32,
    r22: i32,
    r21: i32,
    r20: i32,
) -> isize {
    let _ = (r26, r25, r24, r23, r22, r21);
    static FORMAT: &[u8] =
        b"<3>%s(%d): Unimplemented 32 on 64 syscall #%d!\n\0";

    printk(
        FORMAT.as_ptr() as *const c_char,
        (*current).comm.as_ptr(),
        (*current).pid,
        r20,
    );
    -(ENOSYS as isize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
