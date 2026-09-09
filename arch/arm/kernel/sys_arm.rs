// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/sys_arm.c
 *
 *  Copyright (C) People who wrote linux/arch/i386/kernel/sys_i386.c
 *  Copyright (C) 1995, 1996 Russell King.
 *
 *  This file contains various random system calls that
 *  have a non-standard calling sequence on the Linux/arm
 *  platform.
 */

// The declarations and types supplied by the Linux kernel headers are
// external dependencies of this translation.

extern "C" {
    fn ksys_fadvise64_64(fd: i32, offset: i64, len: i64, advice: i32) -> isize;
}

/*
 * Since loff_t is a 64 bit type we avoid a lot of ABI hassle
 * with a different argument ordering.
 */
#[no_mangle]
pub unsafe extern "C" fn sys_arm_fadvise64_64(
    fd: i32,
    advice: i32,
    offset: i64,
    len: i64,
) -> isize {
    ksys_fadvise64_64(fd, offset, len, advice)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
