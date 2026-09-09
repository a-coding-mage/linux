/* SPDX-License-Identifier: GPL-2.0 */
/* K3 SA2UL crypto accelerator driver header translation. */

// Dependencies supplied by the surrounding kernel/crypto translation are intentionally
// referenced by name and are not implemented here.

pub const SA_ENGINE_STATUS: u32 = 0x0008;
pub const SA_ENGINE_ENABLE_CONTROL: u32 = 0x1000;

pub const SA_EEC_ENCSS_EN: u32 = 0x00000001;
pub const SA_EEC_AUTHSS_EN: u32 = 0x00000002;
pub const SA_EEC_TRNG_EN: u32 = 0x00000008;
pub const SA_EEC_PKA_EN: u32 = 0x00000010;
pub const SA_EEC_CTXCACH_EN: u32 = 0x00000080;
pub const SA_EEC_CPPI_PORT_IN_EN: u32 = 0x00000200;
pub const SA_EEC_CPPI_PORT_OUT_EN: u32 = 0x00000800;

pub const SA_REQ_SUBTYPE_ENC: u32 = 0x0001;
pub const SA_REQ_SUBTYPE_DEC: u32 = 0x0002;
pub const SA_REQ_SUBTYPE_SHIFT: u32 = 16;
pub const SA_REQ_SUBTYPE_MASK: u32 = 0xffff;

pub const SA_DMA_NUM_EPIB_WORDS: usize = 4;
pub const SA_DMA_NUM_PS_WORDS: usize = 16;
pub const NKEY_SZ: usize = 3;
pub const MCI_SZ: usize = 27;
pub const SA_MAX_NUM_CTX: usize = 512;

#[inline]
pub const fn SA_CTX_SIZE_TO_DMA_SIZE(ctx_sz: usize) -> usize {
    if ctx_sz != 0 { ctx_sz / 32 - 1 } else { 0 }
}

pub const SA_CTX_ENC_KEY_OFFSET: usize = 32;
pub const SA_CTX_ENC_AUX1_OFFSET: usize = 64;
pub const SA_CTX_ENC_AUX2_OFFSET: usize = 96;
pub const SA_CTX_ENC_AUX3_OFFSET: usize = 112;
pub const SA_CTX_ENC_AUX4_OFFSET: usize = 128;

pub const SA_ENG_ID_EM1: u32 = 2;
pub const SA_ENG_ID_EM2: u32 = 3;
pub const SA_ENG_ID_AM1: u32 = 4;
pub const SA_ENG_ID_AM2: u32 = 5;
pub const SA_ENG_ID_OUTPORT2: u32 = 20;

pub const SA_CMDL_OFFSET_NESC: usize = 0;
pub const SA_CMDL_OFFSET_LABEL_LEN: usize = 1;
pub const SA_CMDL_OFFSET_DATA_LEN: usize = 2;
pub const SA_CMDL_OFFSET_DATA_OFFSET: usize = 4;
pub const SA_CMDL_OFFSET_OPTION_CTRL1: usize = 5;
pub const SA_CMDL_OFFSET_OPTION_CTRL2: usize = 6;
pub const SA_CMDL_OFFSET_OPTION_CTRL3: usize = 7;
pub const SA_CMDL_OFFSET_OPTION_BYTE: usize = 8;
pub const SA_CMDL_HEADER_SIZE_BYTES: usize = 8;
pub const SA_CMDL_OPTION_BYTES_MAX_SIZE: usize = 72;
pub const SA_CMDL_MAX_SIZE_BYTES: usize = SA_CMDL_HEADER_SIZE_BYTES + SA_CMDL_OPTION_BYTES_MAX_SIZE;

pub const SA_SW_INFO_FLAG_EVICT: u32 = 0x0001;
pub const SA_SW_INFO_FLAG_TEAR: u32 = 0x0002;
pub const SA_SW_INFO_FLAG_NOPD: u32 = 0x0004;

