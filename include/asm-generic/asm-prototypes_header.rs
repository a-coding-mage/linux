/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <linux/bitops.h>
// The C preprocessor undefines these names before declaring the external
// functions; Rust has no equivalent macro namespace operation.

unsafe extern "C" {
    pub fn __memset(
        dest: *mut core::ffi::c_void,
        c: core::ffi::c_int,
        n: __kernel_size_t,
    ) -> *mut core::ffi::c_void;

    pub fn __memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut core::ffi::c_void;

    pub fn __memmove(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut core::ffi::c_void;

    pub fn memset(
        dest: *mut core::ffi::c_void,
        c: core::ffi::c_int,
        n: __kernel_size_t,
    ) -> *mut core::ffi::c_void;

    pub fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut core::ffi::c_void;

    pub fn memmove(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
