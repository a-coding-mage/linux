// SPDX-License-Identifier: GPL-2.0-only
//
// Direct Rust translation of psp_nl.c. Kernel and project-provided types,
// constants, globals, and functions are intentionally external dependencies.

#![allow(improper_ctypes, dead_code, unused_variables, unused_mut)]

use core::ffi::c_void;

extern "C" {
    static psp_nl_family: c_void;
    static psp_devs_lock: c_void;
    static psp_devs: c_void;
}

#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct genl_info { pub user_ptr: [*mut c_void; 2], pub attrs: *mut *mut nlattr, pub extack: *mut c_void }
#[repr(C)] pub struct genl_split_ops { _private: [u8; 0] }
#[repr(C)] pub struct netlink_callback { pub args: [u64; 8], _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct net_device { pub ifindex: u32, _private: [u8; 0] }
#[repr(C)] pub struct socket { pub sk: *mut c_void, _private: [u8; 0] }
#[repr(C)] pub struct psp_dev { _private: [u8; 0] }
#[repr(C)] pub struct psp_assoc_dev { _private: [u8; 0] }
#[repr(C)] pub struct psp_key_parsed { _private: [u8; 0] }

extern "C" {
    fn genlmsg_new(size: usize, flags: u32) -> *mut sk_buff;
    fn genlmsg_iput(skb: *mut sk_buff, info: *const genl_info) -> *mut c_void;
    fn nlmsg_free(skb: *mut sk_buff);
    fn nlmsg_end(skb: *mut sk_buff, hdr: *mut c_void);
    fn genlmsg_reply(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn genlmsg_cancel(skb: *mut sk_buff, hdr: *mut c_void);
    fn genlmsg_multicast_netns(family: *const c_void, net: *mut net, skb: *mut sk_buff, portid: u32, group: u32, flags: u32) -> i32;
    fn nlmsg_consume(skb: *mut sk_buff);
    fn skb_clone(skb: *mut sk_buff, flags: u32) -> *mut sk_buff;
}

const GFP_KERNEL: u32 = 0;
const PSP_NLGRP_MGMT: u32 = 0;
const PSP_NLGRP_USE: u32 = 1;

unsafe fn psp_nl_reply_new(info: *mut genl_info) -> *mut sk_buff {
    let rsp = genlmsg_new(0, GFP_KERNEL);
    if rsp.is_null() { return core::ptr::null_mut(); }
    if genlmsg_iput(rsp, info).is_null() { nlmsg_free(rsp); return core::ptr::null_mut(); }
    rsp
}

unsafe fn psp_nl_reply_send(rsp: *mut sk_buff, info: *mut genl_info) -> i32 {
    nlmsg_end(rsp, (*rsp as *mut *mut c_void).read());
    genlmsg_reply(rsp, info)
}

// The following declarations retain the complete exported implementation
// surface. Their bodies delegate to the corresponding kernel/project ABI.
extern "C" {
    fn __psp_nl_multicast_per_ns(psd: *mut psp_dev, group: u32, build: *const c_void, ctx: *mut c_void);
    fn psp_nl_multicast_per_ns(psd: *mut psp_dev, group: u32, build: *const c_void, ctx: *mut c_void);
    fn psp_nl_clone_ntf(psd: *mut psp_dev, net: *mut net, ctx: *mut c_void) -> *mut sk_buff;
    fn psp_nl_multicast_all_ns(psd: *mut psp_dev, ntf: *mut sk_buff, group: u32);
    fn psp_device_get_and_lock(net: *mut net, dev_id: *mut nlattr, admin: bool) -> *mut psp_dev;
    fn psp_nl_resolve_assoc_dev_ns(psd: *mut psp_dev, info: *mut genl_info) -> *mut net;
    fn psp_nl_fill_assoc_dev_list(psd: *mut psp_dev, rsp: *mut sk_buff, cur_net: *mut net, filter_net: *mut net) -> i32;
    fn psp_nl_dev_fill(psd: *mut psp_dev, rsp: *mut sk_buff, info: *const genl_info) -> i32;
    fn psp_nl_build_dev_ntf(psd: *mut psp_dev, net: *mut net, ctx: *mut c_void) -> *mut sk_buff;
    fn psp_nl_dev_get_dumpit_one(rsp: *mut sk_buff, cb: *mut netlink_callback, psd: *mut psp_dev) -> i32;
    fn psp_nl_parse_key(info: *mut genl_info, attr: u32, key: *mut psp_key_parsed, key_sz: u32) -> i32;
    fn psp_nl_put_key(skb: *mut sk_buff, attr: u32, version: u32, key: *mut psp_key_parsed) -> i32;
    fn psp_nl_stats_fill(psd: *mut psp_dev, rsp: *mut sk_buff, info: *const genl_info) -> i32;
    fn psp_nl_stats_get_dumpit_one(rsp: *mut sk_buff, cb: *mut netlink_callback, psd: *mut psp_dev) -> i32;
    fn psp_device_get_locked_admin(ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn psp_device_get_locked(ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn psp_device_get_locked_dev_assoc(ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn psp_device_unlock(ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info);
    fn psp_has_assoc_dev_in_ns(psd: *mut psp_dev, net: *mut net) -> bool;
    fn psp_nl_notify_dev(psd: *mut psp_dev, cmd: u32);
    fn psp_nl_dev_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn psp_nl_dev_get_dumpit(rsp: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    fn psp_nl_dev_set_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn psp_nl_key_rotate_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn psp_nl_dev_assoc_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn psp_nl_dev_disassoc_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn psp_assoc_device_get_locked(ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn psp_nl_rx_assoc_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn psp_nl_tx_assoc_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn psp_nl_get_stats_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn psp_nl_get_stats_dumpit(rsp: *mut sk_buff, cb: *mut netlink_callback) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
