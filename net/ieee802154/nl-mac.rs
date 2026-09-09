// SPDX-License-Identifier: GPL-2.0-only
// Low-level Rust translation of the Linux IEEE 802.15.4 netlink implementation.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct genl_info { _private: [u8; 0] }
#[repr(C)] pub struct netlink_callback { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }

extern "C" {
    fn ieee802154_devaddr_from_raw(data: *const c_void) -> u64;
    fn nla_data(attr: *const nlattr) -> *const c_void;
    fn nla_get_u16(attr: *const nlattr) -> u16;
    fn nla_put_u16(msg: *mut sk_buff, ty: i32, value: u16) -> i32;
    fn nla_put_u64_64bit(msg: *mut sk_buff, ty: i32, value: u64, pad: i32) -> i32;
    fn swab64(value: u64) -> u64;
    fn le16_to_cpu(value: u16) -> u16;
    fn cpu_to_le16(value: u16) -> u16;
}

#[inline]
unsafe fn nla_put_hwaddr(msg: *mut sk_buff, ty: i32, hwaddr: u64, padattr: i32) -> i32 {
    nla_put_u64_64bit(msg, ty, swab64(hwaddr), padattr)
}

#[inline]
unsafe fn nla_get_hwaddr(nla: *const nlattr) -> u64 {
    ieee802154_devaddr_from_raw(nla_data(nla))
}

#[inline]
unsafe fn nla_put_shortaddr(msg: *mut sk_buff, ty: i32, addr: u16) -> i32 {
    nla_put_u16(msg, ty, le16_to_cpu(addr))
}

#[inline]
unsafe fn nla_get_shortaddr(nla: *const nlattr) -> u16 {
    cpu_to_le16(nla_get_u16(nla))
}

// External kernel entry points and structures used by the translated implementation.
// Their definitions are supplied by the surrounding kernel translation unit.
pub unsafe fn ieee802154_associate_req(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_associate_resp(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_disassociate_req(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_start_req(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_scan_req(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_list_iface(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_dump_iface(_skb: *mut sk_buff, _cb: *mut netlink_callback) -> i32 { todo!() }
pub unsafe fn ieee802154_set_macparams(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_getparams(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_setparams(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_add_key(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_del_key(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_dump_keys(_skb: *mut sk_buff, _cb: *mut netlink_callback) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_add_dev(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_del_dev(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_dump_devs(_skb: *mut sk_buff, _cb: *mut netlink_callback) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_add_devkey(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_del_devkey(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_dump_devkeys(_skb: *mut sk_buff, _cb: *mut netlink_callback) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_add_seclevel(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_del_seclevel(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { todo!() }
pub unsafe fn ieee802154_llsec_dump_seclevels(_skb: *mut sk_buff, _cb: *mut netlink_callback) -> i32 { todo!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
