/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Atomic operations (AMO).
 *
 * Copyright (C) 2020-2025 Loongson Technology Corporation Limited
 */

// Translated from the C header. `atomic_t` and `atomic64_t` are supplied by
// the surrounding kernel bindings.

macro_rules! atomic_op {
    ($op:ident, $i:expr, $asm_op:tt) => {
        paste::paste! {
            #[inline]
            pub unsafe fn [<arch_atomic_ $op>](i: i32, v: *mut atomic_t) {
                core::arch::asm!(
                    concat!("am", stringify!($asm_op), ".w $zero, {i}, {counter}\n"),
                    counter = inout(reg) (*v).counter => _,
                    i = in(reg) $i,
                    options(nostack)
                );
            }
        }
    };
}

macro_rules! atomic_op_return {
    ($op:ident, $i:expr, $asm_op:tt, $c_op:tt, $mb:literal, $suffix:ident) => {
        paste::paste! {
            #[inline]
            pub unsafe fn [<arch_atomic_ $op _return $suffix>](i: i32, v: *mut atomic_t) -> i32 {
                let result: i32;
                core::arch::asm!(
                    concat!("am", stringify!($asm_op), $mb, ".w {result}, {i}, {counter}\n"),
                    counter = inout(reg) (*v).counter => _,
                    result = lateout(reg) result,
                    i = in(reg) $i,
                    options(nostack)
                );
                result $c_op $i
            }
        }
    };
}

macro_rules! atomic_fetch_op {
    ($op:ident, $i:expr, $asm_op:tt, $mb:literal, $suffix:ident) => {
        paste::paste! {
            #[inline]
            pub unsafe fn [<arch_atomic_fetch_ $op $suffix>](i: i32, v: *mut atomic_t) -> i32 {
                let result: i32;
                core::arch::asm!(
                    concat!("am", stringify!($asm_op), $mb, ".w {result}, {i}, {counter}\n"),
                    counter = inout(reg) (*v).counter => _,
                    result = lateout(reg) result,
                    i = in(reg) $i,
                    options(nostack)
                );
                result
            }
        }
    };
}

macro_rules! atomic_ops {
    ($op:ident, $i:expr, $asm_op:tt, $c_op:tt) => {
        atomic_op!($op, $i, $asm_op);
        atomic_op_return!($op, $i, $asm_op, $c_op, "_db", );
        atomic_op_return!($op, $i, $asm_op, $c_op, "", _relaxed);
        atomic_fetch_op!($op, $i, $asm_op, "_db", );
        atomic_fetch_op!($op, $i, $asm_op, "", _relaxed);
    };
}

atomic_ops!(add, i, add, +);
atomic_ops!(sub, -i, add, +);

atomic_op!(and, i, and);
atomic_op!(or, i, or);
atomic_op!(xor, i, xor);

#[cfg(target_pointer_width = "64")]
mod atomic64 {
    macro_rules! atomic64_op {
        ($op:ident, $i:expr, $asm_op:tt) => {
            paste::paste! {
                #[inline]
                pub unsafe fn [<arch_atomic64_ $op>](i: i64, v: *mut atomic64_t) {
                    core::arch::asm!(concat!("am", stringify!($asm_op), ".d $zero, {i}, {counter}\n"), counter = inout(reg) (*v).counter => _, i = in(reg) $i, options(nostack));
                }
            }
        };
    }

    macro_rules! atomic64_fetch_op {
        ($op:ident, $i:expr, $asm_op:tt, $mb:literal, $suffix:ident) => {
            paste::paste! {
                #[inline]
                pub unsafe fn [<arch_atomic64_fetch_ $op $suffix>](i: i64, v: *mut atomic64_t) -> i64 {
                    let result: i64;
                    core::arch::asm!(concat!("am", stringify!($asm_op), $mb, ".d {result}, {i}, {counter}\n"), counter = inout(reg) (*v).counter => _, result = lateout(reg) result, i = in(reg) $i, options(nostack));
                    result
                }
            }
        };
    }

    macro_rules! atomic64_op_return {
        ($op:ident, $i:expr, $asm_op:tt, $c_op:tt, $mb:literal, $suffix:ident) => {
            paste::paste! {
                #[inline]
                pub unsafe fn [<arch_atomic64_ $op _return $suffix>](i: i64, v: *mut atomic64_t) -> i64 {
                    let result: i64;
                    core::arch::asm!(concat!("am", stringify!($asm_op), $mb, ".d {result}, {i}, {counter}\n"), counter = inout(reg) (*v).counter => _, result = lateout(reg) result, i = in(reg) $i, options(nostack));
                    result $c_op $i
                }
            }
        };
    }

    atomic64_op!(add, i, add);
    atomic64_op!(sub, -i, add);
    atomic64_op_return!(add, i, add, +, "_db", );
    atomic64_op_return!(add, i, add, +, "", _relaxed);
    atomic64_op_return!(sub, -i, add, +, "_db", );
    atomic64_op_return!(sub, -i, add, +, "", _relaxed);
    atomic64_op!(and, i, and);
    atomic64_op!(or, i, or);
    atomic64_op!(xor, i, xor);
    atomic64_fetch_op!(add, i, add, "_db", );
    atomic64_fetch_op!(add, i, add, "", _relaxed);
    atomic64_fetch_op!(sub, -i, add, "_db", );
    atomic64_fetch_op!(sub, -i, add, "", _relaxed);
    atomic64_fetch_op!(and, i, and, "_db", );
    atomic64_fetch_op!(and, i, and, "", _relaxed);
    atomic64_fetch_op!(or, i, or, "_db", );
    atomic64_fetch_op!(or, i, or, "", _relaxed);
    atomic64_fetch_op!(xor, i, xor, "_db", );
    atomic64_fetch_op!(xor, i, xor, "", _relaxed);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