pub const SA_CTX_PE_PKT_TYPE_3GPP_AIR: u32 = 0;
pub const SA_CTX_PE_PKT_TYPE_SRTP: u32 = 1;
pub const SA_CTX_PE_PKT_TYPE_IPSEC_AH: u32 = 2;
pub const SA_CTX_PE_PKT_TYPE_IPSEC_ESP: u32 = 3;
pub const SA_CTX_PE_PKT_TYPE_NONE: u32 = 4;
pub const SA_CTX_ENC_TYPE1_SZ: usize = 64;
pub const SA_CTX_ENC_TYPE2_SZ: usize = 96;
pub const SA_CTX_AUTH_TYPE1_SZ: usize = 64;
pub const SA_CTX_AUTH_TYPE2_SZ: usize = 96;
pub const SA_CTX_PHP_PE_CTX_SZ: usize = 64;
pub const SA_CTX_MAX_SZ: usize = 64 + SA_CTX_ENC_TYPE2_SZ + SA_CTX_AUTH_TYPE2_SZ;

pub const SA_CTX_DMA_SIZE_0: u32 = 0;
pub const SA_CTX_DMA_SIZE_64: u32 = 1;
pub const SA_CTX_DMA_SIZE_96: u32 = 2;
pub const SA_CTX_DMA_SIZE_128: u32 = 3;
pub const SA_CTX_SCCTL_OWNER_OFFSET: usize = 0;
pub const SA_SCCTL_FE_AUTH_ENC: u32 = 0x65;
pub const SA_SCCTL_FE_ENC: u32 = 0x8D;
pub const SA_ALIGN_MASK: usize = core::mem::size_of::<u32>() - 1;
// C __aligned(32) is a declaration attribute and remains an integration requirement.
pub const SA_AUTH_SW_CTRL_MD5: u32 = 1;
pub const SA_AUTH_SW_CTRL_SHA1: u32 = 2;
pub const SA_AUTH_SW_CTRL_SHA224: u32 = 3;
pub const SA_AUTH_SW_CTRL_SHA256: u32 = 4;
pub const SA_AUTH_SW_CTRL_SHA384: u32 = 5;
pub const SA_AUTH_SW_CTRL_SHA512: u32 = 6;
pub const SA_MAX_DATA_SZ: u32 = u16::MAX as u32;
pub const SA_UNSAFE_DATA_SZ_MIN: u32 = 240;
pub const SA_UNSAFE_DATA_SZ_MAX: u32 = 255;

#[repr(C)]
pub struct sa_crypto_data {
    pub base: *mut core::ffi::c_void,
    pub match_data: *const sa_match_data,
    pub pdev: *mut platform_device,
    pub sc_pool: *mut dma_pool,
    pub dev: *mut device,
    pub scid_lock: spinlock_t,
    pub sc_id_start: u16,
    pub sc_id_end: u16,
    pub sc_id: u16,
    pub ctx_bm: [core::ffi::c_ulong; (SA_MAX_NUM_CTX + (core::mem::size_of::<core::ffi::c_ulong>() * 8) - 1) / (core::mem::size_of::<core::ffi::c_ulong>() * 8)],
    pub ctx: *mut sa_tfm_ctx,
    pub dma_rx1: *mut dma_chan,
    pub dma_rx2: *mut dma_chan,
    pub dma_tx: *mut dma_chan,
}

#[repr(C)]
pub struct sa_cmdl_param_info { pub index: u16, pub offset: u16, pub size: u16 }
pub const SA_MAX_AUX_DATA_WORDS: usize = 8;

#[repr(C)]
pub struct sa_cmdl_upd_info {
    pub flags: u16, pub submode: u16,
    pub enc_size: sa_cmdl_param_info, pub enc_size2: sa_cmdl_param_info,
    pub enc_offset: sa_cmdl_param_info, pub enc_iv: sa_cmdl_param_info,
    pub enc_iv2: sa_cmdl_param_info, pub aad: sa_cmdl_param_info,
    pub payload: sa_cmdl_param_info, pub auth_size: sa_cmdl_param_info,
    pub auth_size2: sa_cmdl_param_info, pub auth_offset: sa_cmdl_param_info,
    pub auth_iv: sa_cmdl_param_info, pub aux_key_info: sa_cmdl_param_info,
    pub aux_key: [u32; SA_MAX_AUX_DATA_WORDS],
}

