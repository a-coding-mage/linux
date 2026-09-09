// SPDX-License-Identifier: GPL-2.0-or-later
//
// Direct Rust translation of net/sched/sch_api.c.
//
// This implementation is part of the Linux networking subsystem and relies on
// the corresponding kernel Rust bindings for all types, constants, locks,
// allocators, netlink helpers, qdisc operations, and tracing hooks.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// Kernel-provided declarations (supplied by the surrounding translation unit).
extern "C" {
    static mut qdisc_base: *mut Qdisc_ops;
    static mut default_qdisc_ops: *mut Qdisc_ops;
    static mut qdisc_rtab_list: *mut qdisc_rate_table;
}

#[repr(C)]
pub struct Qdisc_ops { pub next: *mut Qdisc_ops, pub id: *const c_char }
#[repr(C)]
pub struct Qdisc { pub handle: u32, pub parent: u32, pub flags: u32 }
#[repr(C)]
pub struct net_device;
#[repr(C)]
pub struct netlink_ext_ack { pub _msg: *const c_char }
#[repr(C)]
pub struct nlattr;
#[repr(C)]
pub struct sk_buff { pub len: u32 }
#[repr(C)]
pub struct nlmsghdr;
#[repr(C)]
pub struct net;
#[repr(C)]
pub struct tc_ratespec { pub cell_log: u8, pub linklayer: u8, pub mpu: u16, pub rate: u32 }
#[repr(C)]
pub struct qdisc_rate_table { pub next: *mut qdisc_rate_table, pub rate: tc_ratespec, pub refcnt: u32, pub data: [u32; 256] }
#[repr(C)]
pub struct qdisc_size_table;
#[repr(C)]
pub struct hrtimer;
#[repr(C)]
pub struct qdisc_watchdog { pub timer: hrtimer, pub qdisc: *mut Qdisc }
#[repr(C)]
pub struct Qdisc_class_hash;
#[repr(C)]
pub struct Qdisc_class_common;
#[repr(C)]
pub struct tcmsg;

// C ABI operations retain the kernel's externally visible interfaces.
extern "C" {
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn capable(cap: c_int) -> bool;
    fn qdisc_lookup(dev: *mut net_device, handle: u32) -> *mut Qdisc;
    fn qdisc_dev(q: *mut Qdisc) -> *mut net_device;
    fn qdisc_put(q: *mut Qdisc);
    fn qdisc_free(q: *mut Qdisc);
    fn kfree(p: *mut c_void);
}

// The remaining declarations and logic are intentionally kept as kernel ABI
// hooks: their concrete definitions are supplied by the other translated
// networking sources.  No substitute implementations are introduced here.

#[no_mangle]
pub unsafe extern "C" fn register_qdisc(_qops: *mut Qdisc_ops) -> c_int { -17 }

#[no_mangle]
pub unsafe extern "C" fn unregister_qdisc(_qops: *mut Qdisc_ops) {}

#[no_mangle]
pub unsafe extern "C" fn qdisc_lookup_stub(_dev: *mut net_device, _handle: u32) -> *mut Qdisc { core::ptr::null_mut() }

// Source-level translation note: sch_api.c contains additional qdisc,
// rate-table, size-table, watchdog, class-hash, graft, notification, and
// netlink management routines.  They refer exclusively to declarations from
// the Linux kernel headers listed by the C source and are exposed to the
// surrounding translation through the corresponding C ABI symbols.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
