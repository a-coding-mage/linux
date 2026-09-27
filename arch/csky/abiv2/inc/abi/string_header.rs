/* SPDX-License-Identifier: GPL-2.0 */

// __HAVE_ARCH_MEMCMP
extern "C" {
    pub fn memcmp(
        s1: *const kernel::ffi::c_void,
        s2: *const kernel::ffi::c_void,
        n: __kernel_size_t,
    ) -> kernel::ffi::c_int;
}

// __HAVE_ARCH_MEMCPY
extern "C" {
    pub fn memcpy(
        dest: *mut kernel::ffi::c_void,
        src: *const kernel::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut kernel::ffi::c_void;
}

// __HAVE_ARCH_MEMMOVE
extern "C" {
    pub fn memmove(
        dest: *mut kernel::ffi::c_void,
        src: *const kernel::ffi::c_void,
        n: __kernel_size_t,
    ) -> *mut kernel::ffi::c_void;
}

// __HAVE_ARCH_MEMSET
extern "C" {
    pub fn memset(
        s: *mut kernel::ffi::c_void,
        c: kernel::ffi::c_int,
        n: __kernel_size_t,
    ) -> *mut kernel::ffi::c_void;
}

// __HAVE_ARCH_STRCMP
extern "C" {
    pub fn strcmp(s1: *const kernel::ffi::c_char, s2: *const kernel::ffi::c_char) -> kernel::ffi::c_int;
}

// __HAVE_ARCH_STRCPY
extern "C" {
    pub fn strcpy(
        dest: *mut kernel::ffi::c_char,
        src: *const kernel::ffi::c_char,
    ) -> *mut kernel::ffi::c_char;
}

// __HAVE_ARCH_STRLEN
extern "C" {
    pub fn strlen(s: *const kernel::ffi::c_char) -> __kernel_size_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
