/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct local_t {
    pub a: atomic_long_t,
}

pub const fn LOCAL_INIT(i: long) -> local_t {
    local_t { a: ATOMIC_LONG_INIT(i) }
}

#[inline]
pub unsafe fn local_read(l: *const local_t) -> long {
    atomic_long_read(core::ptr::addr_of!((*l).a))
}

#[inline]
pub unsafe fn local_set(l: *mut local_t, i: long) {
    atomic_long_set(core::ptr::addr_of_mut!((*l).a), i)
}

#[inline]
pub unsafe fn local_inc(l: *mut local_t) {
    asm_local_inc(core::ptr::addr_of_mut!((*l).a.counter));
}

#[inline]
pub unsafe fn local_dec(l: *mut local_t) {
    asm_local_dec(core::ptr::addr_of_mut!((*l).a.counter));
}

#[inline]
pub unsafe fn local_add(i: long, l: *mut local_t) {
    asm_local_add(core::ptr::addr_of_mut!((*l).a.counter), i);
}

#[inline]
pub unsafe fn local_sub(i: long, l: *mut local_t) {
    asm_local_sub(core::ptr::addr_of_mut!((*l).a.counter), i);
}

/**
 * local_sub_and_test - subtract value from variable and test result
 * @i: integer value to subtract
 * @l: pointer to type local_t
 *
 * Atomically subtracts @i from @l and returns
 * true if the result is zero, or false for all
 * other cases.
 */
#[inline]
pub unsafe fn local_sub_and_test(i: long, l: *mut local_t) -> bool {
    binary_rmwcc_sub(&mut (*l).a.counter, i)
}

/**
 * local_dec_and_test - decrement and test
 * @l: pointer to type local_t
 *
 * Atomically decrements @l by 1 and
 * returns true if the result is 0, or false for all other
 * cases.
 */
#[inline]
pub unsafe fn local_dec_and_test(l: *mut local_t) -> bool {
    unary_rmwcc_dec(&mut (*l).a.counter)
}

/**
 * local_inc_and_test - increment and test
 * @l: pointer to type local_t
 *
 * Atomically increments @l by 1
 * and returns true if the result is zero, or false for all
 * other cases.
 */
#[inline]
pub unsafe fn local_inc_and_test(l: *mut local_t) -> bool {
    unary_rmwcc_inc(&mut (*l).a.counter)
}

/**
 * local_add_negative - add and test if negative
 * @i: integer value to add
 * @l: pointer to type local_t
 *
 * Atomically adds @i to @l and returns true
 * if the result is negative, or false when
 * result is greater than or equal to zero.
 */
#[inline]
pub unsafe fn local_add_negative(i: long, l: *mut local_t) -> bool {
    binary_rmwcc_add_negative(&mut (*l).a.counter, i)
}

#[inline]
pub unsafe fn local_add_return(i: long, l: *mut local_t) -> long {
    let mut i = i;
    let old = asm_local_xadd(core::ptr::addr_of_mut!((*l).a.counter), &mut i);
    i + old
}

#[inline]
pub unsafe fn local_sub_return(i: long, l: *mut local_t) -> long {
    local_add_return(-i, l)
}

#[inline]
pub unsafe fn local_inc_return(l: *mut local_t) -> long { local_add_return(1, l) }

#[inline]
pub unsafe fn local_dec_return(l: *mut local_t) -> long { local_sub_return(1, l) }

#[inline]
pub unsafe fn local_cmpxchg(l: *mut local_t, old: long, new: long) -> long {
    cmpxchg_local(core::ptr::addr_of_mut!((*l).a.counter), old, new)
}

#[inline]
pub unsafe fn local_try_cmpxchg(l: *mut local_t, old: *mut long, new: long) -> bool {
    try_cmpxchg_local(core::ptr::addr_of_mut!((*l).a.counter), old, new)
}

/* Implement local_xchg using CMPXCHG without the LOCK prefix. */
#[inline(always)]
pub unsafe fn local_xchg(l: *mut local_t, n: long) -> long {
    let mut c = local_read(l);
    while !local_try_cmpxchg(l, &mut c, n) {}
    c
}

/** local_add_unless - add unless the number is already a given value */
#[inline(always)]
pub unsafe fn local_add_unless(l: *mut local_t, a: long, u: long) -> bool {
    let mut c = local_read(l);
    loop {
        if c == u { return false; }
        if local_try_cmpxchg(l, &mut c, c + a) { return true; }
    }
}

#[inline]
pub unsafe fn local_inc_not_zero(l: *mut local_t) -> bool {
    local_add_unless(l, 1, 0)
}

// On x86_32 these are no better than the atomic variants. On x86-64 these
// are better than the atomic variants on SMP kernels because they do not use
// a lock prefix.
#[inline] pub unsafe fn __local_inc(l: *mut local_t) { local_inc(l) }
#[inline] pub unsafe fn __local_dec(l: *mut local_t) { local_dec(l) }
#[inline] pub unsafe fn __local_add(i: long, l: *mut local_t) { local_add(i, l) }
#[inline] pub unsafe fn __local_sub(i: long, l: *mut local_t) { local_sub(i, l) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
