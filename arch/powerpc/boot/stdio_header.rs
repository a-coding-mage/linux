/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_void};

pub const ENOMEM: c_int = 12; // Out of Memory
pub const EINVAL: c_int = 22; // Invalid argument
pub const ENOSPC: c_int = 28; // No space left on device

// C va_list declaration represented as an opaque pointer for external use.
pub type va_list = *mut c_void;

unsafe extern "C" {
    pub fn printf(fmt: *const c_char, ...) -> c_int;

    pub fn vsprintf(buf: *mut c_char, fmt: *const c_char, args: va_list) -> c_int;
}

// #define fprintf(fmt, args...) printf(args)
#[macro_export]
macro_rules! fprintf {
    ($fmt:expr, $($args:tt)*) => {{
        let _ = $fmt;
        unsafe { $crate::printf($($args)*) }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
