/* SPDX-License-Identifier: GPL-2.0 */
/* Atomic operations usable in machine independent code */

/* Depends on: linux/types.h, asm/atomic.h, asm/barrier.h */

/*
 * Relaxed variants of xchg, cmpxchg and some atomic operations.
 *
 * We support four variants:
 *
 * - Fully ordered: The default implementation, no suffix required.
 * - Acquire: Provides ACQUIRE semantics, _acquire suffix.
 * - Release: Provides RELEASE semantics, _release suffix.
 * - Relaxed: No ordering guarantees, _relaxed suffix.
 *
 * For compound atomics performing both a load and a store, ACQUIRE
 * semantics apply only to the load and RELEASE semantics only to the
 * store portion of the operation. Note that a failed cmpxchg_acquire
 * does -not- imply any memory ordering constraints.
 *
 * See Documentation/memory-barriers.txt for ACQUIRE/RELEASE definitions.
 */

#[macro_export]
macro_rules! atomic_cond_read_acquire {
    ($v:expr, |$val:ident| $c:expr) => {
        smp_cond_load_acquire!(&raw mut (*$v).counter, |$val| $c)
    };
}

#[macro_export]
macro_rules! atomic_cond_read_relaxed {
    ($v:expr, |$val:ident| $c:expr) => {
        smp_cond_load_relaxed!(&raw mut (*$v).counter, |$val| $c)
    };
}

#[macro_export]
macro_rules! atomic64_cond_read_acquire {
    ($v:expr, |$val:ident| $c:expr) => {
        smp_cond_load_acquire!(&raw mut (*$v).counter, |$val| $c)
    };
}

#[macro_export]
macro_rules! atomic64_cond_read_relaxed {
    ($v:expr, |$val:ident| $c:expr) => {
        smp_cond_load_relaxed!(&raw mut (*$v).counter, |$val| $c)
    };
}

/*
 * The idea here is to build acquire/release variants by adding explicit
 * barriers on top of the relaxed variant. In the case where the relaxed
 * variant is already fully ordered, no additional barriers are needed.
 *
 * If an architecture overrides __atomic_acquire_fence() it will probably
 * want to define smp_mb__after_spinlock().
 */
#[cfg(not(any(CONFIG_RISCV, CONFIG_PPC)))]
#[inline(always)]
pub fn __atomic_acquire_fence() {
    smp_mb__after_atomic()
}

#[cfg(not(any(CONFIG_RISCV, CONFIG_PPC)))]
#[inline(always)]
pub fn __atomic_release_fence() {
    smp_mb__before_atomic()
}

#[inline(always)]
pub fn __atomic_pre_full_fence() {
    smp_mb__before_atomic()
}

#[inline(always)]
pub fn __atomic_post_full_fence() {
    smp_mb__after_atomic()
}

/*
 * The C macros paste "_relaxed" onto @op; the Rust callers pass the relaxed
 * operation (a macro, e.g. arch_xchg_relaxed) directly.
 */
#[macro_export]
macro_rules! __atomic_op_acquire {
    ($op_relaxed:ident, $($args:tt)*) => {{
        let __ret = $op_relaxed!($($args)*);
        __atomic_acquire_fence();
        __ret
    }};
}

#[macro_export]
macro_rules! __atomic_op_release {
    ($op_relaxed:ident, $($args:tt)*) => {{
        __atomic_release_fence();
        $op_relaxed!($($args)*)
    }};
}

#[macro_export]
macro_rules! __atomic_op_fence {
    ($op_relaxed:ident, $($args:tt)*) => {{
        let __ret;
        __atomic_pre_full_fence();
        __ret = $op_relaxed!($($args)*);
        __atomic_post_full_fence();
        __ret
    }};
}

/* Depends on: linux/atomic/atomic-arch-fallback.h, linux/atomic/atomic-long.h,
 * linux/atomic/atomic-instrumented.h */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
