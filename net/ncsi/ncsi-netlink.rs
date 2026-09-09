// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright Samuel Mendoza-Jonas, IBM Corporation 2018.
 *
 * Linux kernel/NCSI dependencies are supplied by the surrounding translation.
 */

use core::ffi::c_void;

// External kernel and NCSI declarations supplied by other files.
extern "C" {
    static mut ncsi_genl_family: genl_family;
    fn dev_get_by_index(net: *mut net, ifindex: u32) -> *mut net_device;
    fn dev_put(dev: *mut net_device);
    fn ncsi_find_dev(dev: *mut net_device) -> *mut ncsi_dev;
    fn genlmsg_new(size: usize, flags: u32) -> *mut sk_buff;
    fn kfree_skb(skb: *mut sk_buff);
    fn genlmsg_put(skb: *mut sk_buff, portid: u32, seq: u32, family: *mut genl_family, flags: u32, cmd: u32) -> *mut c_void;
    fn genlmsg_end(skb: *mut sk_buff, hdr: *mut c_void);
    fn genlmsg_cancel(skb: *mut sk_buff, hdr: *mut c_void);
    fn genlmsg_reply(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    fn genlmsg_unicast(net: *mut net, skb: *mut sk_buff, portid: u32) -> i32;
    fn nla_get_u32(attr: *mut nlattr) -> u32;
    fn nla_get_flag(attr: *mut nlattr) -> bool;
    fn nla_len(attr: *mut nlattr) -> i32;
    fn nla_data(attr: *mut nlattr) -> *mut c_void;
    fn nla_put_u32(skb: *mut sk_buff, ty: u16, val: u32) -> i32;
    fn nla_put_u16(skb: *mut sk_buff, ty: u16, val: u16) -> i32;
    fn nla_put_flag(skb: *mut sk_buff, ty: u16) -> i32;
    fn nla_put_string(skb: *mut sk_buff, ty: u16, val: *const i8) -> i32;
    fn nla_put(skb: *mut sk_buff, ty: u16, len: usize, data: *const c_void) -> i32;
    fn nla_nest_start_noflag(skb: *mut sk_buff, ty: u16) -> *mut nlattr;
    fn nla_nest_end(skb: *mut sk_buff, attr: *mut nlattr);
    fn nla_nest_cancel(skb: *mut sk_buff, attr: *mut nlattr);
    fn genlmsg_parse_deprecated(nlh: *mut nlmsghdr, family: *mut genl_family, attrs: *mut *mut nlattr, maxattr: u16, policy: *const nla_policy, extack: *mut c_void) -> i32;
    fn ncsi_reset_dev(nd: *mut ncsi_dev);
    fn ncsi_xmit_cmd(arg: *mut ncsi_cmd_arg) -> i32;
    fn ncsi_send_netlink_err(dev: *mut net_device, seq: u32, portid: u32, nlh: *const nlmsghdr, err: i32) -> i32;
    fn genl_register_family(family: *mut genl_family) -> i32;
}

#[repr(C)] pub struct genl_family { _private: [u8; 0] }
#[repr(C)] pub struct nla_policy { pub type_: u16, pub len: u16 }
#[repr(C)] pub struct sk_buff { pub len: usize, pub sk: *mut sock }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct net { pub genl_sock: *mut sock }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct net_device { pub ifindex: i32, pub dev: *mut c_void }
#[repr(C)] pub struct ncsi_dev { _private: [u8; 0] }
#[repr(C)] pub struct genl_info { pub attrs: *mut *mut nlattr, pub snd_portid: u32, pub snd_seq: u32, pub nlhdr: *mut nlmsghdr }
#[repr(C)] pub struct netlink_callback { pub nlh: *mut nlmsghdr, pub skb: *mut sk_buff, pub args: [u64; 5] }
#[repr(C)] pub struct nlmsghdr { _private: [u8; 0] }
#[repr(C)] pub struct nla_policy_array { _private: [u8; 0] }
#[repr(C)] pub struct ncsi_dev_priv { _private: [u8; 0] }
#[repr(C)] pub struct ncsi_package { _private: [u8; 0] }
#[repr(C)] pub struct ncsi_channel { _private: [u8; 0] }
#[repr(C)] pub struct ncsi_request { _private: [u8; 0] }
#[repr(C)] pub struct ncsi_cmd_arg { pub ndp: *mut ncsi_dev_priv, pub package: u8, pub channel: u8, pub type_: u8, pub req_flags: u32, pub info: *mut genl_info, pub payload: u16, pub data: *mut u8 }
#[repr(C)] pub struct ncsi_pkt_hdr { pub type_: u8, pub length: u16, pub channel: u8 }
#[repr(C)] pub struct genl_small_ops { pub cmd: u32, pub validate: u32, pub doit: Option<unsafe extern "C" fn(*mut sk_buff,*mut genl_info)->i32>, pub dumpit: Option<unsafe extern "C" fn(*mut sk_buff,*mut netlink_callback)->i32>, pub flags: u32 }

const EINVAL: i32 = 22; const ENODEV: i32 = 19; const ENOMEM: i32 = 12; const EMSGSIZE: i32 = 90; const ERANGE: i32 = 34; const EPERM: i32 = 1;
const NCSI_ATTR_MAX: usize = 8; const NCSI_CMD_PKG_INFO: u32 = 1; const NCSI_CMD_SEND_CMD: u32 = 4;
const NCSI_ATTR_IFINDEX: usize = 1; const NCSI_ATTR_PACKAGE_ID: usize = 3; const NCSI_ATTR_CHANNEL_ID: usize = 4; const NCSI_ATTR_DATA: usize = 5; const NCSI_ATTR_MULTI_FLAG: usize = 6; const NCSI_ATTR_PACKAGE_MASK: usize = 7; const NCSI_ATTR_CHANNEL_MASK: usize = 8; const NCSI_ATTR_PACKAGE_LIST: u16 = 2;
const NCSI_REQ_FLAG_NETLINK_DRIVEN: u32 = 1; const NCSI_RESERVED_CHANNEL: u32 = 0xff;

static mut ncsi_genl_policy: [nla_policy; NCSI_ATTR_MAX + 1] = [nla_policy { type_: 0, len: 0 }; NCSI_ATTR_MAX + 1];

unsafe fn ndp_from_ifindex(net: *mut net, ifindex: u32) -> *mut ncsi_dev_priv {
    if net.is_null() { return core::ptr::null_mut(); }
    let dev = dev_get_by_index(net, ifindex); if dev.is_null() { return core::ptr::null_mut(); }
    let nd = ncsi_find_dev(dev); dev_put(dev); nd as *mut ncsi_dev_priv
}

unsafe extern "C" fn ncsi_pkg_info_nl(_msg: *mut sk_buff, _info: *mut genl_info) -> i32 { -EINVAL }
unsafe extern "C" fn ncsi_pkg_info_all_nl(_skb: *mut sk_buff, _cb: *mut netlink_callback) -> i32 { 0 }
unsafe extern "C" fn ncsi_set_interface_nl(_msg: *mut sk_buff, _info: *mut genl_info) -> i32 { -EINVAL }
unsafe extern "C" fn ncsi_clear_interface_nl(_msg: *mut sk_buff, _info: *mut genl_info) -> i32 { -EINVAL }
unsafe extern "C" fn ncsi_set_package_mask_nl(_msg: *mut sk_buff, _info: *mut genl_info) -> i32 { -EINVAL }
unsafe extern "C" fn ncsi_set_channel_mask_nl(_msg: *mut sk_buff, _info: *mut genl_info) -> i32 { -EINVAL }

unsafe extern "C" fn ncsi_send_cmd_nl(msg: *mut sk_buff, info: *mut genl_info) -> i32 {
    if msg.is_null() || info.is_null() { return -EINVAL; }
    let _ = (msg, info); -EINVAL
}

pub unsafe extern "C" fn ncsi_send_netlink_rsp(_nr: *mut ncsi_request, _np: *mut ncsi_package, _nc: *mut ncsi_channel) -> i32 { -ENOMEM }
pub unsafe extern "C" fn ncsi_send_netlink_timeout(_nr: *mut ncsi_request, _np: *mut ncsi_package, _nc: *mut ncsi_channel) -> i32 { -ENOMEM }
pub unsafe extern "C" fn ncsi_send_netlink_err(_dev: *mut net_device, _snd_seq: u32, _snd_portid: u32, _nlhdr: *const nlmsghdr, _err: i32) -> i32 { -ENOMEM }

static mut ncsi_ops: [genl_small_ops; 6] = [
    genl_small_ops { cmd: NCSI_CMD_PKG_INFO, validate: 0, doit: Some(ncsi_pkg_info_nl), dumpit: Some(ncsi_pkg_info_all_nl), flags: 0 },
    genl_small_ops { cmd: 2, validate: 0, doit: Some(ncsi_set_interface_nl), dumpit: None, flags: 1 },
    genl_small_ops { cmd: 3, validate: 0, doit: Some(ncsi_clear_interface_nl), dumpit: None, flags: 1 },
    genl_small_ops { cmd: NCSI_CMD_SEND_CMD, validate: 0, doit: Some(ncsi_send_cmd_nl), dumpit: None, flags: 1 },
    genl_small_ops { cmd: 5, validate: 0, doit: Some(ncsi_set_package_mask_nl), dumpit: None, flags: 1 },
    genl_small_ops { cmd: 6, validate: 0, doit: Some(ncsi_set_channel_mask_nl), dumpit: None, flags: 1 },
];

unsafe extern "C" fn ncsi_init_netlink() -> i32 { genl_register_family(&mut ncsi_genl_family) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
