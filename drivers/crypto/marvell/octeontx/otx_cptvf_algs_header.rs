/* SPDX-License-Identifier: GPL-2.0
 * Marvell OcteonTX CPT driver
 *
 * Copyright (C) 2019 Marvell International Ltd.
 */

// Dependencies supplied by the surrounding translation unit:
// crypto/hash.h and otx_cpt_common.h.

pub const OTX_CPT_MAX_ENC_KEY_SIZE: usize = 32;
pub const OTX_CPT_MAX_HASH_KEY_SIZE: usize = 64;
pub const OTX_CPT_MAX_KEY_SIZE: usize = OTX_CPT_MAX_ENC_KEY_SIZE + OTX_CPT_MAX_HASH_KEY_SIZE;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum otx_cpt_request_type {
    OTX_CPT_ENC_DEC_REQ = 0x1,
    OTX_CPT_AEAD_ENC_DEC_REQ = 0x2,
    OTX_CPT_AEAD_ENC_DEC_NULL_REQ = 0x3,
    OTX_CPT_PASSTHROUGH_REQ = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum otx_cpt_major_opcodes {
    OTX_CPT_MAJOR_OP_MISC = 0x01,
    OTX_CPT_MAJOR_OP_FC = 0x33,
    OTX_CPT_MAJOR_OP_HMAC = 0x35,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum otx_cpt_req_type { OTX_CPT_AE_CORE_REQ, OTX_CPT_SE_CORE_REQ }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum otx_cpt_cipher_type {
    OTX_CPT_CIPHER_NULL = 0x0, OTX_CPT_DES3_CBC = 0x1, OTX_CPT_DES3_ECB = 0x2,
    OTX_CPT_AES_CBC = 0x3, OTX_CPT_AES_ECB = 0x4, OTX_CPT_AES_CFB = 0x5,
    OTX_CPT_AES_CTR = 0x6, OTX_CPT_AES_GCM = 0x7, OTX_CPT_AES_XTS = 0x8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum otx_cpt_mac_type {
    OTX_CPT_MAC_NULL = 0x0, OTX_CPT_MD5 = 0x1, OTX_CPT_SHA1 = 0x2,
    OTX_CPT_SHA224 = 0x3, OTX_CPT_SHA256 = 0x4, OTX_CPT_SHA384 = 0x5,
    OTX_CPT_SHA512 = 0x6, OTX_CPT_GMAC = 0x7,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum otx_cpt_aes_key_len { OTX_CPT_AES_128_BIT = 0x1, OTX_CPT_AES_192_BIT = 0x2, OTX_CPT_AES_256_BIT = 0x3 }

// C bitfields are represented by their containing 64-bit word; field ordering
// follows __BIG_ENDIAN_BITFIELD / the native little-endian declaration.
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx_cpt_encr_ctrl { pub flags: u64, pub cflags: u64, pub e: u64 }

#[repr(C)]
pub struct otx_cpt_cipher { pub name: *const core::ffi::c_char, pub value: u8 }

#[repr(C)]
pub struct otx_cpt_enc_context { pub enc_ctrl: otx_cpt_encr_ctrl, pub encr_key: [u8; 32], pub encr_iv: [u8; 16] }

#[repr(C)]
pub union otx_cpt_fchmac_ctx { pub e: otx_cpt_fchmac_e, pub s: otx_cpt_fchmac_s }
#[repr(C)]
pub struct otx_cpt_fchmac_e { pub ipad: [u8; 64], pub opad: [u8; 64] }
#[repr(C)]
pub struct otx_cpt_fchmac_s { pub hmac_calc: [u8; 64], pub hmac_recv: [u8; 64] }

#[repr(C)]
pub struct otx_cpt_fc_ctx { pub enc: otx_cpt_enc_context, pub hmac: otx_cpt_fchmac_ctx }

#[repr(C)]
pub struct otx_cpt_enc_ctx { pub key_len: u32, pub enc_key: [u8; OTX_CPT_MAX_KEY_SIZE], pub cipher_type: u8, pub key_type: u8 }
#[repr(C)]
pub struct otx_cpt_des3_ctx { pub key_len: u32, pub des3_key: [u8; OTX_CPT_MAX_KEY_SIZE] }

#[repr(C)]
#[derive(Copy, Clone)]
pub union otx_cpt_offset_ctrl_word { pub flags: u64, pub cflags: u64, pub e: u64 }

#[repr(C)]
pub struct otx_cpt_req_ctx {
    pub cpt_req: otx_cpt_req_info,
    pub ctrl_word: otx_cpt_offset_ctrl_word,
    pub fctx: otx_cpt_fc_ctx,
}

#[repr(C)]
pub struct otx_cpt_sdesc { pub shash: shash_desc }

#[repr(C)]
pub struct otx_cpt_aead_ctx {
    pub key: [u8; OTX_CPT_MAX_KEY_SIZE],
    pub hashalg: *mut crypto_shash,
    pub sdesc: *mut otx_cpt_sdesc,
    pub ipad: *mut u8,
    pub opad: *mut u8,
    pub enc_key_len: u32,
    pub auth_key_len: u32,
    pub cipher_type: u8,
    pub mac_type: u8,
    pub key_type: u8,
    pub is_trunc_hmac: u8,
}

extern "C" {
    pub fn otx_cpt_crypto_init(pdev: *mut pci_dev, mod_: *mut module, pf_type: otx_cptpf_type, engine_type: otx_cptvf_type, num_queues: i32, num_devices: i32) -> i32;
    pub fn otx_cpt_crypto_exit(pdev: *mut pci_dev, mod_: *mut module, engine_type: otx_cptvf_type);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
