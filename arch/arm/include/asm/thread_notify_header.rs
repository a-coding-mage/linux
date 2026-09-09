/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/thread_notify.h
 *
 *  Copyright (C) 2006 Russell King.
 */

// C conditional: declarations below are available only under __KERNEL__ and
// outside __ASSEMBLY__ builds.

use core::ffi::c_int;

// Supplied by the corresponding notifier and thread-info dependencies.
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_notifier_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread_info {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut thread_notify_head: atomic_notifier_head;

    pub fn atomic_notifier_chain_register(
        nh: *mut atomic_notifier_head,
        n: *mut notifier_block,
    ) -> c_int;

    pub fn atomic_notifier_chain_unregister(
        nh: *mut atomic_notifier_head,
        n: *mut notifier_block,
    ) -> c_int;

    pub fn atomic_notifier_call_chain(
        nh: *mut atomic_notifier_head,
        val: usize,
        v: *mut core::ffi::c_void,
    ) -> c_int;
}

#[inline]
pub unsafe fn thread_register_notifier(n: *mut notifier_block) -> c_int {
    unsafe { atomic_notifier_chain_register(&raw mut thread_notify_head, n) }
}

#[inline]
pub unsafe fn thread_unregister_notifier(n: *mut notifier_block) {
    unsafe {
        atomic_notifier_chain_unregister(&raw mut thread_notify_head, n);
    }
}

#[inline]
pub unsafe fn thread_notify(rc: usize, thread: *mut thread_info) {
    unsafe {
        atomic_notifier_call_chain(
            &raw mut thread_notify_head,
            rc,
            thread.cast::<core::ffi::c_void>(),
        );
    }
}

/*
 * These are the reason codes for the thread notifier.
 */
pub const THREAD_NOTIFY_FLUSH: u32 = 0;
pub const THREAD_NOTIFY_EXIT: u32 = 1;
pub const THREAD_NOTIFY_SWITCH: u32 = 2;
pub const THREAD_NOTIFY_COPY: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
