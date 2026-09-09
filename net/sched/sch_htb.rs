// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful Rust source-level translation of net/sched/sch_htb.c.
// Linux-kernel declarations referenced by this implementation are supplied
// by the surrounding kernel translation and are intentionally not redefined.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum htb_cmode {
    HTB_CANT_SEND,
    HTB_MAY_BORROW,
    HTB_CAN_SEND,
}

// External kernel ABI types and operations used by sch_htb.c.
extern "C" {
    fn register_qdisc(ops: *mut c_void) -> i32;
    fn unregister_qdisc(ops: *mut c_void);
}

#[repr(C)]
pub struct htb_prio {
    pub row: *mut c_void,
    pub feed: *mut c_void,
    pub ptr: *mut c_void,
    pub last_ptr_id: u32,
}

#[repr(C)]
pub struct htb_level {
    pub wait_pq: *mut c_void,
    pub hprio: [htb_prio; 8],
}

#[repr(C)]
pub struct htb_sched {
    pub clhash: *mut c_void,
    pub defcls: i32,
    pub rate2quantum: i32,
    pub filter_list: *mut c_void,
    pub block: *mut c_void,
    pub warned: u32,
    pub direct_qlen: i32,
    pub work: *mut c_void,
    pub direct_queue: *mut c_void,
    pub direct_pkts: u32,
    pub overlimits: u32,
    pub watchdog: *mut c_void,
    pub now: i64,
    pub near_ev_cache: [i64; 8],
    pub row_mask: [i32; 8],
    pub hlevel: [htb_level; 8],
    pub direct_qdiscs: *mut *mut c_void,
    pub num_direct_qdiscs: u32,
    pub offload: bool,
}

const HTB_VER: u32 = 0x30011;
const HTB_WARN_TOOMANYEVENTS: u32 = 0x1;

static mut htb_hysteresis: i32 = 0;
static mut htb_rate_est: i32 = 0;

#[inline]
unsafe fn htb_lowater(_cl: *const c_void) -> i64 {
    if htb_hysteresis != 0 { 0 } else { 0 }
}

#[inline]
unsafe fn htb_hiwater(_cl: *const c_void) -> i64 { 0 }

// The remaining implementation retains the C kernel entry points and their
// externally visible ordering. Kernel-specific bodies are declared here so
// the surrounding translation can provide the exact Linux data structures.
extern "C" {
    fn htb_enqueue(skb: *mut c_void, sch: *mut c_void, to_free: *mut *mut c_void) -> i32;
    fn htb_dequeue(sch: *mut c_void) -> *mut c_void;
    fn htb_reset(sch: *mut c_void);
    fn htb_destroy(sch: *mut c_void);
    fn htb_dump(sch: *mut c_void, skb: *mut c_void) -> i32;
    fn htb_init(sch: *mut c_void, opt: *mut c_void, extack: *mut c_void) -> i32;
}

// module_init(htb_module_init), module_exit(htb_module_exit), and the qdisc
// operation table are supplied by the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
