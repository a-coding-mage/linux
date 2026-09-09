/* SPDX-License-Identifier: GPL-2.0 */
// Translated from x86/include/asm/atomic64_32.h.
// C header dependencies and build-time CONFIG_X86_CX8 dispatch are external.

pub type s64 = i64;

#[repr(C, align(8))]
pub struct atomic64_t {
    pub counter: s64,
}

#[inline(always)]
pub unsafe fn arch_atomic64_read_nonatomic(v: *const atomic64_t) -> s64 {
    // Corresponds to __READ_ONCE(v->counter); this intentionally permits a torn read.
    core::ptr::read_volatile(core::ptr::addr_of!((*v).counter))
}

// Assembly-backed operations supplied by the architecture implementation.
extern "C" {
    pub fn arch_cmpxchg64(ptr: *mut s64, old: s64, new: s64) -> s64;
    pub fn arch_try_cmpxchg64(ptr: *mut s64, old: *mut s64, new: s64) -> bool;
    pub fn atomic64_read(v: *const atomic64_t) -> s64;
    pub fn atomic64_set(v: *mut atomic64_t, i: s64);
    pub fn atomic64_xchg(v: *mut atomic64_t, n: s64) -> s64;
    pub fn atomic64_add_return(i: s64, v: *mut atomic64_t) -> s64;
    pub fn atomic64_sub_return(i: s64, v: *mut atomic64_t) -> s64;
    pub fn atomic64_inc_return(v: *mut atomic64_t) -> s64;
    pub fn atomic64_dec_return(v: *mut atomic64_t) -> s64;
    pub fn atomic64_add(i: s64, v: *mut atomic64_t);
    pub fn atomic64_sub(i: s64, v: *mut atomic64_t);
    pub fn atomic64_inc(v: *mut atomic64_t);
    pub fn atomic64_dec(v: *mut atomic64_t);
    pub fn atomic64_add_unless(v: *mut atomic64_t, a: s64, u: s64) -> i32;
    pub fn atomic64_inc_not_zero(v: *mut atomic64_t) -> i32;
    pub fn atomic64_dec_if_positive(v: *mut atomic64_t) -> s64;
}

#[inline(always)]
pub unsafe fn arch_atomic64_cmpxchg(v: *mut atomic64_t, old: s64, new: s64) -> s64 {
    arch_cmpxchg64(core::ptr::addr_of_mut!((*v).counter), old, new)
}

#[inline(always)]
pub unsafe fn arch_atomic64_try_cmpxchg(v: *mut atomic64_t, old: *mut s64, new: s64) -> bool {
    arch_try_cmpxchg64(core::ptr::addr_of_mut!((*v).counter), old, new)
}

#[inline(always)] pub unsafe fn arch_atomic64_xchg(v: *mut atomic64_t, n: s64) -> s64 { atomic64_xchg(v, n) }
#[inline(always)] pub unsafe fn arch_atomic64_set(v: *mut atomic64_t, i: s64) { atomic64_set(v, i) }
#[inline(always)] pub unsafe fn arch_atomic64_read(v: *const atomic64_t) -> s64 { atomic64_read(v) }
#[inline(always)] pub unsafe fn arch_atomic64_add_return(i: s64, v: *mut atomic64_t) -> s64 { atomic64_add_return(i, v) }
#[inline(always)] pub unsafe fn arch_atomic64_sub_return(i: s64, v: *mut atomic64_t) -> s64 { atomic64_sub_return(i, v) }
#[inline(always)] pub unsafe fn arch_atomic64_inc_return(v: *mut atomic64_t) -> s64 { atomic64_inc_return(v) }
#[inline(always)] pub unsafe fn arch_atomic64_dec_return(v: *mut atomic64_t) -> s64 { atomic64_dec_return(v) }
#[inline(always)] pub unsafe fn arch_atomic64_add(i: s64, v: *mut atomic64_t) { atomic64_add(i, v) }
#[inline(always)] pub unsafe fn arch_atomic64_sub(i: s64, v: *mut atomic64_t) { atomic64_sub(i, v) }
#[inline(always)] pub unsafe fn arch_atomic64_inc(v: *mut atomic64_t) { atomic64_inc(v) }
#[inline(always)] pub unsafe fn arch_atomic64_dec(v: *mut atomic64_t) { atomic64_dec(v) }
#[inline(always)] pub unsafe fn arch_atomic64_add_unless(v: *mut atomic64_t, a: s64, u: s64) -> i32 { atomic64_add_unless(v, a, u) }
#[inline(always)] pub unsafe fn arch_atomic64_inc_not_zero(v: *mut atomic64_t) -> i32 { atomic64_inc_not_zero(v) }
#[inline(always)] pub unsafe fn arch_atomic64_dec_if_positive(v: *mut atomic64_t) -> s64 { atomic64_dec_if_positive(v) }

#[inline(always)]
pub unsafe fn arch_atomic64_and(i: s64, v: *mut atomic64_t) {
    let mut val = arch_atomic64_read_nonatomic(v);
    while !arch_atomic64_try_cmpxchg(v, &mut val, val & i) {}
}

#[inline(always)] pub unsafe fn arch_atomic64_fetch_and(i: s64, v: *mut atomic64_t) -> s64 { let mut val = arch_atomic64_read_nonatomic(v); while !arch_atomic64_try_cmpxchg(v, &mut val, val & i) {} val }
#[inline(always)] pub unsafe fn arch_atomic64_or(i: s64, v: *mut atomic64_t) { let mut val = arch_atomic64_read_nonatomic(v); while !arch_atomic64_try_cmpxchg(v, &mut val, val | i) {} }
#[inline(always)] pub unsafe fn arch_atomic64_fetch_or(i: s64, v: *mut atomic64_t) -> s64 { let mut val = arch_atomic64_read_nonatomic(v); while !arch_atomic64_try_cmpxchg(v, &mut val, val | i) {} val }
#[inline(always)] pub unsafe fn arch_atomic64_xor(i: s64, v: *mut atomic64_t) { let mut val = arch_atomic64_read_nonatomic(v); while !arch_atomic64_try_cmpxchg(v, &mut val, val ^ i) {} }
#[inline(always)] pub unsafe fn arch_atomic64_fetch_xor(i: s64, v: *mut atomic64_t) -> s64 { let mut val = arch_atomic64_read_nonatomic(v); while !arch_atomic64_try_cmpxchg(v, &mut val, val ^ i) {} val }
#[inline(always)] pub unsafe fn arch_atomic64_fetch_add(i: s64, v: *mut atomic64_t) -> s64 { let mut val = arch_atomic64_read_nonatomic(v); while !arch_atomic64_try_cmpxchg(v, &mut val, val.wrapping_add(i)) {} val }
#[inline(always)] pub unsafe fn arch_atomic64_fetch_sub(i: s64, v: *mut atomic64_t) -> s64 { arch_atomic64_fetch_add(i.wrapping_neg(), v) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
