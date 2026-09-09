/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations from <linux/rtnetlink.h>, <linux/srcu.h>, and <net/netlink.h>
 * are supplied by the surrounding translation unit. */

pub type RtnlDoitFunc = unsafe extern "C" fn(
    skb: *mut sk_buff,
    nlh: *mut nlmsghdr,
    extack: *mut netlink_ext_ack,
) -> ::core::ffi::c_int;
pub type RtnlDumpitFunc = unsafe extern "C" fn(
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
) -> ::core::ffi::c_int;

#[repr(C)]
pub enum rtnl_link_flags {
    RTNL_FLAG_DOIT_UNLOCKED = 1 << 0,
    /* RTNL_FLAG_DOIT_PERNET and RTNL_FLAG_DOIT_PERNET_WIP alias DOIT_UNLOCKED. */
    RTNL_FLAG_BULK_DEL_SUPPORTED = 1 << 1,
    RTNL_FLAG_DUMP_UNLOCKED = 1 << 2,
    RTNL_FLAG_DUMP_SPLIT_NLM_DONE = 1 << 3, /* legacy behavior */
}

#[repr(C)]
pub enum rtnl_kinds {
    RTNL_KIND_NEW,
    RTNL_KIND_DEL,
    RTNL_KIND_GET,
    RTNL_KIND_SET,
}

pub const RTNL_KIND_MASK: ::core::ffi::c_int = 0x3;

#[inline]
pub unsafe fn rtnl_msgtype_kind(msgtype: ::core::ffi::c_int) -> ::core::ffi::c_int {
    msgtype & RTNL_KIND_MASK
}

#[repr(C)]
pub struct rtnl_msg_handler {
    pub owner: *mut module,
    pub protocol: ::core::ffi::c_int,
    pub msgtype: ::core::ffi::c_int,
    pub doit: Option<RtnlDoitFunc>,
    pub dumpit: Option<RtnlDumpitFunc>,
    pub flags: ::core::ffi::c_int,
}

