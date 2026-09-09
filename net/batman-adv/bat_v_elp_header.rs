/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Linus Lüssing, Marek Lindner
 */

// C dependencies:
// #include "main.h"
// #include <linux/skbuff.h>

extern "C" {
    pub fn batadv_v_elp_iface_enable(
        hard_iface: *mut crate::batadv_hard_iface,
    ) -> ::core::ffi::c_int;

    pub fn batadv_v_elp_iface_disable(
        hard_iface: *mut crate::batadv_hard_iface,
    );

    pub fn batadv_v_elp_iface_activate(
        primary_iface: *mut crate::batadv_hard_iface,
        hard_iface: *mut crate::batadv_hard_iface,
    );

    pub fn batadv_v_elp_primary_iface_set(
        primary_iface: *mut crate::batadv_hard_iface,
    );

    pub fn batadv_v_elp_packet_recv(
        skb: *mut crate::sk_buff,
        if_incoming: *mut crate::batadv_hard_iface,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
