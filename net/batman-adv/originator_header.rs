/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

// Dependencies and types supplied by the surrounding translation unit.

extern "C" {
    pub fn batadv_compare_orig(node: *const hlist_node, data2: *const core::ffi::c_void) -> bool;
    pub fn batadv_originator_init(bat_priv: *mut batadv_priv) -> i32;
    pub fn batadv_originator_free(bat_priv: *mut batadv_priv);
    pub fn batadv_purge_orig_ref(bat_priv: *mut batadv_priv);
    pub fn batadv_orig_node_release(reference: *mut kref);
    pub fn batadv_orig_node_new(bat_priv: *mut batadv_priv, addr: *const u8) -> *mut batadv_orig_node;
    pub fn batadv_hardif_neigh_get(
        hard_iface: *const batadv_hard_iface,
        neigh_addr: *const u8,
    ) -> *mut batadv_hardif_neigh_node;
    pub fn batadv_hardif_neigh_release(reference: *mut kref);
    pub fn batadv_neigh_node_get_or_create(
        orig_node: *mut batadv_orig_node,
        hard_iface: *mut batadv_hard_iface,
        neigh_addr: *const u8,
    ) -> *mut batadv_neigh_node;
    pub fn batadv_neigh_node_release(reference: *mut kref);
    pub fn batadv_orig_router_get(
        orig_node: *mut batadv_orig_node,
        if_outgoing: *const batadv_hard_iface,
    ) -> *mut batadv_neigh_node;
    pub fn batadv_orig_to_router(
        bat_priv: *mut batadv_priv,
        orig_addr: *mut u8,
        if_outgoing: *mut batadv_hard_iface,
    ) -> *mut batadv_neigh_node;
    pub fn batadv_neigh_ifinfo_new(
        neigh: *mut batadv_neigh_node,
        if_outgoing: *mut batadv_hard_iface,
    ) -> *mut batadv_neigh_ifinfo;
    pub fn batadv_neigh_ifinfo_get(
        neigh: *mut batadv_neigh_node,
        if_outgoing: *mut batadv_hard_iface,
    ) -> *mut batadv_neigh_ifinfo;
    pub fn batadv_neigh_ifinfo_release(reference: *mut kref);
    pub fn batadv_hardif_neigh_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn batadv_orig_ifinfo_get(
        orig_node: *mut batadv_orig_node,
        if_outgoing: *mut batadv_hard_iface,
    ) -> *mut batadv_orig_ifinfo;
    pub fn batadv_orig_ifinfo_new(
        orig_node: *mut batadv_orig_node,
        if_outgoing: *mut batadv_hard_iface,
    ) -> *mut batadv_orig_ifinfo;
    pub fn batadv_orig_ifinfo_release(reference: *mut kref);
    pub fn batadv_orig_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn batadv_orig_node_vlan_new(
        orig_node: *mut batadv_orig_node,
        vid: u16,
    ) -> *mut batadv_orig_node_vlan;
    pub fn batadv_orig_node_vlan_get(
        orig_node: *mut batadv_orig_node,
        vid: u16,
    ) -> *mut batadv_orig_node_vlan;
    pub fn batadv_orig_node_vlan_release(reference: *mut kref);
    pub fn batadv_orig_hash_find(
        bat_priv: *mut batadv_priv,
        data: *const core::ffi::c_void,
    ) -> *mut batadv_orig_node;
}

/// Return the index of the orig entry in the hash table.
pub unsafe fn batadv_choose_orig(data: *const core::ffi::c_void, size: u32) -> u32 {
    let mut hash: u32 = 0;
    hash = jhash(data, ETH_ALEN, hash);
    hash % size
}

pub unsafe fn batadv_orig_node_vlan_put(orig_vlan: *mut batadv_orig_node_vlan) {
    if orig_vlan.is_null() {
        return;
    }
    kref_put(&mut (*orig_vlan).refcount, batadv_orig_node_vlan_release);
}

pub unsafe fn batadv_neigh_ifinfo_put(neigh_ifinfo: *mut batadv_neigh_ifinfo) {
    if neigh_ifinfo.is_null() {
        return;
    }
    kref_put(&mut (*neigh_ifinfo).refcount, batadv_neigh_ifinfo_release);
}

pub unsafe fn batadv_hardif_neigh_put(hardif_neigh: *mut batadv_hardif_neigh_node) {
    if hardif_neigh.is_null() {
        return;
    }
    kref_put(&mut (*hardif_neigh).refcount, batadv_hardif_neigh_release);
}

pub unsafe fn batadv_neigh_node_put(neigh_node: *mut batadv_neigh_node) {
    if neigh_node.is_null() {
        return;
    }
    kref_put(&mut (*neigh_node).refcount, batadv_neigh_node_release);
}

pub unsafe fn batadv_orig_ifinfo_put(orig_ifinfo: *mut batadv_orig_ifinfo) {
    if orig_ifinfo.is_null() {
        return;
    }
    kref_put(&mut (*orig_ifinfo).refcount, batadv_orig_ifinfo_release);
}

pub unsafe fn batadv_orig_node_put(orig_node: *mut batadv_orig_node) {
    if orig_node.is_null() {
        return;
    }
    kref_put(&mut (*orig_node).refcount, batadv_orig_node_release);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
