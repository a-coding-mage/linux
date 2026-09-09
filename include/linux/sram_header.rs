/* SPDX-License-Identifier: GPL-2.0-only */
/* Generic SRAM Driver Interface */

use core::ffi::c_void;

#[repr(C)]
pub struct gen_pool {
    _private: [u8; 0],
}

// Corresponds to CONFIG_SRAM_EXEC.
#[cfg(feature = "CONFIG_SRAM_EXEC")]
unsafe extern "C" {
    pub fn sram_exec_copy(
        pool: *mut gen_pool,
        dst: *mut c_void,
        src: *mut c_void,
        size: usize,
    ) -> *mut c_void;
}

// Corresponds to the CONFIG_SRAM_EXEC-disabled build condition.
#[cfg(not(feature = "CONFIG_SRAM_EXEC"))]
#[inline]
pub unsafe fn sram_exec_copy(
    _pool: *mut gen_pool,
    _dst: *mut c_void,
    _src: *mut c_void,
    _size: usize,
) -> *mut c_void {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
