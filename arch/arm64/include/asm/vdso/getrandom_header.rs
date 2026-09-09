/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_VDSO_GETRANDOM_H
// This declaration is excluded when building for the assembler (__ASSEMBLER__).
// Dependencies supplied by the surrounding kernel translation:
// asm/unistd.h, asm/vdso/vsyscall.h, and vdso/datapage.h.

/**
 * getrandom_syscall - Invoke the getrandom() syscall.
 * @buffer: Destination buffer to fill with random bytes.
 * @len:    Size of @buffer in bytes.
 * @flags:  Zero or more GRND_* flags.
 * Returns: The number of random bytes written to @buffer, or a negative value indicating an error.
 */
#[inline(always)]
pub unsafe fn getrandom_syscall(
    _buffer: *mut core::ffi::c_void,
    _len: usize,
    _flags: core::ffi::c_uint,
) -> isize {
    let mut ret: isize;
    let buffer = _buffer;
    let len = _len;
    let flags = _flags;
    let nr: isize = __NR_getrandom as isize;

    core::arch::asm!(
        "svc #0",
        inlateout("x0") buffer => ret,
        in("x1") len,
        in("x2") flags,
        in("x8") nr,
        options(nostack)
    );

    ret
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
