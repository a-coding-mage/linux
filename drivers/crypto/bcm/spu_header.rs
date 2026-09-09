/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2016 Broadcom
 */

/*
 * This file contains the definition of SPU messages. There are currently two
 * SPU message formats: SPU-M and SPU2. The hardware uses different values to
 * identify the same things in SPU-M vs SPU2. So this file defines values that
 * are hardware independent. Software can use these values for any version of
 * SPU hardware. These values are used in APIs in spu.c. Functions internal to
 * spu.c and spu2.c convert these to hardware-specific values.
 */

// Dependencies supplied by the surrounding kernel translation.

pub type SpuCipherAlg = u32;
pub const CIPHER_ALG_NONE: SpuCipherAlg = 0x0;
pub const CIPHER_ALG_RC4: SpuCipherAlg = 0x1;
pub const CIPHER_ALG_DES: SpuCipherAlg = 0x2;
pub const CIPHER_ALG_3DES: SpuCipherAlg = 0x3;
pub const CIPHER_ALG_AES: SpuCipherAlg = 0x4;
pub const CIPHER_ALG_LAST: SpuCipherAlg = 0x5;

pub type SpuCipherMode = u32;
pub const CIPHER_MODE_NONE: SpuCipherMode = 0x0;
pub const CIPHER_MODE_ECB: SpuCipherMode = 0x0;
pub const CIPHER_MODE_CBC: SpuCipherMode = 0x1;
pub const CIPHER_MODE_OFB: SpuCipherMode = 0x2;
pub const CIPHER_MODE_CFB: SpuCipherMode = 0x3;
pub const CIPHER_MODE_CTR: SpuCipherMode = 0x4;
pub const CIPHER_MODE_CCM: SpuCipherMode = 0x5;
pub const CIPHER_MODE_GCM: SpuCipherMode = 0x6;
pub const CIPHER_MODE_XTS: SpuCipherMode = 0x7;
pub const CIPHER_MODE_LAST: SpuCipherMode = 0x8;

pub type SpuCipherType = u32;
pub const CIPHER_TYPE_NONE: SpuCipherType = 0x0;
pub const CIPHER_TYPE_DES: SpuCipherType = 0x0;
pub const CIPHER_TYPE_3DES: SpuCipherType = 0x0;
pub const CIPHER_TYPE_INIT: SpuCipherType = 0x0;
pub const CIPHER_TYPE_AES128: SpuCipherType = 0x0;
pub const CIPHER_TYPE_AES192: SpuCipherType = 0x1;
pub const CIPHER_TYPE_UPDT: SpuCipherType = 0x1;
pub const CIPHER_TYPE_AES256: SpuCipherType = 0x2;

pub type HashAlg = u32;
pub const HASH_ALG_NONE: HashAlg = 0x0;
pub const HASH_ALG_MD5: HashAlg = 0x1;
pub const HASH_ALG_SHA1: HashAlg = 0x2;
pub const HASH_ALG_SHA224: HashAlg = 0x3;
pub const HASH_ALG_SHA256: HashAlg = 0x4;
pub const HASH_ALG_AES: HashAlg = 0x5;
pub const HASH_ALG_SHA384: HashAlg = 0x6;
pub const HASH_ALG_SHA512: HashAlg = 0x7;
/* Keep SHA3 algorithms at the end always */
pub const HASH_ALG_SHA3_224: HashAlg = 0x8;
pub const HASH_ALG_SHA3_256: HashAlg = 0x9;
pub const HASH_ALG_SHA3_384: HashAlg = 0xa;
pub const HASH_ALG_SHA3_512: HashAlg = 0xb;
pub const HASH_ALG_LAST: HashAlg = 0xc;

pub type HashMode = u32;
pub const HASH_MODE_NONE: HashMode = 0x0;
pub const HASH_MODE_HASH: HashMode = 0x0;
pub const HASH_MODE_XCBC: HashMode = 0x0;
pub const HASH_MODE_CMAC: HashMode = 0x1;
pub const HASH_MODE_CTXT: HashMode = 0x1;
pub const HASH_MODE_HMAC: HashMode = 0x2;
pub const HASH_MODE_RABIN: HashMode = 0x4;
pub const HASH_MODE_FHMAC: HashMode = 0x6;
pub const HASH_MODE_CCM: HashMode = 0x5;
pub const HASH_MODE_GCM: HashMode = 0x6;

pub type HashType = u32;
pub const HASH_TYPE_NONE: HashType = 0x0;
pub const HASH_TYPE_FULL: HashType = 0x0;
pub const HASH_TYPE_INIT: HashType = 0x1;
pub const HASH_TYPE_UPDT: HashType = 0x2;
pub const HASH_TYPE_FIN: HashType = 0x3;
pub const HASH_TYPE_AES128: HashType = 0x0;
pub const HASH_TYPE_AES192: HashType = 0x1;
pub const HASH_TYPE_AES256: HashType = 0x2;

