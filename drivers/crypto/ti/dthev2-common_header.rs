/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * K3 DTHE V2 crypto accelerator driver
 *
 * Copyright (C) Texas Instruments 2025 - https://www.ti.com
 * Author: T Pratham <t-pratham@ti.com>
 */

// C header dependencies are supplied by the surrounding kernel translation.

pub const DTHE_REG_SIZE: u32 = 4;
pub const DTHE_DMA_TIMEOUT_MS: u32 = 2000;
/*
 * Size of largest possible key (of all algorithms) to be stored in dthe_tfm_ctx
 * This is currently the keysize of XTS-AES-256 which is 512 bits (64 bytes)
 */
pub const DTHE_MAX_KEYSIZE: usize = AES_MAX_KEY_SIZE * 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum dthe_aes_mode {
    DTHE_AES_ECB = 0,
    DTHE_AES_CBC,
    DTHE_AES_CTR,
    DTHE_AES_XTS,
    DTHE_AES_GCM,
    DTHE_AES_CCM,
}

/* Driver specific struct definitions */

/**
 * struct dthe_data - DTHE_V2 driver instance data
 * @dev: Device pointer
 * @regs: Base address of the register space
 * @list: list node for dev
 * @engine: Crypto engine instance
 * @dma_aes_rx: AES Rx DMA Channel
 * @dma_aes_tx: AES Tx DMA Channel
 * @dma_sha_tx: SHA Tx DMA Channel
 */
#[repr(C)]
pub struct dthe_data {
    pub dev: *mut device,
    pub regs: *mut core::ffi::c_void,
    pub list: list_head,
    pub engine: *mut crypto_engine,
    pub dma_aes_rx: *mut dma_chan,
    pub dma_aes_tx: *mut dma_chan,
    pub dma_sha_tx: *mut dma_chan,
}

/**
 * struct dthe_list - device data list head
 * @dev_list: linked list head
 * @lock: Spinlock protecting accesses to the list
 */
#[repr(C)]
pub struct dthe_list {
    pub dev_list: list_head,
    pub lock: spinlock_t,
}

/**
 * struct dthe_tfm_ctx - Transform ctx struct containing ctx for all sub-components of DTHE V2
 * @dev_data: Device data struct pointer
 * @keylen: AES key length
 * @authsize: Authentication size for modes with authentication
 * @key: AES key
 * @aes_mode: AES mode
 * @aead_fb: Fallback crypto aead handle
 * @skcipher_fb: Fallback crypto skcipher handle for AES-XTS mode
 */
#[repr(C)]
pub struct dthe_tfm_ctx {
    pub dev_data: *mut dthe_data,
    pub keylen: u32,
    pub authsize: u32,
    pub key: [u32; DTHE_MAX_KEYSIZE / core::mem::size_of::<u32>()],
    pub aes_mode: dthe_aes_mode,
    pub fallback: dthe_tfm_ctx_fallback,
}

#[repr(C)]
pub union dthe_tfm_ctx_fallback {
    pub aead_fb: *mut crypto_sync_aead,
    pub skcipher_fb: *mut crypto_sync_skcipher,
}

/**
 * struct dthe_aes_req_ctx - AES engine req ctx struct
 * @enc: flag indicating encryption or decryption operation
 * @padding: padding buffer for handling unaligned data
 * @aes_compl: Completion variable for use in manual completion in case of DMA callback failure
 */
#[repr(C)]
pub struct dthe_aes_req_ctx {
    pub enc: core::ffi::c_int,
    pub padding: [u8; 2 * AES_BLOCK_SIZE],
    pub aes_compl: completion,
}

/* Struct definitions end */

unsafe extern "C" {
    pub fn dthe_get_dev(ctx: *mut dthe_tfm_ctx) -> *mut dthe_data;

    /**
     * dthe_copy_sg - Copy sg entries from src to dst
     * @dst: Destination sg to be filled
     * @src: Source sg to be copied from
     * @buflen: Number of bytes to be copied
     *
     * Description:
     *   Copy buflen bytes of data from src to dst.
     *
     **/
    pub fn dthe_copy_sg(
        dst: *mut scatterlist,
        src: *mut scatterlist,
        buflen: core::ffi::c_int,
    ) -> *mut scatterlist;

    pub fn dthe_register_aes_algs() -> core::ffi::c_int;
    pub fn dthe_unregister_aes_algs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
