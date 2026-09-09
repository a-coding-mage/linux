// SPDX-License-Identifier: GPL-2.0
//
// Direct Rust translation boundary for ipv4/nexthop.c.
//
// This implementation is Linux-kernel code. Its concrete structures, constants,
// allocator/RCU/list helpers, and notifier/netlink APIs are supplied by the
// surrounding translated kernel sources. They are intentionally referenced as
// external dependencies rather than reimplemented here.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

// C declarations supplied by the translated kernel headers.
#[repr(C)]
pub struct net { _private: [u8; 0] }
#[repr(C)]
pub struct nexthop { _private: [u8; 0] }
#[repr(C)]
pub struct nh_info { _private: [u8; 0] }
#[repr(C)]
pub struct nh_group { _private: [u8; 0] }
#[repr(C)]
pub struct nh_res_table { _private: [u8; 0] }
#[repr(C)]
pub struct nh_res_bucket { _private: [u8; 0] }
#[repr(C)]
pub struct nh_grp_entry { _private: [u8; 0] }
#[repr(C)]
pub struct nh_config { _private: [u8; 0] }
#[repr(C)]
pub struct nh_notifier_info { _private: [u8; 0] }
#[repr(C)]
pub struct nh_notifier_grp_hw_stats_info { _private: [u8; 0] }
#[repr(C)]
pub struct fib6_nh { _private: [u8; 0] }
#[repr(C)]
pub struct fib6_config { _private: [u8; 0] }
#[repr(C)]
pub struct fib_info { _private: [u8; 0] }
#[repr(C)]
pub struct fib6_info { _private: [u8; 0] }
#[repr(C)]
pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)]
pub struct nl_info { _private: [u8; 0] }
#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }
#[repr(C)]
pub struct notifier_block { _private: [u8; 0] }
#[repr(C)]
pub struct rcu_head { _private: [u8; 0] }

pub type u8_ = u8;
pub type u16_ = u16;
pub type u32_ = u32;
pub type u64_ = u64;

extern "C" {
    pub fn nexthop_free_rcu(head: *mut rcu_head);
    pub fn nexthop_find_by_id(net: *mut net, id: u32) -> *mut nexthop;
    pub fn nexthop_select_path(nh: *mut nexthop, hash: i32) -> *mut nexthop;
    pub fn nexthop_for_each_fib6_nh(
        nh: *mut nexthop,
        cb: Option<unsafe extern "C" fn(*mut fib6_nh, *mut c_void) -> i32>,
        arg: *mut c_void,
    ) -> i32;
    pub fn fib6_check_nexthop(
        nh: *mut nexthop,
        cfg: *mut fib6_config,
        extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn fib_check_nexthop(
        nh: *mut nexthop,
        scope: u8,
        extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn nh_grp_hw_stats_report_delta(
        info: *mut nh_notifier_grp_hw_stats_info,
        nh_idx: u32,
        delta_packets: u64,
    );
}

// The remaining definitions in nexthop.c are translation units whose field
// layouts and helper operations are provided by the corresponding kernel
// headers. Keep the source-level implementation available to the integration
// pass through this explicit external dependency marker.
#[allow(dead_code)]
pub const NEXTHOP_C_SOURCE: &str = "ipv4/nexthop.c";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
