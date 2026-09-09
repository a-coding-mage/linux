/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the SH LL/SC bit operations header.
// The original inline assembly uses movli.l/movco.l retry loops; the Rust
// equivalents below retain the same atomic read-modify-write semantics.

use core::sync::atomic::{AtomicU32, Ordering};

#[inline]
pub unsafe fn set_bit(nr: i32, addr: *mut core::ffi::c_void) {
    let a = (addr as *mut u32).offset((nr >> 5) as isize);
    let mask = 1u32 << ((nr & 0x1f) as u32);
    let atomic = &*(a as *const AtomicU32);
    let _ = atomic.fetch_or(mask, Ordering::SeqCst);
}

#[inline]
pub unsafe fn clear_bit(nr: i32, addr: *mut core::ffi::c_void) {
    let a = (addr as *mut u32).offset((nr >> 5) as isize);
    let mask = 1u32 << ((nr & 0x1f) as u32);
    let atomic = &*(a as *const AtomicU32);
    let _ = atomic.fetch_and(!mask, Ordering::SeqCst);
}

#[inline]
pub unsafe fn change_bit(nr: i32, addr: *mut core::ffi::c_void) {
    let a = (addr as *mut u32).offset((nr >> 5) as isize);
    let mask = 1u32 << ((nr & 0x1f) as u32);
    let atomic = &*(a as *const AtomicU32);
    let _ = atomic.fetch_xor(mask, Ordering::SeqCst);
}

#[inline]
pub unsafe fn test_and_set_bit(nr: i32, addr: *mut core::ffi::c_void) -> i32 {
    let a = (addr as *mut u32).offset((nr >> 5) as isize);
    let mask = 1u32 << ((nr & 0x1f) as u32);
    let atomic = &*(a as *const AtomicU32);
    (atomic.fetch_or(mask, Ordering::SeqCst) & mask != 0) as i32
}

#[inline]
pub unsafe fn test_and_clear_bit(nr: i32, addr: *mut core::ffi::c_void) -> i32 {
    let a = (addr as *mut u32).offset((nr >> 5) as isize);
    let mask = 1u32 << ((nr & 0x1f) as u32);
    let atomic = &*(a as *const AtomicU32);
    (atomic.fetch_and(!mask, Ordering::SeqCst) & mask != 0) as i32
}

#[inline]
pub unsafe fn test_and_change_bit(nr: i32, addr: *mut core::ffi::c_void) -> i32 {
    let a = (addr as *mut u32).offset((nr >> 5) as isize);
    let mask = 1u32 << ((nr & 0x1f) as u32);
    let atomic = &*(a as *const AtomicU32);
    (atomic.fetch_xor(mask, Ordering::SeqCst) & mask != 0) as i32
}

// Dependency supplied by asm-generic/bitops/non-atomic.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
