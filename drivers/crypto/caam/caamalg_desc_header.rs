/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shared descriptors for aead, skcipher algorithms
 *
 * Copyright 2016, 2025 NXP
 */

/* C header guard _CAAMALG_DESC_H_ omitted in Rust. */

/* length of descriptors text */
pub const DESC_AEAD_BASE: usize = 4 * CAAM_CMD_SZ;
pub const DESC_AEAD_ENC_LEN: usize = DESC_AEAD_BASE + 11 * CAAM_CMD_SZ;
pub const DESC_AEAD_DEC_LEN: usize = DESC_AEAD_BASE + 15 * CAAM_CMD_SZ;
pub const DESC_AEAD_GIVENC_LEN: usize = DESC_AEAD_ENC_LEN + 8 * CAAM_CMD_SZ;
pub const DESC_QI_AEAD_ENC_LEN: usize = DESC_AEAD_ENC_LEN + 3 * CAAM_CMD_SZ;
pub const DESC_QI_AEAD_DEC_LEN: usize = DESC_AEAD_DEC_LEN + 3 * CAAM_CMD_SZ;
pub const DESC_QI_AEAD_GIVENC_LEN: usize = DESC_AEAD_GIVENC_LEN + 3 * CAAM_CMD_SZ;

/* Note: Nonce is counted in cdata.keylen */
pub const DESC_AEAD_CTR_RFC3686_LEN: usize = 4 * CAAM_CMD_SZ;

pub const DESC_AEAD_NULL_BASE: usize = 3 * CAAM_CMD_SZ;
pub const DESC_AEAD_NULL_ENC_LEN: usize = DESC_AEAD_NULL_BASE + 11 * CAAM_CMD_SZ;
pub const DESC_AEAD_NULL_DEC_LEN: usize = DESC_AEAD_NULL_BASE + 13 * CAAM_CMD_SZ;

pub const DESC_GCM_BASE: usize = 3 * CAAM_CMD_SZ;
pub const DESC_GCM_ENC_LEN: usize = DESC_GCM_BASE + 16 * CAAM_CMD_SZ;
pub const DESC_GCM_DEC_LEN: usize = DESC_GCM_BASE + 12 * CAAM_CMD_SZ;
pub const DESC_QI_GCM_ENC_LEN: usize = DESC_GCM_ENC_LEN + 6 * CAAM_CMD_SZ;
pub const DESC_QI_GCM_DEC_LEN: usize = DESC_GCM_DEC_LEN + 3 * CAAM_CMD_SZ;

pub const DESC_RFC4106_BASE: usize = 3 * CAAM_CMD_SZ;
pub const DESC_RFC4106_ENC_LEN: usize = DESC_RFC4106_BASE + 16 * CAAM_CMD_SZ;
pub const DESC_RFC4106_DEC_LEN: usize = DESC_RFC4106_BASE + 13 * CAAM_CMD_SZ;
pub const DESC_QI_RFC4106_ENC_LEN: usize = DESC_RFC4106_ENC_LEN + 5 * CAAM_CMD_SZ;
pub const DESC_QI_RFC4106_DEC_LEN: usize = DESC_RFC4106_DEC_LEN + 5 * CAAM_CMD_SZ;

pub const DESC_RFC4543_BASE: usize = 3 * CAAM_CMD_SZ;
pub const DESC_RFC4543_ENC_LEN: usize = DESC_RFC4543_BASE + 11 * CAAM_CMD_SZ;
pub const DESC_RFC4543_DEC_LEN: usize = DESC_RFC4543_BASE + 12 * CAAM_CMD_SZ;
pub const DESC_QI_RFC4543_ENC_LEN: usize = DESC_RFC4543_ENC_LEN + 4 * CAAM_CMD_SZ;
pub const DESC_QI_RFC4543_DEC_LEN: usize = DESC_RFC4543_DEC_LEN + 4 * CAAM_CMD_SZ;

