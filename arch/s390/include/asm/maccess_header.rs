/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the s390 assembly memory-access header.

// Dependency supplied externally by the surrounding kernel translation:
// #include <linux/types.h>

pub const MEMCPY_REAL_SIZE: usize = PAGE_SIZE;
pub const MEMCPY_REAL_MASK: usize = PAGE_MASK;

pub struct iov_iter;

unsafe extern "C" {
    pub static mut __memcpy_real_area: usize;
    pub static mut memcpy_real_ptep: *mut pte_t;
    pub fn memcpy_real_iter(iter: *mut iov_iter, src: usize, count: usize) -> usize;
    pub fn memcpy_real(dest: *mut core::ffi::c_void, src: usize, count: usize) -> core::ffi::c_int;
}

// The following declaration is present only when CONFIG_CRASH_DUMP is enabled.
#[cfg(feature = "CONFIG_CRASH_DUMP")]
unsafe extern "C" {
    pub fn copy_oldmem_kernel(
        dst: *mut core::ffi::c_void,
        src: usize,
        count: usize,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
