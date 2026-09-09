// SPDX-License-Identifier: GPL-2.0
/* Optimized string functions -- S390 version. */

#![allow(non_snake_case, unused_variables, dead_code)]

use core::arch::asm;
use core::ffi::c_void;

// The C build conditions are retained as cfg comments; feature selection is supplied externally.
#[repr(C)]
pub union register_pair {
    pub even: usize,
    pub odd: usize,
    pub pair: u128,
}

#[inline(always)]
unsafe fn test_facility(_facility: i32) -> bool { false }

#[cfg(any())]
pub unsafe fn __memmove(dest: *mut c_void, src: *const c_void, mut n: usize) -> *mut c_void {
    let mut s = src as *const u8;
    let mut d = dest as *mut u8;
    if n == 0 { return dest; }
    if d as usize <= s as usize || d as usize >= s.add(n) as usize {
        while n >= 256 { asm!("mvc 0(256,{d}),0({s})", d = in(reg) d, s = in(reg) s, options(nostack)); d = d.add(256); s = s.add(256); n -= 256; }
        if n != 0 { core::ptr::copy_nonoverlapping(s, d, n); }
        return dest;
    }
    if test_facility(61) {
        while n >= 256 { core::ptr::copy(s.add(n - 256), d.add(n - 256), 256); n -= 256; }
        if n != 0 { core::ptr::copy(s, d, n); }
    } else {
        while n != 0 { n -= 1; *d.add(n) = *s.add(n); }
    }
    dest
}

#[cfg(any())]
pub unsafe fn __memset(s: *mut c_void, c: i32, mut n: usize) -> *mut c_void {
    let p = s as *mut u8;
    core::ptr::write_bytes(p, c as u8, n);
    s
}

#[cfg(any())]
pub unsafe fn __memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    core::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, n); dest
}

macro_rules! define_memset {
    ($name:ident, $ty:ty) => {
        #[cfg(any())]
        pub unsafe fn $name(mut s: *mut $ty, v: $ty, mut n: usize) -> *mut c_void {
            if n == 0 { return s as *mut c_void; }
            while n >= 256 { core::ptr::write(s, v); core::ptr::write_bytes(s.cast::<u8>().add(core::mem::size_of::<$ty>()), v as u8, 256 - core::mem::size_of::<$ty>()); s = s.cast::<u8>().add(256).cast(); n -= 256; }
            if n == 0 { return s as *mut c_void; }
            core::ptr::write(s, v); s as *mut c_void
        }
    };
}
define_memset!(__memset16, u16);
define_memset!(__memset32, u32);
define_memset!(__memset64, u64);

#[inline]
unsafe fn __strend(mut s: *const u8) -> *mut u8 {
    while *s != 0 { s = s.add(1); } s as *mut u8
}
#[inline]
unsafe fn __strnend(mut s: *const u8, mut n: usize) -> *mut u8 {
    while n != 0 && *s != 0 { s = s.add(1); n -= 1; } s as *mut u8
}

#[cfg(any())]
pub unsafe fn strlen(s: *const i8) -> usize { __strend(s as *const u8).offset_from(s as *const u8) as usize }
#[cfg(any())]
pub unsafe fn strnlen(s: *const i8, n: usize) -> usize { __strnend(s as *const u8, n).offset_from(s as *const u8) as usize }

#[cfg(any())]
pub unsafe fn strcat(mut dest: *mut i8, mut src: *const i8) -> *mut i8 {
    let ret = dest; while *dest != 0 { dest = dest.add(1); }
    while *src != 0 { *dest = *src; dest = dest.add(1); src = src.add(1); } *dest = 0; ret
}
#[cfg(any())]
pub unsafe fn strncat(dest: *mut i8, src: *const i8, n: usize) -> *mut i8 {
    let p = __strend(dest as *const u8); let len = __strnend(src as *const u8, n).offset_from(src as *const u8) as usize; *p.add(len) = 0; core::ptr::copy_nonoverlapping(src as *const u8, p, len); dest
}
#[cfg(any())]
pub unsafe fn strcmp(mut s1: *const u8, mut s2: *const u8) -> i32 {
    loop { let a = *s1; let b = *s2; if a != b { return a as i32 - b as i32; } if a == 0 { return 0; } s1 = s1.add(1); s2 = s2.add(1); }
}

#[cfg(any())]
pub unsafe fn strstr(s1: *const i8, s2: *const i8) -> *mut i8 {
    let l2 = __strend(s2 as *const u8).offset_from(s2 as *const u8) as usize; if l2 == 0 { return s1 as *mut i8; }
    let mut p = s1 as *const u8; let l1 = __strend(p).offset_from(p) as usize;
    let mut left = l1; while left >= l2 { if core::slice::from_raw_parts(p, l2) == core::slice::from_raw_parts(s2 as *const u8, l2) { return p as *mut i8; } p = p.add(1); left -= 1; } core::ptr::null_mut()
}

#[cfg(any())]
pub unsafe fn memchr(s: *const c_void, c: i32, n: usize) -> *mut c_void { let p = core::slice::from_raw_parts(s as *const u8, n); match p.iter().position(|&x| x == c as u8) { Some(i) => (s as *const u8).add(i) as *mut c_void, None => core::ptr::null_mut() } }
#[cfg(any())]
pub unsafe fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> i32 { for i in 0..n { let a = *(s1 as *const u8).add(i); let b = *(s2 as *const u8).add(i); if a != b { return if a < b { -1 } else { 1 }; } } 0 }
#[cfg(any())]
pub unsafe fn memscan(s: *mut c_void, c: i32, n: usize) -> *mut c_void { let p = s as *mut u8; for i in 0..n { if *p.add(i) == c as u8 { return p.add(i) as *mut c_void; } } p.add(n) as *mut c_void }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
