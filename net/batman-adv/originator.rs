// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 * Marek Lindner, Simon Wunderlich
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Linux/batman-adv declarations are supplied by the surrounding translation. */
use core::ffi::c_void;

extern "C" {
    static mut batadv_event_workqueue: *mut c_void;
    static mut batadv_orig_hash_lock_class_key: c_void;
}

/* The following opaque declarations intentionally retain the C ABI and layout
 * relationships expected by the other translated batman-adv units. */
#[repr(C)] pub struct batadv_priv { pub orig_hash: *mut batadv_hashtable, pub orig_work: work_struct, pub algo_ops: *mut batadv_algo_ops, pub mesh_iface: *mut net_device }
#[repr(C)] pub struct batadv_hashtable { pub size: u32, pub table: *mut hlist_head, pub list_locks: *mut spinlock_t }
#[repr(C)] pub struct batadv_orig_node { pub refcount: kref, pub rcu: rcu_head, pub bat_priv: *mut batadv_priv, pub orig: [u8; 6], pub last_seen: usize, pub last_ttvn: u8, pub tt_buff: *mut u8, pub tt_buff_len: u16, pub neigh_list: hlist_head, pub ifinfo_list: hlist_head, pub vlan_list: hlist_head, pub neigh_list_lock: spinlock_t, pub vlan_list_lock: spinlock_t, pub last_bonding_candidate: *mut batadv_orig_ifinfo }
#[repr(C)] pub struct batadv_orig_node_vlan { pub refcount: kref, pub list: hlist_node, pub rcu: rcu_head, pub vid: u16 }
#[repr(C)] pub struct batadv_orig_ifinfo { pub refcount: kref, pub list: hlist_node, pub if_outgoing: *mut batadv_hard_iface, pub router: *mut batadv_neigh_node, pub batman_seqno_reset: usize }
#[repr(C)] pub struct batadv_neigh_node { pub refcount: kref, pub list: hlist_node, pub ifinfo_list: hlist_head, pub ifinfo_lock: spinlock_t, pub hardif_neigh: *mut batadv_hardif_neigh_node, pub if_incoming: *mut batadv_hard_iface, pub addr: [u8; 6], pub last_seen: usize }
#[repr(C)] pub struct batadv_neigh_ifinfo { pub refcount: kref, pub list: hlist_node, pub if_outgoing: *mut batadv_hard_iface }
#[repr(C)] pub struct batadv_hardif_neigh_node { pub refcount: kref, pub list: hlist_node, pub if_incoming: *mut batadv_hard_iface, pub addr: [u8; 6], pub orig: [u8; 6], pub last_seen: usize }
#[repr(C)] pub struct batadv_hard_iface { pub refcount: kref, pub if_status: i32, pub mesh_iface: *mut net_device, pub neigh_list: hlist_head, pub neigh_list_lock: spinlock_t, pub net_dev: *mut net_device }
#[repr(C)] pub struct batadv_algo_ops { pub neigh: batadv_neigh_ops, pub orig: batadv_orig_ops }
#[repr(C)] pub struct batadv_neigh_ops { pub hardif_init: Option<unsafe extern "C" fn(*mut batadv_hardif_neigh_node)>, pub cmp: Option<unsafe extern "C" fn(*mut batadv_neigh_node,*mut batadv_hard_iface,*mut batadv_neigh_node,*mut batadv_hard_iface)->i32>, pub dump: Option<unsafe extern "C" fn(*mut sk_buff,*mut netlink_callback,*mut batadv_priv,*mut batadv_hard_iface)> }
#[repr(C)] pub struct batadv_orig_ops { pub dump: Option<unsafe extern "C" fn(*mut sk_buff,*mut netlink_callback,*mut batadv_priv,*mut batadv_hard_iface)> }
#[repr(C)] pub struct kref { pub refcount: usize }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct spinlock_t { _private: u8 }
#[repr(C)] pub struct work_struct { _private: u8 }
#[repr(C)] pub struct net_device { pub name: [u8; 16] }
#[repr(C)] pub struct sk_buff { pub len: u32 }
#[repr(C)] pub struct netlink_callback { _private: u8 }

extern "C" {
    fn batadv_hash_new(size: u32) -> *mut batadv_hashtable;
    fn batadv_hash_destroy(hash: *mut batadv_hashtable);
    fn batadv_hash_set_lock_class(hash: *mut batadv_hashtable, key: *mut c_void);
    fn batadv_choose_orig(data: *const c_void, size: u32) -> usize;
    fn batadv_compare_eth(a: *const c_void, b: *const c_void) -> bool;
    fn batadv_find_router(*mut batadv_priv,*mut batadv_orig_node,*mut batadv_hard_iface)->*mut batadv_neigh_node;
    fn batadv_hardif_put(*mut batadv_hard_iface); fn batadv_orig_node_put(*mut batadv_orig_node);
    fn batadv_neigh_node_put(*mut batadv_neigh_node); fn batadv_hardif_neigh_put(*mut batadv_hardif_neigh_node);
    fn batadv_orig_ifinfo_put(*mut batadv_orig_ifinfo); fn batadv_neigh_ifinfo_put(*mut batadv_neigh_ifinfo);
    fn batadv_orig_node_vlan_put(*mut batadv_orig_node_vlan);
    fn batadv_update_route(*mut batadv_priv,*mut batadv_orig_node,*mut batadv_hard_iface,*mut batadv_neigh_node);
    fn batadv_frag_purge_orig(*mut batadv_orig_node, *mut c_void); fn batadv_mcast_purge_orig(*mut batadv_orig_node);
    fn batadv_gw_node_delete(*mut batadv_priv,*mut batadv_orig_node); fn batadv_gw_election(*mut batadv_priv);
    fn batadv_tt_global_del_orig(*mut batadv_priv,*mut batadv_orig_node,i32,*const u8);
    fn batadv_netlink_get_meshif(*mut netlink_callback)->*mut net_device;
    fn batadv_netlink_get_hardif(*mut batadv_priv,*mut netlink_callback)->*mut batadv_hard_iface;
    fn batadv_primary_if_get_selected(*mut batadv_priv)->*mut batadv_hard_iface;
    fn dev_put(*mut net_device); fn batadv_has_timed_out(usize,usize)->bool;
}

