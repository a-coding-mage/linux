/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from arch/x86/include/asm/atomic.h. */

/* Depends on: linux/compiler.h, linux/types.h, asm/alternative.h,
 * asm/cmpxchg.h, asm/rmwcc.h, asm/barrier.h */

/*
 * Atomic operations that C can't guarantee us.  Useful for
 * resource counting etc..
 *
 * GEN_UNARY_RMWcc()/GEN_BINARY_RMWcc() become the instruction followed by a
 * set<cc> of the condition flag.
 */

#[inline(always)]
pub unsafe fn arch_atomic_read(v: *const atomic_t) -> kernel::ffi::c_int {
    /*
     * Note for KASAN: we deliberately don't use READ_ONCE_NOCHECK() here,
     * it's non-inlined function that increases binary size and stack usage.
     */
    __READ_ONCE!((*v).counter)
}

#[inline(always)]
pub unsafe fn arch_atomic_set(v: *mut atomic_t, i: kernel::ffi::c_int) {
    __WRITE_ONCE!((*v).counter, i)
}

#[inline(always)]
pub unsafe fn arch_atomic_add(i: kernel::ffi::c_int, v: *mut atomic_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "addl {i:e}, ({counter})"),
            i = in(reg) i,
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic_sub(i: kernel::ffi::c_int, v: *mut atomic_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "subl {i:e}, ({counter})"),
            i = in(reg) i,
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic_sub_and_test(i: kernel::ffi::c_int, v: *mut atomic_t) -> bool {
    let c: u8;
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "subl {i:e}, ({counter})"),
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
pub unsafe fn arch_atomic_inc(v: *mut atomic_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "incl ({counter})"),
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic_dec(v: *mut atomic_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "decl ({counter})"),
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic_dec_and_test(v: *mut atomic_t) -> bool {
    let c: u8;
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "decl ({counter})"),
            "sete {c}",
            counter = in(reg) &raw mut (*v).counter,
            c = out(reg_byte) c,
            options(att_syntax, nostack)
        )
    }
    c != 0
}

#[inline(always)]
pub unsafe fn arch_atomic_inc_and_test(v: *mut atomic_t) -> bool {
    let c: u8;
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "incl ({counter})"),
            "sete {c}",
            counter = in(reg) &raw mut (*v).counter,
            c = out(reg_byte) c,
            options(att_syntax, nostack)
        )
    }
    c != 0
}

#[inline(always)]
pub unsafe fn arch_atomic_add_negative(i: kernel::ffi::c_int, v: *mut atomic_t) -> bool {
    let c: u8;
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "addl {i:e}, ({counter})"),
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
pub unsafe fn arch_atomic_add_return(i: kernel::ffi::c_int, v: *mut atomic_t) -> kernel::ffi::c_int {
    i.wrapping_add(unsafe { xadd!(&raw mut (*v).counter, i) })
}

#[inline(always)]
pub unsafe fn arch_atomic_sub_return(i: kernel::ffi::c_int, v: *mut atomic_t) -> kernel::ffi::c_int {
    unsafe { arch_atomic_add_return(i.wrapping_neg(), v) }
}

#[inline(always)]
pub unsafe fn arch_atomic_fetch_add(i: kernel::ffi::c_int, v: *mut atomic_t) -> kernel::ffi::c_int {
    unsafe { xadd!(&raw mut (*v).counter, i) }
}

#[inline(always)]
pub unsafe fn arch_atomic_fetch_sub(i: kernel::ffi::c_int, v: *mut atomic_t) -> kernel::ffi::c_int {
    unsafe { arch_atomic_fetch_add(i.wrapping_neg(), v) }
}

#[inline(always)]
pub unsafe fn arch_atomic_cmpxchg(v: *mut atomic_t, old: kernel::ffi::c_int, r#new: kernel::ffi::c_int) -> kernel::ffi::c_int {
    unsafe { arch_cmpxchg!(&raw mut (*v).counter, old, r#new) }
}

#[inline(always)]
pub unsafe fn arch_atomic_try_cmpxchg(v: *mut atomic_t, old: *mut kernel::ffi::c_int, r#new: kernel::ffi::c_int) -> bool {
    unsafe { arch_try_cmpxchg!(&raw mut (*v).counter, old, r#new) }
}

#[inline(always)]
pub unsafe fn arch_atomic_xchg(v: *mut atomic_t, r#new: kernel::ffi::c_int) -> kernel::ffi::c_int {
    unsafe { arch_xchg!(&raw mut (*v).counter, r#new) }
}

#[inline(always)]
pub unsafe fn arch_atomic_and(i: kernel::ffi::c_int, v: *mut atomic_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "andl {i:e}, ({counter})"),
            i = in(reg) i,
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic_fetch_and(i: kernel::ffi::c_int, v: *mut atomic_t) -> kernel::ffi::c_int {
    let mut val = unsafe { arch_atomic_read(v) };
    while !unsafe { arch_atomic_try_cmpxchg(v, &raw mut val, val & i) } {}
    val
}

#[inline(always)]
pub unsafe fn arch_atomic_or(i: kernel::ffi::c_int, v: *mut atomic_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "orl {i:e}, ({counter})"),
            i = in(reg) i,
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic_fetch_or(i: kernel::ffi::c_int, v: *mut atomic_t) -> kernel::ffi::c_int {
    let mut val = unsafe { arch_atomic_read(v) };
    while !unsafe { arch_atomic_try_cmpxchg(v, &raw mut val, val | i) } {}
    val
}

#[inline(always)]
pub unsafe fn arch_atomic_xor(i: kernel::ffi::c_int, v: *mut atomic_t) {
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "xorl {i:e}, ({counter})"),
            i = in(reg) i,
            counter = in(reg) &raw mut (*v).counter,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_atomic_fetch_xor(i: kernel::ffi::c_int, v: *mut atomic_t) -> kernel::ffi::c_int {
    let mut val = unsafe { arch_atomic_read(v) };
    while !unsafe { arch_atomic_try_cmpxchg(v, &raw mut val, val ^ i) } {}
    val
}
/* CONFIG_X86_32: Depends on: asm/atomic64_32.h; otherwise: asm/atomic64_64.h */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
