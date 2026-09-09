/* SPDX-License-Identifier: GPL-2.0 */
/* Linux wait queue related types and methods.  External kernel types and
 * functions are intentionally left as dependencies of this translation. */

pub type wait_queue_entry_t = wait_queue_entry;
pub type wait_queue_func_t = unsafe extern "C" fn(
    wq_entry: *mut wait_queue_entry,
    mode: ::core::ffi::c_uint,
    flags: ::core::ffi::c_int,
    key: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;

extern "C" {
    pub fn default_wake_function(
        wq_entry: *mut wait_queue_entry,
        mode: ::core::ffi::c_uint,
        flags: ::core::ffi::c_int,
        key: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

pub const WQ_FLAG_EXCLUSIVE: ::core::ffi::c_uint = 0x01;
pub const WQ_FLAG_WOKEN: ::core::ffi::c_uint = 0x02;
pub const WQ_FLAG_CUSTOM: ::core::ffi::c_uint = 0x04;
pub const WQ_FLAG_DONE: ::core::ffi::c_uint = 0x08;
pub const WQ_FLAG_PRIORITY: ::core::ffi::c_uint = 0x10;

#[repr(C)]
pub struct wait_queue_entry {
    pub flags: ::core::ffi::c_uint,
    pub private: *mut ::core::ffi::c_void,
    pub func: wait_queue_func_t,
    pub entry: list_head,
}

#[repr(C)]
pub struct wait_queue_head {
    pub lock: spinlock_t,
    pub head: list_head,
}
pub type wait_queue_head_t = wait_queue_head;

pub struct task_struct;

extern "C" {
    pub fn __init_waitqueue_head(
        wq_head: *mut wait_queue_head,
        name: *const ::core::ffi::c_char,
        key: *mut lock_class_key,
    );
    pub fn add_wait_queue(*mut wait_queue_head, *mut wait_queue_entry);
    pub fn add_wait_queue_exclusive(*mut wait_queue_head, *mut wait_queue_entry);
    pub fn add_wait_queue_priority(*mut wait_queue_head, *mut wait_queue_entry);
    pub fn add_wait_queue_priority_exclusive(*mut wait_queue_head, *mut wait_queue_entry) -> ::core::ffi::c_int;
    pub fn remove_wait_queue(*mut wait_queue_head, *mut wait_queue_entry);
    pub fn __wake_up(*mut wait_queue_head, ::core::ffi::c_uint, ::core::ffi::c_int, *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn __wake_up_on_current_cpu(*mut wait_queue_head, ::core::ffi::c_uint, *mut ::core::ffi::c_void);
    pub fn __wake_up_locked_key(*mut wait_queue_head, ::core::ffi::c_uint, *mut ::core::ffi::c_void);
    pub fn __wake_up_sync_key(*mut wait_queue_head, ::core::ffi::c_uint, *mut ::core::ffi::c_void);
    pub fn __wake_up_locked_sync_key(*mut wait_queue_head, ::core::ffi::c_uint, *mut ::core::ffi::c_void);
    pub fn __wake_up_locked(*mut wait_queue_head, ::core::ffi::c_uint, ::core::ffi::c_int);
    pub fn __wake_up_sync(*mut wait_queue_head, ::core::ffi::c_uint);
    pub fn __wake_up_pollfree(*mut wait_queue_head);
    pub fn init_wait_entry(*mut wait_queue_entry, ::core::ffi::c_int);
}

#[inline]
pub unsafe fn init_waitqueue_entry(e: *mut wait_queue_entry, p: *mut task_struct) {
    (*e).flags = 0;
    (*e).private = p.cast();
    (*e).func = default_wake_function;
}

#[inline]
pub unsafe fn init_waitqueue_func_entry(e: *mut wait_queue_entry, f: wait_queue_func_t) {
    (*e).flags = 0;
    (*e).private = core::ptr::null_mut();
    (*e).func = f;
}

#[inline]
pub unsafe fn waitqueue_active(h: *mut wait_queue_head) -> ::core::ffi::c_int {
    (!list_empty(&(*h).head)) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn wq_has_single_sleeper(h: *mut wait_queue_head) -> bool { list_is_singular(&(*h).head) }

#[inline]
pub unsafe fn wq_has_sleeper(h: *mut wait_queue_head) -> bool {
    smp_mb();
    waitqueue_active(h) != 0
}

#[inline]
pub unsafe fn __add_wait_queue(h: *mut wait_queue_head, e: *mut wait_queue_entry) {
    /* Equivalent to list_for_each_entry: priority entries remain first. */
    let mut head = &mut (*h).head as *mut list_head;
    let mut wq: *mut wait_queue_entry = core::ptr::null_mut();
    list_for_each_entry!(wq, &(*h).head, entry, {
        if (*wq).flags & WQ_FLAG_PRIORITY == 0 { break; }
        head = &mut (*wq).entry;
    });
    list_add(&mut (*e).entry, head);
}

#[inline]
pub unsafe fn __add_wait_queue_exclusive(h: *mut wait_queue_head, e: *mut wait_queue_entry) {
    (*e).flags |= WQ_FLAG_EXCLUSIVE; __add_wait_queue(h, e);
}
#[inline]
pub unsafe fn __add_wait_queue_entry_tail(h: *mut wait_queue_head, e: *mut wait_queue_entry) { list_add_tail(&mut (*e).entry, &mut (*h).head); }
#[inline]
pub unsafe fn __add_wait_queue_entry_tail_exclusive(h: *mut wait_queue_head, e: *mut wait_queue_entry) { (*e).flags |= WQ_FLAG_EXCLUSIVE; __add_wait_queue_entry_tail(h, e); }
#[inline]
pub unsafe fn __remove_wait_queue(_: *mut wait_queue_head, e: *mut wait_queue_entry) { list_del(&mut (*e).entry); }

#[inline]
pub unsafe fn wake_up_pollfree(h: *mut wait_queue_head) { if waitqueue_active(h) != 0 { __wake_up_pollfree(h); } }

/* The following C expression macros retain their call-site semantics. */
#[macro_export] macro_rules! wake_up { ($x:expr) => { unsafe { $crate::__wake_up($x, TASK_NORMAL, 1, core::ptr::null_mut()) } }; }
#[macro_export] macro_rules! wake_up_nr { ($x:expr,$nr:expr) => { unsafe { $crate::__wake_up($x,TASK_NORMAL,$nr,core::ptr::null_mut()) } }; }
#[macro_export] macro_rules! wake_up_all { ($x:expr) => { unsafe { $crate::__wake_up($x,TASK_NORMAL,0,core::ptr::null_mut()) } }; }
#[macro_export] macro_rules! wake_up_interruptible { ($x:expr) => { unsafe { $crate::__wake_up($x,TASK_INTERRUPTIBLE,1,core::ptr::null_mut()) } }; }
#[macro_export] macro_rules! wake_up_locked { ($x:expr) => { unsafe { $crate::__wake_up_locked($x,TASK_NORMAL,1) } }; }
#[macro_export] macro_rules! wake_up_all_locked { ($x:expr) => { unsafe { $crate::__wake_up_locked($x,TASK_NORMAL,0) } }; }
#[macro_export] macro_rules! wake_up_sync { ($x:expr) => { unsafe { $crate::__wake_up_sync($x,TASK_NORMAL) } }; }
#[macro_export] macro_rules! wake_up_interruptible_nr { ($x:expr,$nr:expr) => { unsafe { $crate::__wake_up($x,TASK_INTERRUPTIBLE,$nr,core::ptr::null_mut()) } }; }
#[macro_export] macro_rules! wake_up_interruptible_all { ($x:expr) => { unsafe { $crate::__wake_up($x,TASK_INTERRUPTIBLE,0,core::ptr::null_mut()) } }; }
#[macro_export] macro_rules! wake_up_interruptible_sync { ($x:expr) => { unsafe { $crate::__wake_up_sync($x,TASK_INTERRUPTIBLE) } }; }
#[macro_export] macro_rules! poll_to_key { ($m:expr) => { ($m as usize as *mut ::core::ffi::c_void) }; }
#[macro_export] macro_rules! key_to_poll { ($m:expr) => { ($m as usize as _) }; }
#[macro_export] macro_rules! wake_up_poll { ($x:expr,$m:expr) => { unsafe { $crate::__wake_up($x,TASK_NORMAL,1,poll_to_key!($m)) } }; }
#[macro_export] macro_rules! wake_up_poll_on_current_cpu { ($x:expr,$m:expr) => { unsafe { $crate::__wake_up_on_current_cpu($x,TASK_NORMAL,poll_to_key!($m)) } }; }
#[macro_export] macro_rules! wake_up_locked_poll { ($x:expr,$m:expr) => { unsafe { $crate::__wake_up_locked_key($x,TASK_NORMAL,poll_to_key!($m)) } }; }
#[macro_export] macro_rules! wake_up_interruptible_poll { ($x:expr,$m:expr) => { unsafe { $crate::__wake_up($x,TASK_INTERRUPTIBLE,1,poll_to_key!($m)) } }; }
#[macro_export] macro_rules! wake_up_interruptible_sync_poll { ($x:expr,$m:expr) => { unsafe { $crate::__wake_up_sync_key($x,TASK_INTERRUPTIBLE,poll_to_key!($m)) } }; }
#[macro_export] macro_rules! wake_up_interruptible_sync_poll_locked { ($x:expr,$m:expr) => { unsafe { $crate::__wake_up_locked_sync_key($x,TASK_INTERRUPTIBLE,poll_to_key!($m)) } }; }

#[macro_export] macro_rules! wait_event { ($wq:expr,$condition:expr) => {{ might_sleep(); while !$condition { unsafe { schedule(); } } }}; }
#[macro_export] macro_rules! io_wait_event { ($wq:expr,$condition:expr) => {{ might_sleep(); while !$condition { unsafe { io_schedule(); } } }}; }
#[macro_export] macro_rules! wait_event_timeout { ($wq:expr,$condition:expr,$timeout:expr) => {{ let mut __ret = $timeout; might_sleep(); while !$condition && __ret != 0 { unsafe { __ret = schedule_timeout(__ret); } } __ret }}; }
#[macro_export] macro_rules! wait_event_freezable_timeout { ($wq:expr,$condition:expr,$timeout:expr) => { wait_event_timeout!($wq,$condition,$timeout) }; }
#[macro_export] macro_rules! wait_event_cmd { ($wq:expr,$condition:expr,$cmd1:expr,$cmd2:expr) => {{ while !$condition { $cmd1; unsafe { schedule(); } $cmd2; } }}; }
#[macro_export] macro_rules! wait_event_exclusive_cmd { ($wq:expr,$condition:expr,$cmd1:expr,$cmd2:expr) => { wait_event_cmd!($wq,$condition,$cmd1,$cmd2) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
