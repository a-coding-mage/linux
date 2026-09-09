/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2014 Intel Corporation. All rights reserved.
 * Copyright (c) 2014 Chelsio, Inc. All rights reserved.
 */

use std::os::raw::c_char;

pub const IWPM_ULIBNAME_SIZE: usize = 32;
pub const IWPM_DEVNAME_SIZE: usize = 32;
pub const IWPM_IFNAME_SIZE: usize = 16;
pub const IWPM_IPADDR_SIZE: usize = 16;

pub const IWPM_INVALID_NLMSG_ERR: i32 = 10;
pub const IWPM_CREATE_MAPPING_ERR: i32 = 11;
pub const IWPM_DUPLICATE_MAPPING_ERR: i32 = 12;
pub const IWPM_UNKNOWN_MAPPING_ERR: i32 = 13;
pub const IWPM_CLIENT_DEV_INFO_ERR: i32 = 14;
pub const IWPM_USER_LIB_INFO_ERR: i32 = 15;
pub const IWPM_REMOTE_QUERY_REJECT: i32 = 16;

#[repr(C)]
pub struct iwpm_dev_data {
    pub dev_name: [c_char; IWPM_DEVNAME_SIZE],
    pub if_name: [c_char; IWPM_IFNAME_SIZE],
}

#[repr(C)]
pub struct iwpm_sa_data {
    pub loc_addr: sockaddr_storage,
    pub mapped_loc_addr: sockaddr_storage,
    pub rem_addr: sockaddr_storage,
    pub mapped_rem_addr: sockaddr_storage,
    pub flags: u32,
}

unsafe extern "C" {
    pub fn iwpm_init(nl_client: u8) -> i32;
    pub fn iwpm_exit(nl_client: u8) -> i32;
    pub fn iwpm_valid_pid() -> i32;
    pub fn iwpm_register_pid(pm_msg: *mut iwpm_dev_data, nl_client: u8) -> i32;
    pub fn iwpm_add_mapping(pm_msg: *mut iwpm_sa_data, nl_client: u8) -> i32;
    pub fn iwpm_add_and_query_mapping(pm_msg: *mut iwpm_sa_data, nl_client: u8) -> i32;
    pub fn iwpm_remove_mapping(local_addr: *mut sockaddr_storage, nl_client: u8) -> i32;
    pub fn iwpm_register_pid_cb(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn iwpm_add_mapping_cb(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn iwpm_add_and_query_mapping_cb(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn iwpm_remote_info_cb(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn iwpm_mapping_error_cb(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn iwpm_mapping_info_cb(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn iwpm_ack_mapping_info_cb(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn iwpm_get_remote_info(
        mapped_loc_addr: *mut sockaddr_storage,
        mapped_rem_addr: *mut sockaddr_storage,
        remote_addr: *mut sockaddr_storage,
        nl_client: u8,
    ) -> i32;
    pub fn iwpm_create_mapinfo(
        local_addr: *mut sockaddr_storage,
        mapped_addr: *mut sockaddr_storage,
        nl_client: u8,
        map_flags: u32,
    ) -> i32;
    pub fn iwpm_remove_mapinfo(
        local_addr: *mut sockaddr_storage,
        mapped_addr: *mut sockaddr_storage,
    ) -> i32;
    pub fn iwpm_hello_cb(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
