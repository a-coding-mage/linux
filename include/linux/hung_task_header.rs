/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Detect Hung Task: detecting tasks stuck in D state
 *
 * Copyright (C) 2025 Tongcheng Travel (www.ly.com)
 * Author: Lance Yang <mingzhe.yang@ly.com>
 */

use core::ffi::c_void;

/*
 * @blocker: Combines lock address and blocking type.
 *
 * The two least significant bits of suitably aligned lock pointers encode
 * the blocking type.  On architectures where this is not guaranteed, or
 * for an unaligned lock, tracking is silently skipped.
 */
pub const BLOCKER_TYPE_MUTEX: usize = 0x00;
pub const BLOCKER_TYPE_SEM: usize = 0x01;
pub const BLOCKER_TYPE_RWSEM_READER: usize = 0x02;
pub const BLOCKER_TYPE_RWSEM_WRITER: usize = 0x03;
pub const BLOCKER_TYPE_MASK: usize = 0x03;

/* CONFIG_DETECT_HUNG_TASK_BLOCKER controls which implementation is built. */
#[cfg(feature = "CONFIG_DETECT_HUNG_TASK_BLOCKER")]
pub unsafe fn hung_task_set_blocker(lock: *mut c_void, type_: usize) {
    let lock_ptr = lock as usize;

    /* WARN_ON_ONCE(!lock_ptr); */
    if lock_ptr == 0 {
        // Dependency-provided WARN_ON_ONCE equivalent.
    }
    /* WARN_ON_ONCE(READ_ONCE(current->blocker)); */
    if lock_ptr & BLOCKER_TYPE_MASK != 0 {
        return;
    }

    /* Dependency-provided current task and WRITE_ONCE operation. */
    unsafe { core::ptr::write_volatile(crate::current_blocker_ptr(), lock_ptr | type_) };
}

#[cfg(feature = "CONFIG_DETECT_HUNG_TASK_BLOCKER")]
pub unsafe fn hung_task_clear_blocker() {
    unsafe { core::ptr::write_volatile(crate::current_blocker_ptr(), 0usize) };
}

#[cfg(feature = "CONFIG_DETECT_HUNG_TASK_BLOCKER")]
pub unsafe fn hung_task_get_blocker_type(blocker: usize) -> usize {
    /* WARN_ON_ONCE(!blocker); */
    blocker & BLOCKER_TYPE_MASK
}

#[cfg(feature = "CONFIG_DETECT_HUNG_TASK_BLOCKER")]
pub unsafe fn hung_task_blocker_to_lock(blocker: usize) -> *mut c_void {
    /* WARN_ON_ONCE(!blocker); */
    (blocker & !BLOCKER_TYPE_MASK) as *mut c_void
}

#[cfg(not(feature = "CONFIG_DETECT_HUNG_TASK_BLOCKER"))]
pub unsafe fn hung_task_set_blocker(_lock: *mut c_void, _type_: usize) {}

#[cfg(not(feature = "CONFIG_DETECT_HUNG_TASK_BLOCKER"))]
pub unsafe fn hung_task_clear_blocker() {}

#[cfg(not(feature = "CONFIG_DETECT_HUNG_TASK_BLOCKER"))]
pub unsafe fn hung_task_get_blocker_type(_blocker: usize) -> usize {
    0
}

#[cfg(not(feature = "CONFIG_DETECT_HUNG_TASK_BLOCKER"))]
pub unsafe fn hung_task_blocker_to_lock(_blocker: usize) -> *mut c_void {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
