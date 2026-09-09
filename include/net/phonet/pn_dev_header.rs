/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * File: pn_dev.h
 *
 * Phonet network device
 *
 * Copyright (C) 2008 Nokia Corporation.
 */

// Translated from pn_dev.h. The following types are supplied by the
// corresponding Linux networking/list/spinlock dependencies.

use core::ffi::c_int;

#[repr(C)]
pub struct phonet_device_list {
    pub list: list_head,
    pub lock: spinlock_t,
}

extern "C" {
    pub fn phonet_device_list(net: *mut net) -> *mut phonet_device_list;
}

#[repr(C)]
pub struct phonet_device {
    pub list: list_head,
    pub netdev: *mut net_device,
    pub addrs: [u64; 1],
    pub rcu: rcu_head,
}

extern "C" {
    pub fn phonet_device_init() -> c_int;
    pub fn phonet_device_exit();
    pub fn phonet_netlink_register() -> c_int;
    pub fn phonet_device_get(net: *mut net) -> *mut net_device;

    pub fn phonet_address_add(dev: *mut net_device, addr: u8) -> c_int;
    pub fn phonet_address_del(dev: *mut net_device, addr: u8) -> c_int;
    pub fn phonet_address_get(dev: *mut net_device, addr: u8) -> u8;
    pub fn phonet_address_lookup(net: *mut net, addr: u8) -> c_int;
    pub fn phonet_address_notify(net: *mut net, event: c_int, ifindex: u32, addr: u8);

    pub fn phonet_route_add(dev: *mut net_device, daddr: u8) -> c_int;
    pub fn phonet_route_del(dev: *mut net_device, daddr: u8) -> c_int;
    pub fn rtm_phonet_notify(net: *mut net, event: c_int, ifindex: u32, dst: u8);
    pub fn phonet_route_get_rcu(net: *mut net, daddr: u8) -> *mut net_device;
    pub fn phonet_route_output(net: *mut net, daddr: u8) -> *mut net_device;
}

pub const PN_NO_ADDR: u8 = 0xff;

extern "C" {
    pub static pn_sock_seq_ops: seq_operations;
    pub static pn_res_seq_ops: seq_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
