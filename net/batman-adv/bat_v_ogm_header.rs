/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Antonio Quartulli
 */

/* Dependency intent preserved from the C header:
 * main.h, linux/skbuff.h, linux/types.h, and linux/workqueue.h.
 */

extern "C" {
    pub fn batadv_v_ogm_init(bat_priv: *mut batadv_priv) -> i32;
    pub fn batadv_v_ogm_free(bat_priv: *mut batadv_priv);
    pub fn batadv_v_ogm_aggr_work(work: *mut work_struct);
    pub fn batadv_v_ogm_iface_enable(hard_iface: *mut batadv_hard_iface) -> i32;
    pub fn batadv_v_ogm_iface_disable(hard_iface: *mut batadv_hard_iface);
    pub fn batadv_v_ogm_orig_get(
        bat_priv: *mut batadv_priv,
        addr: *const u8,
    ) -> *mut batadv_orig_node;
    pub fn batadv_v_ogm_primary_iface_set(primary_iface: *mut batadv_hard_iface);
    pub fn batadv_v_ogm_packet_recv(
        skb: *mut sk_buff,
        if_incoming: *mut batadv_hard_iface,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
