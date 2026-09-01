/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * POWER Data Stream Control Register (DSCR)
 *
 * This header file contains helper functions and macros
 * required for all the DSCR related test cases.
 *
 * Copyright 2012, Anton Blanchard, IBM Corporation.
 * Copyright 2015, Anshuman Khandual, IBM Corporation.
 */

/* C header dependencies removed:
 * <unistd.h>, <stdio.h>, <stdlib.h>, <string.h>, <fcntl.h>, <dirent.h>,
 * <pthread.h>, <sched.h>, <sys/types.h>, <sys/stat.h>, <sys/wait.h>,
 * "reg.h", and "utils.h".
 */

pub type c_int = i32;
pub type c_char = i8;
pub type c_ulong = u64;

pub const THREADS: c_int = 100; /* Max threads */
pub const COUNT: c_int = 100; /* Max iterations */
pub const DSCR_MAX: c_int = 16; /* Max DSCR value */
pub const LEN_MAX: c_int = 100; /* Max name length */

pub const DSCR_DEFAULT: &[u8; 37] = b"/sys/devices/system/cpu/dscr_default\0";
pub const CPU_PATH: &[u8; 25] = b"/sys/devices/system/cpu/\0";

unsafe extern "C" {
    static SPRN_DSCR_PRIV: c_ulong;
    static SPRN_DSCR: c_ulong;

    fn mfspr(spr: c_ulong) -> c_ulong;
    fn mtspr(spr: c_ulong, val: c_ulong);
    fn read_ulong(path: *const c_char, val: *mut c_ulong, base: c_int) -> c_int;
    fn write_ulong(path: *const c_char, val: c_ulong, base: c_int) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

#[inline]
pub unsafe fn rmb() {
    unsafe {
        core::arch::asm!("lwsync", options(nostack, preserves_flags));
    }
}

#[inline]
pub unsafe fn wmb() {
    unsafe {
        core::arch::asm!("lwsync", options(nostack, preserves_flags));
    }
}

macro_rules! READ_ONCE {
    ($x:expr) => {{
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!($x)) }
    }};
}

/* Prilvilege state DSCR access */
#[inline]
pub unsafe fn get_dscr() -> c_ulong {
    unsafe { mfspr(SPRN_DSCR_PRIV) }
}

#[inline]
pub unsafe fn set_dscr(val: c_ulong) {
    unsafe {
        mtspr(SPRN_DSCR_PRIV, val);
    }
}

/* Problem state DSCR access */
#[inline]
pub unsafe fn get_dscr_usr() -> c_ulong {
    unsafe { mfspr(SPRN_DSCR) }
}

#[inline]
pub unsafe fn set_dscr_usr(val: c_ulong) {
    unsafe {
        mtspr(SPRN_DSCR, val);
    }
}

/* Default DSCR access */
pub unsafe fn get_default_dscr() -> c_ulong {
    let err: c_int;
    let mut val: c_ulong = 0;

    unsafe {
        err = read_ulong(DSCR_DEFAULT.as_ptr() as *const c_char, &mut val, 16);
        if err != 0 {
            perror(c"read() failed".as_ptr());
            exit(1);
        }
    }
    val
}

pub unsafe fn set_default_dscr(val: c_ulong) {
    let err: c_int;

    unsafe {
        err = write_ulong(DSCR_DEFAULT.as_ptr() as *const c_char, val, 16);
        if err != 0 {
            perror(c"write() failed".as_ptr());
            exit(1);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
