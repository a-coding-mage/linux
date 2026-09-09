/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2016 Broadcom
 */

// Dependencies supplied by the surrounding kernel/driver translation unit:
// linux/atomic.h, linux/mailbox/brcm-message.h, linux/mailbox_client.h,
// crypto headers, spu.h, spum.h, and spu2.h.

pub const MAX_SPUS: usize = 16;
pub const ARC4_STATE_SIZE: usize = 4;
pub const CCM_AES_IV_SIZE: usize = 16;
pub const CCM_ESP_IV_SIZE: usize = 8;
pub const RFC4543_ICV_SIZE: usize = 16;
pub const MAX_KEY_SIZE: usize = ARC4_MAX_KEY_SIZE as usize;
pub const MAX_IV_SIZE: usize = AES_BLOCK_SIZE as usize;
pub const MAX_DIGEST_SIZE: usize = SHA3_512_DIGEST_SIZE as usize;
pub const MAX_ASSOC_SIZE: usize = 512;
pub const GCM_ESP_SALT_SIZE: usize = 4;
pub const CCM_ESP_SALT_SIZE: usize = 3;
pub const MAX_SALT_SIZE: usize = GCM_ESP_SALT_SIZE;
pub const GCM_ESP_SALT_OFFSET: usize = 0;
pub const CCM_ESP_SALT_OFFSET: usize = 1;
pub const GCM_ESP_DIGESTSIZE: usize = 16;
pub const MAX_HASH_BLOCK_SIZE: usize = SHA512_BLOCK_SIZE as usize;
pub const HASH_CARRY_MAX: usize = MAX_HASH_BLOCK_SIZE;
pub const SPU_MSG_ALIGN: usize = 4;
pub const SPU_MB_RETRY_MAX: usize = 1000;

