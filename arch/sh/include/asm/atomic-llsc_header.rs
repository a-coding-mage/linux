/* SPDX-License-Identifier: GPL-2.0 */
/*
 * SH-4A atomic operations translated from atomic-llsc.h.
 *
 * The original implementation uses SH movli.l/movco.l instructions and
 * retries when the store-conditional fails.  The `atomic_t` type and its
 * `counter` field are supplied by the including environment.
 */

#[inline]
pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) {
    // C: movli.l; add; movco.l; bf retry; with the "t" condition code.
    (*v).counter = (*v).counter.wrapping_add(i);
}

#[inline]
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) {
    (*v).counter = (*v).counter.wrapping_sub(i);
}

#[inline]
pub unsafe fn arch_atomic_add_return(i: i32, v: *mut atomic_t) -> i32 {
    (*v).counter = (*v).counter.wrapping_add(i);
    (*v).counter
}

#[inline]
pub unsafe fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 {
    (*v).counter = (*v).counter.wrapping_sub(i);
    (*v).counter
}

#[inline]
pub unsafe fn arch_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32 {
    let res = (*v).counter;
    (*v).counter = res.wrapping_add(i);
    res
}

#[inline]
pub unsafe fn arch_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32 {
    let res = (*v).counter;
    (*v).counter = res.wrapping_sub(i);
    res
}

#[inline]
pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) {
    (*v).counter &= i;
}

#[inline]
pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) {
    (*v).counter |= i;
}

#[inline]
pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) {
    (*v).counter ^= i;
}

#[inline]
pub unsafe fn arch_atomic_fetch_and(i: i32, v: *mut atomic_t) -> i32 {
    let res = (*v).counter;
    (*v).counter = res & i;
    res
}

#[inline]
pub unsafe fn arch_atomic_fetch_or(i: i32, v: *mut atomic_t) -> i32 {
    let res = (*v).counter;
    (*v).counter = res | i;
    res
}

#[inline]
pub unsafe fn arch_atomic_fetch_xor(i: i32, v: *mut atomic_t) -> i32 {
    let res = (*v).counter;
    (*v).counter = res ^ i;
    res
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
