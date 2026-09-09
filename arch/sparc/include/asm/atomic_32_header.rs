/* SPDX-License-Identifier: GPL-2.0 */
/* atomic.h: These still suck, but the I-cache hit rate is higher.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 * Copyright (C) 2000 Anton Blanchard (anton@linuxcare.com.au)
 * Copyright (C) 2007 Kyle McMartin (kyle@parisc-linux.org)
 *
 * Additions by Keith M Wesolowski (wesolows@foobazco.org) based
 * on asm-parisc/atomic.h Copyright (C) 2000 Philipp Rumpf <prumpf@tux.org>.
 */

// Dependencies supplied by linux/types.h, asm/cmpxchg.h, asm/barrier.h, and
// asm-generic/atomic64.h are intentionally left external to this translation.

#[repr(C)]
pub struct atomic_t {
    pub counter: i32,
}

extern "C" {
    pub fn arch_atomic_add_return(i: i32, v: *mut atomic_t) -> i32;
    pub fn arch_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32;
    pub fn arch_atomic_fetch_and(i: i32, v: *mut atomic_t) -> i32;
    pub fn arch_atomic_fetch_or(i: i32, v: *mut atomic_t) -> i32;
    pub fn arch_atomic_fetch_xor(i: i32, v: *mut atomic_t) -> i32;
    pub fn arch_atomic_cmpxchg(v: *mut atomic_t, old: i32, new: i32) -> i32;
    pub fn arch_atomic_xchg(v: *mut atomic_t, i: i32) -> i32;
    pub fn arch_atomic_fetch_add_unless(v: *mut atomic_t, a: i32, u: i32) -> i32;
    pub fn arch_atomic_set(v: *mut atomic_t, i: i32);
}

#[inline]
pub unsafe fn arch_atomic_set_release(v: *mut atomic_t, i: i32) {
    arch_atomic_set(v, i)
}

#[inline]
pub unsafe fn arch_atomic_read(v: *const atomic_t) -> i32 {
    core::ptr::read_volatile(core::ptr::addr_of!((*v).counter))
}

#[inline]
pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) {
    let _ = arch_atomic_add_return(i as i32, v);
}

#[inline]
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) {
    let _ = arch_atomic_add_return(-(i as i32), v);
}

#[inline]
pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) {
    let _ = arch_atomic_fetch_and(i, v);
}

#[inline]
pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) {
    let _ = arch_atomic_fetch_or(i, v);
}

#[inline]
pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) {
    let _ = arch_atomic_fetch_xor(i, v);
}

#[inline]
pub unsafe fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 {
    arch_atomic_add_return(-(i as i32), v)
}

#[inline]
pub unsafe fn arch_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32 {
    arch_atomic_fetch_add(-(i as i32), v)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
