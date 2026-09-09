// SPDX-License-Identifier: GPL-2.0
/*
 * llc_core.c - Minimum needed routines for sap handling and module init/exit
 *
 * Copyright (c) 1997 by Procom Technology, Inc.
 *		 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

// Linux kernel headers supplied by other translation units.

use core::ffi::c_int;

// External types and symbols supplied by the surrounding kernel translation.
use crate::{
    llc_rcv, llc_sap, llc_sap_hold_safe, llc_sap_put, net_device, packet_type,
    sk_buff, ETH_P_802_2, GFP_ATOMIC, LLC_SAP_STATE_ACTIVE,
    LLC_SK_LADDR_HASH_ENTRIES,
};

#[no_mangle]
pub static mut llc_sap_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut llc_sap_list_lock: spinlock_t = spinlock_t { _private: [] };

extern "C" {
    fn kzalloc(size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn kfree_rcu(ptr: *mut llc_sap, rcu: *mut rcu_head);
    fn rcu_read_lock_bh();
    fn rcu_read_unlock_bh();
    fn spin_lock_bh(lock: *mut spinlock_t);
    fn spin_unlock_bh(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn refcount_set(refcnt: *mut refcount_t, value: c_int);
    fn init_hlist_nulls_head(head: *mut hlist_nulls_head, nulls: usize);
    fn list_add_tail_rcu(node: *mut list_head, head: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);
    fn dev_add_pack(pt: *mut packet_type);
    fn dev_remove_pack(pt: *mut packet_type);
    fn warn_on(condition: bool) -> bool;
}

// These kernel layout types are defined by the corresponding translated headers.
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { pub _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { pub refs: c_int }
#[repr(C)] pub struct hlist_nulls_head { pub first: *mut core::ffi::c_void }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: Option<unsafe extern "C" fn(*mut rcu_head)> }
pub type gfp_t = u32;

unsafe fn llc_sap_alloc() -> *mut llc_sap {
    let sap = kzalloc(core::mem::size_of::<llc_sap>(), GFP_ATOMIC) as *mut llc_sap;
    if !sap.is_null() {
        // sap->laddr.mac - leave as a null, it's filled by bind
        (*sap).state = LLC_SAP_STATE_ACTIVE;
        spin_lock_init(&mut (*sap).sk_lock);
        for i in 0..LLC_SK_LADDR_HASH_ENTRIES {
            init_hlist_nulls_head(&mut (*sap).sk_laddr_hash[i], i);
        }
        refcount_set(&mut (*sap).refcnt, 1);
    }
    sap
}

unsafe fn __llc_sap_find(sap_value: u8) -> *mut llc_sap {
    let mut sap: *mut llc_sap = core::ptr::null_mut();
    // list_for_each_entry(sap, &llc_sap_list, node)
    let mut pos = llc_sap_list.next;
    while !pos.is_null() && pos != &mut llc_sap_list {
        let candidate = pos as *mut llc_sap;
        if (*candidate).laddr.lsap == sap_value { sap = candidate; break; }
        pos = (*pos).next;
    }
    sap
}

#[no_mangle]
pub unsafe extern "C" fn llc_sap_find(sap_value: u8) -> *mut llc_sap {
    rcu_read_lock_bh();
    let mut sap = __llc_sap_find(sap_value);
    if sap.is_null() || !llc_sap_hold_safe(sap) { sap = core::ptr::null_mut(); }
    rcu_read_unlock_bh();
    sap
}

#[no_mangle]
pub unsafe extern "C" fn llc_sap_open(
    lsap: u8,
    func: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device, *mut packet_type, *mut net_device) -> c_int>,
) -> *mut llc_sap {
    let mut sap: *mut llc_sap = core::ptr::null_mut();
    spin_lock_bh(&mut llc_sap_list_lock);
    if !__llc_sap_find(lsap).is_null() { spin_unlock_bh(&mut llc_sap_list_lock); return sap; }
    sap = llc_sap_alloc();
    if !sap.is_null() {
        (*sap).laddr.lsap = lsap;
        (*sap).rcv_func = func;
        list_add_tail_rcu(&mut (*sap).node, &mut llc_sap_list);
    }
    spin_unlock_bh(&mut llc_sap_list_lock);
    sap
}

#[no_mangle]
pub unsafe extern "C" fn llc_sap_close(sap: *mut llc_sap) {
    warn_on((*sap).sk_count != 0);
    spin_lock_bh(&mut llc_sap_list_lock);
    list_del_rcu(&mut (*sap).node);
    spin_unlock_bh(&mut llc_sap_list_lock);
    kfree_rcu(sap, &mut (*sap).rcu);
}

static mut llc_packet_type: packet_type = packet_type { type_: u16::to_be(ETH_P_802_2), func: Some(llc_rcv) };

unsafe extern "C" fn llc_init() -> c_int { dev_add_pack(&mut llc_packet_type); 0 }
unsafe extern "C" fn llc_exit() { dev_remove_pack(&mut llc_packet_type); }

// module_init(llc_init); module_exit(llc_exit);
// EXPORT_SYMBOL(llc_sap_list); EXPORT_SYMBOL(llc_sap_find);
// EXPORT_SYMBOL(llc_sap_open); EXPORT_SYMBOL(llc_sap_close);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Procom 1997, Jay Schullist 2001, Arnaldo C. Melo 2001-2003");
// MODULE_DESCRIPTION("LLC IEEE 802.2 core support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