unsafe extern "C" {
    pub fn rtnl_unregister_all(protocol: ::core::ffi::c_int);
    pub fn __rtnl_register_many(handlers: *const rtnl_msg_handler, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn __rtnl_unregister_many(handlers: *const rtnl_msg_handler, n: ::core::ffi::c_int);
}

/* C macros: ARRAY_SIZE(handlers) is intentionally left to the caller's Rust slice/array length. */

#[inline]
pub unsafe fn rtnl_msg_family(nlh: *const nlmsghdr) -> ::core::ffi::c_int {
    if nlmsg_len(nlh) >= ::core::mem::size_of::<rtgenmsg>() {
        (*(nlmsg_data(nlh) as *mut rtgenmsg)).rtgen_family as ::core::ffi::c_int
    } else {
        AF_UNSPEC
    }
}

#[repr(C)]
pub struct rtnl_newlink_params {
    pub src_net: *mut net,
    pub link_net: *mut net,
    pub peer_net: *mut net,
    pub tb: *mut *mut nlattr,
    pub data: *mut *mut nlattr,
}

#[inline]
pub unsafe fn rtnl_newlink_link_net(p: *mut rtnl_newlink_params) -> *mut net {
    if !(*p).link_net.is_null() { (*p).link_net } else { (*p).src_net }
}

#[inline]
pub unsafe fn rtnl_newlink_peer_net(p: *mut rtnl_newlink_params) -> *mut net {
    if !(*p).peer_net.is_null() { (*p).peer_net } else { rtnl_newlink_link_net(p) }
}

#[repr(C)]
pub struct rtnl_link_ops {
    pub list: list_head,
    pub srcu: srcu_struct,
    pub kind: *const ::core::ffi::c_char,
    pub priv_size: usize,
    pub alloc: Option<unsafe extern "C" fn(*mut nlattr, *const ::core::ffi::c_char, u8, u32, u32) -> *mut net_device>,
    pub setup: Option<unsafe extern "C" fn(*mut net_device)>,
    pub netns_refund: bool,
    pub peer_type: u16,
    pub maxtype: u32,
    pub policy: *const nla_policy,
    pub validate: Option<unsafe extern "C" fn(*mut *mut nlattr, *mut *mut nlattr, *mut netlink_ext_ack) -> ::core::ffi::c_int>,
    pub newlink: Option<unsafe extern "C" fn(*mut net_device, *mut rtnl_newlink_params, *mut netlink_ext_ack) -> ::core::ffi::c_int>,
    pub changelink: Option<unsafe extern "C" fn(*mut net_device, *mut *mut nlattr, *mut *mut nlattr, *mut netlink_ext_ack) -> ::core::ffi::c_int>,
    pub dellink: Option<unsafe extern "C" fn(*mut net_device, *mut list_head)>,
    pub get_size: Option<unsafe extern "C" fn(*const net_device) -> usize>,
    pub fill_info: Option<unsafe extern "C" fn(*mut sk_buff, *const net_device) -> ::core::ffi::c_int>,
    pub get_xstats_size: Option<unsafe extern "C" fn(*const net_device) -> usize>,
    pub fill_xstats: Option<unsafe extern "C" fn(*mut sk_buff, *const net_device) -> ::core::ffi::c_int>,
    pub get_num_tx_queues: Option<unsafe extern "C" fn() -> u32>,
    pub get_num_rx_queues: Option<unsafe extern "C" fn() -> u32>,
    pub slave_maxtype: u32,
    pub slave_policy: *const nla_policy,
    pub slave_changelink: Option<unsafe extern "C" fn(*mut net_device, *mut net_device, *mut *mut nlattr, *mut *mut nlattr, *mut netlink_ext_ack) -> ::core::ffi::c_int>,
    pub get_slave_size: Option<unsafe extern "C" fn(*const net_device, *const net_device) -> usize>,
    pub fill_slave_info: Option<unsafe extern "C" fn(*mut sk_buff, *const net_device, *const net_device) -> ::core::ffi::c_int>,
    pub get_link_net: Option<unsafe extern "C" fn(*const net_device) -> *mut net>,
    pub get_linkxstats_size: Option<unsafe extern "C" fn(*const net_device, ::core::ffi::c_int) -> usize>,
    pub fill_linkxstats: Option<unsafe extern "C" fn(*mut sk_buff, *const net_device, *mut ::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct rtnl_af_ops {
    pub list: list_head,
    pub srcu: srcu_struct,
    pub family: ::core::ffi::c_int,
    pub fill_link_af: Option<unsafe extern "C" fn(*mut sk_buff, *const net_device, u32) -> ::core::ffi::c_int>,
    pub get_link_af_size: Option<unsafe extern "C" fn(*const net_device, u32) -> usize>,
    pub validate_link_af: Option<unsafe extern "C" fn(*const net_device, *const nlattr, *mut netlink_ext_ack) -> ::core::ffi::c_int>,
    pub set_link_af: Option<unsafe extern "C" fn(*mut net_device, *const nlattr, *mut netlink_ext_ack) -> ::core::ffi::c_int>,
    pub fill_stats_af: Option<unsafe extern "C" fn(*mut sk_buff, *const net_device) -> ::core::ffi::c_int>,
    pub get_stats_af_size: Option<unsafe extern "C" fn(*const net_device) -> usize>,
}

unsafe extern "C" {
    pub fn rtnl_link_register(ops: *mut rtnl_link_ops) -> ::core::ffi::c_int;
    pub fn rtnl_link_unregister(ops: *mut rtnl_link_ops);
    pub fn rtnl_af_register(ops: *mut rtnl_af_ops) -> ::core::ffi::c_int;
    pub fn rtnl_af_unregister(ops: *mut rtnl_af_ops);
    pub fn rtnl_link_get_net(src_net: *mut net, tb: *mut *mut nlattr) -> *mut net;
    pub fn rtnl_create_link(net: *mut net, ifname: *const ::core::ffi::c_char, name_assign_type: u8, ops: *const rtnl_link_ops, tb: *mut *mut nlattr, extack: *mut netlink_ext_ack) -> *mut net_device;
    pub fn rtnl_delete_link(dev: *mut net_device, portid: u32, nlh: *const nlmsghdr) -> ::core::ffi::c_int;
    pub fn rtnl_configure_link(dev: *mut net_device, ifm: *const ifinfomsg, portid: u32, nlh: *const nlmsghdr) -> ::core::ffi::c_int;
    pub fn rtnl_nla_parse_ifinfomsg(tb: *mut *mut nlattr, nla_peer: *const nlattr, exterr: *mut netlink_ext_ack) -> ::core::ffi::c_int;
    pub fn rtnl_get_net_ns_capable(sk: *mut sock, netnsid: ::core::ffi::c_int) -> *mut net;
    pub fn rtnl_dev_link_net_capable(dev: *const net_device, link_net: *const net) -> bool;
}

/* MODULE_ALIAS_RTNL_LINK(kind) expands to MODULE_ALIAS("rtnl-link-" kind). */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
