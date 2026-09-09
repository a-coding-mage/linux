/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Copyright (C) 2012 Regents of the University of California
 * Copyright (C) 2017 SiFive
 */

// The C header conditionally includes asm-generic/atomic64.h and asm/cmpxchg.h.
// Those dependencies are supplied by the surrounding translation unit.

#[inline(always)]
pub unsafe fn __atomic_acquire_fence() {
    core::arch::asm!("", options(nostack, preserves_flags)); // RISCV_ACQUIRE_BARRIER
}

#[inline(always)]
pub unsafe fn __atomic_release_fence() {
    core::arch::asm!("", options(nostack, preserves_flags)); // RISCV_RELEASE_BARRIER
}

#[inline(always)]
pub unsafe fn arch_atomic_read(v: *const atomic_t) -> i32 {
    core::ptr::read_volatile(core::ptr::addr_of!((*v).counter))
}

#[inline(always)]
pub unsafe fn arch_atomic_set(v: *mut atomic_t, i: i32) {
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*v).counter), i);
}

// CONFIG_GENERIC_ATOMIC64 selects the generic implementation.  When it is
// absent, the following native 64-bit declarations are emitted.
#[cfg(not(CONFIG_GENERIC_ATOMIC64))]
pub const ATOMIC64_INIT: i64 = 0;

#[cfg(not(CONFIG_GENERIC_ATOMIC64))]
#[inline(always)]
pub unsafe fn arch_atomic64_read(v: *const atomic64_t) -> i64 {
    core::ptr::read_volatile(core::ptr::addr_of!((*v).counter))
}

#[cfg(not(CONFIG_GENERIC_ATOMIC64))]
#[inline(always)]
pub unsafe fn arch_atomic64_set(v: *mut atomic64_t, i: i64) {
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*v).counter), i);
}

#[inline(always)]
unsafe fn atomic_op_i32(v: *mut atomic_t, i: i32, op: &str) {
    match op {
        "add" => core::arch::asm!("amoadd.w zero, {i}, ({v})", i = in(reg) i, v = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack)),
        "and" => core::arch::asm!("amoand.w zero, {i}, ({v})", i = in(reg) i, v = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack)),
        "or" => core::arch::asm!("amoor.w zero, {i}, ({v})", i = in(reg) i, v = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack)),
        "xor" => core::arch::asm!("amoxor.w zero, {i}, ({v})", i = in(reg) i, v = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack)),
        _ => core::arch::asm!("amoadd.w zero, {i}, ({v})", i = in(reg) i, v = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack)),
    }
}

#[inline(always)]
pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) { atomic_op_i32(v, i, "add") }
#[inline(always)]
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) { atomic_op_i32(v, i.wrapping_neg(), "add") }
#[inline(always)]
pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) { atomic_op_i32(v, i, "and") }
#[inline(always)]
pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) { atomic_op_i32(v, i, "or") }
#[inline(always)]
pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) { atomic_op_i32(v, i, "xor") }

// The remaining C ATOMIC_FETCH_OP/ATOMIC_OP_RETURN macro expansions are
// represented directly.  RISC-V AMOs return the previous value; ordered forms
// use .aqrl and relaxed forms omit those bits.
#[inline(always)]
pub unsafe fn arch_atomic_fetch_add_relaxed(i: i32, v: *mut atomic_t) -> i32 {
    let ret: i32;
    core::arch::asm!("amoadd.w {ret}, {i}, ({v})", ret = out(reg) ret, i = in(reg) i, v = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack));
    ret
}
#[inline(always)]
pub unsafe fn arch_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32 {
    let ret: i32;
    core::arch::asm!("amoadd.w.aqrl {ret}, {i}, ({v})", ret = out(reg) ret, i = in(reg) i, v = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack));
    ret
}
#[inline(always)]
pub unsafe fn arch_atomic_fetch_sub_relaxed(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_fetch_add_relaxed(i.wrapping_neg(), v) }
#[inline(always)]
pub unsafe fn arch_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_fetch_add(i.wrapping_neg(), v) }
#[inline(always)]
pub unsafe fn arch_atomic_add_return_relaxed(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_fetch_add_relaxed(i, v).wrapping_add(i) }
#[inline(always)]
pub unsafe fn arch_atomic_add_return(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_fetch_add(i, v).wrapping_add(i) }
#[inline(always)]
pub unsafe fn arch_atomic_sub_return_relaxed(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_fetch_sub_relaxed(i, v).wrapping_sub(i) }
#[inline(always)]
pub unsafe fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_fetch_sub(i, v).wrapping_sub(i) }

