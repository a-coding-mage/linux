/* SPDX-License-Identifier: GPL-2.0-only */

macro_rules! offsetof {
    ($TYPE:ty, $MEMBER:tt) => {
        ::core::mem::offset_of!($TYPE, $MEMBER)
    };
}

/* In C++ this header defines NULL as 0; otherwise as ((void *)0). */
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut();
