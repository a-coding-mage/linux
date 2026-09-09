/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the Linux compiler, errno, and generic
// error-injection headers are intentionally not implemented here.

#[cfg(feature = "CONFIG_FUNCTION_ERROR_INJECTION")]
extern "C" {
    pub fn within_error_injection_list(addr: usize) -> bool;
    pub fn get_injectable_error_type(addr: usize) -> i32;
}

#[cfg(not(feature = "CONFIG_FUNCTION_ERROR_INJECTION"))]
#[inline]
pub fn within_error_injection_list(_addr: usize) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_FUNCTION_ERROR_INJECTION"))]
#[inline]
pub fn get_injectable_error_type(_addr: usize) -> i32 {
    // EOPNOTSUPP is supplied by the Linux errno dependency.
    -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
