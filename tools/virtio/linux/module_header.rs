/* SPDX-License-Identifier: GPL-2.0 */

// C dependency intent: #include <linux/export.h>

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[macro_export]
macro_rules! MODULE_LICENSE {
    ($__MODULE_LICENSE_value:expr) => {
        #[allow(non_upper_case_globals)]
        #[allow(dead_code)]
        static __MODULE_LICENSE_name: *const ::core::ffi::c_char = $__MODULE_LICENSE_value;
    };
}

// C conditional intent:
// #ifndef MODULE_AUTHOR
// #define MODULE_AUTHOR(x)
// #endif
#[macro_export]
macro_rules! MODULE_AUTHOR {
    ($x:expr) => {};
}

// C conditional intent:
// #ifndef MODULE_DESCRIPTION
// #define MODULE_DESCRIPTION(x)
// #endif
#[macro_export]
macro_rules! MODULE_DESCRIPTION {
    ($x:expr) => {};
}
