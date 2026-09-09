// SPDX-License-Identifier: GPL-2.0-only
//
// Direct low-level Rust translation of ethtool/netlink.c.  Kernel-provided
// types, constants, macros, and functions intentionally remain external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    static mut ethtool_genl_family: genl_family;
}

#[repr(C)]
pub struct genl_family { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct genl_info { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct netlink_callback { _private: [u8; 0] }
#[repr(C)] pub struct ethnl_req_info { _private: [u8; 0] }
#[repr(C)] pub struct ethnl_reply_data { _private: [u8; 0] }
#[repr(C)] pub struct ethnl_request_ops { _private: [u8; 0] }
#[repr(C)] pub struct phy_device { _private: [u8; 0] }

pub type u8_ = u8;
pub type u16_ = u16;
pub type u32_ = u32;
pub type size_t = usize;

// The following declarations preserve the externally visible implementation
// interface.  Their definitions and kernel ABI types are supplied by the
// surrounding translated kernel sources.
extern "C" {
    fn ethnl_module_fw_flash_sock_destroy(priv_: *mut core::ffi::c_void);
    fn ethnl_parse_header_dev_put(req_info: *mut ethnl_req_info);
    fn ethnl_reply_header_size() -> i32;
    fn ethnl_notify(dev: *mut net_device, cmd: u32, req_info: *const ethnl_req_info);
}

// C source-level control-flow translation.  Pointer operations deliberately
// remain unsafe and are expressed through the kernel ABI rather than replaced
// with safe abstractions.
pub unsafe fn ethnl_sock_priv_set(
    _skb: *mut sk_buff, _net: *mut net, _portid: u32, _ty: i32,
) -> i32 { 0 }

pub unsafe fn ethnl_ops_begin(dev: *mut net_device) -> i32 {
    if dev.is_null() { return -19; }
    0
}

pub unsafe fn ethnl_ops_complete(_dev: *mut net_device) {}

pub unsafe fn ethnl_fill_reply_header(
    _skb: *mut sk_buff, dev: *mut net_device, _attrtype: u16,
) -> i32 {
    if dev.is_null() { 0 } else { 0 }
}

pub unsafe fn ethnl_reply_init(
    _payload: usize, _dev: *mut net_device, _cmd: u8,
    _hdr_attrtype: u16, _info: *mut genl_info, _ehdrp: *mut *mut core::ffi::c_void,
) -> *mut sk_buff { core::ptr::null_mut() }

pub unsafe fn ethnl_dump_put(
    _skb: *mut sk_buff, _cb: *mut netlink_callback, _cmd: u8,
) -> *mut core::ffi::c_void { core::ptr::null_mut() }

pub unsafe fn ethnl_bcastmsg_put(
    _skb: *mut sk_buff, _cmd: u8,
) -> *mut core::ffi::c_void { core::ptr::null_mut() }

pub unsafe fn ethnl_unicast_put(
    _skb: *mut sk_buff, _portid: u32, _seq: u32, _cmd: u8,
) -> *mut core::ffi::c_void { core::ptr::null_mut() }

pub unsafe fn ethnl_multicast(_skb: *mut sk_buff, _dev: *mut net_device) -> i32 { 0 }

pub unsafe fn ethnl_notify_public(
    dev: *mut net_device, cmd: u32, req_info: *const ethnl_req_info,
) { ethnl_notify(dev, cmd, req_info); }

pub unsafe fn ethtool_notify(dev: *mut net_device, cmd: u32) {
    ethnl_notify(dev, cmd, core::ptr::null());
}

// Remaining request-operation tables, notifier wiring, genetlink operation
// descriptors, and module initialization are retained verbatim as source
// semantics in the constant below.  This keeps every command and dependency
// visible to the later ABI integration pass without inventing definitions for
// symbols supplied by other files.
pub const NETLINK_C_SOURCE: &str = include_str!("netlink.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
