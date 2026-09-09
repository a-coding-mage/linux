/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2011-2014 Autronica Fire and Security AS
 *
 * Author(s):
 *	2011-2014 Arvid Brodin, arvid.brodin@alten.se
 *
 * include file for HSR and PRP.
 */

use core::ffi::c_void;

pub const ETH_ALEN: usize = 6;
pub const HSR_PT_PORTS: usize = 3;
pub const BITS_PER_LONG: usize = usize::BITS as usize;

#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct hsr_port { _private: [u8; 0] }
#[repr(C)] pub struct hsr_priv { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
pub type hsr_port_type = i32;

#[repr(C)]
pub struct hsr_frame_info {
    pub skb_std: *mut sk_buff,
    pub skb_hsr: *mut sk_buff,
    pub skb_prp: *mut sk_buff,
    pub port_rcv: *mut hsr_port,
    pub node_src: *mut hsr_node,
    pub sequence_nr: u16,
    pub is_supervision: bool,
    pub is_proxy_supervision: bool,
    pub is_vlan: bool,
    pub is_local_dest: bool,
    pub is_local_exclusive: bool,
    pub is_from_san: bool,
    pub dst_in_node_db: bool,
    pub dst_in_proxy_node_db: bool,
}

extern "C" {
    pub fn hsr_del_self_node(hsr: *mut hsr_priv);
    pub fn hsr_del_nodes(node_db: *mut list_head);
    pub fn hsr_get_node(port: *mut hsr_port, node_db: *mut list_head,
                        skb: *mut sk_buff, is_sup: bool,
                        rx_port: hsr_port_type) -> *mut hsr_node;
    pub fn hsr_handle_sup_frame(frame: *mut hsr_frame_info);
    pub fn hsr_addr_is_self(hsr: *mut hsr_priv, addr: *mut u8) -> bool;
    pub fn hsr_addr_is_redbox(hsr: *mut hsr_priv, addr: *mut u8) -> bool;
    pub fn hsr_addr_subst_source(node: *mut hsr_node, skb: *mut sk_buff);
    pub fn hsr_addr_subst_dest(node_src: *mut hsr_node, skb: *mut sk_buff,
                               port: *mut hsr_port);
    pub fn hsr_register_frame_in(node: *mut hsr_node, port: *mut hsr_port,
                                 sequence_nr: u16);
    pub fn hsr_register_frame_out(port: *mut hsr_port,
                                  frame: *mut hsr_frame_info) -> i32;
    pub fn hsr_prune_nodes(t: *mut timer_list);
    pub fn hsr_prune_proxy_nodes(t: *mut timer_list);
    pub fn hsr_create_self_node(hsr: *mut hsr_priv,
                                addr_a: *const u8, addr_b: *const u8) -> i32;
    pub fn hsr_get_next_node(hsr: *mut hsr_priv, pos: *mut c_void,
                             addr: *mut u8) -> *mut c_void;
    pub fn hsr_get_node_data(hsr: *mut hsr_priv, addr: *const u8,
                             addr_b: *mut u8, addr_b_ifindex: *mut u32,
                             if1_age: *mut i32, if1_seq: *mut u16,
                             if2_age: *mut i32, if2_seq: *mut u16) -> i32;
    pub fn prp_handle_san_frame(san: bool, port: hsr_port_type,
                                node: *mut hsr_node);
    pub fn prp_update_san_info(node: *mut hsr_node, is_sup: bool);
    pub fn hsr_is_node_in_db(node_db: *mut list_head, addr: *const u8) -> bool;
    pub fn prp_register_frame_out(port: *mut hsr_port,
                                  frame: *mut hsr_frame_info) -> i32;
}

// Corresponds to: #if IS_ENABLED(CONFIG_KUNIT)
#[cfg(feature = "CONFIG_KUNIT")]
extern "C" {
    pub fn hsr_get_seq_block(node: *mut hsr_node, block_idx: u16) -> *mut hsr_seq_block;
}

pub const HSR_SEQ_BLOCK_SHIFT: usize = 7;
pub const HSR_SEQ_BLOCK_SIZE: usize = 1 << HSR_SEQ_BLOCK_SHIFT;
pub const HSR_SEQ_BLOCK_MASK: usize = HSR_SEQ_BLOCK_SIZE - 1;
pub const HSR_MAX_SEQ_BLOCKS: usize = 64;

#[inline] pub const fn hsr_seq_block_index(sequence_nr: usize) -> usize { sequence_nr >> HSR_SEQ_BLOCK_SHIFT }
#[inline] pub const fn hsr_seq_block_bit(sequence_nr: usize) -> usize { sequence_nr & HSR_SEQ_BLOCK_MASK }

#[repr(C)]
pub struct hsr_seq_block {
    pub time: usize,
    pub block_idx: u16,
    pub seq_nrs: [[usize; (HSR_SEQ_BLOCK_SIZE + BITS_PER_LONG - 1) / BITS_PER_LONG]; 0],
}

#[repr(C)]
pub struct hsr_node {
    pub mac_list: list_head,
    pub seq_out_lock: spinlock_t,
    pub macaddress_A: [u8; ETH_ALEN],
    pub macaddress_B: [u8; ETH_ALEN],
    pub addr_B_port: hsr_port_type,
    pub time_in: [usize; HSR_PT_PORTS],
    pub time_in_stale: [bool; HSR_PT_PORTS],
    pub san_a: bool,
    pub san_b: bool,
    pub removed: bool,
    pub seq_blocks: xarray,
    pub block_buf: *mut c_void,
    pub next_block: u32,
    pub seq_port_cnt: u32,
    pub rcu_head: rcu_head,
}

#[inline]
pub unsafe fn hsr_seq_block_size(node: *const hsr_node) -> usize {
    debug_assert!((*node).seq_port_cnt != 0); // WARN_ON_ONCE(node->seq_port_cnt == 0)
    (*node).seq_port_cnt as usize * core::mem::size_of::<usize>()
        * ((HSR_SEQ_BLOCK_SIZE + BITS_PER_LONG - 1) / BITS_PER_LONG)
        + core::mem::size_of::<hsr_seq_block>()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
