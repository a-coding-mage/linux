/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_PPC_BOOK3S_64 controls whether this implementation is selected.
// The non-Book3S-64 case is supplied by asm-generic/local.h.

#[repr(C)]
pub struct local_t {
    pub v: ::core::ffi::c_long,
}

#[macro_export]
macro_rules! LOCAL_INIT {
    ($i:expr) => {
        $crate::local_t { v: $i }
    };
}

#[inline]
pub unsafe fn local_read(l: *const local_t) -> ::core::ffi::c_long {
    ::core::ptr::read_volatile(::core::ptr::addr_of!((*l).v))
}

#[inline]
pub unsafe fn local_set(l: *mut local_t, i: ::core::ffi::c_long) {
    ::core::ptr::write_volatile(::core::ptr::addr_of_mut!((*l).v), i);
}

#[inline]
pub unsafe fn local_add(i: ::core::ffi::c_long, l: *mut local_t) {
    let mut flags: ::core::ffi::c_ulong = 0;
    powerpc_local_irq_pmu_save(&mut flags);
    (*l).v += i;
    powerpc_local_irq_pmu_restore(flags);
}

#[inline]
pub unsafe fn local_add_return(a: ::core::ffi::c_long, l: *mut local_t) -> ::core::ffi::c_long {
    let mut flags: ::core::ffi::c_ulong = 0;
    powerpc_local_irq_pmu_save(&mut flags);
    let t = (*l).v + a;
    (*l).v = t;
    powerpc_local_irq_pmu_restore(flags);
    t
}

#[inline]
pub unsafe fn local_sub(i: ::core::ffi::c_long, l: *mut local_t) {
    let mut flags: ::core::ffi::c_ulong = 0;
    powerpc_local_irq_pmu_save(&mut flags);
    (*l).v -= i;
    powerpc_local_irq_pmu_restore(flags);
}

#[inline]
pub unsafe fn local_sub_return(a: ::core::ffi::c_long, l: *mut local_t) -> ::core::ffi::c_long {
    let mut flags: ::core::ffi::c_ulong = 0;
    powerpc_local_irq_pmu_save(&mut flags);
    let t = (*l).v - a;
    (*l).v = t;
    powerpc_local_irq_pmu_restore(flags);
    t
}

#[inline]
pub unsafe fn local_add_negative(a: ::core::ffi::c_long, l: *mut local_t) -> bool {
    local_add_return(a, l) < 0
}

#[inline]
pub unsafe fn local_inc_return(l: *mut local_t) -> ::core::ffi::c_long {
    local_add_return(1_i64 as ::core::ffi::c_long, l)
}

#[inline]
pub unsafe fn local_inc(l: *mut local_t) -> ::core::ffi::c_long {
    local_inc_return(l)
}

/* local_inc_and_test - increment and test
 * @l: pointer of type local_t
 *
 * Atomically increments @l by 1
 * and returns true if the result is zero, or false for all
 * other cases.
 */
#[inline]
pub unsafe fn local_inc_and_test(l: *mut local_t) -> bool {
    local_inc_return(l) == 0
}

#[inline]
pub unsafe fn local_dec_return(l: *mut local_t) -> ::core::ffi::c_long {
    local_sub_return(1_i64 as ::core::ffi::c_long, l)
}

#[inline]
pub unsafe fn local_dec(l: *mut local_t) -> ::core::ffi::c_long {
    local_dec_return(l)
}

#[inline]
pub unsafe fn local_sub_and_test(a: ::core::ffi::c_long, l: *mut local_t) -> bool {
    local_sub_return(a, l) == 0
}

#[inline]
pub unsafe fn local_dec_and_test(l: *mut local_t) -> bool {
    local_dec_return(l) == 0
}

#[inline]
pub unsafe fn local_cmpxchg(
    l: *mut local_t,
    o: ::core::ffi::c_long,
    n: ::core::ffi::c_long,
) -> ::core::ffi::c_long {
    let mut flags: ::core::ffi::c_ulong = 0;
    powerpc_local_irq_pmu_save(&mut flags);
    let t = (*l).v;
    if t == o {
        (*l).v = n;
    }
    powerpc_local_irq_pmu_restore(flags);
    t
}

#[inline]
pub unsafe fn local_try_cmpxchg(
    l: *mut local_t,
    po: *mut ::core::ffi::c_long,
    n: ::core::ffi::c_long,
) -> bool {
    let o = *po;
    let r = local_cmpxchg(l, o, n);
    if r != o {
        *po = r;
    }
    r == o
}

#[inline]
pub unsafe fn local_xchg(l: *mut local_t, n: ::core::ffi::c_long) -> ::core::ffi::c_long {
    let mut flags: ::core::ffi::c_ulong = 0;
    powerpc_local_irq_pmu_save(&mut flags);
    let t = (*l).v;
    (*l).v = n;
    powerpc_local_irq_pmu_restore(flags);
    t
}

/**
 * local_add_unless - add unless the number is already a given value
 * @l: pointer of type local_t
 * @a: the amount to add to v...
 * @u: ...unless v is equal to u.
 *
 * Atomically adds @a to @l, if @v was not already @u.
 * Returns true if the addition was done.
 */
#[inline]
pub unsafe fn local_add_unless(
    l: *mut local_t,
    a: ::core::ffi::c_long,
    u: ::core::ffi::c_long,
) -> bool {
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut ret = false;
    powerpc_local_irq_pmu_save(&mut flags);
    if (*l).v != u {
        (*l).v += a;
        ret = true;
    }
    powerpc_local_irq_pmu_restore(flags);
    ret
}

#[inline]
pub unsafe fn local_inc_not_zero(l: *mut local_t) -> bool {
    local_add_unless(l, 1, 0)
}

/* Per-cpu local_t operations; these take a variable, not an address. */
#[inline]
pub unsafe fn __local_inc(l: *mut local_t) -> ::core::ffi::c_long {
    let old = (*l).v;
    (*l).v += 1;
    old
}

#[inline]
pub unsafe fn __local_dec(l: *mut local_t) -> ::core::ffi::c_long {
    let old = (*l).v;
    (*l).v += 1;
    old
}

#[inline]
pub unsafe fn __local_add(i: ::core::ffi::c_long, l: *mut local_t) -> ::core::ffi::c_long {
    (*l).v += i;
    (*l).v
}

#[inline]
pub unsafe fn __local_sub(i: ::core::ffi::c_long, l: *mut local_t) -> ::core::ffi::c_long {
    (*l).v -= i;
    (*l).v
}

extern "C" {
    fn powerpc_local_irq_pmu_save(flags: *mut ::core::ffi::c_ulong);
    fn powerpc_local_irq_pmu_restore(flags: ::core::ffi::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
