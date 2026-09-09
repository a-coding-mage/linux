/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Generic implementation of 64-bit atomics using spinlocks,
 * useful on processors that don't have 64-bit atomic instructions.
 *
 * Copyright © 2009 Paul Mackerras, IBM Corp. <paulus@au1.ibm.com>
 */

#[repr(C, align(8))]
pub struct atomic64_t {
    pub counter: i64,
}

#[macro_export]
macro_rules! ATOMIC64_INIT {
    ($i:expr) => {
        $crate::atomic64_t { counter: $i }
    };
}

unsafe extern "C" {
    pub fn generic_atomic64_read(v: *const atomic64_t) -> i64;
    pub fn generic_atomic64_set(v: *mut atomic64_t, i: i64);

    pub fn generic_atomic64_add(a: i64, v: *mut atomic64_t);
    pub fn generic_atomic64_add_return(a: i64, v: *mut atomic64_t) -> i64;
    pub fn generic_atomic64_fetch_add(a: i64, v: *mut atomic64_t) -> i64;

    pub fn generic_atomic64_sub(a: i64, v: *mut atomic64_t);
    pub fn generic_atomic64_sub_return(a: i64, v: *mut atomic64_t) -> i64;
    pub fn generic_atomic64_fetch_sub(a: i64, v: *mut atomic64_t) -> i64;

    pub fn generic_atomic64_and(a: i64, v: *mut atomic64_t);
    pub fn generic_atomic64_fetch_and(a: i64, v: *mut atomic64_t) -> i64;
    pub fn generic_atomic64_or(a: i64, v: *mut atomic64_t);
    pub fn generic_atomic64_fetch_or(a: i64, v: *mut atomic64_t) -> i64;
    pub fn generic_atomic64_xor(a: i64, v: *mut atomic64_t);
    pub fn generic_atomic64_fetch_xor(a: i64, v: *mut atomic64_t) -> i64;

    pub fn generic_atomic64_dec_if_positive(v: *mut atomic64_t) -> i64;
    pub fn generic_atomic64_cmpxchg(v: *mut atomic64_t, o: i64, n: i64) -> i64;
    pub fn generic_atomic64_xchg(v: *mut atomic64_t, new: i64) -> i64;
    pub fn generic_atomic64_fetch_add_unless(v: *mut atomic64_t, a: i64, u: i64) -> i64;
}

pub use generic_atomic64_read as arch_atomic64_read;
pub use generic_atomic64_set as arch_atomic64_set;
pub use generic_atomic64_set as arch_atomic64_set_release;

pub use generic_atomic64_add as arch_atomic64_add;
pub use generic_atomic64_add_return as arch_atomic64_add_return;
pub use generic_atomic64_fetch_add as arch_atomic64_fetch_add;
pub use generic_atomic64_sub as arch_atomic64_sub;
pub use generic_atomic64_sub_return as arch_atomic64_sub_return;
pub use generic_atomic64_fetch_sub as arch_atomic64_fetch_sub;

pub use generic_atomic64_and as arch_atomic64_and;
pub use generic_atomic64_fetch_and as arch_atomic64_fetch_and;
pub use generic_atomic64_or as arch_atomic64_or;
pub use generic_atomic64_fetch_or as arch_atomic64_fetch_or;
pub use generic_atomic64_xor as arch_atomic64_xor;
pub use generic_atomic64_fetch_xor as arch_atomic64_fetch_xor;

pub use generic_atomic64_dec_if_positive as arch_atomic64_dec_if_positive;
pub use generic_atomic64_cmpxchg as arch_atomic64_cmpxchg;
pub use generic_atomic64_xchg as arch_atomic64_xchg;
pub use generic_atomic64_fetch_add_unless as arch_atomic64_fetch_add_unless;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
