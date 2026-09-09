/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_char;

#[cfg(any(CONFIG_UBSAN_TRAP, CONFIG_UBSAN_KVM_EL2))]
unsafe extern "C" {
    pub fn report_ubsan_failure(check_type: u32) -> *const c_char;
}

#[cfg(not(any(CONFIG_UBSAN_TRAP, CONFIG_UBSAN_KVM_EL2)))]
#[inline]
pub unsafe fn report_ubsan_failure(_check_type: u32) -> *const c_char {
    core::ptr::null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
