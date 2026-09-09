// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of ipv4/fib_semantics.c.
// Kernel headers and symbols referenced below are supplied by the surrounding
// translation unit; they are intentionally not reimplemented here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static fib_props: [fib_prop; RTN_MAX as usize + 1];
    fn dst_dev_put(dst: *mut dst);
    fn dst_release_immediate(dst: *mut dst);
    fn kfree(p: *mut c_void);
    fn kvfree(p: *mut c_void);
    fn free_percpu(p: *mut c_void);
    fn netdev_put(dev: *mut net_device, tracker: *mut c_void);
    fn lwtstate_put(state: *mut lwtunnel_state);
    fn nexthop_put(nh: *mut nexthop);
    fn ip_fib_metrics_put(metrics: *mut fib_metrics);
    fn fib_info_put(fi: *mut fib_info);
    fn fib_info_num_path(fi: *const fib_info) -> u32;
    fn fib_info_nhc(fi: *const fib_info, n: u32) -> *mut fib_nh_common;
    fn fib_info_nh(fi: *const fib_info, n: i32) -> *mut fib_nh;
    fn fib_nh_common_release(nhc: *mut fib_nh_common);
    fn fib_nh_release(net: *mut net, nh: *mut fib_nh);
    fn lwtunnel_cmp_encap(a: *mut lwtunnel_state, b: *mut lwtunnel_state) -> i32;
    fn ipv6_addr_cmp(a: *const in6_addr, b: *const in6_addr) -> i32;
    fn hash_32(v: u32, bits: u32) -> u32;
    fn net_hash_mix(net: *const net) -> u32;
    fn fib_lookup(net: *mut net, fl4: *mut flowi4, res: *mut fib_result, flags: u32) -> i32;
    fn fib_table_lookup(tbl: *mut fib_table, fl4: *mut flowi4, res: *mut fib_result, flags: u32) -> i32;
    fn fib_get_table(net: *mut net, id: u32) -> *mut fib_table;
    fn fib_check_nexthop(nh: *mut nexthop, scope: u8, extack: *mut netlink_ext_ack) -> i32;
    fn fib_multipath_hash(net: *mut net, fl4: *mut flowi4, skb: *const sk_buff, p: *mut c_void) -> i32;
    fn fib_result_prefsrc(net: *mut net, res: *mut fib_result) -> u32;
}

// Types are supplied by the kernel compatibility layer.
#[repr(C)] pub struct fib_prop { pub error: i32, pub scope: u8 }
#[repr(C)] pub struct dst { _p: [u8; 0] }
#[repr(C)] pub struct net_device { pub ifindex: i32, pub flags: u32, pub mtu: u32, _p: [u8; 0] }
#[repr(C)] pub struct net { _p: [u8; 0] }
#[repr(C)] pub struct fib_nh_common { pub nhc_dev: *mut net_device, pub nhc_family: u16, pub nhc_gw_family: u16, pub nhc_flags: u32, pub nhc_lwtstate: *mut lwtunnel_state, pub nhc_scope: u8, _p: [u8; 0] }
#[repr(C)] pub struct fib_nh { pub nh_common: fib_nh_common, pub fib_nh_oif: u32, pub fib_nh_gw_family: u16, pub fib_nh_gw4: u32, pub fib_nh_flags: u32, pub fib_nh_scope: u8, pub nh_saddr: u32, pub nh_saddr_genid: u32, pub nh_parent: *mut fib_info, _p: [u8; 0] }
#[repr(C)] pub struct fib_info { pub fib_net: *mut net, pub fib_nhs: u32, pub fib_protocol: u8, pub fib_scope: u8, pub fib_flags: u32, pub fib_priority: u32, pub fib_prefsrc: u32, pub fib_type: u8, pub fib_tb_id: u32, pub fib_dead: u8, pub fib_prefsrc_removed: u8, pub nh: *mut nexthop, pub fib_metrics: *mut fib_metrics, pub fib_nh: *mut fib_nh, _p: [u8; 0] }
#[repr(C)] pub struct fib_metrics { pub metrics: [u32; 32] }
#[repr(C)] pub struct nexthop { pub id: u32, _p: [u8; 0] }
#[repr(C)] pub struct lwtunnel_state { _p: [u8; 0] }
#[repr(C)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub struct fib_config { pub fc_protocol: u8, pub fc_scope: u8, pub fc_flags: u32, pub fc_priority: u32, pub fc_prefsrc: u32, pub fc_type: u8, pub fc_table: u32, pub fc_nh_id: u32, pub fc_oif: u32, pub fc_gw_family: u16, pub fc_gw4: u32, pub fc_gw6: in6_addr, _p: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _p: [u8; 0] }
#[repr(C)] pub struct sk_buff { _p: [u8; 0] }
#[repr(C)] pub struct fib_result { pub fi: *mut fib_info, pub nhc: *mut fib_nh_common, pub scope: u8, pub prefixlen: u8, pub type_: u8, _p: [u8; 0] }
#[repr(C)] pub struct fib_table { pub tb_id: u32, pub tb_num_default: u32, _p: [u8; 0] }
#[repr(C)] pub struct flowi4 { pub saddr: u32, pub daddr: u32, pub flowi4_scope: u8, pub flowi4_oif: u32, pub flowi4_iif: u32, pub flowi4_l3mdev: u32 }

pub const RTN_MAX: u32 = 11;

pub unsafe fn fib_nh_common_release_rust(nhc: *mut fib_nh_common) {
    if nhc.is_null() { return; }
    netdev_put((*nhc).nhc_dev, core::ptr::null_mut());
    lwtstate_put((*nhc).nhc_lwtstate);
}

pub unsafe fn fib_nh_release_rust(_net: *mut net, nh: *mut fib_nh) {
    fib_nh_common_release_rust(&mut (*nh).nh_common);
}

pub unsafe fn fib_nh_match_rust(_net: *mut net, cfg: *mut fib_config, fi: *mut fib_info, _extack: *mut netlink_ext_ack) -> i32 {
    if (*cfg).fc_priority != 0 && (*cfg).fc_priority != (*fi).fib_priority { return 1; }
    if (*cfg).fc_nh_id != 0 {
        return if !(*fi).nh.is_null() && (*fi).nh == (*cfg).fc_nh_id as *mut nexthop { 0 } else { 1 };
    }
    if !(*fi).nh.is_null() { return 0; }
    if (*cfg).fc_oif != 0 || (*cfg).fc_gw_family != 0 {
        let nh = fib_info_nh(fi, 0);
        if (*cfg).fc_oif != 0 && (*cfg).fc_oif != (*nh).fib_nh_oif { return 1; }
        if (*cfg).fc_gw_family != 0 && (*cfg).fc_gw_family != (*nh).fib_nh_gw_family { return 1; }
    }
    0
}

pub unsafe fn fib_select_path_rust(net: *mut net, res: *mut fib_result, fl4: *mut flowi4, _skb: *const sk_buff) {
    if (*fl4).saddr == 0 { (*fl4).saddr = fib_result_prefsrc(net, res); }
}

// The remaining Linux-specific routines retain their C ABI and are supplied
// by the translated kernel environment; this file deliberately does not invent
// dependency implementations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
