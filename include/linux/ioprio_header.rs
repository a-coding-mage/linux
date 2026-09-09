/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux scheduler, I/O context,
// and UAPI headers are referenced here but not implemented in this translation.

/* Default IO priority. */
pub const IOPRIO_DEFAULT: ::core::ffi::c_int =
    IOPRIO_PRIO_VALUE(IOPRIO_CLASS_NONE, 0);

/* Check that a priority value has a valid class. */
#[inline]
pub unsafe fn ioprio_valid(ioprio: u16) -> bool {
    let class: u16 = IOPRIO_PRIO_CLASS(ioprio);
    class > IOPRIO_CLASS_NONE && class <= IOPRIO_CLASS_IDLE
}

/*
 * If process has set io priority explicitly, use that. If not, convert
 * the CPU scheduler nice value to an IO priority.
 */
#[inline]
pub unsafe fn task_nice_ioprio(task: *mut task_struct) -> ::core::ffi::c_int {
    (task_nice(task) + 20) / 5
}

/*
 * This is for the case where the task hasn't asked for a specific IO class.
 * Check for idle and RT task process, and return the appropriate IO class.
 */
#[inline]
pub unsafe fn task_nice_ioclass(task: *mut task_struct) -> ::core::ffi::c_int {
    if (*task).policy == SCHED_IDLE {
        IOPRIO_CLASS_IDLE
    } else if rt_or_dl_task_policy(task) {
        IOPRIO_CLASS_RT
    } else {
        IOPRIO_CLASS_BE
    }
}

#[cfg(feature = "CONFIG_BLOCK")]
#[inline]
pub unsafe fn __get_task_ioprio(p: *mut task_struct) -> ::core::ffi::c_int {
    let ioc: *mut io_context = (*p).io_context;
    let mut prio: ::core::ffi::c_int;

    if ioc.is_null() {
        return IOPRIO_PRIO_VALUE(task_nice_ioclass(p), task_nice_ioprio(p));
    }

    if p != current {
        lockdep_assert_held(&(*p).alloc_lock);
    }

    prio = (*ioc).ioprio;
    if IOPRIO_PRIO_CLASS(prio) == IOPRIO_CLASS_NONE {
        prio = IOPRIO_PRIO_VALUE(task_nice_ioclass(p), task_nice_ioprio(p));
    }
    prio
}

#[cfg(not(feature = "CONFIG_BLOCK"))]
#[inline]
pub unsafe fn __get_task_ioprio(_p: *mut task_struct) -> ::core::ffi::c_int {
    IOPRIO_DEFAULT
}

#[inline]
pub unsafe fn get_current_ioprio() -> ::core::ffi::c_int {
    __get_task_ioprio(current)
}

extern "C" {
    pub fn set_task_ioprio(
        task: *mut task_struct,
        ioprio: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "CONFIG_BLOCK")]
    pub fn ioprio_check_cap(ioprio: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_BLOCK"))]
#[inline]
pub unsafe fn ioprio_check_cap(_ioprio: ::core::ffi::c_int) -> ::core::ffi::c_int {
    -ENOTBLK
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
