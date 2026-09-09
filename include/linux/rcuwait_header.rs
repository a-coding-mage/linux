/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/rcupdate.h, linux/sched/signal.h, and linux/types.h.

#[allow(non_snake_case)]
pub const __RCUWAIT_INITIALIZER: () = ();

#[inline]
pub unsafe fn rcuwait_init(w: *mut rcuwait) {
    (*w).task = core::ptr::null_mut();
}

/*
 * Note: this provides no serialization and, just as with waitqueues,
 * requires care to estimate as to whether or not the wait is active.
 */
#[inline]
pub unsafe fn rcuwait_active(w: *mut rcuwait) -> core::ffi::c_int {
    if rcu_access_pointer((*w).task).is_null() { 0 } else { 1 }
}

extern "C" {
    pub fn rcuwait_wake_up(w: *mut rcuwait) -> core::ffi::c_int;
}

/*
 * The caller is responsible for locking around rcuwait_wait_event(),
 * and [prepare_to/finish]_rcuwait() such that writes to @task are
 * properly serialized.
 */

#[inline]
pub unsafe fn prepare_to_rcuwait(w: *mut rcuwait) {
    rcu_assign_pointer((*w).task, current);
}

extern "C" {
    pub fn finish_rcuwait(w: *mut rcuwait);
}

/* The `rcuwait` type and the referenced kernel primitives are supplied externally. */
#[macro_export]
macro_rules! ___rcuwait_wait_event {
    ($w:expr, $condition:expr, $state:expr, $ret:expr, $cmd:expr) => {{
        let mut __ret: core::ffi::c_long = $ret;
        unsafe { $crate::prepare_to_rcuwait($w); }
        loop {
            /*
             * Implicit barrier (A) pairs with (B) in
             * rcuwait_wake_up().
             */
            unsafe { set_current_state($state); }
            if $condition {
                break;
            }

            if unsafe { signal_pending_state($state, current) } {
                __ret = -(EINTR as core::ffi::c_long);
                break;
            }

            $cmd;
        }
        unsafe { $crate::finish_rcuwait($w); }
        __ret
    }};
}

#[macro_export]
macro_rules! rcuwait_wait_event {
    ($w:expr, $condition:expr, $state:expr) => {
        $crate::___rcuwait_wait_event!($w, $condition, $state, 0, schedule())
    };
}

#[macro_export]
macro_rules! __rcuwait_wait_event_timeout {
    ($w:expr, $condition:expr, $state:expr, $timeout:expr) => {
        $crate::___rcuwait_wait_event!(
            $w,
            ___wait_cond_timeout!($condition),
            $state,
            $timeout,
            __ret = schedule_timeout(__ret)
        )
    };
}

#[macro_export]
macro_rules! rcuwait_wait_event_timeout {
    ($w:expr, $condition:expr, $state:expr, $timeout:expr) => {{
        let mut __ret: core::ffi::c_long = $timeout;
        if !___wait_cond_timeout!($condition) {
            __ret = $crate::__rcuwait_wait_event_timeout!($w, $condition, $state, $timeout);
        }
        __ret
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
