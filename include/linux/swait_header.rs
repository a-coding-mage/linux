/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/swait.h. The following names are supplied by the
// corresponding kernel headers/dependencies.

use core::ffi::{c_char, c_int, c_long};

#[repr(C)]
pub struct task_struct;
#[repr(C)]
pub struct lock_class_key;
#[repr(C)]
pub struct raw_spinlock_t;
#[repr(C)]
pub struct list_head;

#[repr(C)]
pub struct swait_queue_head {
    pub lock: raw_spinlock_t,
    pub task_list: list_head,
}

#[repr(C)]
pub struct swait_queue {
    pub task: *mut task_struct,
    pub task_list: list_head,
}

// __SWAITQUEUE_INITIALIZER(name): task = current and LIST_HEAD_INIT(name.task_list)
// DECLARE_SWAITQUEUE(name) declares a swait_queue with the initializer above.
// __SWAIT_QUEUE_HEAD_INITIALIZER(name) initializes the raw spinlock and list head.
// DECLARE_SWAIT_QUEUE_HEAD(name) declares a swait_queue_head with that initializer.

extern "C" {
    pub fn __init_swait_queue_head(
        q: *mut swait_queue_head,
        name: *const c_char,
        key: *mut lock_class_key,
    );
    pub fn swake_up_one(q: *mut swait_queue_head);
    pub fn swake_up_all(q: *mut swait_queue_head);
    pub fn swake_up_locked(q: *mut swait_queue_head, wake_flags: c_int);
    pub fn prepare_to_swait_exclusive(
        q: *mut swait_queue_head,
        wait: *mut swait_queue,
        state: c_int,
    );
    pub fn prepare_to_swait_event(
        q: *mut swait_queue_head,
        wait: *mut swait_queue,
        state: c_int,
    ) -> c_long;
    pub fn __finish_swait(q: *mut swait_queue_head, wait: *mut swait_queue);
    pub fn finish_swait(q: *mut swait_queue_head, wait: *mut swait_queue);
}

/// Locklessly test for waiters on the queue.
#[inline]
pub unsafe fn swait_active(wq: *mut swait_queue_head) -> c_int {
    // Equivalent to: return !list_empty(&wq->task_list);
    (!list_empty(unsafe { &(*wq).task_list })) as c_int
}

/// Check if there are any waiting processes.
#[inline]
pub unsafe fn swq_has_sleeper(wq: *mut swait_queue_head) -> bool {
    // smp_mb();
    unsafe { smp_mb() };
    unsafe { swait_active(wq) != 0 }
}

extern "C" {
    fn list_empty(head: &list_head) -> bool;
    fn smp_mb();
}

// init_swait_queue_head(q) uses a function-local static lock_class_key and
// passes the stringified q name to __init_swait_queue_head.

// The following wait-event macros retain the C macro control flow and are
// expressed as Rust macro_rules! items so their condition and command remain
// caller-provided expressions.
#[macro_export]
macro_rules! ___swait_event {
    ($wq:expr, $condition:expr, $state:expr, $ret:expr, $cmd:expr) => {{
        let mut __wait: $crate::swait_queue = unsafe { core::mem::zeroed() };
        let mut __ret = $ret;
        unsafe { $crate::INIT_LIST_HEAD(&mut __wait.task_list) };
        loop {
            let __int = unsafe {
                $crate::prepare_to_swait_event(&mut $wq, &mut __wait, $state)
            };
            if $condition { break; }
            if unsafe { $crate::___wait_is_interruptible($state) } && __int != 0 {
                __ret = __int;
                break;
            }
            $cmd;
        }
        unsafe { $crate::finish_swait(&mut $wq, &mut __wait) };
        __ret
    }};
}

#[macro_export]
macro_rules! __swait_event {
    ($wq:expr, $condition:expr) => {{
        let _ = $crate::___swait_event!($wq, $condition, TASK_UNINTERRUPTIBLE, 0, schedule());
    }};
}

#[macro_export]
macro_rules! swait_event_exclusive {
    ($wq:expr, $condition:expr) => {{
        if !($condition) { $crate::__swait_event!($wq, $condition); }
    }};
}

#[macro_export]
macro_rules! __swait_event_timeout {
    ($wq:expr, $condition:expr, $timeout:expr) => {
        $crate::___swait_event!($wq, $crate::___wait_cond_timeout!($condition), TASK_UNINTERRUPTIBLE, $timeout, __ret = schedule_timeout(__ret))
    };
}

#[macro_export]
macro_rules! swait_event_timeout_exclusive {
    ($wq:expr, $condition:expr, $timeout:expr) => {{
        let mut __ret = $timeout;
        if !$crate::___wait_cond_timeout!($condition) {
            __ret = $crate::__swait_event_timeout!($wq, $condition, $timeout);
        }
        __ret
    }};
}

#[macro_export]
macro_rules! __swait_event_interruptible {
    ($wq:expr, $condition:expr) => {
        $crate::___swait_event!($wq, $condition, TASK_INTERRUPTIBLE, 0, schedule())
    };
}

#[macro_export]
macro_rules! swait_event_interruptible_exclusive {
    ($wq:expr, $condition:expr) => {{
        let mut __ret = 0;
        if !($condition) { __ret = $crate::__swait_event_interruptible!($wq, $condition); }
        __ret
    }};
}

#[macro_export]
macro_rules! __swait_event_interruptible_timeout {
    ($wq:expr, $condition:expr, $timeout:expr) => {
        $crate::___swait_event!($wq, $crate::___wait_cond_timeout!($condition), TASK_INTERRUPTIBLE, $timeout, __ret = schedule_timeout(__ret))
    };
}

#[macro_export]
macro_rules! swait_event_interruptible_timeout_exclusive {
    ($wq:expr, $condition:expr, $timeout:expr) => {{
        let mut __ret = $timeout;
        if !$crate::___wait_cond_timeout!($condition) {
            __ret = $crate::__swait_event_interruptible_timeout!($wq, $condition, $timeout);
        }
        __ret
    }};
}

#[macro_export]
macro_rules! __swait_event_idle {
    ($wq:expr, $condition:expr) => {{
        let _ = $crate::___swait_event!($wq, $condition, TASK_IDLE, 0, schedule());
    }};
}

#[macro_export]
macro_rules! swait_event_idle_exclusive {
    ($wq:expr, $condition:expr) => {{
        if !($condition) { $crate::__swait_event_idle!($wq, $condition); }
    }};
}

#[macro_export]
macro_rules! __swait_event_idle_timeout {
    ($wq:expr, $condition:expr, $timeout:expr) => {
        $crate::___swait_event!($wq, $crate::___wait_cond_timeout!($condition), TASK_IDLE, $timeout, __ret = schedule_timeout(__ret))
    };
}

#[macro_export]
macro_rules! swait_event_idle_timeout_exclusive {
    ($wq:expr, $condition:expr, $timeout:expr) => {{
        let mut __ret = $timeout;
        if !$crate::___wait_cond_timeout!($condition) {
            __ret = $crate::__swait_event_idle_timeout!($wq, $condition, $timeout);
        }
        __ret
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
