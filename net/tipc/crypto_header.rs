/* SPDX-License-Identifier: GPL-2.0 */
/*
 * net/tipc/crypto.h: Include file for TIPC crypto
 *
 * Copyright (c) 2019, Ericsson AB
 * All rights reserved.
 */

// C header dependencies: core.h, node.h, msg.h, and bearer.h.
// This translation is active when CONFIG_TIPC_CRYPTO is enabled.

pub const TIPC_EVERSION: i32 = 7;

/* AEAD aes(gcm) */
pub const TIPC_AES_GCM_KEY_SIZE_128: usize = 16;
pub const TIPC_AES_GCM_KEY_SIZE_192: usize = 24;
pub const TIPC_AES_GCM_KEY_SIZE_256: usize = 32;
pub const TIPC_AES_GCM_SALT_SIZE: usize = 4;
pub const TIPC_AES_GCM_IV_SIZE: usize = 12;
pub const TIPC_AES_GCM_TAG_SIZE: usize = 16;

pub const CLUSTER_KEY: i32 = 1;
pub const PER_NODE_KEY: i32 = 1 << 1;

extern "C" {
    pub static mut sysctl_tipc_max_tfms: i32;
    pub static mut sysctl_tipc_key_exchange_enabled: i32;
}

// External types supplied by the dependent TIPC headers.
pub enum tipc_crypto {}
pub enum net {}
pub enum tipc_node {}
pub enum sk_buff {}
pub enum tipc_bearer {}
pub enum tipc_media_addr {}
pub enum tipc_aead_key {}
pub enum genl_info {}
pub enum tipc_msg {}

pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;
pub type __be16 = u16;
pub type __be32 = u32;
pub type __be64 = u64;

// The C bitfields occupy the first two bytes. Their endian-specific bit
// ordering is selected by the target's __LITTLE_ENDIAN_BITFIELD/
// __BIG_ENDIAN_BITFIELD configuration.
#[repr(C, packed)]
pub struct tipc_ehdr_fields {
    pub bitfield_0: u8,
    pub bitfield_1: u8,
    pub reserved_2: __be16,
}

#[repr(C)]
pub union tipc_ehdr_first {
    pub fields: tipc_ehdr_fields,
    pub w0: __be32,
}

#[repr(C)]
pub union tipc_ehdr_addr {
    pub addr: __be32,
    // For a LINK_CONFIG message only.
    pub id: [u8; NODE_ID_LEN],
}

#[repr(C, packed)]
pub struct tipc_ehdr {
    pub first: tipc_ehdr_first,
    pub seqno: __be64,
    pub address: tipc_ehdr_addr,
}

pub const EHDR_SIZE: usize = core::mem::offset_of!(tipc_ehdr, address) + core::mem::size_of::<__be32>();
pub const EHDR_CFG_SIZE: usize = core::mem::size_of::<tipc_ehdr>();
pub const EHDR_MIN_SIZE: usize = EHDR_SIZE;
pub const EHDR_MAX_SIZE: usize = EHDR_CFG_SIZE;
pub const EMSG_OVERHEAD: usize = EHDR_SIZE + TIPC_AES_GCM_TAG_SIZE;

extern "C" {
    pub fn tipc_crypto_start(crypto: *mut *mut tipc_crypto, net: *mut net, node: *mut tipc_node) -> i32;
    pub fn tipc_crypto_stop(crypto: *mut *mut tipc_crypto);
    pub fn tipc_crypto_timeout(rx: *mut tipc_crypto);
    pub fn tipc_crypto_xmit(net: *mut net, skb: *mut *mut sk_buff, b: *mut tipc_bearer, dst: *mut tipc_media_addr, dnode: *mut tipc_node) -> i32;
    pub fn tipc_crypto_rcv(net: *mut net, rx: *mut tipc_crypto, skb: *mut *mut sk_buff, b: *mut tipc_bearer) -> i32;
    pub fn tipc_crypto_key_init(c: *mut tipc_crypto, ukey: *mut tipc_aead_key, mode: u8, master_key: bool) -> i32;
    pub fn tipc_crypto_key_flush(c: *mut tipc_crypto);
    pub fn tipc_crypto_key_distr(tx: *mut tipc_crypto, key: u8, dest: *mut tipc_node) -> i32;
    pub fn tipc_crypto_msg_rcv(net: *mut net, skb: *mut sk_buff);
    pub fn tipc_crypto_rekeying_sched(tx: *mut tipc_crypto, changed: bool, new_intv: u32);
    pub fn tipc_aead_key_validate(ukey: *mut tipc_aead_key, info: *mut genl_info) -> i32;
    pub fn tipc_ehdr_validate(skb: *mut sk_buff) -> bool;
}

extern "C" {
    pub fn msg_bits(m: *mut tipc_msg, w: u32, pos: u32, mask: u32) -> u32;
    pub fn msg_set_bits(m: *mut tipc_msg, w: u32, pos: u32, mask: u32, val: u32);
}

#[inline]
pub unsafe fn msg_key_gen(m: *mut tipc_msg) -> u32 { msg_bits(m, 4, 16, 0xffff) }

#[inline]
pub unsafe fn msg_set_key_gen(m: *mut tipc_msg, gen: u32) { msg_set_bits(m, 4, 16, 0xffff, gen); }

#[inline]
pub unsafe fn msg_key_mode(m: *mut tipc_msg) -> u32 { msg_bits(m, 4, 0, 0xf) }

#[inline]
pub unsafe fn msg_set_key_mode(m: *mut tipc_msg, mode: u32) { msg_set_bits(m, 4, 0, 0xf, mode); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
