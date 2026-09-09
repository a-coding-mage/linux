// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by <uapi/linux/unistd.h> and <linux/kbuild.h>.

// C preprocessor conditionals for architecture-dependent syscall numbers are
// preserved below as comments; the corresponding constants are supplied by
// the target build environment.

extern "C" {
    fn COMMENT(text: *const core::ffi::c_char);
    fn DEFINE(name: u64, value: u64);
}

// Equivalent of: #define SYSNR(_NR) DEFINE(SYS ## _NR, _NR)
#[inline]
unsafe fn sysnr(name: u64, number: u64) {
    DEFINE(name, number);
}

#[allow(non_snake_case)]
pub unsafe fn syscall_defines() {
    COMMENT(b"Linux system call numbers.\0".as_ptr() as *const core::ffi::c_char);
    sysnr(SYS__NR_write, __NR_write);
    sysnr(SYS__NR_read, __NR_read);

    // #ifdef __NR_mmap2
    sysnr(SYS__NR_mmap2, __NR_mmap2);
    // #endif

    // #ifdef __NR_mmap
    sysnr(SYS__NR_mmap, __NR_mmap);
    // #endif
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
