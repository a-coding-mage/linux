/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2016 Broadcom
 */

/*
 * This file contains SPU message definitions specific to SPU2.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum spu2_cipher_type {
    SPU2_CIPHER_TYPE_NONE = 0x0,
    SPU2_CIPHER_TYPE_AES128 = 0x1,
    SPU2_CIPHER_TYPE_AES192 = 0x2,
    SPU2_CIPHER_TYPE_AES256 = 0x3,
    SPU2_CIPHER_TYPE_DES = 0x4,
    SPU2_CIPHER_TYPE_3DES = 0x5,
    SPU2_CIPHER_TYPE_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum spu2_cipher_mode {
    SPU2_CIPHER_MODE_ECB = 0x0,
    SPU2_CIPHER_MODE_CBC = 0x1,
    SPU2_CIPHER_MODE_CTR = 0x2,
    SPU2_CIPHER_MODE_CFB = 0x3,
    SPU2_CIPHER_MODE_OFB = 0x4,
    SPU2_CIPHER_MODE_XTS = 0x5,
    SPU2_CIPHER_MODE_CCM = 0x6,
    SPU2_CIPHER_MODE_GCM = 0x7,
    SPU2_CIPHER_MODE_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum spu2_hash_type {
    SPU2_HASH_TYPE_NONE = 0x0,
    SPU2_HASH_TYPE_AES128 = 0x1,
    SPU2_HASH_TYPE_AES192 = 0x2,
    SPU2_HASH_TYPE_AES256 = 0x3,
    SPU2_HASH_TYPE_MD5 = 0x6,
    SPU2_HASH_TYPE_SHA1 = 0x7,
    SPU2_HASH_TYPE_SHA224 = 0x8,
    SPU2_HASH_TYPE_SHA256 = 0x9,
    SPU2_HASH_TYPE_SHA384 = 0xa,
    SPU2_HASH_TYPE_SHA512 = 0xb,
    SPU2_HASH_TYPE_SHA512_224 = 0xc,
    SPU2_HASH_TYPE_SHA512_256 = 0xd,
    SPU2_HASH_TYPE_SHA3_224 = 0xe,
    SPU2_HASH_TYPE_SHA3_256 = 0xf,
    SPU2_HASH_TYPE_SHA3_384 = 0x10,
    SPU2_HASH_TYPE_SHA3_512 = 0x11,
    SPU2_HASH_TYPE_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum spu2_hash_mode {
    SPU2_HASH_MODE_CMAC = 0x0,
    SPU2_HASH_MODE_CBC_MAC = 0x1,
    SPU2_HASH_MODE_XCBC_MAC = 0x2,
    SPU2_HASH_MODE_HMAC = 0x3,
    SPU2_HASH_MODE_RABIN = 0x4,
    SPU2_HASH_MODE_CCM = 0x5,
    SPU2_HASH_MODE_GCM = 0x6,
    SPU2_HASH_MODE_RESERVED = 0x7,
    SPU2_HASH_MODE_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum spu2_ret_md_opts {
    SPU2_RET_NO_MD = 0, // return no metadata
    SPU2_RET_FMD_OMD = 1, // return both FMD and OMD
    SPU2_RET_FMD_ONLY = 2, // return only FMD
    SPU2_RET_FMD_OMD_IV = 3, // return FMD and OMD with just IVs
}

/* Fixed Metadata format */
#[repr(C)]
pub struct SPU2_FMD {
    pub ctrl0: u64,
    pub ctrl1: u64,
    pub ctrl2: u64,
    pub ctrl3: u64,
}

pub const FMD_SIZE: usize = core::mem::size_of::<SPU2_FMD>();

/* Fixed part of request message header length in bytes. Just FMD. */
pub const SPU2_REQ_FIXED_LEN: usize = FMD_SIZE;
pub const SPU2_HEADER_ALLOC_LEN: usize = SPU_REQ_FIXED_LEN + 2 * MAX_KEY_SIZE + 2 * MAX_IV_SIZE;

