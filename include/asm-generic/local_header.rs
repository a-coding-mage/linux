/* SPDX-License-Identifier: GPL-2.0 */

/*
 * A signed long type for operations which are atomic for a single CPU.
 * Usually used in combination with per-cpu variables.
 *
 * This is the default implementation, which uses atomic_long_t.  Which is
 * rather pointless.  The whole point behind local_t is that some processors
 * can perform atomic adds and subtracts in a manner which is atomic wrt IRQs
 * running on this CPU.  local_t allows exploitation of such capabilities.
 */

/* Implement in terms of atomics. */

/* Don't use typedef: don't want them to be mixed with atomic_t's. */
#[repr(C)]
pub struct local_t {
    pub a: atomic_long_t,
}

#[macro_export]
macro_rules! LOCAL_INIT {
    ($i:expr) => {
        local_t { a: ATOMIC_LONG_INIT!($i) }
    };
}

#[inline]
pub unsafe fn local_read(l: *const local_t) -> isize {
    atomic_long_read(&(*l).a)
}

#[inline]
pub unsafe fn local_set(l: *mut local_t, i: isize) {
    atomic_long_set(&mut (*l).a, i)
}

#[inline]
pub unsafe fn local_inc(l: *mut local_t) {
    atomic_long_inc(&mut (*l).a)
}

#[inline]
pub unsafe fn local_dec(l: *mut local_t) {
    atomic_long_dec(&mut (*l).a)
}

#[inline]
pub unsafe fn local_add(i: isize, l: *mut local_t) {
    atomic_long_add(i, &mut (*l).a)
}

#[inline]
pub unsafe fn local_sub(i: isize, l: *mut local_t) {
    atomic_long_sub(i, &mut (*l).a)
}

#[inline]
pub unsafe fn local_sub_and_test(i: isize, l: *mut local_t) -> bool {
    atomic_long_sub_and_test(i, &mut (*l).a)
}

#[inline]
pub unsafe fn local_dec_and_test(l: *mut local_t) -> bool {
    atomic_long_dec_and_test(&mut (*l).a)
}

#[inline]
pub unsafe fn local_inc_and_test(l: *mut local_t) -> bool {
    atomic_long_inc_and_test(&mut (*l).a)
}

#[inline]
pub unsafe fn local_add_negative(i: isize, l: *mut local_t) -> bool {
    atomic_long_add_negative(i, &mut (*l).a)
}

#[inline]
pub unsafe fn local_add_return(i: isize, l: *mut local_t) -> isize {
    atomic_long_add_return(i, &mut (*l).a)
}

#[inline]
pub unsafe fn local_sub_return(i: isize, l: *mut local_t) -> isize {
    atomic_long_sub_return(i, &mut (*l).a)
}

#[inline]
pub unsafe fn local_inc_return(l: *mut local_t) -> isize {
    atomic_long_inc_return(&mut (*l).a)
}

#[inline]
pub unsafe fn local_cmpxchg(l: *mut local_t, o: isize, n: isize) -> isize {
    atomic_long_cmpxchg(&mut (*l).a, o, n)
}

#[inline]
pub unsafe fn local_try_cmpxchg(l: *mut local_t, po: *mut isize, n: isize) -> bool {
    atomic_long_try_cmpxchg(&mut (*l).a, po, n)
}

#[inline]
pub unsafe fn local_xchg(l: *mut local_t, n: isize) -> isize {
    atomic_long_xchg(&mut (*l).a, n)
}

#[inline]
pub unsafe fn local_add_unless(l: *mut local_t, a: isize, u: isize) -> bool {
    atomic_long_add_unless(&mut (*l).a, a, u)
}

#[inline]
pub unsafe fn local_inc_not_zero(l: *mut local_t) -> bool {
    atomic_long_inc_not_zero(&mut (*l).a)
}

/* Non-atomic variants, ie. preemption disabled and won't be touched
 * in interrupt, etc.  Some archs can optimize this case well. */
#[inline]
pub unsafe fn __local_inc(l: *mut local_t) {
    local_set(l, local_read(l) + 1)
}

#[inline]
pub unsafe fn __local_dec(l: *mut local_t) {
    local_set(l, local_read(l) - 1)
}

#[inline]
pub unsafe fn __local_add(i: isize, l: *mut local_t) {
    local_set(l, local_read(l) + i)
}

#[inline]
pub unsafe fn __local_sub(i: isize, l: *mut local_t) {
    local_set(l, local_read(l) - i)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
