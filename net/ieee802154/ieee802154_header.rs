/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2007, 2008, 2009 Siemens AG
 */

// C header guard: IEEE_802154_LOCAL_H

extern "C" {
    pub fn ieee802154_nl_init() -> ::core::ffi::c_int;
    pub fn ieee802154_nl_exit();
}

// C designated-initializer macros. The caller supplies a compatible Rust
// struct literal context and the referenced symbols.
#[macro_export]
macro_rules! IEEE802154_OP {
    ($cmd:expr, $func:expr) => {{
        .cmd = $cmd,
        .doit = $func,
        .dumpit = None,
        .flags = GENL_ADMIN_PERM,
    }};
}

#[macro_export]
macro_rules! IEEE802154_OP_RELAXED {
    ($cmd:expr, $func:expr) => {{
        .cmd = $cmd,
        .doit = $func,
        .dumpit = None,
        .flags = GENL_ADMIN_PERM,
        .validate = GENL_DONT_VALIDATE_STRICT,
    }};
}

#[macro_export]
macro_rules! IEEE802154_DUMP {
    ($cmd:expr, $func:expr, $dump:expr) => {{
        .cmd = $cmd,
        .doit = $func,
        .dumpit = $dump,
    }};
}

#[macro_export]
macro_rules! IEEE802154_DUMP_PRIV {
    ($cmd:expr, $func:expr, $dump:expr) => {{
        .cmd = $cmd,
        .doit = $func,
        .dumpit = $dump,
        .flags = GENL_ADMIN_PERM,
    }};
}

#[repr(C)]
pub struct genl_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_callback {
    _private: [u8; 0],
}

#[repr(C)]
pub struct genl_family {
    _private: [u8; 0],
}

extern "C" {
    pub fn ieee802154_nl_create(flags: ::core::ffi::c_int, req: u8) -> *mut sk_buff;
    pub fn ieee802154_nl_mcast(msg: *mut sk_buff, group: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ieee802154_nl_new_reply(
        info: *mut genl_info,
        flags: ::core::ffi::c_int,
        req: u8,
    ) -> *mut sk_buff;
    pub fn ieee802154_nl_reply(msg: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;

    pub static mut nl802154_family: genl_family;

    pub fn ieee802154_list_phy(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_dump_phy(skb: *mut sk_buff, cb: *mut netlink_callback) -> ::core::ffi::c_int;
    pub fn ieee802154_add_iface(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_del_iface(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;

    pub fn ieee802154_associate_req(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_associate_resp(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_disassociate_req(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_scan_req(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_start_req(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_list_iface(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_dump_iface(skb: *mut sk_buff, cb: *mut netlink_callback) -> ::core::ffi::c_int;
    pub fn ieee802154_set_macparams(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;

    pub fn ieee802154_llsec_getparams(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_llsec_setparams(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_llsec_add_key(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_llsec_del_key(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_llsec_dump_keys(skb: *mut sk_buff, cb: *mut netlink_callback) -> ::core::ffi::c_int;
    pub fn ieee802154_llsec_add_dev(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_llsec_del_dev(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_llsec_dump_devs(skb: *mut sk_buff, cb: *mut netlink_callback) -> ::core::ffi::c_int;
    pub fn ieee802154_llsec_add_devkey(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_llsec_del_devkey(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_llsec_dump_devkeys(skb: *mut sk_buff, cb: *mut netlink_callback) -> ::core::ffi::c_int;
    pub fn ieee802154_llsec_add_seclevel(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_llsec_del_seclevel(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn ieee802154_llsec_dump_seclevels(skb: *mut sk_buff, cb: *mut netlink_callback) -> ::core::ffi::c_int;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ieee802154_mcgrp_ids {
    IEEE802154_COORD_MCGRP,
    IEEE802154_BEACON_MCGRP,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