pub const SA_PSDATA_CTX_WORDS: usize = 4;
pub const SA_MAX_CMDL_WORDS: usize = SA_DMA_NUM_PS_WORDS - SA_PSDATA_CTX_WORDS;

#[repr(C)]
pub struct sa_ctx_info {
    pub sc: *mut u8, pub sc_phys: dma_addr_t, pub sc_id: u16, pub cmdl_size: u16,
    pub cmdl: [u32; SA_MAX_CMDL_WORDS], pub cmdl_upd_info: sa_cmdl_upd_info,
    pub epib: [u32; SA_DMA_NUM_EPIB_WORDS],
}

#[repr(C)]
pub union sa_tfm_ctx_fallback {
    pub skcipher: *mut crypto_skcipher,
    pub ahash: *mut crypto_ahash,
    pub aead: *mut crypto_aead,
}

#[repr(C)]
pub struct sa_tfm_ctx {
    pub dev_data: *mut sa_crypto_data, pub enc: sa_ctx_info, pub dec: sa_ctx_info,
    pub auth: sa_ctx_info, pub keylen: core::ffi::c_int, pub iv_idx: core::ffi::c_int,
    pub key: [u32; 256 / core::mem::size_of::<u32>()], pub authkey: [u8; SHA512_BLOCK_SIZE],
    pub shash: *mut crypto_shash, pub fallback: sa_tfm_ctx_fallback,
}

#[repr(C)]
pub struct sa_sha_req_ctx {
    pub dev_data: *mut sa_crypto_data,
    pub cmdl: [u32; SA_MAX_CMDL_WORDS + SA_PSDATA_CTX_WORDS],
    pub fallback_req: ahash_request,
}

#[repr(C)]
pub enum sa_submode { SA_MODE_GEN = 0, SA_MODE_CCM, SA_MODE_GCM, SA_MODE_GMAC }

#[repr(C)]
pub enum sa_ealg_id {
    SA_EALG_ID_NONE = 0, SA_EALG_ID_NULL, SA_EALG_ID_AES_CTR, SA_EALG_ID_AES_F8,
    SA_EALG_ID_AES_CBC, SA_EALG_ID_DES_CBC, SA_EALG_ID_3DES_CBC, SA_EALG_ID_CCM,
    SA_EALG_ID_GCM, SA_EALG_ID_AES_ECB, SA_EALG_ID_LAST,
}

#[repr(C)]
pub enum sa_aalg_id {
    SA_AALG_ID_NONE = 0, SA_AALG_ID_NULL = SA_EALG_ID_LAST as isize, SA_AALG_ID_MD5,
    SA_AALG_ID_SHA1, SA_AALG_ID_SHA2_224, SA_AALG_ID_SHA2_256, SA_AALG_ID_SHA2_512,
    SA_AALG_ID_HMAC_MD5, SA_AALG_ID_HMAC_SHA1, SA_AALG_ID_HMAC_SHA2_224,
    SA_AALG_ID_HMAC_SHA2_256, SA_AALG_ID_GMAC, SA_AALG_ID_CMAC, SA_AALG_ID_CBC_MAC,
    SA_AALG_ID_AES_XCBC,
}

#[repr(C)]
pub enum sa_eng_algo_id {
    SA_ENG_ALGO_ECB = 0, SA_ENG_ALGO_CBC, SA_ENG_ALGO_CFB, SA_ENG_ALGO_OFB,
    SA_ENG_ALGO_CTR, SA_ENG_ALGO_F8, SA_ENG_ALGO_F8F9, SA_ENG_ALGO_GCM,
    SA_ENG_ALGO_GMAC, SA_ENG_ALGO_CCM, SA_ENG_ALGO_CMAC, SA_ENG_ALGO_CBCMAC,
    SA_NUM_ENG_ALGOS,
}

#[repr(C)]
pub struct sa_eng_info { pub eng_id: u8, pub sc_size: u16 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
