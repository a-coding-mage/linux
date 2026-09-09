/* SPDX-License-Identifier: GPL-2.0 */
/* atomic.h: Thankfully the V9 is at least reasonable for this
 *           stuff.
 *
 * Copyright (C) 1996, 1997, 2000, 2012 David S. Miller (davem@redhat.com)
 */

// C header guard: __ARCH_SPARC64_ATOMIC__
// Dependencies supplied by the surrounding translation unit:
// linux/types.h, asm/cmpxchg.h, asm/barrier.h

/* #define ATOMIC64_INIT(i) { (i) } */
#[macro_export]
macro_rules! ATOMIC64_INIT {
    ($i:expr) => { $i };
}

/* READ_ONCE/WRITE_ONCE preserve the source-level volatile access intent. */
#[macro_export]
macro_rules! arch_atomic_read {
    ($v:expr) => {{ unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*($v)).counter)) } }};
}
#[macro_export]
macro_rules! arch_atomic64_read {
    ($v:expr) => {{ unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*($v)).counter)) } }};
}

#[macro_export]
macro_rules! arch_atomic_set {
    ($v:expr, $i:expr) => {{ unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!((*($v)).counter), $i); } }};
}
#[macro_export]
macro_rules! arch_atomic64_set {
    ($v:expr, $i:expr) => {{ unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!((*($v)).counter), $i); } }};
}

extern "C" {
    pub fn arch_atomic_add(i: i32, v: *mut atomic_t);
    pub fn arch_atomic64_add(i: i64, v: *mut atomic64_t);
    pub fn arch_atomic_add_return(i: i32, v: *mut atomic_t) -> i32;
    pub fn arch_atomic64_add_return(i: i64, v: *mut atomic64_t) -> i64;
    pub fn arch_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32;
    pub fn arch_atomic64_fetch_add(i: i64, v: *mut atomic64_t) -> i64;

    pub fn arch_atomic_sub(i: i32, v: *mut atomic_t);
    pub fn arch_atomic64_sub(i: i64, v: *mut atomic64_t);
    pub fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32;
    pub fn arch_atomic64_sub_return(i: i64, v: *mut atomic64_t) -> i64;
    pub fn arch_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32;
    pub fn arch_atomic64_fetch_sub(i: i64, v: *mut atomic64_t) -> i64;

    pub fn arch_atomic_and(i: i32, v: *mut atomic_t);
    pub fn arch_atomic64_and(i: i64, v: *mut atomic64_t);
    pub fn arch_atomic_fetch_and(i: i32, v: *mut atomic_t) -> i32;
    pub fn arch_atomic64_fetch_and(i: i64, v: *mut atomic64_t) -> i64;

    pub fn arch_atomic_or(i: i32, v: *mut atomic_t);
    pub fn arch_atomic64_or(i: i64, v: *mut atomic64_t);
    pub fn arch_atomic_fetch_or(i: i32, v: *mut atomic_t) -> i32;
    pub fn arch_atomic64_fetch_or(i: i64, v: *mut atomic64_t) -> i64;

    pub fn arch_atomic_xor(i: i32, v: *mut atomic_t);
    pub fn arch_atomic64_xor(i: i64, v: *mut atomic64_t);
    pub fn arch_atomic_fetch_xor(i: i32, v: *mut atomic_t) -> i32;
    pub fn arch_atomic64_fetch_xor(i: i64, v: *mut atomic64_t) -> i64;

    pub fn arch_atomic64_dec_if_positive(v: *mut atomic64_t) -> i64;
}

// The C self-referential aliases preserve the architecture interface names.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
