/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (C) 2020 Marvell.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const OTX2_CPT_MAX_ENC_KEY_SIZE: usize = 32;
pub const OTX2_CPT_MAX_HASH_KEY_SIZE: usize = 64;
pub const OTX2_CPT_MAX_KEY_SIZE: usize =
    OTX2_CPT_MAX_ENC_KEY_SIZE + OTX2_CPT_MAX_HASH_KEY_SIZE;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum otx2_cpt_request_type {
    OTX2_CPT_ENC_DEC_REQ = 0x1,
    OTX2_CPT_AEAD_ENC_DEC_REQ = 0x2,
    OTX2_CPT_AEAD_ENC_DEC_NULL_REQ = 0x3,
    OTX2_CPT_PASSTHROUGH_REQ = 0x4,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum otx2_cpt_major_opcodes {
    OTX2_CPT_MAJOR_OP_MISC = 0x01,
    OTX2_CPT_MAJOR_OP_FC = 0x33,
    OTX2_CPT_MAJOR_OP_HMAC = 0x35,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum otx2_cpt_cipher_type {
    OTX2_CPT_CIPHER_NULL = 0x0,
    OTX2_CPT_DES3_CBC = 0x1,
    OTX2_CPT_DES3_ECB = 0x2,
    OTX2_CPT_AES_CBC = 0x3,
    OTX2_CPT_AES_ECB = 0x4,
    OTX2_CPT_AES_CFB = 0x5,
    OTX2_CPT_AES_CTR = 0x6,
    OTX2_CPT_AES_GCM = 0x7,
    OTX2_CPT_AES_XTS = 0x8,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum otx2_cpt_mac_type {
    OTX2_CPT_MAC_NULL = 0x0,
    OTX2_CPT_MD5 = 0x1,
    OTX2_CPT_SHA1 = 0x2,
    OTX2_CPT_SHA224 = 0x3,
    OTX2_CPT_SHA256 = 0x4,
    OTX2_CPT_SHA384 = 0x5,
    OTX2_CPT_SHA512 = 0x6,
    OTX2_CPT_GMAC = 0x7,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum otx2_cpt_aes_key_len {
    OTX2_CPT_AES_128_BIT = 0x1,
    OTX2_CPT_AES_192_BIT = 0x2,
    OTX2_CPT_AES_256_BIT = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_encr_ctrl_bits {
    // C bitfields; the containing u64 preserves their packed storage.
    pub bits: u64,
}

#[repr(C)]
pub union otx2_cpt_encr_ctrl {
    pub u: u64,
    pub e: otx2_cpt_encr_ctrl_bits,
}

#[repr(C)]
pub struct otx2_cpt_cipher {
    pub name: *const core::ffi::c_char,
    pub value: u8,
}

#[repr(C)]
pub struct otx2_cpt_fc_enc_ctx {
    pub enc_ctrl: otx2_cpt_encr_ctrl,
    pub encr_key: [u8; 32],
    pub encr_iv: [u8; 16],
}

#[repr(C)]
pub struct otx2_cpt_fc_hmac_ctx_e {
    pub ipad: [u8; 64],
    pub opad: [u8; 64],
}

#[repr(C)]
pub struct otx2_cpt_fc_hmac_ctx_s {
    pub hmac_calc: [u8; 64], /* HMAC calculated */
    pub hmac_recv: [u8; 64], /* HMAC received */
}

#[repr(C)]
pub union otx2_cpt_fc_hmac_ctx {
    pub e: otx2_cpt_fc_hmac_ctx_e,
    pub s: otx2_cpt_fc_hmac_ctx_s,
}

#[repr(C)]
pub struct otx2_cpt_fc_ctx {
    pub enc: otx2_cpt_fc_enc_ctx,
    pub hmac: otx2_cpt_fc_hmac_ctx,
}

#[repr(C)]
pub struct otx2_cpt_enc_ctx {
    pub key_len: u32,
    pub enc_key: [u8; OTX2_CPT_MAX_KEY_SIZE],
    pub cipher_type: u8,
    pub key_type: u8,
    pub enc_align_len: u8,
    pub fbk_cipher: *mut crypto_skcipher,
    pub pdev: *mut pci_dev,
    pub er_ctx: cn10k_cpt_errata_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_offset_ctrl_bits {
    pub bits: u64,
}

#[repr(C)]
pub union otx2_cpt_offset_ctrl {
    pub flags: u64,
    pub e: otx2_cpt_offset_ctrl_bits,
}

#[repr(C)]
pub union otx2_cpt_req_ctx_request {
    pub sk_fbk_req: skcipher_request,
    pub fbk_req: aead_request,
}

#[repr(C)]
pub struct otx2_cpt_req_ctx {
    pub cpt_req: otx2_cpt_req_info,
    pub ctrl_word: otx2_cpt_offset_ctrl,
    pub fctx: otx2_cpt_fc_ctx,
    pub request: otx2_cpt_req_ctx_request,
}

#[repr(C)]
pub struct otx2_cpt_sdesc {
    pub shash: shash_desc,
}

#[repr(C)]
pub struct otx2_cpt_aead_ctx {
    pub key: [u8; OTX2_CPT_MAX_KEY_SIZE],
    pub hashalg: *mut crypto_shash,
    pub sdesc: *mut otx2_cpt_sdesc,
    pub fbk_cipher: *mut crypto_aead,
    pub er_ctx: cn10k_cpt_errata_ctx,
    pub pdev: *mut pci_dev,
    pub ipad: *mut u8,
    pub opad: *mut u8,
    pub enc_key_len: u32,
    pub auth_key_len: u32,
    pub cipher_type: u8,
    pub mac_type: u8,
    pub key_type: u8,
    pub is_trunc_hmac: u8,
    pub enc_align_len: u8,
}

extern "C" {
    pub fn otx2_cpt_crypto_init(
        pdev: *mut pci_dev,
        mod_: *mut module,
        num_queues: i32,
        num_devices: i32,
    ) -> i32;
    pub fn otx2_cpt_crypto_exit(pdev: *mut pci_dev, mod_: *mut module);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