/* FMD ctrl0 field masks */
pub const SPU2_CIPH_ENCRYPT_EN: u64 = 0x1;
pub const SPU2_CIPH_TYPE: u64 = 0xF0;
pub const SPU2_CIPH_TYPE_SHIFT: u32 = 4;
pub const SPU2_CIPH_MODE: u64 = 0xF00;
pub const SPU2_CIPH_MODE_SHIFT: u32 = 8;
pub const SPU2_CFB_MASK: u64 = 0x7000;
pub const SPU2_CFB_MASK_SHIFT: u32 = 12;
pub const SPU2_PROTO_SEL: u64 = 0xF00000;
pub const SPU2_PROTO_SEL_SHIFT: u32 = 20;
pub const SPU2_HASH_FIRST: u64 = 0x1000000;
pub const SPU2_CHK_TAG: u64 = 0x2000000;
pub const SPU2_HASH_TYPE: u64 = 0x1F0000000;
pub const SPU2_HASH_TYPE_SHIFT: u32 = 28;
pub const SPU2_HASH_MODE: u64 = 0xF000000000;
pub const SPU2_HASH_MODE_SHIFT: u32 = 36;
pub const SPU2_CIPH_PAD_EN: u64 = 0x100000000000;
pub const SPU2_CIPH_PAD: u64 = 0xFF000000000000;
pub const SPU2_CIPH_PAD_SHIFT: u32 = 48;

/* FMD ctrl1 field masks */
pub const SPU2_TAG_LOC: u64 = 0x1;
pub const SPU2_HAS_FR_DATA: u64 = 0x2;
pub const SPU2_HAS_AAD1: u64 = 0x4;
pub const SPU2_HAS_NAAD: u64 = 0x8;
pub const SPU2_HAS_AAD2: u64 = 0x10;
pub const SPU2_HAS_ESN: u64 = 0x20;
pub const SPU2_HASH_KEY_LEN: u64 = 0xFF00;
pub const SPU2_HASH_KEY_LEN_SHIFT: u32 = 8;
pub const SPU2_CIPH_KEY_LEN: u64 = 0xFF00000;
pub const SPU2_CIPH_KEY_LEN_SHIFT: u32 = 20;
pub const SPU2_GENIV: u64 = 0x10000000;
pub const SPU2_HASH_IV: u64 = 0x20000000;
pub const SPU2_RET_IV: u64 = 0x40000000;
pub const SPU2_RET_IV_LEN: u64 = 0xF00000000;
pub const SPU2_RET_IV_LEN_SHIFT: u32 = 32;
pub const SPU2_IV_OFFSET: u64 = 0xF000000000;
pub const SPU2_IV_OFFSET_SHIFT: u32 = 36;
pub const SPU2_IV_LEN: u64 = 0x1F0000000000;
pub const SPU2_IV_LEN_SHIFT: u32 = 40;
pub const SPU2_HASH_TAG_LEN: u64 = 0x7F000000000000;
pub const SPU2_HASH_TAG_LEN_SHIFT: u32 = 48;
pub const SPU2_RETURN_MD: u64 = 0x300000000000000;
pub const SPU2_RETURN_MD_SHIFT: u32 = 56;
pub const SPU2_RETURN_FD: u64 = 0x400000000000000;
pub const SPU2_RETURN_AAD1: u64 = 0x800000000000000;
pub const SPU2_RETURN_NAAD: u64 = 0x1000000000000000;
pub const SPU2_RETURN_AAD2: u64 = 0x2000000000000000;
pub const SPU2_RETURN_PAY: u64 = 0x4000000000000000;

