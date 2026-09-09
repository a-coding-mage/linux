/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * drivers/net/bond/bond_options.h - bonding options
 * Copyright (c) 2013 Nikolay Aleksandrov <nikolay@redhat.com>
 */

use core::ffi::c_char;
use core::mem::size_of;
use core::ptr;

/* C dependencies: struct netlink_ext_ack, struct nlattr, struct net_device,
 * and struct slave are supplied by other translation units. */
#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct nlattr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct slave {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bonding {
    _private: [u8; 0],
}

pub const BOND_OPT_MAX_NAMELEN: usize = 32;
#[inline]
pub const fn BOND_OPT_VALID(opt: u32) -> bool { opt < BOND_OPT_LAST }
#[inline]
pub const fn BOND_MODE_ALL_EX(x: u64) -> u64 { !x }

pub const BOND_OPTFLAG_NOSLAVES: u32 = 1 << 0;
pub const BOND_OPTFLAG_IFDOWN: u32 = 1 << 1;
pub const BOND_OPTFLAG_RAWVAL: u32 = 1 << 2;

pub const BOND_VALFLAG_DEFAULT: u32 = 1 << 0;
pub const BOND_VALFLAG_MIN: u32 = 1 << 1;
pub const BOND_VALFLAG_MAX: u32 = 1 << 2;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BondOptId {
    BOND_OPT_MODE,
    BOND_OPT_PACKETS_PER_SLAVE,
    BOND_OPT_XMIT_HASH,
    BOND_OPT_ARP_VALIDATE,
    BOND_OPT_ARP_ALL_TARGETS,
    BOND_OPT_FAIL_OVER_MAC,
    BOND_OPT_ARP_INTERVAL,
    BOND_OPT_ARP_TARGETS,
    BOND_OPT_DOWNDELAY,
    BOND_OPT_UPDELAY,
    BOND_OPT_LACP_RATE,
    BOND_OPT_MINLINKS,
    BOND_OPT_AD_SELECT,
    BOND_OPT_NUM_PEER_NOTIF,
    BOND_OPT_MIIMON,
    BOND_OPT_PRIMARY,
    BOND_OPT_PRIMARY_RESELECT,
    BOND_OPT_USE_CARRIER,
    BOND_OPT_ACTIVE_SLAVE,
    BOND_OPT_QUEUE_ID,
    BOND_OPT_ALL_SLAVES_ACTIVE,
    BOND_OPT_RESEND_IGMP,
    BOND_OPT_LP_INTERVAL,
    BOND_OPT_SLAVES,
    BOND_OPT_TLB_DYNAMIC_LB,
    BOND_OPT_AD_ACTOR_SYS_PRIO,
    BOND_OPT_AD_ACTOR_SYSTEM,
    BOND_OPT_AD_USER_PORT_KEY,
    BOND_OPT_NUM_PEER_NOTIF_ALIAS,
    BOND_OPT_PEER_NOTIF_DELAY,
    BOND_OPT_LACP_ACTIVE,
    BOND_OPT_MISSED_MAX,
    BOND_OPT_NS_TARGETS,
    BOND_OPT_PRIO,
    BOND_OPT_COUPLED_CONTROL,
    BOND_OPT_BROADCAST_NEIGH,
    BOND_OPT_ACTOR_PORT_PRIO,
    BOND_OPT_LACP_STRICT,
    BOND_OPT_LAST,
}
pub const BOND_OPT_LAST: u32 = BondOptId::BOND_OPT_LAST as u32;

pub const BOND_OPT_EXTRA_MAXLEN: usize = 16;

#[repr(C)]
pub union bond_opt_value_extra {
    pub extra: [c_char; BOND_OPT_EXTRA_MAXLEN],
    pub slave_dev: *mut net_device,
}

#[repr(C)]
pub struct bond_opt_value {
    pub string: *mut c_char,
    pub value: u64,
    pub flags: u32,
    pub extra: bond_opt_value_extra,
}

pub type BondOptionSet = unsafe extern "C" fn(*mut bonding, *const bond_opt_value) -> i32;

#[repr(C)]
pub struct bond_option {
    pub id: i32,
    pub name: *const c_char,
    pub desc: *const c_char,
    pub flags: u32,
    pub unsuppmodes: usize,
    pub values: *const bond_opt_value,
    pub set: Option<BondOptionSet>,
}

#[inline]
pub unsafe fn __bond_opt_init(
    optval: *mut bond_opt_value,
    string: *mut c_char,
    value: u64,
    extra: *const core::ffi::c_void,
    extra_len: usize,
) {
    ptr::write_bytes(optval.cast::<u8>(), 0, size_of::<bond_opt_value>());
    (*optval).value = u64::MAX;
    if value != u64::MAX {
        (*optval).value = value;
    } else if !string.is_null() {
        (*optval).string = string;
    }
    if !extra.is_null() && extra_len <= BOND_OPT_EXTRA_MAXLEN {
        ptr::copy_nonoverlapping(
            extra.cast::<u8>(),
            (*optval).extra.extra.as_mut_ptr().cast::<u8>(),
            extra_len,
        );
    }
}

#[inline]
pub unsafe fn bond_opt_initval(optval: *mut bond_opt_value, value: u64) {
    __bond_opt_init(optval, ptr::null_mut(), value, ptr::null(), 0)
}
#[inline]
pub unsafe fn bond_opt_initstr(optval: *mut bond_opt_value, str_: *mut c_char) {
    __bond_opt_init(optval, str_, u64::MAX, ptr::null(), 0)
}
#[inline]
pub unsafe fn bond_opt_initextra(optval: *mut bond_opt_value, extra: *const core::ffi::c_void, extra_len: usize) {
    __bond_opt_init(optval, ptr::null_mut(), u64::MAX, extra, extra_len)
}
#[inline]
pub unsafe fn bond_opt_slave_initval(optval: *mut bond_opt_value, slave_dev: *mut net_device, value: u64) {
    __bond_opt_init(optval, ptr::null_mut(), value, (&slave_dev as *const *mut net_device).cast(), size_of::<*mut net_device>())
}

unsafe extern "C" {
    pub fn __bond_opt_set(
        bond: *mut bonding, option: u32, val: *mut bond_opt_value,
        bad_attr: *mut nlattr, extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn __bond_opt_set_notify(bond: *mut bonding, option: u32, val: *mut bond_opt_value) -> i32;
    pub fn bond_opt_tryset_rtnl(bond: *mut bonding, option: u32, buf: *mut c_char) -> i32;
    pub fn bond_opt_parse(opt: *const bond_option, val: *mut bond_opt_value) -> *const bond_opt_value;
    pub fn bond_opt_get(option: u32) -> *const bond_option;
    pub fn bond_opt_get_by_name(name: *const c_char) -> *const bond_option;
    pub fn bond_opt_get_val(option: u32, val: u64) -> *const bond_opt_value;
    pub fn bond_option_arp_ip_targets_clear(bond: *mut bonding);
    /* Corresponds to #if IS_ENABLED(CONFIG_IPV6). */
    #[cfg(feature = "CONFIG_IPV6")]
    pub fn bond_option_ns_ip6_targets_clear(bond: *mut bonding);
    pub fn bond_slave_ns_maddrs_add(bond: *mut bonding, slave: *mut slave);
    pub fn bond_slave_ns_maddrs_del(bond: *mut bonding, slave: *mut slave);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