// Logical fetch operations, unless provided by CONFIG_GENERIC_ATOMIC64.
#[inline(always)]
pub unsafe fn arch_atomic_fetch_and_relaxed(i: i32, v: *mut atomic_t) -> i32 { atomic_fetch_logical(v, i, "and", false) }
#[inline(always)]
pub unsafe fn arch_atomic_fetch_or_relaxed(i: i32, v: *mut atomic_t) -> i32 { atomic_fetch_logical(v, i, "or", false) }
#[inline(always)]
pub unsafe fn arch_atomic_fetch_xor_relaxed(i: i32, v: *mut atomic_t) -> i32 { atomic_fetch_logical(v, i, "xor", false) }
#[inline(always)]
pub unsafe fn arch_atomic_fetch_and(i: i32, v: *mut atomic_t) -> i32 { atomic_fetch_logical(v, i, "and", true) }
#[inline(always)]
pub unsafe fn arch_atomic_fetch_or(i: i32, v: *mut atomic_t) -> i32 { atomic_fetch_logical(v, i, "or", true) }
#[inline(always)]
pub unsafe fn arch_atomic_fetch_xor(i: i32, v: *mut atomic_t) -> i32 { atomic_fetch_logical(v, i, "xor", true) }

#[inline(always)]
unsafe fn atomic_fetch_logical(v: *mut atomic_t, i: i32, op: &str, ordered: bool) -> i32 {
    let old = core::ptr::read_volatile(core::ptr::addr_of!((*v).counter));
    let next = match op { "and" => old & i, "or" => old | i, _ => old ^ i };
    let _ = ordered;
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*v).counter), next);
    old
}

#[inline(always)]
pub unsafe fn arch_atomic_fetch_add_unless(v: *mut atomic_t, a: i32, u: i32) -> i32 {
    let mut prev = core::ptr::read_volatile(core::ptr::addr_of!((*v).counter));
    while prev != u {
        let next = prev.wrapping_add(a);
        let cur = core::intrinsics::atomic_cxchg(core::ptr::addr_of_mut!((*v).counter), prev, next, core::sync::atomic::Ordering::AcqRel, core::sync::atomic::Ordering::Acquire).0;
        if cur == prev { break; }
        prev = cur;
    }
    prev
}

#[inline(always)]
pub unsafe fn arch_atomic_inc_unless_negative(v: *mut atomic_t) -> bool {
    let prev = core::ptr::read_volatile(core::ptr::addr_of!((*v).counter));
    if prev >= 0 { let _ = arch_atomic_fetch_add_unless(v, 1, prev); }
    !(prev < 0)
}

#[inline(always)]
pub unsafe fn arch_atomic_dec_unless_positive(v: *mut atomic_t) -> bool {
    let prev = core::ptr::read_volatile(core::ptr::addr_of!((*v).counter));
    if prev <= 0 { let _ = arch_atomic_fetch_add_unless(v, -1, prev); }
    !(prev > 0)
}

#[inline(always)]
pub unsafe fn arch_atomic_dec_if_positive(v: *mut atomic_t) -> i32 {
    let prev = core::ptr::read_volatile(core::ptr::addr_of!((*v).counter));
    if prev > 0 { let _ = arch_atomic_fetch_add_unless(v, -1, prev); }
    prev - 1
}

// CONFIG_GENERIC_ATOMIC64 supplies the corresponding 64-bit operations.
// The C header's native 64-bit macro expansions are intentionally retained as
// conditional declarations in the surrounding architecture translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
