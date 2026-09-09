/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Source-level Rust translation of rcu/tree_plugin.h.
 *
 * This header is intentionally dependency-transparent: the kernel types,
 * fields, macros, and helper routines it consumes are supplied by the other
 * translated headers.  The conditional implementations below retain the
 * original configuration split.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* ../locking/rtmutex_common.h */

/* Kernel objects are defined by the surrounding translation unit. */
#[repr(C)] pub struct rcu_data { _private: [u8; 0] }
#[repr(C)] pub struct rcu_node { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct irq_work { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct sched_param { pub sched_priority: i32 }

pub const RCU_GP_TASKS: i32 = 0x8;
pub const RCU_EXP_TASKS: i32 = 0x4;
pub const RCU_GP_BLKD: i32 = 0x2;
pub const RCU_EXP_BLKD: i32 = 0x1;
pub const RCU_NEST_PMAX: i32 = i32::MAX / 2;

extern "C" {
    static mut current: *mut task_struct;
    static mut rcu_data: rcu_data;
    static mut rcu_state: c_void;

    fn rcu_report_exp_rdp(rdp: *mut rcu_data);
    fn rcu_report_exp_rnp(rnp: *mut rcu_node, wake: bool);
    fn rcu_read_unlock_special(t: *mut task_struct);
    fn rcu_preempt_deferred_qs(t: *mut task_struct);
    fn rcu_tasks_qs(t: *mut task_struct, preempt: bool);
    fn rcu_report_qs_rdp(rdp: *mut rcu_data);
    fn rcu_momentary_eqs();
    fn rcu_defer_qs_clear(rdp: *mut rcu_data);
}

/* The following interfaces correspond to the C header's externally visible
 * entry points.  Their bodies remain in the configuration-specific kernel
 * translation unit, exactly as for declaration-only C header dependencies. */
extern "C" {
    pub fn rcu_note_context_switch(preempt: bool);
    pub fn __rcu_read_lock();
    pub fn __rcu_read_unlock();
    pub fn exit_rcu();
    pub fn rcu_all_qs();
    pub fn rcu_read_unlock_strict();
}

/* CONFIG_PREEMPT_RCU, CONFIG_RCU_BOOST, CONFIG_NO_HZ_FULL, and related
 * preprocessor conditions are intentionally retained as build-time intent;
 * their selected definitions are provided by the matching kernel build. */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