pub type AeadType = u32;
pub const AES_CCM: AeadType = 0;
pub const AES_GCM: AeadType = 1;
pub const AUTHENC: AeadType = 2;
pub const AEAD_TYPE_LAST: AeadType = 3;

extern "C" {
    pub static mut hash_alg_name: [*mut core::ffi::c_char; HASH_ALG_LAST as usize];
    pub static mut aead_alg_name: [*mut core::ffi::c_char; AEAD_TYPE_LAST as usize];
}

#[repr(C)]
pub struct SpuRequestOpts {
    pub is_inbound: bool,
    pub auth_first: bool,
    pub is_aead: bool,
    pub is_esp: bool,
    pub bd_suppress: bool,
    pub is_rfc4543: bool,
}

#[repr(C)]
pub struct SpuCipherParms {
    pub alg: SpuCipherAlg,
    pub mode: SpuCipherMode,
    pub type_: SpuCipherType,
    pub key_buf: *mut u8,
    pub key_len: u16,
    pub iv_buf: *mut u8,
    pub iv_len: u16,
}

#[repr(C)]
pub struct SpuHashParms {
    pub alg: HashAlg,
    pub mode: HashMode,
    pub type_: HashType,
    pub digestsize: u8,
    pub key_buf: *mut u8,
    pub key_len: u16,
    pub prebuf_len: u16,
    pub pad_len: i32,
}

#[repr(C)]
pub struct SpuAeadParms {
    pub assoc_size: u32,
    pub iv_len: u16,
    pub aad_pad_len: u8,
    pub data_pad_len: u8,
    pub return_iv: bool,
    pub ret_iv_len: u32,
    pub ret_iv_off: u32,
}

pub const SPU_RX_STATUS_LEN: u32 = 4;
pub const SPU_STAT_PAD_MAX: u32 = 4;
pub const SPU_GCM_CCM_ALIGN: u32 = 16;
pub const SPU_PAD_LEN_MAX: u32 = SPU_GCM_CCM_ALIGN + MAX_HASH_BLOCK_SIZE + SPU_STAT_PAD_MAX;
pub const SPU_SUPDT_LEN: u32 = 260;
pub const SPU_INVALID_ICV: u32 = 1;
pub const SPU_MAX_PAYLOAD_INF: u32 = 0xFFFFFFFF;
pub const SPU_XTS_TWEAK_SIZE: u32 = 16;
pub const CCM_B0_ADATA: u32 = 0x40;
pub const CCM_B0_ADATA_SHIFT: u32 = 6;
pub const CCM_B0_M_PRIME: u32 = 0x38;
pub const CCM_B0_M_PRIME_SHIFT: u32 = 3;
pub const CCM_B0_L_PRIME: u32 = 0x07;
pub const CCM_B0_L_PRIME_SHIFT: u32 = 0;
pub const CCM_ESP_L_VALUE: u32 = 4;

pub const unsafe fn spu_req_incl_icv(cipher_mode: SpuCipherMode, is_encrypt: bool) -> bool {
    if (cipher_mode == CIPHER_MODE_GCM) && !is_encrypt {
        return true;
    }
    if (cipher_mode == CIPHER_MODE_CCM) && !is_encrypt {
        return true;
    }
    false
}

pub const unsafe fn spu_real_db_size(
    assoc_size: u32, aead_iv_buf_len: u32, prebuf_len: u32, data_size: u32,
    aad_pad_len: u32, gcm_pad_len: u32, hash_pad_len: u32,
) -> u32 {
    assoc_size + aead_iv_buf_len + prebuf_len + data_size + aad_pad_len + gcm_pad_len + hash_pad_len
}

extern "C" {
    pub fn spum_dump_msg_hdr(buf: *mut u8, buf_len: core::ffi::c_uint);
    pub fn spum_ns2_ctx_max_payload(cipher_alg: SpuCipherAlg, cipher_mode: SpuCipherMode, blocksize: core::ffi::c_uint) -> u32;
    pub fn spum_nsp_ctx_max_payload(cipher_alg: SpuCipherAlg, cipher_mode: SpuCipherMode, blocksize: core::ffi::c_uint) -> u32;
    pub fn spum_payload_length(spu_hdr: *mut u8) -> u32;
    pub fn spum_response_hdr_len(auth_key_len: u16, enc_key_len: u16, is_hash: bool) -> u16;
    pub fn spum_hash_pad_len(hash_alg: HashAlg, hash_mode: HashMode, chunksize: u32, hash_block_size: u16) -> u16;
    pub fn spum_gcm_ccm_pad_len(cipher_mode: SpuCipherMode, data_size: core::ffi::c_uint) -> u32;
    pub fn spum_assoc_resp_len(cipher_mode: SpuCipherMode, assoc_len: core::ffi::c_uint, iv_len: core::ffi::c_uint, is_encrypt: bool) -> u32;
    pub fn spum_aead_ivlen(cipher_mode: SpuCipherMode, iv_len: u16) -> u8;
    pub fn spum_hash_type(src_sent: u32) -> HashType;
    pub fn spum_digest_size(alg_digest_size: u32, alg: HashAlg, htype: HashType) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
