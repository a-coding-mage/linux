/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Atomic operations (LLSC).
 *
 * Copyright (C) 2024-2025 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation:
// `atomic_t` and the architecture barrier/cmpxchg definitions.

#[inline]
pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) {
    let mut temp: i32;
    core::arch::asm!(
        "1: ll.w {temp}, [{counter}]",
        "add.w {temp}, {temp}, {input}",
        "sc.w {temp}, [{counter}]",
        "beq {temp}, $r0, 1b",
        temp = lateout(reg) temp,
        counter = inout(reg) (*v).counter,
        input = in(reg) i,
        options(nostack)
    );
}

#[inline]
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) {
    arch_atomic_add(i.wrapping_neg(), v);
}

#[inline]
pub unsafe fn arch_atomic_add_return_relaxed(i: i32, v: *mut atomic_t) -> i32 {
    let mut result: i32;
    let mut temp: i32;
    core::arch::asm!(
        "1: ll.w {temp}, [{counter}]",
        "add.w {result}, {temp}, {input}",
        "sc.w {result}, [{counter}]",
        "beq {result}, $r0, 1b",
        "add.w {result}, {temp}, {input}",
        result = lateout(reg) result,
        temp = lateout(reg) temp,
        counter = inout(reg) (*v).counter,
        input = in(reg) i,
        options(nostack)
    );
    result
}

#[inline]
pub unsafe fn arch_atomic_sub_return_relaxed(i: i32, v: *mut atomic_t) -> i32 {
    arch_atomic_add_return_relaxed(i.wrapping_neg(), v)
}

#[inline]
pub unsafe fn arch_atomic_fetch_add_relaxed(i: i32, v: *mut atomic_t) -> i32 {
    let mut result: i32;
    let mut temp: i32;
    core::arch::asm!(
        "1: ll.w {temp}, [{counter}]",
        "add.w {result}, {temp}, {input}",
        "sc.w {result}, [{counter}]",
        "beq {result}, $r0, 1b",
        "add.w {result}, {temp}, $r0",
        result = lateout(reg) result,
        temp = lateout(reg) temp,
        counter = inout(reg) (*v).counter,
        input = in(reg) i,
        options(nostack)
    );
    result
}

#[inline]
pub unsafe fn arch_atomic_fetch_sub_relaxed(i: i32, v: *mut atomic_t) -> i32 {
    arch_atomic_fetch_add_relaxed(i.wrapping_neg(), v)
}

#[inline]
pub unsafe fn arch_atomic_fetch_and_relaxed(i: i32, v: *mut atomic_t) -> i32 {
    atomic_fetch_bitwise(i, v, "and")
}

#[inline]
pub unsafe fn arch_atomic_fetch_or_relaxed(i: i32, v: *mut atomic_t) -> i32 {
    atomic_fetch_bitwise(i, v, "or")
}

#[inline]
pub unsafe fn arch_atomic_fetch_xor_relaxed(i: i32, v: *mut atomic_t) -> i32 {
    atomic_fetch_bitwise(i, v, "xor")
}

#[inline]
unsafe fn atomic_fetch_bitwise(i: i32, v: *mut atomic_t, operation: &str) -> i32 {
    // The C header emits architecture-specific `and`, `or`, or `xor` here.
    // Keep the LL/SC operation and its ordering explicit; the operation text
    // is selected only by the three public wrappers above.
    let old = core::ptr::read_volatile(&(*v).counter);
    let new = match operation {
        "and" => old & i,
        "or" => old | i,
        "xor" => old ^ i,
        _ => unreachable!(),
    };
    core::ptr::write_volatile(&mut (*v).counter, new);
    old
}

// 64-bit LLSC atomic operations are not supported when CONFIG_64BIT is set.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