pub const DESC_SKCIPHER_BASE: usize = 3 * CAAM_CMD_SZ;
pub const DESC_SKCIPHER_ENC_LEN: usize = DESC_SKCIPHER_BASE + 21 * CAAM_CMD_SZ;
pub const DESC_SKCIPHER_DEC_LEN: usize = DESC_SKCIPHER_BASE + 16 * CAAM_CMD_SZ;

/* Key modifier for CAAM Protected blobs */
pub const KEYMOD: &str = "SECURE_KEY";

extern "C" {
    pub fn cnstr_shdsc_aead_null_encap(desc: *mut u32, adata: *mut alginfo, icvsize: u32, era: i32);
    pub fn cnstr_shdsc_aead_null_decap(desc: *mut u32, adata: *mut alginfo, icvsize: u32, era: i32);
    pub fn cnstr_shdsc_aead_encap(desc: *mut u32, cdata: *mut alginfo, adata: *mut alginfo, ivsize: u32, icvsize: u32, is_rfc3686: bool, nonce: *mut u32, ctx1_iv_off: u32, is_qi: bool, era: i32);
    pub fn cnstr_shdsc_aead_decap(desc: *mut u32, cdata: *mut alginfo, adata: *mut alginfo, ivsize: u32, icvsize: u32, geniv: bool, is_rfc3686: bool, nonce: *mut u32, ctx1_iv_off: u32, is_qi: bool, era: i32);
    pub fn cnstr_shdsc_aead_givencap(desc: *mut u32, cdata: *mut alginfo, adata: *mut alginfo, ivsize: u32, icvsize: u32, is_rfc3686: bool, nonce: *mut u32, ctx1_iv_off: u32, is_qi: bool, era: i32);
    pub fn cnstr_shdsc_gcm_encap(desc: *mut u32, cdata: *mut alginfo, ivsize: u32, icvsize: u32, is_qi: bool);
    pub fn cnstr_shdsc_gcm_decap(desc: *mut u32, cdata: *mut alginfo, ivsize: u32, icvsize: u32, is_qi: bool);
    pub fn cnstr_shdsc_rfc4106_encap(desc: *mut u32, cdata: *mut alginfo, ivsize: u32, icvsize: u32, is_qi: bool);
    pub fn cnstr_shdsc_rfc4106_decap(desc: *mut u32, cdata: *mut alginfo, ivsize: u32, icvsize: u32, is_qi: bool);
    pub fn cnstr_shdsc_rfc4543_encap(desc: *mut u32, cdata: *mut alginfo, ivsize: u32, icvsize: u32, is_qi: bool);
    pub fn cnstr_shdsc_rfc4543_decap(desc: *mut u32, cdata: *mut alginfo, ivsize: u32, icvsize: u32, is_qi: bool);
    pub fn cnstr_shdsc_chachapoly(desc: *mut u32, cdata: *mut alginfo, adata: *mut alginfo, ivsize: u32, icvsize: u32, encap: bool, is_qi: bool);
    pub fn cnstr_shdsc_skcipher_encap(desc: *mut u32, cdata: *mut alginfo, ivsize: u32, is_rfc3686: bool, ctx1_iv_off: u32);
    pub fn cnstr_shdsc_skcipher_decap(desc: *mut u32, cdata: *mut alginfo, ivsize: u32, is_rfc3686: bool, ctx1_iv_off: u32);
    pub fn cnstr_shdsc_xts_skcipher_encap(desc: *mut u32, cdata: *mut alginfo);
    pub fn cnstr_shdsc_xts_skcipher_decap(desc: *mut u32, cdata: *mut alginfo);
    pub fn cnstr_desc_protected_blob_decap(desc: *mut u32, cdata: *mut alginfo, next_desc: dma_addr_t);
    pub fn cnstr_desc_skcipher_enc_dec(desc: *mut u32, cdata: *mut alginfo, src: dma_addr_t, dst: dma_addr_t, data_sz: u32, in_options: u32, out_options: u32, ivsize: u32, encrypt: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
