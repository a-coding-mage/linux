/* SPDX-License-Identifier: GPL-2.0-only */
/* AMD Cryptographic Coprocessor (CCP) driver header translation. */

// Dependencies supplied by the surrounding kernel translation:
// linux/scatterlist.h, linux/workqueue.h, linux/list.h, crypto/aes.h,
// crypto/sha1.h, and crypto/sha2.h.

pub struct ccp_device;
pub struct ccp_cmd;

// CONFIG_CRYPTO_DEV_SP_CCP controls whether these functions are externally provided.
#[cfg(feature = "CONFIG_CRYPTO_DEV_SP_CCP")]
extern "C" {
    pub fn ccp_present() -> ::core::ffi::c_int;
    pub fn ccp_version() -> u32;
    pub fn ccp_enqueue_cmd(cmd: *mut ccp_cmd) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_SP_CCP"))]
#[inline]
pub unsafe fn ccp_present() -> ::core::ffi::c_int { -19 }
#[cfg(not(feature = "CONFIG_CRYPTO_DEV_SP_CCP"))]
#[inline]
pub unsafe fn ccp_version() -> u32 { 0 }
#[cfg(not(feature = "CONFIG_CRYPTO_DEV_SP_CCP"))]
#[inline]
pub unsafe fn ccp_enqueue_cmd(_cmd: *mut ccp_cmd) -> ::core::ffi::c_int { -19 }

pub const CCP_VSIZE: u32 = 16;
pub const CCP_VMASK: u32 = (1u32 << CCP_VSIZE) - 1;
#[inline]
pub const fn CCP_VERSION(v: u32, r: u32) -> u32 { (v << CCP_VSIZE) | (r & CCP_VMASK) }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ccp_aes_type { CCP_AES_TYPE_128 = 0, CCP_AES_TYPE_192, CCP_AES_TYPE_256, CCP_AES_TYPE__LAST }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum ccp_aes_mode { CCP_AES_MODE_ECB = 0, CCP_AES_MODE_CBC, CCP_AES_MODE_OFB, CCP_AES_MODE_CFB, CCP_AES_MODE_CTR, CCP_AES_MODE_CMAC, CCP_AES_MODE_GHASH, CCP_AES_MODE_GCTR, CCP_AES_MODE_GCM, CCP_AES_MODE_GMAC, CCP_AES_MODE__LAST }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum ccp_aes_action { CCP_AES_ACTION_DECRYPT = 0, CCP_AES_ACTION_ENCRYPT, CCP_AES_ACTION__LAST }
pub const CCP_AES_GHASHAAD: ccp_aes_action = ccp_aes_action::CCP_AES_ACTION_DECRYPT;
pub const CCP_AES_GHASHFINAL: ccp_aes_action = ccp_aes_action::CCP_AES_ACTION_ENCRYPT;

#[repr(C)]
pub struct ccp_aes_engine { pub type_: ccp_aes_type, pub mode: ccp_aes_mode, pub action: ccp_aes_action, pub authsize: u32, pub key: *mut scatterlist, pub key_len: u32, pub iv: *mut scatterlist, pub iv_len: u32, pub src: *mut scatterlist, pub dst: *mut scatterlist, pub src_len: u64, pub cmac_final: u32, pub cmac_key: *mut scatterlist, pub cmac_key_len: u32, pub aad_len: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ccp_xts_aes_unit_size { CCP_XTS_AES_UNIT_SIZE_16 = 0, CCP_XTS_AES_UNIT_SIZE_512, CCP_XTS_AES_UNIT_SIZE_1024, CCP_XTS_AES_UNIT_SIZE_2048, CCP_XTS_AES_UNIT_SIZE_4096, CCP_XTS_AES_UNIT_SIZE__LAST }
#[repr(C)]
pub struct ccp_xts_aes_engine { pub type_: ccp_aes_type, pub action: ccp_aes_action, pub unit_size: ccp_xts_aes_unit_size, pub key: *mut scatterlist, pub key_len: u32, pub iv: *mut scatterlist, pub iv_len: u32, pub src: *mut scatterlist, pub dst: *mut scatterlist, pub src_len: u64, pub final_: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ccp_sha_type { CCP_SHA_TYPE_1 = 1, CCP_SHA_TYPE_224, CCP_SHA_TYPE_256, CCP_SHA_TYPE_384, CCP_SHA_TYPE_512, CCP_SHA_TYPE__LAST }
#[repr(C)]
pub struct ccp_sha_engine { pub type_: ccp_sha_type, pub ctx: *mut scatterlist, pub ctx_len: u32, pub src: *mut scatterlist, pub src_len: u64, pub opad: *mut scatterlist, pub opad_len: u32, pub first: u32, pub final_: u32, pub msg_bits: u64 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ccp_des3_mode { CCP_DES3_MODE_ECB = 0, CCP_DES3_MODE_CBC, CCP_DES3_MODE_CFB, CCP_DES3_MODE__LAST }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum ccp_des3_type { CCP_DES3_TYPE_168 = 1, CCP_DES3_TYPE__LAST }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum ccp_des3_action { CCP_DES3_ACTION_DECRYPT = 0, CCP_DES3_ACTION_ENCRYPT, CCP_DES3_ACTION__LAST }
#[repr(C)]
pub struct ccp_des3_engine { pub type_: ccp_des3_type, pub mode: ccp_des3_mode, pub action: ccp_des3_action, pub key: *mut scatterlist, pub key_len: u32, pub iv: *mut scatterlist, pub iv_len: u32, pub src: *mut scatterlist, pub dst: *mut scatterlist, pub src_len: u64 }

#[repr(C)]
pub struct ccp_rsa_engine { pub key_size: u32, pub exp: *mut scatterlist, pub exp_len: u32, pub mod_: *mut scatterlist, pub mod_len: u32, pub src: *mut scatterlist, pub dst: *mut scatterlist, pub src_len: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ccp_passthru_bitwise { CCP_PASSTHRU_BITWISE_NOOP = 0, CCP_PASSTHRU_BITWISE_AND, CCP_PASSTHRU_BITWISE_OR, CCP_PASSTHRU_BITWISE_XOR, CCP_PASSTHRU_BITWISE_MASK, CCP_PASSTHRU_BITWISE__LAST }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum ccp_passthru_byteswap { CCP_PASSTHRU_BYTESWAP_NOOP = 0, CCP_PASSTHRU_BYTESWAP_32BIT, CCP_PASSTHRU_BYTESWAP_256BIT, CCP_PASSTHRU_BYTESWAP__LAST }
#[repr(C)]
pub struct ccp_passthru_engine { pub bit_mod: ccp_passthru_bitwise, pub byte_swap: ccp_passthru_byteswap, pub mask: *mut scatterlist, pub mask_len: u32, pub src: *mut scatterlist, pub dst: *mut scatterlist, pub src_len: u64, pub final_: u32 }
#[repr(C)]
pub struct ccp_passthru_nomap_engine { pub bit_mod: ccp_passthru_bitwise, pub byte_swap: ccp_passthru_byteswap, pub mask: dma_addr_t, pub mask_len: u32, pub src_dma: dma_addr_t, pub dst_dma: dma_addr_t, pub src_len: u64, pub final_: u32 }

pub const CCP_ECC_MODULUS_BYTES: u32 = 48;
pub const CCP_ECC_MAX_OPERANDS: u32 = 6;
pub const CCP_ECC_MAX_OUTPUTS: u32 = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub enum ccp_ecc_function { CCP_ECC_FUNCTION_MMUL_384BIT = 0, CCP_ECC_FUNCTION_MADD_384BIT, CCP_ECC_FUNCTION_MINV_384BIT, CCP_ECC_FUNCTION_PADD_384BIT, CCP_ECC_FUNCTION_PMUL_384BIT, CCP_ECC_FUNCTION_PDBL_384BIT }
#[repr(C)]
pub struct ccp_ecc_modular_math { pub operand_1: *mut scatterlist, pub operand_1_len: u32, pub operand_2: *mut scatterlist, pub operand_2_len: u32, pub result: *mut scatterlist, pub result_len: u32 }
#[repr(C)]
pub struct ccp_ecc_point { pub x: *mut scatterlist, pub x_len: u32, pub y: *mut scatterlist, pub y_len: u32 }
#[repr(C)]
pub struct ccp_ecc_point_math { pub point_1: ccp_ecc_point, pub point_2: ccp_ecc_point, pub domain_a: *mut scatterlist, pub domain_a_len: u32, pub scalar: *mut scatterlist, pub scalar_len: u32, pub result: ccp_ecc_point }
#[repr(C)]
pub union ccp_ecc_engine_u { pub mm: ccp_ecc_modular_math, pub pm: ccp_ecc_point_math }
#[repr(C)]
pub struct ccp_ecc_engine { pub function: ccp_ecc_function, pub mod_: *mut scatterlist, pub mod_len: u32, pub u: ccp_ecc_engine_u, pub ecc_result: u16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ccp_engine { CCP_ENGINE_AES = 0, CCP_ENGINE_XTS_AES_128, CCP_ENGINE_DES3, CCP_ENGINE_SHA, CCP_ENGINE_RSA, CCP_ENGINE_PASSTHRU, CCP_ENGINE_ZLIB_DECOMPRESS, CCP_ENGINE_ECC, CCP_ENGINE__LAST }
pub const CCP_CMD_MAY_BACKLOG: u32 = 0x00000001;
pub const CCP_CMD_PASSTHRU_NO_DMA_MAP: u32 = 0x00000002;

#[repr(C)]
pub union ccp_cmd_u { pub aes: ccp_aes_engine, pub xts: ccp_xts_aes_engine, pub des3: ccp_des3_engine, pub sha: ccp_sha_engine, pub rsa: ccp_rsa_engine, pub passthru: ccp_passthru_engine, pub passthru_nomap: ccp_passthru_nomap_engine, pub ecc: ccp_ecc_engine }
#[repr(C)]
pub struct ccp_cmd { pub entry: list_head, pub work: work_struct, pub ccp: *mut ccp_device, pub ret: ::core::ffi::c_int, pub flags: u32, pub engine: ccp_engine, pub engine_error: u32, pub u: ccp_cmd_u, pub callback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int)>, pub data: *mut ::core::ffi::c_void }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
