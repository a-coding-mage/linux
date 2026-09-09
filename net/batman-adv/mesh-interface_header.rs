/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner
 */

// Dependency intent from main.h and Linux kernel headers:
// main.h, linux/kref.h, linux/netdevice.h, linux/skbuff.h, linux/types.h

extern "C" {
    pub fn batadv_skb_head_push(skb: *mut sk_buff, len: u32) -> i32;
    pub fn batadv_interface_rx(
        mesh_iface: *mut net_device,
        skb: *mut sk_buff,
        hdr_size: i32,
        orig_node: *mut batadv_orig_node,
    );
    pub fn batadv_meshif_is_valid(net_dev: *const net_device) -> bool;
    pub static mut batadv_link_ops: rtnl_link_ops;
    pub fn batadv_meshif_create_vlan(bat_priv: *mut batadv_priv, vid: u16) -> i32;
    pub fn batadv_meshif_destroy_vlan(
        bat_priv: *mut batadv_priv,
        vlan: *mut batadv_meshif_vlan,
    );
    pub fn batadv_meshif_vlan_release(ref_: *mut kref);
    pub fn batadv_meshif_vlan_get(
        bat_priv: *mut batadv_priv,
        vid: u16,
    ) -> *mut batadv_meshif_vlan;

    pub fn kref_put(
        ref_: *mut kref,
        release: unsafe extern "C" fn(*mut kref),
    ) -> bool;
}

/**
 * batadv_meshif_vlan_put() - decrease the vlan object refcounter and
 *  possibly release it
 * @vlan: the vlan object to release
 */
#[inline]
pub unsafe fn batadv_meshif_vlan_put(vlan: *mut batadv_meshif_vlan) {
    if vlan.is_null() {
        return;
    }

    kref_put(&mut (*vlan).refcount, batadv_meshif_vlan_release);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
