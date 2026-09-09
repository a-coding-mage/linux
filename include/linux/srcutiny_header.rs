/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Sleepable Read-Copy Update mechanism for mutual exclusion,
 *	tiny variant.
 *
 * Copyright (C) IBM Corporation, 2017
 *
 * Author: Paul McKenney <paulmck@linux.ibm.com>
 */

use core::ffi::{c_char, c_int, c_short, c_ulong, c_void};

// Types and operations supplied by the surrounding kernel translation unit.
#[repr(C)]
pub struct irq_work {
    pub func: Option<unsafe extern "C" fn(*mut irq_work)>,
}
#[repr(C)] pub struct swait_queue_head { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct lockdep_map { _private: [u8; 0] }
#[repr(C)] pub struct srcu_ctr { _private: [u8; 0] }

#[repr(C)]
pub struct srcu_struct {
    pub srcu_lock_nesting: [c_short; 2], // srcu_read_lock() nesting depth.
    pub srcu_gp_running: u8,             // GP workqueue running?
    pub srcu_gp_waiting: u8,             // GP waiting for readers?
    pub srcu_idx: c_ulong,               // Current reader array element in bit 0x2.
    pub srcu_idx_max: c_ulong,           // Furthest future srcu_idx request.
    pub srcu_wq: swait_queue_head,       // Last srcu_read_unlock() wakes GP.
    pub srcu_cb_head: *mut rcu_head,     // Pending callbacks: Head.
    pub srcu_cb_tail: *mut *mut rcu_head, // Pending callbacks: Tail.
    pub srcu_work: work_struct,           // For driving grace periods.
    pub srcu_irq_work: irq_work,          // Defer schedule_work() to irq work.
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    pub dep_map: lockdep_map,
}

extern "C" {
    pub fn srcu_drive_gp(wp: *mut work_struct);
    pub fn srcu_tiny_irq_work(irq_work: *mut irq_work);
    pub fn synchronize_srcu(ssp: *mut srcu_struct);
    pub fn __srcu_read_unlock(ssp: *mut srcu_struct, idx: c_int);
    pub fn preempt_disable();
    pub fn preempt_enable();
    pub fn __acquire_shared(ssp: *mut srcu_struct);
    pub fn __release_shared(ssp: *mut srcu_struct);
    pub fn pr_alert(fmt: *const c_char, ...);
}

// The C initializer macros depend on kernel initializers and dependency-local
// __SRCU_DEP_MAP_INIT; retain the source-level construction as a Rust macro.
#[macro_export]
macro_rules! __SRCU_STRUCT_INIT {
    ($name:ident, $($ignored:expr),*) => {
        $crate::srcu_struct {
            srcu_lock_nesting: [0, 0],
            srcu_gp_running: 0,
            srcu_gp_waiting: 0,
            srcu_idx: 0,
            srcu_idx_max: 0,
            srcu_wq: unsafe { core::mem::zeroed() },
            srcu_cb_head: core::ptr::null_mut(),
            srcu_cb_tail: &mut $name.srcu_cb_head,
            srcu_work: unsafe { core::mem::zeroed() },
            srcu_irq_work: $crate::irq_work { func: Some($crate::srcu_tiny_irq_work) },
            ..unsafe { core::mem::zeroed() }
        }
    };
}

#[macro_export] macro_rules! DEFINE_SRCU { ($name:ident) => { let mut $name = $crate::__SRCU_STRUCT_INIT!($name, $name, $name, $name); }; }
#[macro_export] macro_rules! DEFINE_STATIC_SRCU { ($name:ident) => { static mut $name: $crate::srcu_struct = unsafe { core::mem::zeroed() }; }; }
#[macro_export] macro_rules! DEFINE_SRCU_FAST { ($name:ident) => { $crate::DEFINE_SRCU!($name) }; }
#[macro_export] macro_rules! DEFINE_STATIC_SRCU_FAST { ($name:ident) => { $crate::DEFINE_STATIC_SRCU!($name) }; }
#[macro_export] macro_rules! DEFINE_SRCU_FAST_UPDOWN { ($name:ident) => { $crate::DEFINE_SRCU!($name) }; }
#[macro_export] macro_rules! DEFINE_STATIC_SRCU_FAST_UPDOWN { ($name:ident) => { $crate::DEFINE_STATIC_SRCU!($name) }; }

