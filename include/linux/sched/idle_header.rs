/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/sched.h.

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum cpu_idle_type {
    __CPU_NOT_IDLE = 0,
    CPU_IDLE,
    CPU_NEWLY_IDLE,
    CPU_MAX_IDLE_TYPES,
}

extern "C" {
    pub fn wake_up_if_idle(cpu: ::core::ffi::c_int);
}

/*
 * Idle thread specific functions to determine the need_resched
 * polling state.
 *
 * The following items are selected by the source-level TIF_POLLING_NRFLAG
 * and _ASM_GENERIC_BITOPS_INSTRUMENTED_ATOMIC_H build conditions.
 */

#[cfg(feature = "TIF_POLLING_NRFLAG")]
#[inline(always)]
pub unsafe fn __current_set_polling() {
    #[cfg(feature = "_ASM_GENERIC_BITOPS_INSTRUMENTED_ATOMIC_H")]
    {
        arch_set_bit(
            TIF_POLLING_NRFLAG,
            (&mut current_thread_info().flags as *mut _).cast::<::core::ffi::c_ulong>(),
        );
    }
    #[cfg(not(feature = "_ASM_GENERIC_BITOPS_INSTRUMENTED_ATOMIC_H"))]
    {
        set_bit(
            TIF_POLLING_NRFLAG,
            (&mut current_thread_info().flags as *mut _).cast::<::core::ffi::c_ulong>(),
        );
    }
}

#[cfg(feature = "TIF_POLLING_NRFLAG")]
#[inline(always)]
pub unsafe fn __current_clr_polling() {
    #[cfg(feature = "_ASM_GENERIC_BITOPS_INSTRUMENTED_ATOMIC_H")]
    {
        arch_clear_bit(
            TIF_POLLING_NRFLAG,
            (&mut current_thread_info().flags as *mut _).cast::<::core::ffi::c_ulong>(),
        );
    }
    #[cfg(not(feature = "_ASM_GENERIC_BITOPS_INSTRUMENTED_ATOMIC_H"))]
    {
        clear_bit(
            TIF_POLLING_NRFLAG,
            (&mut current_thread_info().flags as *mut _).cast::<::core::ffi::c_ulong>(),
        );
    }
}

#[cfg(feature = "TIF_POLLING_NRFLAG")]
#[inline(always)]
pub unsafe fn current_set_polling_and_test() -> bool {
    __current_set_polling();
    /* Polling state must be visible before NEED_RESCHED is tested. */
    smp_mb__after_atomic();
    unlikely(tif_need_resched())
}

#[cfg(feature = "TIF_POLLING_NRFLAG")]
#[inline(always)]
pub unsafe fn current_clr_polling_and_test() -> bool {
    __current_clr_polling();
    /* Polling state must be visible before NEED_RESCHED is tested. */
    smp_mb__after_atomic();
    unlikely(tif_need_resched())
}

#[cfg(feature = "TIF_POLLING_NRFLAG")]
#[inline(always)]
pub unsafe fn current_clr_polling() {
    __current_clr_polling();
    /* paired with resched_curr() */
    smp_mb__after_atomic();
    preempt_fold_need_resched();
}

#[cfg(not(feature = "TIF_POLLING_NRFLAG"))]
#[inline(always)]
pub unsafe fn __current_set_polling() {}

#[cfg(not(feature = "TIF_POLLING_NRFLAG"))]
#[inline(always)]
pub unsafe fn __current_clr_polling() {}

#[cfg(not(feature = "TIF_POLLING_NRFLAG"))]
#[inline(always)]
pub unsafe fn current_set_polling_and_test() -> bool {
    unlikely(tif_need_resched())
}

#[cfg(not(feature = "TIF_POLLING_NRFLAG"))]
#[inline(always)]
pub unsafe fn current_clr_polling_and_test() -> bool {
    unlikely(tif_need_resched())
}

#[cfg(not(feature = "TIF_POLLING_NRFLAG"))]
#[inline(always)]
pub unsafe fn current_clr_polling() {
    __current_clr_polling();
    smp_mb(); /* paired with resched_curr() */
    preempt_fold_need_resched();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
