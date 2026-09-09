/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich, Antonio Quartulli
 */

/* Declarations supplied by main.h and the Linux kernel headers are external
 * dependencies of this translated header. */

#[repr(C)]
pub struct batadv_priv {
    _private: [u8; 0],
}
#[repr(C)]
pub struct batadv_orig_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct batadv_tt_global_entry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct net_device {
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
pub struct kref {
    _private: [u8; 0],
}

extern "C" {
    pub fn batadv_tt_init(bat_priv: *mut batadv_priv) -> i32;
    pub fn batadv_tt_local_add(
        mesh_iface: *mut net_device,
        addr: *const u8,
        vid: u16,
        ifindex: i32,
        mark: u32,
    ) -> bool;
    pub fn batadv_tt_local_remove(
        bat_priv: *mut batadv_priv,
        addr: *const u8,
        vid: u16,
        message: *const core::ffi::c_char,
        roaming: bool,
    ) -> u16;
    pub fn batadv_tt_local_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn batadv_tt_global_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn batadv_tt_global_del_orig(
        bat_priv: *mut batadv_priv,
        orig_node: *mut batadv_orig_node,
        match_vid: i32,
        message: *const core::ffi::c_char,
    );
    pub fn batadv_tt_global_hash_find(
        bat_priv: *mut batadv_priv,
        addr: *const u8,
        vid: u16,
    ) -> *mut batadv_tt_global_entry;
    pub fn batadv_tt_global_entry_release(ref_: *mut kref);
    pub fn batadv_tt_global_hash_count(
        bat_priv: *mut batadv_priv,
        addr: *const u8,
        vid: u16,
    ) -> i32;
    pub fn batadv_transtable_search(
        bat_priv: *mut batadv_priv,
        src: *const u8,
        addr: *const u8,
        vid: u16,
    ) -> *mut batadv_orig_node;
    pub fn batadv_tt_free(bat_priv: *mut batadv_priv);
    pub fn batadv_is_my_client(bat_priv: *mut batadv_priv, addr: *const u8, vid: u16) -> bool;
    pub fn batadv_is_ap_isolated(
        bat_priv: *mut batadv_priv,
        src: *mut u8,
        dst: *mut u8,
        vid: u16,
    ) -> bool;
    pub fn batadv_tt_local_commit_changes(bat_priv: *mut batadv_priv);
    pub fn batadv_tt_global_client_is_roaming(
        bat_priv: *mut batadv_priv,
        addr: *mut u8,
        vid: u16,
    ) -> bool;
    pub fn batadv_tt_local_client_is_roaming(
        bat_priv: *mut batadv_priv,
        addr: *mut u8,
        vid: u16,
    ) -> bool;
    pub fn batadv_tt_local_resize_to_mtu(mesh_iface: *mut net_device);
    pub fn batadv_tt_add_temporary_global_entry(
        bat_priv: *mut batadv_priv,
        orig_node: *mut batadv_orig_node,
        addr: *const u8,
        vid: u16,
    ) -> bool;
    pub fn batadv_tt_global_is_isolated(
        bat_priv: *mut batadv_priv,
        addr: *const u8,
        vid: u16,
    ) -> bool;

    pub fn batadv_tt_cache_init() -> i32;
    pub fn batadv_tt_cache_destroy();
}

/*
 * batadv_tt_global_entry_put() - decrement the tt_global_entry refcounter and
 *  possibly release it
 * @tt_global_entry: tt_global_entry to be free'd
 *
 * The containing structure's `common.refcount` layout is supplied by the
 * dependent translation unit, so this inline operation is represented by the
 * corresponding external release operation here.
 */
#[inline]
pub unsafe fn batadv_tt_global_entry_put(tt_global_entry: *mut batadv_tt_global_entry) {
    if tt_global_entry.is_null() {
        return;
    }
    /* TODO: invoke kref_put on (*tt_global_entry).common.refcount once the
     * dependent batadv_tt_global_entry layout is available. */
    batadv_tt_global_entry_release(tt_global_entry.cast::<kref>());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
