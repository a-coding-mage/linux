/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations and definitions are relevant when building the kernel. */

/* __HAVE_ARCH_MEMCPY */
unsafe extern "C" {
    pub fn memcpy(dest: *mut kernel::ffi::c_void, src: *const kernel::ffi::c_void, n: usize)
        -> *mut kernel::ffi::c_void;
}

/* __HAVE_ARCH_MEMMOVE */
unsafe extern "C" {
    pub fn memmove(dest: *mut kernel::ffi::c_void, src: *const kernel::ffi::c_void, n: usize)
        -> *mut kernel::ffi::c_void;
}

/* For backward compatibility with modules.  Unused otherwise. */
unsafe extern "C" {
    pub fn __memcpy(
        dest: *mut kernel::ffi::c_void,
        src: *const kernel::ffi::c_void,
        n: usize,
    ) -> *mut kernel::ffi::c_void;
}

/* __HAVE_ARCH_MEMSET */
unsafe extern "C" {
    pub fn __constant_c_memset(
        s: *mut kernel::ffi::c_void,
        c: kernel::ffi::c_ulong,
        n: usize,
    ) -> *mut kernel::ffi::c_void;
    pub fn ___memset(s: *mut kernel::ffi::c_void, c: kernel::ffi::c_int, n: usize)
        -> *mut kernel::ffi::c_void;
    pub fn memset(s: *mut kernel::ffi::c_void, c: kernel::ffi::c_int, n: usize)
        -> *mut kernel::ffi::c_void;
}

/*
 * GCC's __builtin_constant_p branches have no direct Rust equivalent.  The
 * runtime operations below preserve the corresponding C operation and its
 * return value; callers may still use the architecture-provided routines.
 */
pub unsafe fn __memset(
    s: *mut kernel::ffi::c_void,
    c: kernel::ffi::c_int,
    n: usize,
) -> *mut kernel::ffi::c_void {
    /* __builtin_constant_p(c) / __builtin_constant_p(n) */
    core::ptr::write_bytes(s.cast::<u8>(), c as u8, n);
    s
}

/* __HAVE_ARCH_STRCPY */
unsafe extern "C" {
    pub fn strcpy(dest: *mut kernel::ffi::c_char, src: *const kernel::ffi::c_char) -> *mut kernel::ffi::c_char;
}
/* __HAVE_ARCH_STRCAT */
unsafe extern "C" {
    pub fn strcat(dest: *mut kernel::ffi::c_char, src: *const kernel::ffi::c_char) -> *mut kernel::ffi::c_char;
}
/* __HAVE_ARCH_STRNCAT */
unsafe extern "C" {
    pub fn strncat(
        dest: *mut kernel::ffi::c_char,
        src: *const kernel::ffi::c_char,
        n: usize,
    ) -> *mut kernel::ffi::c_char;
}
/* __HAVE_ARCH_STRCHR */
unsafe extern "C" {
    pub fn strchr(s: *const kernel::ffi::c_char, c: kernel::ffi::c_int) -> *mut kernel::ffi::c_char;
}
/* __HAVE_ARCH_STRRCHR */
unsafe extern "C" {
    pub fn strrchr(s: *const kernel::ffi::c_char, c: kernel::ffi::c_int) -> *mut kernel::ffi::c_char;
}
/* __HAVE_ARCH_STRLEN */
unsafe extern "C" {
    pub fn strlen(s: *const kernel::ffi::c_char) -> usize;
}
/* __HAVE_ARCH_MEMCHR */
unsafe extern "C" {
    pub fn memchr(
        s: *const kernel::ffi::c_void,
        c: kernel::ffi::c_int,
        n: usize,
    ) -> *mut kernel::ffi::c_void;
}

/* __HAVE_ARCH_MEMSET16 */
unsafe extern "C" {
    pub fn __memset16(dest: *mut kernel::ffi::c_void, v: u16, count: usize)
        -> *mut kernel::ffi::c_void;
}

pub unsafe fn memset16(p: *mut u16, v: u16, n: usize) -> *mut kernel::ffi::c_void {
    /* __builtin_constant_p(v): the constant path uses the repeated-byte word. */
    let value = v;
    for i in 0..n {
        p.add(i).write(value);
    }
    p.cast()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
