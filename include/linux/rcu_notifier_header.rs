/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Read-Copy Update notifiers, initially RCU CPU stall notifier.
 * Separate from rcupdate.h to avoid #include loops.
 *
 * Copyright (C) 2023 Paul E. McKenney.
 */

// Actions for RCU CPU stall notifier calls.
pub const RCU_STALL_NOTIFY_NORM: i32 = 1;
pub const RCU_STALL_NOTIFY_EXP: i32 = 2;

// Equivalent of:
// #if defined(CONFIG_RCU_STALL_COMMON) && defined(CONFIG_RCU_CPU_STALL_NOTIFIER)
#[cfg(all(feature = "CONFIG_RCU_STALL_COMMON", feature = "CONFIG_RCU_CPU_STALL_NOTIFIER"))]
extern "C" {
    pub fn rcu_stall_chain_notifier_register(n: *mut notifier_block) -> i32;
    pub fn rcu_stall_chain_notifier_unregister(n: *mut notifier_block) -> i32;
}

// The following declarations are supplied by the corresponding Linux
// notifier/types dependencies.
#[allow(non_camel_case_types)]
pub type notifier_block = crate::notifier_block;

// No RCU CPU stall warnings in Tiny RCU.
// Equivalent of the inverse build-time condition above.
#[cfg(not(all(feature = "CONFIG_RCU_STALL_COMMON", feature = "CONFIG_RCU_CPU_STALL_NOTIFIER")))]
#[inline]
pub unsafe fn rcu_stall_chain_notifier_register(n: *mut notifier_block) -> i32 {
    let _ = n;
    -(EEXIST as i32)
}

#[cfg(not(all(feature = "CONFIG_RCU_STALL_COMMON", feature = "CONFIG_RCU_CPU_STALL_NOTIFIER")))]
#[inline]
pub unsafe fn rcu_stall_chain_notifier_unregister(n: *mut notifier_block) -> i32 {
    let _ = n;
    -(ENOENT as i32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
