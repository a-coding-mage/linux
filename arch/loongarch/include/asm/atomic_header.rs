/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Atomic operations.
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// C dependencies supplied by the surrounding kernel translation:
// linux/types.h, asm/barrier.h, asm/cmpxchg.h, and the selected atomic backend.

#[cfg(target_pointer_width = "32")]
pub const __LL: &str = "ll.w\t";
#[cfg(target_pointer_width = "32")]
pub const __SC: &str = "sc.w\t";
#[cfg(target_pointer_width = "32")]
pub const __AMADD: &str = "amadd.w\t";
#[cfg(target_pointer_width = "32")]
pub const __AMOR: &str = "amor.w\t\t";
#[cfg(target_pointer_width = "32")]
pub const __AMAND_DB: &str = "amand_db.w\t";
#[cfg(target_pointer_width = "32")]
pub const __AMOR_DB: &str = "amor_db.w\t";
#[cfg(target_pointer_width = "32")]
pub const __AMXOR_DB: &str = "amxor_db.w\t";
#[cfg(target_pointer_width = "64")]
pub const __LL: &str = "ll.d\t";
#[cfg(target_pointer_width = "64")]
pub const __SC: &str = "sc.d\t";
#[cfg(target_pointer_width = "64")]
pub const __AMADD: &str = "amadd.d\t";
#[cfg(target_pointer_width = "64")]
pub const __AMOR: &str = "amor.d\t\t";
#[cfg(target_pointer_width = "64")]
pub const __AMAND_DB: &str = "amand_db.d\t";
#[cfg(target_pointer_width = "64")]
pub const __AMOR_DB: &str = "amor_db.d\t";
#[cfg(target_pointer_width = "64")]
pub const __AMXOR_DB: &str = "amxor_db.d\t";

// ATOMIC_INIT(i) expands to { (i) } in C.

#[inline]
pub unsafe fn arch_atomic_fetch_add_unless(v: *mut atomic_t, a: i32, u: i32) -> i32 {
    let mut prev: i32;
    let mut rc: i32;
    core::arch::asm!(
        "0: ll.w {p}, [{c}]",
        "beq {p}, {u}, 1f",
        "add.w {rc}, {p}, {a}",
        "sc.w {rc}, [{c}]",
        "beqz {rc}, 0b",
        "b 2f",
        "1:",
        "2:",
        p = out(reg) prev, rc = out(reg) rc,
        c = in(reg) core::ptr::addr_of_mut!((*v).counter),
        a = in(reg) a, u = in(reg) u,
        options(nostack)
    );
    prev
}

#[inline]
pub unsafe fn arch_atomic_sub_if_positive(i: i32, v: *mut atomic_t) -> i32 {
    let mut result: i32;
    let mut temp: i32;
    core::arch::asm!(
        "1: ll.w {temp}, [{counter}]",
        "sub.w {result}, {temp}, {i}",
        "move {temp}, {result}",
        "bltz {result}, 2f",
        "sc.w {temp}, [{counter}]",
        "beqz {temp}, 1b",
        "2:",
        result = out(reg) result, temp = out(reg) temp,
        counter = in(reg) core::ptr::addr_of_mut!((*v).counter),
        i = in(reg) i,
        options(nostack)
    );
    result
}

#[inline]
pub unsafe fn arch_atomic_dec_if_positive(v: *mut atomic_t) -> i32 {
    arch_atomic_sub_if_positive(1, v)
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn arch_atomic64_fetch_add_unless(v: *mut atomic64_t, a: i64, u: i64) -> i64 {
    let mut prev: i64;
    let mut rc: i64;
    core::arch::asm!(
        "0: ll.d {p}, [{c}]",
        "beq {p}, {u}, 1f",
        "add.d {rc}, {p}, {a}",
        "sc.d {rc}, [{c}]",
        "beqz {rc}, 0b",
        "b 2f",
        "1:", "2:",
        p = out(reg) prev, rc = out(reg) rc,
        c = in(reg) core::ptr::addr_of_mut!((*v).counter),
        a = in(reg) a, u = in(reg) u,
        options(nostack)
    );
    prev
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn arch_atomic64_sub_if_positive(i: i64, v: *mut atomic64_t) -> i64 {
    let mut result: i64;
    let mut temp: i64;
    core::arch::asm!(
        "1: ll.d {temp}, [{counter}]",
        "sub.d {result}, {temp}, {i}",
        "move {temp}, {result}",
        "bltz {result}, 2f",
        "sc.d {temp}, [{counter}]",
        "beqz {temp}, 1b",
        "2:",
        result = out(reg) result, temp = out(reg) temp,
        counter = in(reg) core::ptr::addr_of_mut!((*v).counter),
        i = in(reg) i,
        options(nostack)
    );
    result
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn arch_atomic64_dec_if_positive(v: *mut atomic64_t) -> i64 {
    arch_atomic64_sub_if_positive(1, v)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
