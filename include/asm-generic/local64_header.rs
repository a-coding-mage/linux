/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the Linux percpu, asm/types, asm/local, and
// linux/atomic interfaces are intentionally left external.

#[cfg(target_pointer_width = "64")]
pub struct local64_t {
    pub a: local_t,
}

#[cfg(target_pointer_width = "64")]
impl local64_t {
    #[inline]
    pub const fn new(i: i64) -> Self {
        Self { a: LOCAL_INIT(i) }
    }
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_read(l: *const local64_t) -> i64 {
    local_read(core::ptr::addr_of!((*l).a))
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_set(l: *mut local64_t, i: i64) {
    local_set(core::ptr::addr_of_mut!((*l).a), i)
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_inc(l: *mut local64_t) { local_inc(core::ptr::addr_of_mut!((*l).a)); }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_dec(l: *mut local64_t) { local_dec(core::ptr::addr_of_mut!((*l).a)); }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_add(i: i64, l: *mut local64_t) { local_add(i, core::ptr::addr_of_mut!((*l).a)); }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_sub(i: i64, l: *mut local64_t) { local_sub(i, core::ptr::addr_of_mut!((*l).a)); }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_sub_and_test(i: i64, l: *mut local64_t) -> bool { local_sub_and_test(i, core::ptr::addr_of_mut!((*l).a)) }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_dec_and_test(l: *mut local64_t) -> bool { local_dec_and_test(core::ptr::addr_of_mut!((*l).a)) }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_inc_and_test(l: *mut local64_t) -> bool { local_inc_and_test(core::ptr::addr_of_mut!((*l).a)) }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_add_negative(i: i64, l: *mut local64_t) -> bool { local_add_negative(i, core::ptr::addr_of_mut!((*l).a)) }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_add_return(i: i64, l: *mut local64_t) -> i64 { local_add_return(i, core::ptr::addr_of_mut!((*l).a)) }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_sub_return(i: i64, l: *mut local64_t) -> i64 { local_sub_return(i, core::ptr::addr_of_mut!((*l).a)) }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_inc_return(l: *mut local64_t) -> i64 { local_inc_return(core::ptr::addr_of_mut!((*l).a)) }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_cmpxchg(l: *mut local64_t, old: i64, new: i64) -> i64 { local_cmpxchg(core::ptr::addr_of_mut!((*l).a), old, new) }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_try_cmpxchg(l: *mut local64_t, old: *mut i64, new: i64) -> bool { local_try_cmpxchg(core::ptr::addr_of_mut!((*l).a), old as *mut isize, new) }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_xchg(l: *mut local64_t, n: i64) -> i64 { local_xchg(core::ptr::addr_of_mut!((*l).a), n) }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_add_unless(l: *mut local64_t, a: i64, u: i64) -> bool { local_add_unless(core::ptr::addr_of_mut!((*l).a), a, u) }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn local64_inc_not_zero(l: *mut local64_t) -> bool { local_inc_not_zero(core::ptr::addr_of_mut!((*l).a)) }

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn __local64_inc(l: *mut local64_t) { local64_set(l, local64_read(l) + 1); }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn __local64_dec(l: *mut local64_t) { local64_set(l, local64_read(l) - 1); }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn __local64_add(i: i64, l: *mut local64_t) { local64_set(l, local64_read(l) + i); }
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn __local64_sub(i: i64, l: *mut local64_t) { local64_set(l, local64_read(l) - i); }

#[cfg(not(target_pointer_width = "64"))]
pub struct local64_t {
    pub a: atomic64_t,
}

#[cfg(not(target_pointer_width = "64"))]
impl local64_t {
    #[inline]
    pub const fn new(i: i64) -> Self { Self { a: ATOMIC_LONG_INIT(i) } }
}

// The 32-bit implementation is expressed in terms of the external atomic64
// interface, preserving the source header's macro mappings.
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_read(l: *const local64_t) -> i64 { atomic64_read(core::ptr::addr_of!((*l).a)) }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_set(l: *mut local64_t, i: i64) { atomic64_set(core::ptr::addr_of_mut!((*l).a), i); }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_inc(l: *mut local64_t) { atomic64_inc(core::ptr::addr_of_mut!((*l).a)); }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_dec(l: *mut local64_t) { atomic64_dec(core::ptr::addr_of_mut!((*l).a)); }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_add(i: i64, l: *mut local64_t) { atomic64_add(i, core::ptr::addr_of_mut!((*l).a)); }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_sub(i: i64, l: *mut local64_t) { atomic64_sub(i, core::ptr::addr_of_mut!((*l).a)); }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_cmpxchg(l: *mut local64_t, o: i64, n: i64) -> i64 { atomic64_cmpxchg(core::ptr::addr_of_mut!((*l).a), o, n) }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_xchg(l: *mut local64_t, n: i64) -> i64 { atomic64_xchg(core::ptr::addr_of_mut!((*l).a), n) }

#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_try_cmpxchg(l: *mut local64_t, po: *mut i64, n: i64) -> bool { atomic64_try_cmpxchg(core::ptr::addr_of_mut!((*l).a), po, n) }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_sub_and_test(i: i64, l: *mut local64_t) -> bool { atomic64_sub_and_test(i, core::ptr::addr_of_mut!((*l).a)) }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_dec_and_test(l: *mut local64_t) -> bool { atomic64_dec_and_test(core::ptr::addr_of_mut!((*l).a)) }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_inc_and_test(l: *mut local64_t) -> bool { atomic64_inc_and_test(core::ptr::addr_of_mut!((*l).a)) }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_add_negative(i: i64, l: *mut local64_t) -> bool { atomic64_add_negative(i, core::ptr::addr_of_mut!((*l).a)) }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_add_return(i: i64, l: *mut local64_t) -> i64 { atomic64_add_return(i, core::ptr::addr_of_mut!((*l).a)) }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_sub_return(i: i64, l: *mut local64_t) -> i64 { atomic64_sub_return(i, core::ptr::addr_of_mut!((*l).a)) }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_inc_return(l: *mut local64_t) -> i64 { atomic64_inc_return(core::ptr::addr_of_mut!((*l).a)) }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_add_unless(l: *mut local64_t, a: i64, u: i64) -> bool { atomic64_add_unless(core::ptr::addr_of_mut!((*l).a), a, u) }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn local64_inc_not_zero(l: *mut local64_t) -> bool { atomic64_inc_not_zero(core::ptr::addr_of_mut!((*l).a)) }

#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn __local64_inc(l: *mut local64_t) { local64_set(l, local64_read(l) + 1); }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn __local64_dec(l: *mut local64_t) { local64_set(l, local64_read(l) - 1); }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn __local64_add(i: i64, l: *mut local64_t) { local64_set(l, local64_read(l) + i); }
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn __local64_sub(i: i64, l: *mut local64_t) { local64_set(l, local64_read(l) - i); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
