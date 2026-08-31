/* SPDX-License-Identifier: GPL-2.0-only */

/* C header guard removed. */
/* C dependency intent: #include <stddef.h> for size_t. */

pub type size_t = usize;

unsafe extern "C" {
    #[link_name = "__builtin_memset"]
    pub fn memset(_s: *mut core::ffi::c_void, _c: core::ffi::c_int, _n: size_t) -> *mut core::ffi::c_void;

    #[link_name = "__builtin_memcpy"]
    pub fn memcpy(
        _dest: *mut core::ffi::c_void,
        _src: *const core::ffi::c_void,
        _n: size_t,
    ) -> *mut core::ffi::c_void;

    #[link_name = "__builtin_strlen"]
    pub fn strlen(_s: *const core::ffi::c_char) -> size_t;
}