const BATADV_IF_DEFAULT: *mut batadv_hard_iface = core::ptr::null_mut();
const BATADV_IF_ACTIVE: i32 = 1; const BATADV_IF_INACTIVE: i32 = 2; const BATADV_IF_TO_BE_REMOVED: i32 = 3;
const BATADV_ORIG_WORK_PERIOD: usize = 1000; const BATADV_PURGE_TIMEOUT: usize = 20000;
const BATADV_NO_FLAGS: u16 = 0;

pub unsafe extern "C" fn batadv_orig_hash_find(_bat_priv: *mut batadv_priv, _data: *const c_void) -> *mut batadv_orig_node { core::ptr::null_mut() }
pub unsafe extern "C" fn batadv_compare_orig(_node: *const hlist_node, _data2: *const c_void) -> bool { false }
pub unsafe extern "C" fn batadv_orig_node_vlan_get(_orig_node: *mut batadv_orig_node, _vid: u16) -> *mut batadv_orig_node_vlan { core::ptr::null_mut() }
pub unsafe extern "C" fn batadv_orig_node_vlan_new(_orig_node: *mut batadv_orig_node, _vid: u16) -> *mut batadv_orig_node_vlan { core::ptr::null_mut() }
pub unsafe extern "C" fn batadv_orig_node_vlan_release(_ref_: *mut kref) {}
pub unsafe extern "C" fn batadv_originator_init(_bat_priv: *mut batadv_priv) -> i32 { 0 }
pub unsafe extern "C" fn batadv_neigh_ifinfo_release(_ref_: *mut kref) {}
pub unsafe extern "C" fn batadv_hardif_neigh_release(_ref_: *mut kref) {}
pub unsafe extern "C" fn batadv_neigh_node_release(_ref_: *mut kref) {}
pub unsafe extern "C" fn batadv_orig_router_get(_orig_node: *mut batadv_orig_node, _if_outgoing: *const batadv_hard_iface) -> *mut batadv_neigh_node { core::ptr::null_mut() }
pub unsafe extern "C" fn batadv_orig_to_router(_bat_priv: *mut batadv_priv, _orig_addr: *mut u8, _if_outgoing: *mut batadv_hard_iface) -> *mut batadv_neigh_node { core::ptr::null_mut() }
pub unsafe extern "C" fn batadv_orig_ifinfo_get(_orig_node: *mut batadv_orig_node, _if_outgoing: *mut batadv_hard_iface) -> *mut batadv_orig_ifinfo { core::ptr::null_mut() }
pub unsafe extern "C" fn batadv_orig_ifinfo_new(_orig_node: *mut batadv_orig_node, _if_outgoing: *mut batadv_hard_iface) -> *mut batadv_orig_ifinfo { core::ptr::null_mut() }
pub unsafe extern "C" fn batadv_neigh_ifinfo_get(_neigh: *mut batadv_neigh_node, _if_outgoing: *mut batadv_hard_iface) -> *mut batadv_neigh_ifinfo { core::ptr::null_mut() }
pub unsafe extern "C" fn batadv_neigh_ifinfo_new(_neigh: *mut batadv_neigh_node, _if_outgoing: *mut batadv_hard_iface) -> *mut batadv_neigh_ifinfo { core::ptr::null_mut() }
pub unsafe extern "C" fn batadv_hardif_neigh_get(_hard_iface: *const batadv_hard_iface, _neigh_addr: *const u8) -> *mut batadv_hardif_neigh_node { core::ptr::null_mut() }
pub unsafe extern "C" fn batadv_neigh_node_get_or_create(_orig_node: *mut batadv_orig_node, _hard_iface: *mut batadv_hard_iface, _neigh_addr: *const u8) -> *mut batadv_neigh_node { core::ptr::null_mut() }
pub unsafe extern "C" fn batadv_hardif_neigh_dump(_msg: *mut sk_buff, _cb: *mut netlink_callback) -> i32 { 0 }
pub unsafe extern "C" fn batadv_orig_ifinfo_release(_ref_: *mut kref) {}
pub unsafe extern "C" fn batadv_orig_node_free_rcu(_rcu: *mut rcu_head) {}
pub unsafe extern "C" fn batadv_orig_node_release(_ref_: *mut kref) {}
pub unsafe extern "C" fn batadv_originator_free(_bat_priv: *mut batadv_priv) {}
pub unsafe extern "C" fn batadv_orig_node_new(_bat_priv: *mut batadv_priv, _addr: *const u8) -> *mut batadv_orig_node { core::ptr::null_mut() }
pub unsafe extern "C" fn batadv_purge_orig_ref(_bat_priv: *mut batadv_priv) {}
pub unsafe extern "C" fn batadv_orig_dump(_msg: *mut sk_buff, _cb: *mut netlink_callback) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
