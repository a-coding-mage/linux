/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Atomic operations for the Hexagon architecture
 *
 * Copyright (c) 2010-2013, The Linux Foundation. All rights reserved.
 */

// C dependencies supplied externally: linux/types.h, asm/cmpxchg.h,
// and asm/barrier.h.  `atomic_t` and `READ_ONCE` are therefore referenced
// but not defined here.

/* Normal writes in our arch don't clear lock reservations. */
#[inline]
pub unsafe fn arch_atomic_set(v: *mut atomic_t, new: i32) {
    core::arch::asm!(
        "1: r6 = memw_locked({0})",
        "memw_locked({0}, p0) = {1}",
        "if (!P0) jump 1b",
        in(reg) &mut (*v).counter,
        in(reg) new,
        out("r6") _,
        out("p0") _,
        options(nostack)
    );
}

#[inline]
pub unsafe fn arch_atomic_set_release(v: *mut atomic_t, i: i32) { arch_atomic_set(v, i) }

#[inline]
pub unsafe fn arch_atomic_read(v: *const atomic_t) -> i32 {
    // Equivalent to READ_ONCE((v)->counter).
    core::ptr::read_volatile(&(*v).counter)
}

macro_rules! atomic_op {
    ($name:ident, $op:literal) => {
        #[inline]
        pub unsafe fn $name(i: i32, v: *mut atomic_t) {
            let mut output: i32;
            core::arch::asm!(
                "1: {0} = memw_locked({1})",
                concat!("{0} = ", $op, "({0},{2})"),
                "memw_locked({1},P3)={0}",
                "if (!P3) jump 1b",
                lateout(reg) output,
                in(reg) &mut (*v).counter,
                in(reg) i,
                out("p3") _,
                options(nostack)
            );
        }
    };
}

macro_rules! atomic_op_return {
    ($name:ident, $op:literal) => {
        #[inline]
        pub unsafe fn $name(i: i32, v: *mut atomic_t) -> i32 {
            let mut output: i32;
            core::arch::asm!(
                "1: {0} = memw_locked({1})",
                concat!("{0} = ", $op, "({0},{2})"),
                "memw_locked({1},P3)={0}",
                "if (!P3) jump 1b",
                lateout(reg) output,
                in(reg) &mut (*v).counter,
                in(reg) i,
                out("p3") _,
                options(nostack)
            );
            output
        }
    };
}

macro_rules! atomic_fetch_op {
    ($name:ident, $op:literal) => {
        #[inline]
        pub unsafe fn $name(i: i32, v: *mut atomic_t) -> i32 {
            let mut output: i32;
            let mut val: i32;
            core::arch::asm!(
                "1: {0} = memw_locked({2})",
                concat!("{1} = ", $op, "({0},{3})"),
                "memw_locked({2},P3)={1}",
                "if (!P3) jump 1b",
                lateout(reg) output,
                lateout(reg) val,
                in(reg) &mut (*v).counter,
                in(reg) i,
                out("p3") _,
                options(nostack)
            );
            output
        }
    };
}

atomic_op!(arch_atomic_add, "add");
atomic_op_return!(arch_atomic_add_return, "add");
atomic_fetch_op!(arch_atomic_fetch_add, "add");
atomic_op!(arch_atomic_sub, "sub");
atomic_op_return!(arch_atomic_sub_return, "sub");
atomic_fetch_op!(arch_atomic_fetch_sub, "sub");
atomic_op!(arch_atomic_and, "and");
atomic_fetch_op!(arch_atomic_fetch_and, "and");
atomic_op!(arch_atomic_or, "or");
atomic_fetch_op!(arch_atomic_fetch_or, "or");
atomic_op!(arch_atomic_xor, "xor");
atomic_fetch_op!(arch_atomic_fetch_xor, "xor");

#[inline]
pub unsafe fn arch_atomic_fetch_add_unless(v: *mut atomic_t, a: i32, u: i32) -> i32 {
    let mut oldval: i32;
    let mut tmp: i32;
    core::arch::asm!(
        "1: {0} = memw_locked({2})",
        "{{ p3 = cmp.eq({0}, {4}); if (p3.new) jump:nt 2f; {1} = add({0}, {3}); }}",
        "memw_locked({2}, p3) = {1}",
        "{{ if (!p3) jump 1b; }}",
        "2:",
        lateout(reg) oldval,
        lateout(reg) tmp,
        in(reg) v,
        in(reg) a,
        in(reg) u,
        out("p3") _,
        options(nostack)
    );
    oldval
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
