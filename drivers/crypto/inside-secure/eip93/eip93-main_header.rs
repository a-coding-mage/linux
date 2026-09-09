/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2019 - 2021
 *
 * Richard van Schagen <vschagen@icloud.com>
 * Christian Marangi <ansuelsmth@gmail.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// crypto/internal/aead.h, crypto/internal/hash.h, crypto/internal/skcipher.h,
// linux/bitfield.h, and linux/interrupt.h.

pub const EIP93_RING_BUSY_DELAY: u32 = 500;

pub const EIP93_RING_NUM: u32 = 512;
pub const EIP93_RING_BUSY: u32 = 32;
pub const EIP93_CRA_PRIORITY: u32 = 1500;

macro_rules! EIP93_RING_SA_STATE_ADDR {
    ($base:expr, $idx:expr) => { ($base) + ($idx) };
}

macro_rules! EIP93_RING_SA_STATE_DMA {
    ($dma_base:expr, $idx:expr) => {
        (($dma_base) as u32) + (($idx) * core::mem::size_of::<sa_state>()) as u32
    };
}

pub const EIP93_ALG_DES: u32 = 1 << 0;
pub const EIP93_ALG_3DES: u32 = 1 << 1;
pub const EIP93_ALG_AES: u32 = 1 << 2;
pub const EIP93_ALG_MASK: u32 = 0b111;

pub const EIP93_HASH_MD5: u32 = 1 << 3;
pub const EIP93_HASH_SHA1: u32 = 1 << 4;
pub const EIP93_HASH_SHA224: u32 = 1 << 5;
pub const EIP93_HASH_SHA256: u32 = 1 << 6;
pub const EIP93_HASH_HMAC: u32 = 1 << 7;
pub const EIP93_HASH_MASK: u32 = 0b1111 << 3;

pub const EIP93_MODE_CBC: u32 = 1 << 8;
pub const EIP93_MODE_ECB: u32 = 1 << 9;
pub const EIP93_MODE_CTR: u32 = 1 << 10;
pub const EIP93_MODE_RFC3686: u32 = 1 << 11;
pub const EIP93_MODE_MASK: u32 = 0b111 << 8;

pub const EIP93_ENCRYPT: u32 = 1 << 12;
pub const EIP93_DECRYPT: u32 = 1 << 13;
pub const EIP93_BUSY: u32 = 1 << 14;

pub const EIP93_DESC_DMA_IV: u32 = 1 << 0;
pub const EIP93_DESC_IPSEC: u32 = 1 << 1;
pub const EIP93_DESC_FINISH: u32 = 1 << 2;
pub const EIP93_DESC_LAST: u32 = 1 << 3;
pub const EIP93_DESC_FAKE_HMAC: u32 = 1 << 4;
pub const EIP93_DESC_PRNG: u32 = 1 << 5;
pub const EIP93_DESC_HASH: u32 = 1 << 6;
pub const EIP93_DESC_AEAD: u32 = 1 << 7;
pub const EIP93_DESC_SKCIPHER: u32 = 1 << 8;
pub const EIP93_DESC_ASYNC: u32 = 1 << 9;

macro_rules! IS_DMA_IV { ($desc_flags:expr) => { ($desc_flags) & EIP93_DESC_DMA_IV }; }
macro_rules! IS_DES { ($flags:expr) => { ($flags) & EIP93_ALG_DES }; }
macro_rules! IS_3DES { ($flags:expr) => { ($flags) & EIP93_ALG_3DES }; }
macro_rules! IS_AES { ($flags:expr) => { ($flags) & EIP93_ALG_AES }; }
macro_rules! IS_HASH_MD5 { ($flags:expr) => { ($flags) & EIP93_HASH_MD5 }; }
macro_rules! IS_HASH_SHA1 { ($flags:expr) => { ($flags) & EIP93_HASH_SHA1 }; }
macro_rules! IS_HASH_SHA224 { ($flags:expr) => { ($flags) & EIP93_HASH_SHA224 }; }
macro_rules! IS_HASH_SHA256 { ($flags:expr) => { ($flags) & EIP93_HASH_SHA256 }; }
macro_rules! IS_HMAC { ($flags:expr) => { ($flags) & EIP93_HASH_HMAC }; }
macro_rules! IS_CBC { ($mode:expr) => { ($mode) & EIP93_MODE_CBC }; }
macro_rules! IS_ECB { ($mode:expr) => { ($mode) & EIP93_MODE_ECB }; }
macro_rules! IS_CTR { ($mode:expr) => { ($mode) & EIP93_MODE_CTR }; }
macro_rules! IS_RFC3686 { ($mode:expr) => { ($mode) & EIP93_MODE_RFC3686 }; }
macro_rules! IS_BUSY { ($flags:expr) => { ($flags) & EIP93_BUSY }; }
macro_rules! IS_ENCRYPT { ($dir:expr) => { ($dir) & EIP93_ENCRYPT }; }
macro_rules! IS_DECRYPT { ($dir:expr) => { ($dir) & EIP93_DECRYPT }; }
macro_rules! IS_CIPHER { ($flags:expr) => { ($flags) & (EIP93_ALG_DES | EIP93_ALG_3DES | EIP93_ALG_AES) }; }
macro_rules! IS_HASH { ($flags:expr) => { ($flags) & (EIP93_HASH_MD5 | EIP93_HASH_SHA1 | EIP93_HASH_SHA224 | EIP93_HASH_SHA256) }; }

#[repr(C)]
pub struct eip93_desc_ring {
    pub base: *mut core::ffi::c_void,
    pub base_end: *mut core::ffi::c_void,
    pub base_dma: dma_addr_t,
    pub read: *mut core::ffi::c_void,
    pub write: *mut core::ffi::c_void,
    pub offset: u32,
}

#[repr(C)]
pub struct eip93_state_pool {
    pub base: *mut core::ffi::c_void,
    pub base_dma: dma_addr_t,
}

#[repr(C)]
pub struct eip93_ring {
    pub done_task: tasklet_struct,
    pub cdr: eip93_desc_ring,
    pub rdr: eip93_desc_ring,
    pub write_lock: spinlock_t,
    pub read_lock: spinlock_t,
    pub idr_lock: spinlock_t,
    pub crypto_async_idr: idr,
}

#[repr(C)]
pub struct eip93_device {
    pub base: *mut core::ffi::c_void,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub irq: i32,
    pub ring: [eip93_ring; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum eip93_alg_type {
    EIP93_ALG_TYPE_AEAD,
    EIP93_ALG_TYPE_SKCIPHER,
    EIP93_ALG_TYPE_HASH,
}

#[repr(C)]
pub union eip93_alg_union {
    pub aead: core::mem::ManuallyDrop<aead_alg>,
    pub skcipher: core::mem::ManuallyDrop<skcipher_alg>,
    pub ahash: core::mem::ManuallyDrop<ahash_alg>,
}

#[repr(C)]
pub struct eip93_alg_template {
    pub eip93: *mut eip93_device,
    pub type_: eip93_alg_type,
    pub flags: u32,
    pub alg: eip93_alg_union,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