// Dummy structure for srcu_notifier_head.
#[repr(C)] pub struct srcu_usage {}
#[macro_export] macro_rules! __SRCU_USAGE_INIT { ($name:ident) => { $crate::srcu_usage {} }; }

pub unsafe fn __srcu_read_lock(ssp: *mut srcu_struct) -> c_int {
    preempt_disable();
    let idx = (((core::ptr::read_volatile(&(*ssp).srcu_idx) + 1) & 0x2) >> 1) as c_int;
    let p = &mut (*ssp).srcu_lock_nesting[idx as usize];
    *p = core::ptr::read_volatile(p).wrapping_add(1);
    preempt_enable();
    __acquire_shared(ssp);
    idx
}

pub unsafe fn __srcu_ptr_to_ctr(_ssp: *mut srcu_struct, scpp: *mut srcu_ctr) -> c_int {
    scpp as isize as c_int
}

pub unsafe fn __srcu_ctr_to_ptr(_ssp: *mut srcu_struct, idx: c_int) -> *mut srcu_ctr {
    idx as isize as *mut srcu_ctr
}

pub unsafe fn __srcu_read_lock_fast(ssp: *mut srcu_struct) -> *mut srcu_ctr {
    __srcu_ctr_to_ptr(ssp, __srcu_read_lock(ssp))
}
pub unsafe fn __srcu_read_unlock_fast(ssp: *mut srcu_struct, scp: *mut srcu_ctr) {
    __srcu_read_unlock(ssp, __srcu_ptr_to_ctr(ssp, scp));
}
pub unsafe fn __srcu_read_lock_fast_updown(ssp: *mut srcu_struct) -> *mut srcu_ctr {
    __srcu_ctr_to_ptr(ssp, __srcu_read_lock(ssp))
}
pub unsafe fn __srcu_read_unlock_fast_updown(ssp: *mut srcu_struct, scp: *mut srcu_ctr) {
    __srcu_read_unlock(ssp, __srcu_ptr_to_ctr(ssp, scp));
}

pub unsafe fn synchronize_srcu_expedited(ssp: *mut srcu_struct) { synchronize_srcu(ssp); }
pub unsafe fn srcu_barrier(ssp: *mut srcu_struct) { synchronize_srcu(ssp); }
pub unsafe fn srcu_expedite_current(_ssp: *mut srcu_struct) {}
#[macro_export] macro_rules! srcu_check_read_flavor { ($ssp:expr, $read_flavor:expr) => {}; }

pub unsafe fn srcu_torture_stats_print(ssp: *mut srcu_struct, tt: *mut c_char, tf: *mut c_char) {
    let idx = (((core::ptr::read_volatile(&(*ssp).srcu_idx) + 1) & 0x2) >> 1) as c_int;
    pr_alert(b"%s%s Tiny SRCU per-CPU(idx=%d): (%hd,%hd) gp: %lu->%lu\0".as_ptr() as *const c_char,
        tt, tf, idx, core::ptr::read_volatile(&(*ssp).srcu_lock_nesting[(idx == 0) as usize]),
        core::ptr::read_volatile(&(*ssp).srcu_lock_nesting[idx as usize]),
        core::ptr::read_volatile(&(*ssp).srcu_idx), core::ptr::read_volatile(&(*ssp).srcu_idx_max));
}

pub unsafe fn srcu_readers_active(ssp: *mut srcu_struct) -> bool {
    core::ptr::read_volatile(&(*ssp).srcu_lock_nesting[0]) != 0 ||
    core::ptr::read_volatile(&(*ssp).srcu_lock_nesting[1]) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
