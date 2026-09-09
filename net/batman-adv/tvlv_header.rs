/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

// Translated from tvlv.h. Declarations supplied by main.h, linux headers, and
// uapi/linux/batadv_packet.h remain external dependencies.

use core::ffi::c_void;

#[repr(C)]
pub struct batadv_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct batadv_ogm_buf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct batadv_ogm_packet {
    _private: [u8; 0],
}

#[repr(C)]
pub struct batadv_orig_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

extern "C" {
    pub fn batadv_tvlv_container_register(
        bat_priv: *mut batadv_priv,
        type_: u8,
        version: u8,
        tvlv_value: *mut c_void,
        tvlv_value_len: u16,
    );

    pub fn batadv_tvlv_container_ogm_append(
        bat_priv: *mut batadv_priv,
        ogm_buff: *mut batadv_ogm_buf,
    ) -> i32;

    pub fn batadv_tvlv_ogm_receive(
        bat_priv: *mut batadv_priv,
        batadv_ogm_packet: *mut batadv_ogm_packet,
        orig_node: *mut batadv_orig_node,
    );

    pub fn batadv_tvlv_container_unregister(
        bat_priv: *mut batadv_priv,
        type_: u8,
        version: u8,
    );

    pub fn batadv_tvlv_handler_register(
        bat_priv: *mut batadv_priv,
        optr: Option<
            unsafe extern "C" fn(
                bat_priv: *mut batadv_priv,
                orig: *mut batadv_orig_node,
                flags: u8,
                tvlv_value: *mut c_void,
                tvlv_value_len: u16,
            ),
        >,
        uptr: Option<
            unsafe extern "C" fn(
                bat_priv: *mut batadv_priv,
                src: *mut u8,
                dst: *mut u8,
                tvlv_value: *mut c_void,
                tvlv_value_len: u16,
            ) -> i32,
        >,
        mptr: Option<unsafe extern "C" fn(skb: *mut sk_buff) -> i32>,
        type_: u8,
        version: u8,
        flags: u8,
    );

    pub fn batadv_tvlv_handler_unregister(
        bat_priv: *mut batadv_priv,
        type_: u8,
        version: u8,
    );

    pub fn batadv_tvlv_containers_process(
        bat_priv: *mut batadv_priv,
        packet_type: u8,
        orig_node: *mut batadv_orig_node,
        skb: *mut sk_buff,
        tvlv_buff: *mut c_void,
        tvlv_buff_len: u16,
    ) -> i32;

    pub fn batadv_tvlv_unicast_send(
        bat_priv: *mut batadv_priv,
        src: *const u8,
        dst: *const u8,
        type_: u8,
        version: u8,
        tvlv_value: *mut c_void,
        tvlv_value_len: u16,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
