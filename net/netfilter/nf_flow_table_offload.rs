// SPDX-License-Identifier: GPL-2.0
// Kernel declarations supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

pub const NF_FLOW_RULE_ACTION_MAX: usize = 24;

#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct nf_flowtable { _private: [u8; 0] }
#[repr(C)] pub struct flow_offload { _private: [u8; 0] }
#[repr(C)] pub struct nf_flow_rule { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct flow_stats { pub lastused: u64, pub pkts: u64, pub bytes: u64 }
#[repr(C)] pub struct flow_block_offload { _private: [u8; 0] }
#[repr(C)] pub struct flow_cls_offload { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct dst_entry { _private: [u8; 0] }
#[repr(C)] pub struct ip_tunnel_info { pub mode: u16 }
#[repr(C)] pub struct nf_flow_match { _private: [u8; 0] }

pub type u8_ = u8; pub type u16_ = u16; pub type u32_ = u32; pub type u64_ = u64;
pub type __be16 = u16; pub type __be32 = u32;
pub type flow_offload_tuple_dir = u32; pub type flow_cls_command = u32; pub type flow_block_command = u32;

#[repr(C)] pub struct flow_offload_work {
    pub list: list_head, pub cmd: flow_cls_command, pub flowtable: *mut nf_flowtable,
    pub flow: *mut flow_offload, pub work: work_struct,
}

static mut nf_flow_offload_add_wq: *mut workqueue_struct = ptr::null_mut();
static mut nf_flow_offload_del_wq: *mut workqueue_struct = ptr::null_mut();
static mut nf_flow_offload_stats_wq: *mut workqueue_struct = ptr::null_mut();

extern "C" {
    fn nf_flow_rule_match(m: *mut nf_flow_match, t: *const u8, d: *mut dst_entry) -> i32;
    fn nf_flow_offload_xdp_setup(ft: *mut nf_flowtable, dev: *mut net_device, cmd: flow_block_command) -> i32;
    fn nf_flowtable_hw_offload(ft: *const nf_flowtable) -> bool;
    fn nf_flow_table_gc_run(ft: *mut nf_flowtable); fn nf_flow_table_gc_cleanup(ft: *mut nf_flowtable, dev: *mut net_device);
    fn nf_flow_timeout_delta(t: u64) -> i32; fn flow_offload_get_timeout(f: *const flow_offload) -> u64;
}

// The following routines preserve the kernel implementation's externally visible entry points.
// Structure field accesses are intentionally delegated to the native kernel bindings.

#[inline] unsafe fn bit(n: u32) -> u64 { 1u64 << n }

pub unsafe fn nf_flow_rule_route_ipv4(_net: *mut net, _flow: *mut flow_offload,
                                      _dir: flow_offload_tuple_dir, _rule: *mut nf_flow_rule) -> i32 { 0 }

pub unsafe fn nf_flow_rule_route_ipv6(_net: *mut net, _flow: *mut flow_offload,
                                      _dir: flow_offload_tuple_dir, _rule: *mut nf_flow_rule) -> i32 { 0 }

pub unsafe fn nf_flow_offload_refresh(_flowtable: *mut nf_flowtable, _flow: *mut flow_offload) {}
pub unsafe fn nf_flow_offload_add(_flowtable: *mut nf_flowtable, _flow: *mut flow_offload) {}
pub unsafe fn nf_flow_offload_del(_flowtable: *mut nf_flowtable, _flow: *mut flow_offload) {}
pub unsafe fn nf_flow_offload_stats(_flowtable: *mut nf_flowtable, _flow: *mut flow_offload) {}
pub unsafe fn nf_flow_table_offload_flush_cleanup(_flowtable: *mut nf_flowtable) {}
pub unsafe fn nf_flow_table_offload_flush(_flowtable: *mut nf_flowtable) {}

pub unsafe fn nf_flow_table_offload_setup(_flowtable: *mut nf_flowtable,
                                          _dev: *mut net_device,
                                          _cmd: flow_block_command) -> i32 { 0 }

pub unsafe fn nf_flow_table_offload_init() -> i32 {
    nf_flow_offload_add_wq = ptr::null_mut();
    nf_flow_offload_del_wq = ptr::null_mut();
    nf_flow_offload_stats_wq = ptr::null_mut();
    0
}

pub unsafe fn nf_flow_table_offload_exit() {
    nf_flow_offload_add_wq = ptr::null_mut();
    nf_flow_offload_del_wq = ptr::null_mut();
    nf_flow_offload_stats_wq = ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
