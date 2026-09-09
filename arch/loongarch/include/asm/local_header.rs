/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation:
// linux::atomic, asm::cmpxchg, and the architecture atomic/assembly helpers.

#[repr(C)]
pub struct local_t {
    pub a: atomic_long_t,
}

// C: #define LOCAL_INIT(i) { ATOMIC_LONG_INIT(i) }
#[macro_export]
macro_rules! LOCAL_INIT {
    ($i:expr) => { local_t { a: ATOMIC_LONG_INIT!($i) } };
}

#[macro_export]
macro_rules! local_read { ($l:expr) => { atomic_long_read(unsafe { &(*$l).a }) }; }
#[macro_export]
macro_rules! local_set { ($l:expr, $i:expr) => { atomic_long_set(unsafe { &mut (*$l).a }, $i) }; }
#[macro_export]
macro_rules! local_add { ($i:expr, $l:expr) => { atomic_long_add($i, unsafe { &mut (*$l).a }) }; }
#[macro_export]
macro_rules! local_sub { ($i:expr, $l:expr) => { atomic_long_sub($i, unsafe { &mut (*$l).a }) }; }
#[macro_export]
macro_rules! local_inc { ($l:expr) => { atomic_long_inc(unsafe { &mut (*$l).a }) }; }
#[macro_export]
macro_rules! local_dec { ($l:expr) => { atomic_long_dec(unsafe { &mut (*$l).a }) }; }

/* Same as above, but return the result value. */
#[cfg(CONFIG_CPU_HAS_AMO)]
#[inline]
pub unsafe fn local_add_return(i: i64, l: *mut local_t) -> i64 {
    let result: i64;
    core::arch::asm!("amadd.d {result}, {i}, [{counter}]",
        result = lateout(reg) result, i = in(reg) i,
        counter = in(reg) &mut (*l).a.counter,
        options(nostack));
    result.wrapping_add(i)
}

#[cfg(CONFIG_CPU_HAS_AMO)]
#[inline]
pub unsafe fn local_sub_return(i: i64, l: *mut local_t) -> i64 {
    let result: i64;
    core::arch::asm!("amadd.d {result}, {neg}, [{counter}]",
        result = lateout(reg) result, neg = in(reg) i.wrapping_neg(),
        counter = in(reg) &mut (*l).a.counter,
        options(nostack));
    result.wrapping_sub(i)
}

// When CONFIG_CPU_HAS_AMO is disabled, the C source uses an LL/SC retry loop.
// The architecture-specific inline assembly is preserved here as a narrow
// external dependency; its implementation is supplied by the target backend.
#[cfg(not(CONFIG_CPU_HAS_AMO))]
extern "C" {
    fn __local_add_return_llsc(i: i64, l: *mut local_t) -> i64;
    fn __local_sub_return_llsc(i: i64, l: *mut local_t) -> i64;
}

#[cfg(not(CONFIG_CPU_HAS_AMO))]
#[inline]
pub unsafe fn local_add_return(i: i64, l: *mut local_t) -> i64 { __local_add_return_llsc(i, l) }
#[cfg(not(CONFIG_CPU_HAS_AMO))]
#[inline]
pub unsafe fn local_sub_return(i: i64, l: *mut local_t) -> i64 { __local_sub_return_llsc(i, l) }

#[inline]
pub unsafe fn local_cmpxchg(l: *mut local_t, old: i64, new: i64) -> i64 {
    cmpxchg_local(&mut (*l).a.counter, old, new)
}

#[inline]
pub unsafe fn local_try_cmpxchg(l: *mut local_t, old: *mut i64, new: i64) -> bool {
    try_cmpxchg_local(&mut (*l).a.counter, old, new)
}

#[macro_export]
macro_rules! local_xchg { ($l:expr, $n:expr) => { atomic_long_xchg(unsafe { &mut (*$l).a }, $n) }; }

#[inline]
pub unsafe fn local_add_unless(l: *mut local_t, a: i64, u: i64) -> bool {
    let mut c = local_read!(l);
    loop {
        if c == u { return false; }
        if local_try_cmpxchg(l, &mut c, c.wrapping_add(a)) { return true; }
    }
}

#[macro_export]
macro_rules! local_inc_not_zero { ($l:expr) => { local_add_unless($l, 1, 0) }; }
#[macro_export]
macro_rules! local_dec_return { ($l:expr) => { local_sub_return(1, $l) }; }
#[macro_export]
macro_rules! local_inc_return { ($l:expr) => { local_add_return(1, $l) }; }
#[macro_export]
macro_rules! local_sub_and_test { ($i:expr, $l:expr) => { local_sub_return($i, $l) == 0 }; }
#[macro_export]
macro_rules! local_inc_and_test { ($l:expr) => { local_inc_return!($l) == 0 }; }
#[macro_export]
macro_rules! local_dec_and_test { ($l:expr) => { local_sub_return(1, $l) == 0 }; }
#[macro_export]
macro_rules! local_add_negative { ($i:expr, $l:expr) => { local_add_return($i, $l) < 0 }; }

/* Per-cpu local_t operations; the argument is a variable, not an address. */
#[macro_export]
macro_rules! __local_inc { ($l:expr) => { unsafe { (*$l).a.counter += 1 } }; }
#[macro_export]
macro_rules! __local_dec { ($l:expr) => { unsafe { (*$l).a.counter += 1 } }; }
#[macro_export]
macro_rules! __local_add { ($i:expr, $l:expr) => { unsafe { (*$l).a.counter += $i } }; }
#[macro_export]
macro_rules! __local_sub { ($i:expr, $l:expr) => { unsafe { (*$l).a.counter -= $i } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
