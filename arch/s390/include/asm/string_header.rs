/* SPDX-License-Identifier: GPL-2.0 */
/* S390 version; translated from the C header. */

// The following architecture markers are represented as Rust cfg intent.
#[cfg(any())]
const __HAVE_ARCH_MEMCPY: () = ();
#[cfg(any())]
const __HAVE_ARCH_MEMMOVE: () = ();
#[cfg(any())]
const __HAVE_ARCH_MEMSET: () = ();

unsafe extern "C" {
    pub fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize)
        -> *mut core::ffi::c_void;
    pub fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    pub fn memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize)
        -> *mut core::ffi::c_void;
}

// !defined(CONFIG_KASAN) && !defined(CONFIG_KMSAN)
#[cfg(not(any(feature = "CONFIG_KASAN", feature = "CONFIG_KMSAN")))]
unsafe extern "C" {
    pub fn memcmp(s1: *const core::ffi::c_void, s2: *const core::ffi::c_void, n: usize) -> i32;
    pub fn strcmp(s1: *const core::ffi::c_char, s2: *const core::ffi::c_char) -> i32;
    pub fn strncat(
        dest: *mut core::ffi::c_char,
        src: *const core::ffi::c_char,
        n: usize,
    ) -> *mut core::ffi::c_char;
    pub fn strstr(
        s1: *const core::ffi::c_char,
        s2: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
}

unsafe extern "C" {
    pub fn __memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize)
        -> *mut core::ffi::c_void;
    pub fn __memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    pub fn __memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize)
        -> *mut core::ffi::c_void;
    pub fn __memset16(s: *mut u16, v: u16, count: usize) -> *mut core::ffi::c_void;
    pub fn __memset32(s: *mut u32, v: u32, count: usize) -> *mut core::ffi::c_void;
    pub fn __memset64(s: *mut u64, v: u64, count: usize) -> *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn memset16(s: *mut u16, v: u16, count: usize) -> *mut core::ffi::c_void {
    __memset16(s, v, count.wrapping_mul(core::mem::size_of::<u16>()))
}

#[inline]
pub unsafe fn memset32(s: *mut u32, v: u32, count: usize) -> *mut core::ffi::c_void {
    __memset32(s, v, count.wrapping_mul(core::mem::size_of::<u32>()))
}

#[inline]
pub unsafe fn memset64(s: *mut u64, v: u64, count: usize) -> *mut core::ffi::c_void {
    __memset64(s, v, count.wrapping_mul(core::mem::size_of::<u64>()))
}

#[inline]
pub unsafe fn memchr(s: *const core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void {
    // s390: SRST searches from s through s+n, with the original inline asm retrying on overflow.
    let p = s.cast::<u8>();
    let byte = c as u8;
    let mut i = 0usize;
    while i < n {
        if *p.add(i) == byte { return p.add(i).cast_mut().cast(); }
        i += 1;
    }
    p.add(n).cast_mut().cast()
}

#[inline]
pub unsafe fn memscan(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void {
    memchr(s.cast_const(), c, n)
}

#[inline]
pub unsafe fn strcat(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char)
    -> *mut core::ffi::c_char
{
    // s390 inline asm: SRST finds the terminator, then MVST appends src including its terminator.
    let ret = dst;
    let mut end = dst;
    while *end != 0 { end = end.add(1); }
    let mut from = src;
    loop {
        let ch = *from;
        *end = ch;
        if ch == 0 { break; }
        from = from.add(1);
        end = end.add(1);
    }
    ret
}

#[inline]
pub unsafe fn strlen(s: *const core::ffi::c_char) -> usize {
    // s390 inline asm: SRST scans for the terminating NUL.
    let mut p = s;
    while *p != 0 { p = p.add(1); }
    p.offset_from(s) as usize
}

#[inline]
pub unsafe fn strnlen(s: *const core::ffi::c_char, n: usize) -> usize {
    // s390 inline asm: SRST scans up to the supplied end pointer.
    let mut i = 0usize;
    while i < n && *s.add(i) != 0 { i += 1; }
    i
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
