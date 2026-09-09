/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 1997 by Procom Technology, Inc.
 * 		 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

/* Dependencies supplied by the surrounding kernel translation. */

use core::ffi::c_int;

pub const IFHWADDRLEN: usize = 6;

pub const LLC_SAP_STATE_INACTIVE: u8 = 1;
pub const LLC_SAP_STATE_ACTIVE: u8 = 2;

pub const LLC_SK_DEV_HASH_BITS: u32 = 6;
pub const LLC_SK_DEV_HASH_ENTRIES: usize = 1usize << LLC_SK_DEV_HASH_BITS;

pub const LLC_SK_LADDR_HASH_BITS: u32 = 6;
pub const LLC_SK_LADDR_HASH_ENTRIES: usize = 1usize << LLC_SK_LADDR_HASH_BITS;

#[repr(C)]
pub struct llc_addr {
    pub lsap: u8,
    pub mac: [u8; IFHWADDRLEN],
}

/**
 * struct llc_sap - Defines the SAP component
 *
 * @station - station this sap belongs to
 * @state - sap state
 * @p_bit - only lowest-order bit used
 * @f_bit - only lowest-order bit used
 * @laddr - SAP value in this 'lsap'
 * @node - entry in station sap_list
 * @sk_list - LLC sockets this one manages
 */
#[repr(C)]
pub struct llc_sap {
    pub state: u8,
    pub p_bit: u8,
    pub f_bit: u8,
    pub refcnt: refcount_t,
    pub rcv_func: Option<unsafe extern "C" fn(
        skb: *mut sk_buff,
        dev: *mut net_device,
        pt: *mut packet_type,
        orig_dev: *mut net_device,
    ) -> c_int>,
    pub laddr: llc_addr,
    pub node: list_head,
    pub sk_lock: spinlock_t,
    pub sk_count: c_int,
    pub sk_laddr_hash: [hlist_nulls_head; LLC_SK_LADDR_HASH_ENTRIES],
    pub sk_dev_hash: [hlist_head; LLC_SK_DEV_HASH_ENTRIES],
    pub rcu: rcu_head,
}

#[inline]
pub unsafe fn llc_sk_dev_hash(sap: *mut llc_sap, ifindex: c_int) -> *mut hlist_head {
    let bucket = hash_32(ifindex as u32, LLC_SK_DEV_HASH_BITS) as usize;
    (*sap).sk_dev_hash.as_mut_ptr().add(bucket)
}

#[inline]
pub unsafe fn llc_sk_laddr_hashfn(_sap: *mut llc_sap, laddr: *const llc_addr) -> u32 {
    hash_32(jhash((*laddr).mac.as_ptr(), core::mem::size_of_val(&(*laddr).mac), 0),
            LLC_SK_LADDR_HASH_BITS)
}

#[inline]
pub unsafe fn llc_sk_laddr_hash(
    sap: *mut llc_sap,
    laddr: *const llc_addr,
) -> *mut hlist_nulls_head {
    (*sap).sk_laddr_hash.as_mut_ptr().add(llc_sk_laddr_hashfn(sap, laddr) as usize)
}

pub const LLC_DEST_INVALID: c_int = 0;
pub const LLC_DEST_SAP: c_int = 1;
pub const LLC_DEST_CONN: c_int = 2;

extern "C" {
    pub static mut llc_sap_list: list_head;

    pub fn llc_rcv(skb: *mut sk_buff, dev: *mut net_device, pt: *mut packet_type,
                   orig_dev: *mut net_device) -> c_int;
    pub fn llc_mac_hdr_init(skb: *mut sk_buff, sa: *const u8, da: *const u8) -> c_int;
    pub fn llc_add_pack(type_: c_int, handler: Option<unsafe extern "C" fn(*mut llc_sap, *mut sk_buff)>);
    pub fn llc_remove_pack(type_: c_int);
    pub fn llc_set_station_handler(handler: Option<unsafe extern "C" fn(*mut sk_buff)>);
    pub fn llc_sap_open(lsap: u8, rcv: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device,
                                                                      *mut packet_type, *mut net_device) -> c_int>) -> *mut llc_sap;

    pub fn llc_sap_close(sap: *mut llc_sap);
    pub fn llc_sap_find(sap_value: u8) -> *mut llc_sap;
    pub fn llc_build_and_send_ui_pkt(sap: *mut llc_sap, skb: *mut sk_buff,
                                     dmac: *const u8, dsap: u8) -> c_int;
    pub fn llc_sap_handler(sap: *mut llc_sap, skb: *mut sk_buff);
    pub fn llc_conn_handler(sap: *mut llc_sap, skb: *mut sk_buff);
    pub fn llc_station_init();
    pub fn llc_station_exit();
}

#[inline]
pub unsafe fn llc_sap_hold(sap: *mut llc_sap) {
    refcount_inc(&mut (*sap).refcnt);
}

#[inline]
pub unsafe fn llc_sap_hold_safe(sap: *mut llc_sap) -> bool {
    refcount_inc_not_zero(&mut (*sap).refcnt)
}

#[inline]
pub unsafe fn llc_sap_put(sap: *mut llc_sap) {
    if refcount_dec_and_test(&mut (*sap).refcnt) {
        llc_sap_close(sap);
    }
}

#[cfg(CONFIG_PROC_FS)]
extern "C" {
    pub fn llc_proc_init() -> c_int;
    pub fn llc_proc_exit();
}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub const fn llc_proc_init() -> c_int { 0 }

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub fn llc_proc_exit() {}

#[cfg(CONFIG_SYSCTL)]
extern "C" {
    pub fn llc_sysctl_init() -> c_int;
    pub fn llc_sysctl_exit();
    pub static mut sysctl_llc2_ack_timeout: c_int;
    pub static mut sysctl_llc2_busy_timeout: c_int;
    pub static mut sysctl_llc2_p_timeout: c_int;
    pub static mut sysctl_llc2_rej_timeout: c_int;
}

#[cfg(not(CONFIG_SYSCTL))]
#[inline]
pub const fn llc_sysctl_init() -> c_int { 0 }

#[cfg(not(CONFIG_SYSCTL))]
#[inline]
pub fn llc_sysctl_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
