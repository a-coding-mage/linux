/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_VDSO_GETRANDOM_H
// The original declarations are excluded when compiling as assembler.
// Dependencies supplied by the included headers remain external to this file.

use core::ffi::c_void;

extern "C" {
    fn syscall3(number: isize, arg1: isize, arg2: isize, arg3: isize) -> isize;
}

/**
 * getrandom_syscall - Invoke the getrandom() syscall.
 * @buffer: Destination buffer to fill with random bytes.
 * @len:    Size of @buffer in bytes.
 * @flags:  Zero or more GRND_* flags.
 * Returns: The number of random bytes written to @buffer, or a negative value indicating an error.
 */
#[inline(always)]
unsafe fn getrandom_syscall(buffer: *mut c_void, len: usize, flags: u32) -> isize {
    syscall3(
        __NR_getrandom as isize,
        buffer as isize,
        len as isize,
        flags as isize,
    )
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
