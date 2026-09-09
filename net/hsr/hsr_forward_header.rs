/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2011-2014 Autronica Fire and Security AS
 *
 * Author(s):
 *	2011-2014 Arvid Brodin, arvid.brodin@alten.se
 *
 * include file for HSR and PRP.
 */

// Dependencies supplied by the surrounding kernel translation:
// #include <linux/netdevice.h>
// #include "hsr_main.h"

extern "C" {
    pub fn hsr_forward_skb(skb: *mut sk_buff, port: *mut hsr_port);
    pub fn prp_create_tagged_frame(
        frame: *mut hsr_frame_info,
        port: *mut hsr_port,
    ) -> *mut sk_buff;
    pub fn hsr_create_tagged_frame(
        frame: *mut hsr_frame_info,
        port: *mut hsr_port,
    ) -> *mut sk_buff;
    pub fn hsr_get_untagged_frame(
        frame: *mut hsr_frame_info,
        port: *mut hsr_port,
    ) -> *mut sk_buff;
    pub fn prp_get_untagged_frame(
        frame: *mut hsr_frame_info,
        port: *mut hsr_port,
    ) -> *mut sk_buff;
    pub fn prp_drop_frame(frame: *mut hsr_frame_info, port: *mut hsr_port) -> bool;
    pub fn hsr_drop_frame(frame: *mut hsr_frame_info, port: *mut hsr_port) -> bool;
    pub fn prp_fill_frame_info(
        proto: u16,
        skb: *mut sk_buff,
        frame: *mut hsr_frame_info,
    ) -> i32;
    pub fn hsr_fill_frame_info(
        proto: u16,
        skb: *mut sk_buff,
        frame: *mut hsr_frame_info,
    ) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
