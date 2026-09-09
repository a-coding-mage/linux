/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

/* \file cc_aead.h
 * ARM CryptoCell AEAD Crypto API
 */

/* Dependencies supplied by the surrounding kernel/Rust translation. */

/* mac_cmp - HW writes 8 B but all bytes hold the same value */
pub const ICV_CMP_SIZE: usize = 8;
pub const CCM_CONFIG_BUF_SIZE: usize = AES_BLOCK_SIZE * 3;
pub const MAX_MAC_SIZE: usize = SHA256_DIGEST_SIZE;

/* defines for AES GCM configuration buffer */
pub const GCM_BLOCK_LEN_SIZE: usize = 8;

pub const GCM_BLOCK_RFC4_IV_OFFSET: usize = 4;
pub const GCM_BLOCK_RFC4_IV_SIZE: usize = 8; /* IV size for rfc's */
pub const GCM_BLOCK_RFC4_NONCE_OFFSET: usize = 0;
pub const GCM_BLOCK_RFC4_NONCE_SIZE: usize = 4;

/* Offsets into AES CCM configuration buffer */
pub const CCM_B0_OFFSET: usize = 0;
pub const CCM_A0_OFFSET: usize = 16;
pub const CCM_CTR_COUNT_0_OFFSET: usize = 32;
/* CCM B0 and CTR_COUNT constants. */
pub const CCM_BLOCK_NONCE_OFFSET: usize = 1; /* Nonce offset inside B0 and CTR_COUNT */
pub const CCM_BLOCK_NONCE_SIZE: usize = 3; /* Nonce size inside B0 and CTR_COUNT */
pub const CCM_BLOCK_IV_OFFSET: usize = 4; /* IV offset inside B0 and CTR_COUNT */
pub const CCM_BLOCK_IV_SIZE: usize = 8; /* IV size inside B0 and CTR_COUNT */

#[repr(i32)]
pub enum AeadCcmHeaderSize {
    CcmHeaderSizeNull = -1,
    CcmHeaderSizeZero = 0,
    CcmHeaderSize2 = 2,
    CcmHeaderSize6 = 6,
    CcmHeaderSizeMax = S32_MAX,
}

#[repr(C)]
pub struct AeadReqCtxGcmLenBlock {
    pub len_a: [u8; GCM_BLOCK_LEN_SIZE],
    pub len_c: [u8; GCM_BLOCK_LEN_SIZE],
}

#[repr(C)]
pub struct AeadReqCtx {
    /* Allocate cache line although only 4 bytes are needed to
     * assure next field falls @ cache line
     * Used for both: digest HW compare and CCM/GCM MAC value
     */
    pub mac_buf: [u8; MAX_MAC_SIZE],
    pub ctr_iv: [u8; AES_BLOCK_SIZE],

    // used in gcm
    pub gcm_iv_inc1: [u8; AES_BLOCK_SIZE],
    pub gcm_iv_inc2: [u8; AES_BLOCK_SIZE],
    pub hkey: [u8; AES_BLOCK_SIZE],
    pub gcm_len_block: AeadReqCtxGcmLenBlock,

    pub ccm_config: [u8; CCM_CONFIG_BUF_SIZE],
    /* HW actual size input */
    pub hw_iv_size: core::ffi::c_uint,
    /* used to prevent cache coherence problem */
    pub backup_mac: [u8; MAX_MAC_SIZE],
    pub backup_iv: *mut u8, /* store orig iv */
    pub assoclen: u32, /* size of AAD buffer to authenticate */
    pub mac_buf_dma_addr: dma_addr_t, /* internal ICV DMA buffer */
    /* buffer for internal ccm configurations */
    pub ccm_iv0_dma_addr: dma_addr_t,
    pub icv_dma_addr: dma_addr_t, /* Phys. address of ICV */

    // used in gcm
    /* buffer for internal gcm configurations */
    pub gcm_iv_inc1_dma_addr: dma_addr_t,
    /* buffer for internal gcm configurations */
    pub gcm_iv_inc2_dma_addr: dma_addr_t,
    pub hkey_dma_addr: dma_addr_t, /* Phys. address of hkey */
    pub gcm_block_len_dma_addr: dma_addr_t, /* Phys. address of gcm block len */

    pub icv_virt_addr: *mut u8, /* Virt. address of ICV */
    pub gen_ctx: async_gen_req_ctx,
    pub assoc: cc_mlli,
    pub src: cc_mlli,
    pub dst: cc_mlli,
    pub src_sgl: *mut scatterlist,
    pub dst_sgl: *mut scatterlist,
    pub src_offset: core::ffi::c_uint,
    pub dst_offset: core::ffi::c_uint,
    pub assoc_buff_type: cc_req_dma_buf_type,
    pub data_buff_type: cc_req_dma_buf_type,
    pub mlli_params: mlli_params,
    pub cryptlen: core::ffi::c_uint,
    pub ccm_adata_sg: scatterlist,
    pub ccm_hdr_size: AeadCcmHeaderSize,
    pub req_authsize: core::ffi::c_uint,
    pub cipher_mode: drv_cipher_mode,
    pub is_icv_fragmented: bool,
    pub is_single_pass: bool,
    pub plaintext_authenticate_only: bool, // for gcm_rfc4543
}

pub extern "C" {
    pub fn cc_aead_alloc(drvdata: *mut cc_drvdata) -> core::ffi::c_int;
    pub fn cc_aead_free(drvdata: *mut cc_drvdata) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
