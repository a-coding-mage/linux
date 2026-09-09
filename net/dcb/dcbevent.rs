// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010, Intel Corporation.
 *
 * Author: John Fastabend <john.r.fastabend@intel.com>
 */

use core::ffi::c_void;
use core::mem::MaybeUninit;

// Types and functions supplied by the Linux notifier infrastructure.
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_notifier_head {
    _private: [u8; 0],
}

extern "C" {
    fn atomic_notifier_chain_register(
        nh: *mut atomic_notifier_head,
        nb: *mut notifier_block,
    ) -> i32;
    fn atomic_notifier_chain_unregister(
        nh: *mut atomic_notifier_head,
        nb: *mut notifier_block,
    ) -> i32;
    fn atomic_notifier_call_chain(
        nh: *mut atomic_notifier_head,
        val: c_ulong,
        v: *mut c_void,
    ) -> i32;
}

type c_ulong = core::ffi::c_ulong;

// Corresponds to ATOMIC_NOTIFIER_HEAD(dcbevent_notif_chain).
static mut dcbevent_notif_chain: MaybeUninit<atomic_notifier_head> = MaybeUninit::uninit();

pub unsafe fn register_dcbevent_notifier(nb: *mut notifier_block) -> i32 {
    atomic_notifier_chain_register(
        dcbevent_notif_chain.as_mut_ptr(),
        nb,
    )
}

pub unsafe fn unregister_dcbevent_notifier(nb: *mut notifier_block) -> i32 {
    atomic_notifier_chain_unregister(
        dcbevent_notif_chain.as_mut_ptr(),
        nb,
    )
}

pub unsafe fn call_dcbevent_notifiers(val: c_ulong, v: *mut c_void) -> i32 {
    atomic_notifier_call_chain(
        dcbevent_notif_chain.as_mut_ptr(),
        val,
        v,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
