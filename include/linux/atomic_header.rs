/* SPDX-License-Identifier: GPL-2.0 */
/* Atomic operations usable in machine independent code */

/* Dependencies supplied by the Linux types, architecture, and atomic headers. */

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
    ($v:expr, $c:expr) => {
        smp_cond_load_acquire(unsafe { &(*($v as *const _)).counter }, $c)
    };
}

#[macro_export]
macro_rules! atomic_cond_read_relaxed {
    ($v:expr, $c:expr) => {
        smp_cond_load_relaxed(unsafe { &(*($v as *const _)).counter }, $c)
    };
}

#[macro_export]
macro_rules! atomic64_cond_read_acquire {
    ($v:expr, $c:expr) => {
        smp_cond_load_acquire(unsafe { &(*($v as *const _)).counter }, $c)
    };
}

#[macro_export]
macro_rules! atomic64_cond_read_relaxed {
    ($v:expr, $c:expr) => {
        smp_cond_load_relaxed(unsafe { &(*($v as *const _)).counter }, $c)
    };
}

/* Architecture overrides may provide these fences. */
#[macro_export]
macro_rules! __atomic_acquire_fence {
    () => { smp_mb__after_atomic!() };
}

#[macro_export]
macro_rules! __atomic_release_fence {
    () => { smp_mb__before_atomic!() };
}

#[macro_export]
macro_rules! __atomic_pre_full_fence {
    () => { smp_mb__before_atomic!() };
}

#[macro_export]
macro_rules! __atomic_post_full_fence {
    () => { smp_mb__after_atomic!() };
}

#[macro_export]
macro_rules! __atomic_op_acquire {
    ($relaxed:ident, $($args:expr),* $(,)?) => {{
        let __ret = $relaxed($($args),*);
        __atomic_acquire_fence!();
        __ret
    }};
}

#[macro_export]
macro_rules! __atomic_op_release {
    ($relaxed:ident, $($args:expr),* $(,)?) => {{
        __atomic_release_fence!();
        $relaxed($($args),*)
    }};
}

#[macro_export]
macro_rules! __atomic_op_fence {
    ($relaxed:ident, $($args:expr),* $(,)?) => {{
        __atomic_pre_full_fence!();
        let __ret = $relaxed($($args),*);
        __atomic_post_full_fence!();
        __ret
    }};
}

/* Declarations supplied by the architecture and Linux atomic headers. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
