/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of x86/include/asm/string_64.h. */

#[cfg(feature = "__KERNEL__")]
use core::ffi::{c_char, c_int, c_void};

#[cfg(feature = "__KERNEL__")]
extern "C" {
    pub fn memcpy(to: *mut c_void, from: *const c_void, len: usize) -> *mut c_void;
    pub fn __memcpy(to: *mut c_void, from: *const c_void, len: usize) -> *mut c_void;
    pub fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    pub fn __memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    pub fn memmove(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
    pub fn __memmove(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
    pub fn memcmp(cs: *const c_void, ct: *const c_void, count: usize) -> c_int;
    pub fn strlen(s: *const c_char) -> usize;
    pub fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
}

/* KCFI_REFERENCE(__memset) and KCFI_REFERENCE(__memmove). */

/* Under !CONFIG_KMSAN, the architecture supplies these assembly routines. */
#[cfg(all(feature = "__KERNEL__", not(feature = "CONFIG_KMSAN")))]
#[inline]
pub unsafe fn memset16(s: *mut u16, v: u16, n: usize) -> *mut c_void {
    let s0 = s;
    core::arch::asm!("rep stosw", inout("rdi") s => _, inout("rcx") n => _, in("rax") v, options(nostack, preserves_flags));
    s0 as *mut c_void
}

#[cfg(all(feature = "__KERNEL__", not(feature = "CONFIG_KMSAN")))]
#[inline]
pub unsafe fn memset32(s: *mut u32, v: u32, n: usize) -> *mut c_void {
    let s0 = s;
    core::arch::asm!("rep stosl", inout("rdi") s => _, inout("rcx") n => _, in("rax") v, options(nostack, preserves_flags));
    s0 as *mut c_void
}

#[cfg(all(feature = "__KERNEL__", not(feature = "CONFIG_KMSAN")))]
#[inline]
pub unsafe fn memset64(s: *mut u64, v: u64, n: usize) -> *mut c_void {
    let s0 = s;
    core::arch::asm!("rep stosq", inout("rdi") s => _, inout("rcx") n => _, in("rax") v, options(nostack, preserves_flags));
    s0 as *mut c_void
}

#[cfg(feature = "__KERNEL__")]
extern "C" {
    pub fn __memcpy_flushcache(dst: *mut c_void, src: *const c_void, cnt: usize);
}

/* CONFIG_ARCH_HAS_UACCESS_FLUSHCACHE provides the following interface. */
#[cfg(all(feature = "__KERNEL__", feature = "CONFIG_ARCH_HAS_UACCESS_FLUSHCACHE"))]
#[inline(always)]
pub unsafe fn memcpy_flushcache(dst: *mut c_void, src: *const c_void, cnt: usize) {
    match cnt {
        4 => core::arch::asm!("movntil [{dst}], {value}", dst = in(reg) dst, value = in(reg) *(src as *const u32)),
        8 => core::arch::asm!("movntiq [{dst}], {value}", dst = in(reg) dst, value = in(reg) *(src as *const u64)),
        16 => {
            core::arch::asm!("movntiq [{dst}], {value}", dst = in(reg) dst, value = in(reg) *(src as *const u64));
            core::arch::asm!("movntiq [{dst}], {value}", dst = in(reg) (dst as *mut u8).add(8), value = in(reg) *((src as *const u8).add(8) as *const u64));
        }
        _ => __memcpy_flushcache(dst, src, cnt),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
