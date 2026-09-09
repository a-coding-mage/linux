/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the Linux atomic/per-CPU headers are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct local_t {
    pub a: atomic_long_t,
}

pub const fn LOCAL_INIT(i: c_long) -> local_t {
    local_t { a: ATOMIC_LONG_INIT(i) }
}

#[inline]
pub unsafe fn local_read(l: *const local_t) -> c_long {
    atomic_long_read(&(*l).a)
}

#[inline]
pub unsafe fn local_set(l: *mut local_t, i: c_long) {
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
pub unsafe fn local_add(i: c_long, l: *mut local_t) {
    atomic_long_add(i, &mut (*l).a)
}

#[inline]
pub unsafe fn local_sub(i: c_long, l: *mut local_t) {
    atomic_long_sub(i, &mut (*l).a)
}

#[inline]
pub unsafe fn local_add_return(i: c_long, l: *mut local_t) -> c_long {
    // Alpha ldq_l/stq_c loop translated to the equivalent atomic compare-exchange loop.
    let mut old = local_read(l);
    loop {
        let new = old.wrapping_add(i);
        let observed = cmpxchg_local(&mut (*l).a.counter, old, new);
        if observed == old {
            return new;
        }
        old = observed;
    }
}

#[inline]
pub unsafe fn local_sub_return(i: c_long, l: *mut local_t) -> c_long {
    // Alpha ldq_l/stq_c loop translated to the equivalent atomic compare-exchange loop.
    let mut old = local_read(l);
    loop {
        let new = old.wrapping_sub(i);
        let observed = cmpxchg_local(&mut (*l).a.counter, old, new);
        if observed == old {
            return new;
        }
        old = observed;
    }
}

#[inline]
pub unsafe fn local_cmpxchg(l: *mut local_t, old: c_long, new: c_long) -> c_long {
    cmpxchg_local(&mut (*l).a.counter, old, new)
}

#[inline]
pub unsafe fn local_try_cmpxchg(l: *mut local_t, old: *mut c_long, new: c_long) -> bool {
    try_cmpxchg_local(&mut (*l).a.counter, old as *mut s64, new)
}

#[inline]
pub unsafe fn local_xchg(l: *mut local_t, n: c_long) -> c_long {
    xchg_local(&mut (*l).a.counter, n)
}

#[inline]
pub unsafe fn local_add_unless(l: *mut local_t, a: c_long, u: c_long) -> bool {
    let mut c = local_read(l);
    loop {
        if c == u {
            return false;
        }
        if local_try_cmpxchg(l, &mut c, c.wrapping_add(a)) {
            return true;
        }
    }
}

#[inline]
pub unsafe fn local_inc_not_zero(l: *mut local_t) -> bool {
    local_add_unless(l, 1, 0)
}

#[inline]
pub unsafe fn local_add_negative(a: c_long, l: *mut local_t) -> bool {
    local_add_return(a, l) < 0
}

#[inline]
pub unsafe fn local_dec_return(l: *mut local_t) -> c_long { local_sub_return(1, l) }

#[inline]
pub unsafe fn local_inc_return(l: *mut local_t) -> c_long { local_add_return(1, l) }

#[inline]
pub unsafe fn local_sub_and_test(i: c_long, l: *mut local_t) -> bool { local_sub_return(i, l) == 0 }

#[inline]
pub unsafe fn local_inc_and_test(l: *mut local_t) -> bool { local_add_return(1, l) == 0 }

#[inline]
pub unsafe fn local_dec_and_test(l: *mut local_t) -> bool { local_sub_return(1, l) == 0 }

/* Verify if faster than atomic ops */
#[inline]
pub unsafe fn __local_inc(l: *mut local_t) { (*l).a.counter += 1; }

#[inline]
pub unsafe fn __local_dec(l: *mut local_t) { (*l).a.counter += 1; }

#[inline]
pub unsafe fn __local_add(i: c_long, l: *mut local_t) { (*l).a.counter += i; }

#[inline]
pub unsafe fn __local_sub(i: c_long, l: *mut local_t) { (*l).a.counter -= i; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
