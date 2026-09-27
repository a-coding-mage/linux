/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from arch/x86/include/asm/atomic64_64.h. */

/* Depends on: linux/types.h, asm/alternative.h, asm/cmpxchg.h */

/* The 64-bit atomic type */

#[macro_export]
macro_rules! ATOMIC64_INIT {
    ($i:expr) => {
        atomic64_t { counter: $i }
    };
}

#[inline(always)]
pub unsafe fn arch_atomic64_read(v: *const atomic64_t) -> s64 {
    __READ_ONCE!((*v).counter)
}

#[inline(always)]
pub unsafe fn arch_atomic64_set(v: *mut atomic64_t, i: s64) {
    __WRITE_ONCE!((*v).counter, i)
}

#[inline(always)]
pub unsafe fn arch_atomic64_add(i: s64, v: *mut atomic64_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "addq {i:r}, ({counter})"),
            i = in(reg) i,
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic64_sub(i: s64, v: *mut atomic64_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "subq {i:r}, ({counter})"),
            i = in(reg) i,
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic64_sub_and_test(i: s64, v: *mut atomic64_t) -> bool {
    let c: u8;
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "subq {i:r}, ({counter})"),
            "sete {c}",
            i = in(reg) i,
            counter = in(reg) &raw mut (*v).counter,
            c = out(reg_byte) c,
            options(att_syntax, nostack)
        )
    }
    c != 0
}

#[inline(always)]
pub unsafe fn arch_atomic64_inc(v: *mut atomic64_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "incq ({counter})"),
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic64_dec(v: *mut atomic64_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "decq ({counter})"),
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic64_dec_and_test(v: *mut atomic64_t) -> bool {
    let c: u8;
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "decq ({counter})"),
            "sete {c}",
            counter = in(reg) &raw mut (*v).counter,
            c = out(reg_byte) c,
            options(att_syntax, nostack)
        )
    }
    c != 0
}

#[inline(always)]
pub unsafe fn arch_atomic64_inc_and_test(v: *mut atomic64_t) -> bool {
    let c: u8;
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "incq ({counter})"),
            "sete {c}",
            counter = in(reg) &raw mut (*v).counter,
            c = out(reg_byte) c,
            options(att_syntax, nostack)
        )
    }
    c != 0
}

#[inline(always)]
pub unsafe fn arch_atomic64_add_negative(i: s64, v: *mut atomic64_t) -> bool {
    let c: u8;
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "addq {i:r}, ({counter})"),
            "sets {c}",
            i = in(reg) i,
            counter = in(reg) &raw mut (*v).counter,
            c = out(reg_byte) c,
            options(att_syntax, nostack)
        )
    }
    c != 0
}

#[inline(always)]
pub unsafe fn arch_atomic64_add_return(i: s64, v: *mut atomic64_t) -> s64 {
    i.wrapping_add(unsafe { xadd!(&raw mut (*v).counter, i) })
}

#[inline(always)]
pub unsafe fn arch_atomic64_sub_return(i: s64, v: *mut atomic64_t) -> s64 {
    unsafe { arch_atomic64_add_return(i.wrapping_neg(), v) }
}

#[inline(always)]
pub unsafe fn arch_atomic64_fetch_add(i: s64, v: *mut atomic64_t) -> s64 {
    unsafe { xadd!(&raw mut (*v).counter, i) }
}

#[inline(always)]
pub unsafe fn arch_atomic64_fetch_sub(i: s64, v: *mut atomic64_t) -> s64 {
    unsafe { arch_atomic64_fetch_add(i.wrapping_neg(), v) }
}

#[inline(always)]
pub unsafe fn arch_atomic64_cmpxchg(v: *mut atomic64_t, old: s64, r#new: s64) -> s64 {
    unsafe { arch_cmpxchg!(&raw mut (*v).counter, old, r#new) }
}

#[inline(always)]
pub unsafe fn arch_atomic64_try_cmpxchg(v: *mut atomic64_t, old: *mut s64, r#new: s64) -> bool {
    unsafe { arch_try_cmpxchg!(&raw mut (*v).counter, old, r#new) }
}

#[inline(always)]
pub unsafe fn arch_atomic64_xchg(v: *mut atomic64_t, r#new: s64) -> s64 {
    unsafe { arch_xchg!(&raw mut (*v).counter, r#new) }
}

#[inline(always)]
pub unsafe fn arch_atomic64_and(i: s64, v: *mut atomic64_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "andq {i:r}, ({counter})"),
            i = in(reg) i,
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic64_fetch_and(i: s64, v: *mut atomic64_t) -> s64 {
    let mut val = unsafe { arch_atomic64_read(v) };
    while !unsafe { arch_atomic64_try_cmpxchg(v, &raw mut val, val & i) } {}
    val
}

#[inline(always)]
pub unsafe fn arch_atomic64_or(i: s64, v: *mut atomic64_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "orq {i:r}, ({counter})"),
            i = in(reg) i,
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic64_fetch_or(i: s64, v: *mut atomic64_t) -> s64 {
    let mut val = unsafe { arch_atomic64_read(v) };
    while !unsafe { arch_atomic64_try_cmpxchg(v, &raw mut val, val | i) } {}
    val
}

#[inline(always)]
pub unsafe fn arch_atomic64_xor(i: s64, v: *mut atomic64_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "xorq {i:r}, ({counter})"),
            i = in(reg) i,
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic64_fetch_xor(i: s64, v: *mut atomic64_t) -> s64 {
    let mut val = unsafe { arch_atomic64_read(v) };
    while !unsafe { arch_atomic64_try_cmpxchg(v, &raw mut val, val ^ i) } {}
    val
}
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
