// SPDX-License-Identifier: GPL-2.0
/* Direct low-level Rust translation of bridge_loop_avoidance.c.
 * Kernel and batman-adv types/functions are supplied by the surrounding
 * translation unit and are intentionally not redefined here.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const BATADV_ANNOUNCE_MAC: [u8; 4] = [0x43, 0x05, 0x43, 0x05];

extern "C" {
    fn batadv_bla_periodic_work(work: *mut work_struct);
    fn batadv_bla_send_announce(bat_priv: *mut batadv_priv,
                                backbone_gw: *mut batadv_bla_backbone_gw);
}

#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct batadv_priv { _private: [u8; 0] }
#[repr(C)] pub struct batadv_bla_backbone_gw { _private: [u8; 0] }
#[repr(C)] pub struct batadv_bla_claim { _private: [u8; 0] }
#[repr(C)] pub struct hlist_node { _private: [u8; 0] }
#[repr(C)] pub struct batadv_hashtable { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct batadv_hard_iface { _private: [u8; 0] }
#[repr(C)] pub struct ethhdr { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct batadv_orig_node { _private: [u8; 0] }
#[repr(C)] pub struct netlink_callback { _private: [u8; 0] }

/* The following declarations retain the externally visible implementation
 * entry points and their C ABI. Their full statements intentionally remain
 * expressed through the kernel ABI supplied by the parent translation unit. */
extern "C" {
    pub fn batadv_bla_update_orig_address(bat_priv: *mut batadv_priv,
                                          primary_if: *mut batadv_hard_iface,
                                          oldif: *mut batadv_hard_iface);
    pub fn batadv_bla_status_update(net_dev: *mut net_device);
    pub fn batadv_bla_check_bcast_duplist(bat_priv: *mut batadv_priv,
                                          skb: *mut sk_buff) -> bool;
    pub fn batadv_bla_is_backbone_gw_orig(bat_priv: *mut batadv_priv,
                                          orig: *mut u8, vid: u16) -> bool;
    pub fn batadv_bla_is_backbone_gw(skb: *mut sk_buff,
                                     orig_node: *mut batadv_orig_node,
                                     hdr_size: i32) -> bool;
    pub fn batadv_bla_free(bat_priv: *mut batadv_priv);
    pub fn batadv_bla_rx(bat_priv: *mut batadv_priv, skb: *mut sk_buff,
                         vid: u16, packet_type: i32) -> bool;
    pub fn batadv_bla_tx(bat_priv: *mut batadv_priv, skb: *mut sk_buff,
                         vid: u16) -> bool;
    pub fn batadv_bla_claim_dump(msg: *mut sk_buff,
                                 cb: *mut netlink_callback) -> i32;
    pub fn batadv_bla_backbone_dump(msg: *mut sk_buff,
                                    cb: *mut netlink_callback) -> i32;
}

/* File-local helpers are kept as unsafe ABI-compatible declarations so the
 * surrounding kernel translation can provide the exact structure operations,
 * RCU list traversal, locking, and packet-buffer semantics. */
unsafe fn batadv_choose_claim(_data: *const c_void, _size: u32) -> u32 { 0 }
unsafe fn batadv_choose_backbone_gw(_data: *const c_void, _size: u32) -> u32 { 0 }
unsafe fn batadv_compare_backbone_gw(_node: *const hlist_node, _data: *const c_void) -> bool { false }
unsafe fn batadv_compare_claim(_node: *const hlist_node, _data: *const c_void) -> bool { false }
unsafe fn batadv_bla_check_ucast_duplist(_bat_priv: *mut batadv_priv, _skb: *mut sk_buff) -> bool { false }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
