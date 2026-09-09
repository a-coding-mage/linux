/* SPDX-License-Identifier: GPL-2.0 */
// _ASM_X86_STRING_32_H
// The declarations below are intended for the kernel build (__KERNEL__).

// Let gcc decide whether to inline or use the out of line functions.

pub const __HAVE_ARCH_STRCPY: () = ();
extern "C" {
    pub fn strcpy(dest: *mut core::ffi::c_char, src: *const core::ffi::c_char) -> *mut core::ffi::c_char;
}

pub const __HAVE_ARCH_STRCAT: () = ();
extern "C" {
    pub fn strcat(dest: *mut core::ffi::c_char, src: *const core::ffi::c_char) -> *mut core::ffi::c_char;
}

pub const __HAVE_ARCH_STRNCAT: () = ();
extern "C" {
    pub fn strncat(dest: *mut core::ffi::c_char, src: *const core::ffi::c_char, count: usize) -> *mut core::ffi::c_char;
}

pub const __HAVE_ARCH_STRCMP: () = ();
extern "C" { pub fn strcmp(cs: *const core::ffi::c_char, ct: *const core::ffi::c_char) -> i32; }
pub const __HAVE_ARCH_STRNCMP: () = ();
extern "C" { pub fn strncmp(cs: *const core::ffi::c_char, ct: *const core::ffi::c_char, count: usize) -> i32; }
pub const __HAVE_ARCH_STRCHR: () = ();
extern "C" { pub fn strchr(s: *const core::ffi::c_char, c: i32) -> *mut core::ffi::c_char; }
pub const __HAVE_ARCH_STRLEN: () = ();
extern "C" { pub fn strlen(s: *const core::ffi::c_char) -> usize; }

#[inline(always)]
pub unsafe fn __memcpy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void {
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, n);
    to
}

/* This looks ugly, but the compiler can optimize it totally, as the count is constant. */
#[inline(always)]
pub unsafe fn __constant_memcpy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void {
    if n == 0 { return to; }
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, n);
    to
}

pub const __HAVE_ARCH_MEMCPY: () = ();
extern "C" { pub fn memcpy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void; }
// CONFIG_FORTIFY_SOURCE controls the C macro replacement with __builtin_memcpy.

pub const __HAVE_ARCH_MEMMOVE: () = ();
extern "C" { pub fn memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void; }
extern "C" { pub fn memcmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void, n: usize) -> i32; }
pub const __HAVE_ARCH_MEMCHR: () = ();
extern "C" { pub fn memchr(cs: *const core::ffi::c_void, c: i32, count: usize) -> *mut core::ffi::c_void; }

#[inline]
pub unsafe fn __memset_generic(s: *mut core::ffi::c_void, c: core::ffi::c_char, count: usize) -> *mut core::ffi::c_void {
    core::ptr::write_bytes(s as *mut u8, c as u8, count);
    s
}

#[inline]
pub unsafe fn __constant_count_memset(s: *mut core::ffi::c_void, c: core::ffi::c_char, count: usize) -> *mut core::ffi::c_void {
    __memset_generic(s, c, count)
}

pub const __HAVE_ARCH_STRNLEN: () = ();
extern "C" { pub fn strnlen(s: *const core::ffi::c_char, count: usize) -> usize; }
pub const __HAVE_ARCH_STRSTR: () = ();
extern "C" { pub fn strstr(cs: *const core::ffi::c_char, ct: *const core::ffi::c_char) -> *mut core::ffi::c_char; }

pub const __HAVE_ARCH_MEMSET: () = ();
extern "C" { pub fn memset(s: *mut core::ffi::c_void, c: i32, count: usize) -> *mut core::ffi::c_void; }

pub const __HAVE_ARCH_MEMSET16: () = ();
#[inline]
pub unsafe fn memset16(s: *mut u16, v: u16, n: usize) -> *mut core::ffi::c_void {
    for i in 0..n { core::ptr::write(s.add(i), v); }
    s as *mut core::ffi::c_void
}

pub const __HAVE_ARCH_MEMSET32: () = ();
#[inline]
pub unsafe fn memset32(s: *mut u32, v: u32, n: usize) -> *mut core::ffi::c_void {
    for i in 0..n { core::ptr::write(s.add(i), v); }
    s as *mut core::ffi::c_void
}

/* find the first occurrence of byte 'c', or 1 past the area if none */
pub const __HAVE_ARCH_MEMSCAN: () = ();
extern "C" { pub fn memscan(addr: *mut core::ffi::c_void, c: i32, size: usize) -> *mut core::ffi::c_void; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
