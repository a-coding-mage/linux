/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2022-2024 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

// The C header includes <asm/unistd.h>, which supplies __NR_getrandom.

/// Invoke the getrandom() syscall.
///
/// `buffer` is the destination buffer to fill with random bytes, `len` is its
/// size in bytes, and `flags` contains zero or more GRND_* flags.
/// Returns the number of random bytes written to `buffer`, or a negative value
/// indicating an error.
#[inline(always)]
pub unsafe fn getrandom_syscall(
    buffer: *mut core::ffi::c_void,
    len: usize,
    flags: u32,
) -> isize {
    let mut ret: isize;

    core::arch::asm!(
        "syscall",
        inlateout("rax") __NR_getrandom as isize => ret,
        in("rdi") buffer,
        in("rsi") len,
        in("rdx") flags,
        lateout("rcx") _,
        lateout("r11") _,
    );

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
