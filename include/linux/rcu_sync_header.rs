/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * RCU-based infrastructure for lightweight reader-writer locking
 *
 * Copyright (c) 2015, Red Hat, Inc.
 *
 * Author: Oleg Nesterov <oleg@redhat.com>
 */

// Dependencies supplied by the Linux wait-queue and RCU subsystems are
// intentionally left as external names, corresponding to the C includes.

#[repr(C)]
pub struct rcu_sync {
    pub gp_state: core::ffi::c_int,
    pub gp_count: core::ffi::c_int,
    pub gp_wait: wait_queue_head_t,
    pub cb_head: rcu_head,
}

/**
 * rcu_sync_is_idle() - Are readers permitted to use their fastpaths?
 * @rsp: Pointer to rcu_sync structure to use for synchronization
 *
 * Returns true if readers are permitted to use their fastpaths.  Must be
 * invoked within some flavor of RCU read-side critical section.
 */
#[inline]
pub unsafe fn rcu_sync_is_idle(rsp: *mut rcu_sync) -> bool {
    // C: RCU_LOCKDEP_WARN(!rcu_read_lock_any_held(),
    //     "suspicious rcu_sync_is_idle() usage");
    // The lockdep diagnostic is provided by the surrounding RCU subsystem.
    core::ptr::read_volatile(core::ptr::addr_of!((*rsp).gp_state) as *const core::ffi::c_int) == 0
}

extern "C" {
    pub fn rcu_sync_init(rsp: *mut rcu_sync);
    pub fn rcu_sync_enter(rsp: *mut rcu_sync);
    pub fn rcu_sync_exit(rsp: *mut rcu_sync);
    pub fn rcu_sync_dtor(rsp: *mut rcu_sync);
}

#[macro_export]
macro_rules! __RCU_SYNC_INITIALIZER {
    ($name:ident) => {
        rcu_sync {
            gp_state: 0,
            gp_count: 0,
            gp_wait: __WAIT_QUEUE_HEAD_INITIALIZER!($name.gp_wait),
            // The C initializer leaves cb_head zero-initialized.
            cb_head: unsafe { core::mem::zeroed() },
        }
    };
}

#[macro_export]
macro_rules! DEFINE_RCU_SYNC {
    ($name:ident) => {
        let mut $name: rcu_sync = $crate::__RCU_SYNC_INITIALIZER!($name);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
