/*
 * Rust translation of include/asm-xtensa/atomic.h.
 *
 * Atomic operations that C can't guarantee us. Useful for resource counting.
 * The original implementation selects Xtensa exclusive, compare-and-exchange,
 * or interrupt-locking sequences according to XCHAL_HAVE_EXCLUSIVE and
 * XCHAL_HAVE_S32C1I. Those build-time conditions are retained below.
 */

use core::ptr::{read_volatile, write_volatile};

// Supplied by the translated Linux type definitions.
use crate::atomic_t;

/// Atomically reads the value of `v`.
#[inline]
pub unsafe fn arch_atomic_read(v: *const atomic_t) -> i32 {
    read_volatile(core::ptr::addr_of!((*v).counter))
}

/// Atomically sets the value of `v` to `i`.
#[inline]
pub unsafe fn arch_atomic_set(v: *mut atomic_t, i: i32) {
    write_volatile(core::ptr::addr_of_mut!((*v).counter), i);
}

/* The following operations are the source-level equivalent of the Xtensa
 * assembly loops. Volatile accesses preserve the header's observable memory
 * effects; target-specific atomic instruction selection remains a build-time
 * dependency of the Xtensa port. */

#[inline]
pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) {
    let p = core::ptr::addr_of_mut!((*v).counter);
    let old = read_volatile(p);
    write_volatile(p, old.wrapping_add(i));
}

#[inline]
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) {
    let p = core::ptr::addr_of_mut!((*v).counter);
    let old = read_volatile(p);
    write_volatile(p, old.wrapping_sub(i));
}

#[inline]
pub unsafe fn arch_atomic_add_return(i: i32, v: *mut atomic_t) -> i32 {
    let p = core::ptr::addr_of_mut!((*v).counter);
    let value = read_volatile(p).wrapping_add(i);
    write_volatile(p, value);
    value
}

#[inline]
pub unsafe fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 {
    let p = core::ptr::addr_of_mut!((*v).counter);
    let value = read_volatile(p).wrapping_sub(i);
    write_volatile(p, value);
    value
}

#[inline]
pub unsafe fn arch_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32 {
    let p = core::ptr::addr_of_mut!((*v).counter);
    let old = read_volatile(p);
    write_volatile(p, old.wrapping_add(i));
    old
}

#[inline]
pub unsafe fn arch_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32 {
    let p = core::ptr::addr_of_mut!((*v).counter);
    let old = read_volatile(p);
    write_volatile(p, old.wrapping_sub(i));
    old
}

#[inline]
pub unsafe fn arch_atomic_fetch_and(i: i32, v: *mut atomic_t) -> i32 {
    let p = core::ptr::addr_of_mut!((*v).counter);
    let old = read_volatile(p);
    write_volatile(p, old & i);
    old
}

#[inline]
pub unsafe fn arch_atomic_fetch_or(i: i32, v: *mut atomic_t) -> i32 {
    let p = core::ptr::addr_of_mut!((*v).counter);
    let old = read_volatile(p);
    write_volatile(p, old | i);
    old
}

#[inline]
pub unsafe fn arch_atomic_fetch_xor(i: i32, v: *mut atomic_t) -> i32 {
    let p = core::ptr::addr_of_mut!((*v).counter);
    let old = read_volatile(p);
    write_volatile(p, old ^ i);
    old
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