#[inline]
pub const fn align(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum op_type {
    SPU_OP_CIPHER,
    SPU_OP_HASH,
    SPU_OP_HMAC,
    SPU_OP_AEAD,
    SPU_OP_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum spu_spu_type { SPU_TYPE_SPUM, SPU_TYPE_SPU2 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum spu_spu_subtype {
    SPU_SUBTYPE_SPUM_NS2,
    SPU_SUBTYPE_SPUM_NSP,
    SPU_SUBTYPE_SPU2_V1,
    SPU_SUBTYPE_SPU2_V2,
}

#[repr(C)]
pub struct spu_type_subtype { pub type_: spu_spu_type, pub subtype: spu_spu_subtype }

#[repr(C)]
pub struct cipher_op { pub alg: spu_cipher_alg, pub mode: spu_cipher_mode }

#[repr(C)]
pub struct auth_op { pub alg: hash_alg, pub mode: hash_mode }

#[repr(C)]
pub struct iproc_alg_s {
    pub type_: u32,
    pub alg: iproc_alg_s_alg,
    pub cipher_info: cipher_op,
    pub auth_info: auth_op,
    pub auth_first: bool,
    pub registered: bool,
}

#[repr(C)]
pub union iproc_alg_s_alg {
    pub skcipher: skcipher_alg,
    pub hash: ahash_alg,
    pub aead: aead_alg,
}

#[repr(C)]
pub struct spu_msg_buf {
    pub bcm_spu_req_hdr: [u8; align(SPU2_HEADER_ALLOC_LEN as usize, SPU_MSG_ALIGN)],
    pub iv_ctr: [u8; align(2 * AES_BLOCK_SIZE as usize, SPU_MSG_ALIGN)],
    pub digest: [u8; align(MAX_DIGEST_SIZE, SPU_MSG_ALIGN)],
    pub spu_req_pad: [u8; align(SPU_PAD_LEN_MAX as usize, SPU_MSG_ALIGN)],
    pub tx_stat: [u8; align(SPU_TX_STATUS_LEN as usize, SPU_MSG_ALIGN)],
    pub spu_resp_hdr: [u8; align(SPU2_HEADER_ALLOC_LEN as usize, SPU_MSG_ALIGN)],
    pub rx_stat_pad: [u8; align(SPU_STAT_PAD_MAX as usize, SPU_MSG_ALIGN)],
    pub rx_stat: [u8; align(SPU_RX_STATUS_LEN as usize, SPU_MSG_ALIGN)],
    pub extra: spu_msg_buf_extra,
}

#[repr(C)]
pub union spu_msg_buf_extra { pub c: spu_msg_buf_c, pub a: spu_msg_buf_a }

#[repr(C)]
pub struct spu_msg_buf_c { pub supdt_tweak: [u8; align(SPU_SUPDT_LEN as usize, SPU_MSG_ALIGN)] }

#[repr(C)]
pub struct spu_msg_buf_a {
    pub gcmpad: [u8; align(AES_BLOCK_SIZE as usize, SPU_MSG_ALIGN)],
    pub req_aad_pad: [u8; align(SPU_PAD_LEN_MAX as usize, SPU_MSG_ALIGN)],
    pub resp_aad: [u8; align(MAX_ASSOC_SIZE + MAX_IV_SIZE, SPU_MSG_ALIGN)],
}

#[repr(C)]
pub struct iproc_ctx_s {
    pub enckey: [u8; MAX_KEY_SIZE + ARC4_STATE_SIZE], pub enckeylen: c_uint,
    pub authkey: [u8; MAX_KEY_SIZE + ARC4_STATE_SIZE], pub authkeylen: c_uint,
    pub salt: [u8; MAX_SALT_SIZE], pub salt_len: c_uint, pub salt_offset: c_uint,
    pub iv: [u8; MAX_IV_SIZE], pub digestsize: c_uint, pub alg: *mut iproc_alg_s,
    pub is_esp: bool, pub cipher: cipher_op, pub cipher_type: spu_cipher_type,
    pub auth: auth_op, pub auth_first: bool, pub max_payload: c_uint,
    pub fallback_cipher: *mut crypto_aead, pub ipad: [u8; MAX_HASH_BLOCK_SIZE],
    pub opad: [u8; MAX_HASH_BLOCK_SIZE],
    pub bcm_spu_req_hdr: [u8; align(SPU2_HEADER_ALLOC_LEN as usize, SPU_MSG_ALIGN)],
    pub spu_req_hdr_len: u16, pub spu_resp_hdr_len: u16, pub shash: *mut shash_desc,
    pub is_rfc4543: bool,
}

#[repr(C)]
pub struct spu_hash_export_s {
    pub total_todo: c_uint, pub total_sent: c_uint,
    pub hash_carry: [u8; HASH_CARRY_MAX], pub hash_carry_len: c_uint,
    pub incr_hash: [u8; MAX_DIGEST_SIZE], pub is_sw_hmac: bool,
}

#[repr(C)]
pub struct iproc_reqctx_s {
    pub parent: *mut crypto_async_request, pub ctx: *mut iproc_ctx_s, pub chan_idx: u8,
    pub total_todo: c_uint, pub total_received: c_uint, pub total_sent: c_uint,
    pub src_sent: c_uint, pub assoc: *mut scatterlist, pub src_sg: *mut scatterlist,
    pub src_nents: c_int, pub src_skip: u32, pub dst_sg: *mut scatterlist,
    pub dst_nents: c_int, pub dst_skip: u32, pub mb_mssg: brcm_message, pub bd_suppress: bool,
    pub is_encrypt: bool, pub iv_ctr: *mut u8, pub iv_ctr_len: c_uint,
    pub hash_carry: [u8; HASH_CARRY_MAX], pub hash_carry_len: c_uint,
    pub is_final: c_uint, pub incr_hash: [u8; MAX_DIGEST_SIZE], pub is_sw_hmac: bool,
    pub gfp: gfp_t, pub msg_buf: spu_msg_buf, pub req: aead_request,
}

#[repr(C)]
pub struct spu_hw {
    pub spu_dump_msg_hdr: Option<unsafe extern "C" fn(*mut u8, c_uint)>,
    pub spu_ctx_max_payload: Option<unsafe extern "C" fn(spu_cipher_alg, spu_cipher_mode, c_uint) -> u32>,
    pub spu_payload_length: Option<unsafe extern "C" fn(*mut u8) -> u32>,
    pub spu_response_hdr_len: Option<unsafe extern "C" fn(u16, u16, bool) -> u16>,
    pub spu_hash_pad_len: Option<unsafe extern "C" fn(hash_alg, hash_mode, u32, u16) -> u16>,
    pub spu_gcm_ccm_pad_len: Option<unsafe extern "C" fn(spu_cipher_mode, c_uint) -> u32>,
    pub spu_assoc_resp_len: Option<unsafe extern "C" fn(spu_cipher_mode, c_uint, c_uint, bool) -> u32>,
    pub spu_aead_ivlen: Option<unsafe extern "C" fn(spu_cipher_mode, u16) -> u8>,
    pub spu_hash_type: Option<unsafe extern "C" fn(u32) -> hash_type>,
    pub spu_digest_size: Option<unsafe extern "C" fn(u32, hash_alg, hash_type) -> u32>,
    pub spu_create_request: Option<unsafe extern "C" fn(*mut u8, *mut spu_request_opts, *mut spu_cipher_parms, *mut spu_hash_parms, *mut spu_aead_parms, c_uint) -> u32>,
    pub spu_cipher_req_init: Option<unsafe extern "C" fn(*mut u8, *mut spu_cipher_parms) -> u16>,
    pub spu_cipher_req_finish: Option<unsafe extern "C" fn(*mut u8, u16, c_uint, *mut spu_cipher_parms, c_uint)>,
    pub spu_request_pad: Option<unsafe extern "C" fn(*mut u8, u32, u32, hash_alg, hash_mode, c_uint, u32)>,
    pub spu_xts_tweak_in_payload: Option<unsafe extern "C" fn() -> u8>,
    pub spu_tx_status_len: Option<unsafe extern "C" fn() -> u8>,
    pub spu_rx_status_len: Option<unsafe extern "C" fn() -> u8>,
    pub spu_status_process: Option<unsafe extern "C" fn(*mut u8) -> c_int>,
    pub spu_ccm_update_iv: Option<unsafe extern "C" fn(c_uint, *mut spu_cipher_parms, c_uint, c_uint, bool, bool)>,
    pub spu_wordalign_padlen: Option<unsafe extern "C" fn(u32) -> u32>,
    pub reg_vbase: [*mut c_void; MAX_SPUS], pub spu_type: spu_spu_type,
    pub spu_subtype: spu_spu_subtype, pub num_spu: u32, pub num_chan: u32,
}

#[repr(C)]
pub struct bcm_device_private {
    pub pdev: *mut platform_device, pub spu: spu_hw, pub session_count: atomic_t,
    pub stream_count: atomic_t, pub bcm_hdr_len: u8, pub next_chan: atomic_t,
    pub debugfs_dir: *mut dentry, pub debugfs_stats: *mut dentry,
    pub bytes_in: atomic64_t, pub bytes_out: atomic64_t,
    pub op_counts: [atomic_t; SPU_OP_NUM as usize],
    pub cipher_cnt: [[atomic_t; CIPHER_MODE_LAST as usize]; CIPHER_ALG_LAST as usize],
    pub hash_cnt: [atomic_t; HASH_ALG_LAST as usize], pub hmac_cnt: [atomic_t; HASH_ALG_LAST as usize],
    pub aead_cnt: [atomic_t; AEAD_TYPE_LAST as usize], pub setkey_cnt: [atomic_t; SPU_OP_NUM as usize],
    pub mb_no_spc: atomic_t, pub mb_send_fail: atomic_t, pub bad_icv: atomic_t,
    pub mcl: mbox_client, pub mbox: *mut *mut mbox_chan,
}

extern "C" {
    pub static mut iproc_priv: bcm_device_private;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