/* FMD ctrl2 field masks */
pub const SPU2_AAD1_OFFSET: u64 = 0xFFF;
pub const SPU2_AAD1_LEN: u64 = 0xFF000;
pub const SPU2_AAD1_LEN_SHIFT: u32 = 12;
pub const SPU2_AAD2_OFFSET: u64 = 0xFFF00000;
pub const SPU2_AAD2_OFFSET_SHIFT: u32 = 20;
pub const SPU2_PL_OFFSET: u64 = 0xFFFFFFFF00000000;
pub const SPU2_PL_OFFSET_SHIFT: u32 = 32;

/* FMD ctrl3 field masks */
pub const SPU2_PL_LEN: u64 = 0xFFFFFFFF;
pub const SPU2_TLS_LEN: u64 = 0xFFFF00000000;
pub const SPU2_TLS_LEN_SHIFT: u32 = 32;

/* Max value that can be represented in the Payload Length field of the ctrl3 word of FMD. */
pub const SPU2_MAX_PAYLOAD: u64 = SPU2_PL_LEN;

/* Error values returned in STATUS field of response messages */
pub const SPU2_INVALID_ICV: u32 = 1;

extern "C" {
    pub fn spu2_dump_msg_hdr(buf: *mut u8, buf_len: ::core::ffi::c_uint);
    pub fn spu2_ctx_max_payload(cipher_alg: enum_spu_cipher_alg, cipher_mode: enum_spu_cipher_mode, blocksize: ::core::ffi::c_uint) -> u32;
    pub fn spu2_payload_length(spu_hdr: *mut u8) -> u32;
    pub fn spu2_response_hdr_len(auth_key_len: u16, enc_key_len: u16, is_hash: bool) -> u16;
    pub fn spu2_hash_pad_len(hash_alg: enum_hash_alg, hash_mode: enum_hash_mode, chunksize: u32, hash_block_size: u16) -> u16;
    pub fn spu2_gcm_ccm_pad_len(cipher_mode: enum_spu_cipher_mode, data_size: ::core::ffi::c_uint) -> u32;
    pub fn spu2_assoc_resp_len(cipher_mode: enum_spu_cipher_mode, assoc_len: ::core::ffi::c_uint, iv_len: ::core::ffi::c_uint, is_encrypt: bool) -> u32;
    pub fn spu2_aead_ivlen(cipher_mode: enum_spu_cipher_mode, iv_len: u16) -> u8;
    pub fn spu2_hash_type(src_sent: u32) -> enum_hash_type;
    pub fn spu2_digest_size(alg_digest_size: u32, alg: enum_hash_alg, htype: enum_hash_type) -> u32;
    pub fn spu2_create_request(spu_hdr: *mut u8, req_opts: *mut spu_request_opts, cipher_parms: *mut spu_cipher_parms, hash_parms: *mut spu_hash_parms, aead_parms: *mut spu_aead_parms, data_size: ::core::ffi::c_uint) -> u32;
    pub fn spu2_cipher_req_init(spu_hdr: *mut u8, cipher_parms: *mut spu_cipher_parms) -> u16;
    pub fn spu2_cipher_req_finish(spu_hdr: *mut u8, spu_req_hdr_len: u16, is_inbound: ::core::ffi::c_uint, cipher_parms: *mut spu_cipher_parms, data_size: ::core::ffi::c_uint);
    pub fn spu2_request_pad(pad_start: *mut u8, gcm_padding: u32, hash_pad_len: u32, auth_alg: enum_hash_alg, auth_mode: enum_hash_mode, total_sent: ::core::ffi::c_uint, status_padding: u32);
    pub fn spu2_xts_tweak_in_payload() -> u8;
    pub fn spu2_tx_status_len() -> u8;
    pub fn spu2_rx_status_len() -> u8;
    pub fn spu2_status_process(statp: *mut u8) -> ::core::ffi::c_int;
    pub fn spu2_ccm_update_iv(digestsize: ::core::ffi::c_uint, cipher_parms: *mut spu_cipher_parms, assoclen: ::core::ffi::c_uint, chunksize: ::core::ffi::c_uint, is_encrypt: bool, is_esp: bool);
    pub fn spu2_wordalign_padlen(data_size: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
