/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner
 */

/* C header guard: _NET_BATMAN_ADV_GATEWAY_CLIENT_H_ */
/* C dependencies: main.h, linux/kref.h, linux/netlink.h, linux/skbuff.h,
 * linux/types.h, and uapi/linux/batadv_packet.h. */

// Opaque types supplied by the translated dependencies.
pub enum batadv_priv {}
pub enum batadv_orig_node {}
pub enum batadv_tvlv_gateway_data {}
pub enum kref {}
pub enum sk_buff {}
pub enum netlink_callback {}

#[repr(C)]
pub enum batadv_dhcp_recipient {
    /* Enumerators are supplied by uapi/linux/batadv_packet.h. */
}

extern "C" {
    pub fn batadv_gw_check_client_stop(bat_priv: *mut batadv_priv);
    pub fn batadv_gw_reselect(bat_priv: *mut batadv_priv);
    pub fn batadv_gw_election(bat_priv: *mut batadv_priv);
    pub fn batadv_gw_get_selected_orig(bat_priv: *mut batadv_priv) -> *mut batadv_orig_node;
    pub fn batadv_gw_check_election(
        bat_priv: *mut batadv_priv,
        orig_node: *mut batadv_orig_node,
    );
    pub fn batadv_gw_node_update(
        bat_priv: *mut batadv_priv,
        orig_node: *mut batadv_orig_node,
        gateway: *mut batadv_tvlv_gateway_data,
    );
    pub fn batadv_gw_node_delete(
        bat_priv: *mut batadv_priv,
        orig_node: *mut batadv_orig_node,
    );
    pub fn batadv_gw_node_free(bat_priv: *mut batadv_priv);
    pub fn batadv_gw_node_release(reference: *mut kref);
    pub fn batadv_gw_get_selected_gw_node(bat_priv: *mut batadv_priv) -> *mut batadv_gw_node;
    pub fn batadv_gw_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> ::core::ffi::c_int;
    pub fn batadv_gw_out_of_range(bat_priv: *mut batadv_priv, skb: *mut sk_buff) -> bool;
    pub fn batadv_gw_dhcp_recipient_get(
        skb: *mut sk_buff,
        header_len: *mut ::core::ffi::c_uint,
        chaddr: *mut u8,
    ) -> batadv_dhcp_recipient;
    pub fn batadv_gw_node_get(
        bat_priv: *mut batadv_priv,
        orig_node: *mut batadv_orig_node,
    ) -> *mut batadv_gw_node;

    pub fn kref_put(reference: *mut kref, release: unsafe extern "C" fn(*mut kref)) -> bool;
}

pub enum batadv_gw_node {}

/**
 * batadv_gw_node_put() - decrement the gw_node refcounter and possibly release
 *  it
 * @gw_node: gateway node to free
 */
#[inline]
pub unsafe fn batadv_gw_node_put(gw_node: *mut batadv_gw_node) {
    if gw_node.is_null() {
        return;
    }

    // C: kref_put(&gw_node->refcount, batadv_gw_node_release);
    // The containing structure and refcount field are supplied by its dependency.
    kref_put(gw_node as *mut kref, batadv_gw_node_release);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
