/* SPDX-License-Identifier: GPL-2.0-only */

use core::ffi::c_char;

/*
 * To use this vDSO parser, first call one of the vdso_init_* functions.
 * If you've already parsed auxv, then pass the value of AT_SYSINFO_EHDR
 * to vdso_init_from_sysinfo_ehdr.  Otherwise pass auxv to vdso_init_from_auxv.
 * Then call vdso_sym for each symbol you want.  For example, to look up
 * gettimeofday on x86_64, use:
 *
 *     <some pointer> = vdso_sym("LINUX_2.6", "gettimeofday");
 * or
 *     <some pointer> = vdso_sym("LINUX_2.6", "__vdso_gettimeofday");
 *
 * vdso_sym will return 0 if the symbol doesn't exist or if the init function
 * failed or was not called.  vdso_sym is a little slow, so its return value
 * should be cached.
 *
 * vdso_sym is threadsafe; the init functions are not.
 *
 * These are the prototypes:
 */
unsafe extern "C" {
    pub fn vdso_sym(version: *const c_char, name: *const c_char) -> *mut core::ffi::c_void;
    pub fn vdso_init_from_sysinfo_ehdr(base: usize);
}
