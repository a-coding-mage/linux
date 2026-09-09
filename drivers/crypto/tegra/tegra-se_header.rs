/* SPDX-License-Identifier: GPL-2.0-only
 * SPDX-FileCopyrightText: Copyright (c) 2023 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
 *
 * Header file for NVIDIA Security Engine driver.
 *
 * C dependencies are supplied by the surrounding translation unit.
 */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

pub const fn field_prep(mask: u32, value: u32) -> u32 { (value << mask.trailing_zeros()) & mask }
pub const fn field_get(mask: u32, value: u32) -> u32 { (value & mask) >> mask.trailing_zeros() }

pub const SE_OWNERSHIP: u32 = 0x14;
pub const fn SE_OWNERSHIP_UID(x: u32) -> u32 { field_get(0xff, x) }
pub const TEGRA_GPSE_ID: u32 = 3;
pub const SE_STREAM_ID: u32 = 0x90;

pub const SE_SHA_CFG: u32 = 0x4004;
pub const SE_SHA_IN_ADDR: u32 = 0x400c;
pub const SE_SHA_KEY_ADDR: u32 = 0x4094;
pub const SE_SHA_KEY_DATA: u32 = 0x4098;
pub const SE_SHA_KEYMANIFEST: u32 = 0x409c;
pub const SE_SHA_CRYPTO_CFG: u32 = 0x40a4;
pub const SE_SHA_KEY_DST: u32 = 0x40a8;
pub const SE_SHA_SRC_KSLT: u32 = 0x4180;
pub const SE_SHA_TGT_KSLT: u32 = 0x4184;
pub const SE_SHA_MSG_LENGTH: u32 = 0x401c;
pub const SE_SHA_OPERATION: u32 = 0x407c;
pub const SE_SHA_HASH_RESULT: u32 = 0x40b0;

pub const fn SE_SHA_ENC_MODE(x: u32) -> u32 { field_prep(0xff000000, x) }
pub const SE_SHA_ENC_MODE_SHA1: u32 = SE_SHA_ENC_MODE(0);
pub const SE_SHA_ENC_MODE_SHA224: u32 = SE_SHA_ENC_MODE(4);
pub const SE_SHA_ENC_MODE_SHA256: u32 = SE_SHA_ENC_MODE(5);
pub const SE_SHA_ENC_MODE_SHA384: u32 = SE_SHA_ENC_MODE(6);
pub const SE_SHA_ENC_MODE_SHA512: u32 = SE_SHA_ENC_MODE(7);
pub const SE_SHA_ENC_MODE_SHA_CTX_INTEGRITY: u32 = SE_SHA_ENC_MODE(8);
pub const SE_SHA_ENC_MODE_SHA3_224: u32 = SE_SHA_ENC_MODE(9);
pub const SE_SHA_ENC_MODE_SHA3_256: u32 = SE_SHA_ENC_MODE(10);
pub const SE_SHA_ENC_MODE_SHA3_384: u32 = SE_SHA_ENC_MODE(11);
pub const SE_SHA_ENC_MODE_SHA3_512: u32 = SE_SHA_ENC_MODE(12);
pub const SE_SHA_ENC_MODE_SHAKE128: u32 = SE_SHA_ENC_MODE(13);
pub const SE_SHA_ENC_MODE_SHAKE256: u32 = SE_SHA_ENC_MODE(14);
pub const SE_SHA_ENC_MODE_HMAC_SHA256_1KEY: u32 = SE_SHA_ENC_MODE(0);
pub const SE_SHA_ENC_MODE_HMAC_SHA256_2KEY: u32 = SE_SHA_ENC_MODE(1);
pub const SE_SHA_ENC_MODE_SM3_256: u32 = SE_SHA_ENC_MODE(0);

pub const fn SE_SHA_CFG_ENC_ALG(x: u32) -> u32 { field_prep(0xf000, x) }
pub const SE_SHA_ENC_ALG_NOP: u32 = SE_SHA_CFG_ENC_ALG(0);
pub const SE_SHA_ENC_ALG_SHA_ENC: u32 = SE_SHA_CFG_ENC_ALG(1);
pub const SE_SHA_ENC_ALG_RNG: u32 = SE_SHA_CFG_ENC_ALG(2);
pub const SE_SHA_ENC_ALG_SHA: u32 = SE_SHA_CFG_ENC_ALG(3);
pub const SE_SHA_ENC_ALG_SM3: u32 = SE_SHA_CFG_ENC_ALG(4);
pub const SE_SHA_ENC_ALG_HMAC: u32 = SE_SHA_CFG_ENC_ALG(7);
pub const SE_SHA_ENC_ALG_KDF: u32 = SE_SHA_CFG_ENC_ALG(8);
pub const SE_SHA_ENC_ALG_KEY_INVLD: u32 = SE_SHA_CFG_ENC_ALG(10);
pub const SE_SHA_ENC_ALG_KEY_INQUIRE: u32 = SE_SHA_CFG_ENC_ALG(12);
pub const SE_SHA_ENC_ALG_INS: u32 = SE_SHA_CFG_ENC_ALG(13);
pub const SE_SHA_OP_LASTBUF: u32 = 1 << 16;
pub const SE_SHA_OP_WRSTALL: u32 = 1 << 15;
pub const fn SE_SHA_OP_OP(x: u32) -> u32 { x & 7 }
pub const SE_SHA_OP_START: u32 = SE_SHA_OP_OP(1);
pub const SE_SHA_OP_RESTART_OUT: u32 = SE_SHA_OP_OP(2);
pub const SE_SHA_OP_RESTART_IN: u32 = SE_SHA_OP_OP(4);
pub const SE_SHA_OP_RESTART_INOUT: u32 = SE_SHA_OP_OP(5);
pub const SE_SHA_OP_DUMMY: u32 = SE_SHA_OP_OP(6);
pub const fn SE_SHA_CFG_DEC_ALG(x: u32) -> u32 { field_prep(0xf00, x) }
pub const SE_SHA_DEC_ALG_NOP: u32 = SE_SHA_CFG_DEC_ALG(0);
pub const SE_SHA_DEC_ALG_AES_DEC: u32 = SE_SHA_CFG_DEC_ALG(1);
pub const SE_SHA_DEC_ALG_HMAC: u32 = SE_SHA_CFG_DEC_ALG(7);
pub const SE_SHA_DEC_ALG_HMAC_VERIFY: u32 = SE_SHA_CFG_DEC_ALG(9);
pub const fn SE_SHA_CFG_DST(x: u32) -> u32 { field_prep(0x1c, x) }
pub const SE_SHA_DST_MEMORY: u32 = SE_SHA_CFG_DST(0);
pub const SE_SHA_DST_HASH_REG: u32 = SE_SHA_CFG_DST(1);
pub const SE_SHA_DST_KEYTABLE: u32 = SE_SHA_CFG_DST(2);
pub const SE_SHA_DST_SRK: u32 = SE_SHA_CFG_DST(3);
pub const SE_SHA_TASK_HASH_INIT: u32 = 1;

/* Remaining register definitions and declarations retain the C header's external dependency types. */
pub const SE_AES0_CFG: u32 = 0x1004;
pub const SE_AES0_CRYPTO_CONFIG: u32 = 0x1008;
pub const SE_AES0_KEY_DST: u32 = 0x1030;
pub const SE_AES0_OPERATION: u32 = 0x1038;
pub const SE_AES0_LINEAR_CTR: u32 = 0x101c;
pub const SE_AES0_LAST_BLOCK: u32 = 0x102c;
pub const SE_AES0_KEY_ADDR: u32 = 0x10bc;
pub const SE_AES0_KEY_DATA: u32 = 0x10c0;
pub const SE_AES0_CMAC_RESULT: u32 = 0x10c4;
pub const SE_AES0_SRC_KSLT: u32 = 0x1100;
pub const SE_AES0_TGT_KSLT: u32 = 0x1104;
pub const SE_AES0_KEYMANIFEST: u32 = 0x1114;
pub const SE_AES0_AAD_LEN: u32 = 0x112c;
pub const SE_AES0_CRYPTO_MSG_LEN: u32 = 0x1134;
pub const SE_AES1_CFG: u32 = 0x2004;
pub const SE_AES1_CRYPTO_CONFIG: u32 = 0x2008;
pub const SE_AES1_KEY_DST: u32 = 0x2030;
pub const SE_AES1_OPERATION: u32 = 0x2038;
pub const SE_AES1_LINEAR_CTR: u32 = 0x201c;
pub const SE_AES1_LAST_BLOCK: u32 = 0x202c;
pub const SE_AES1_KEY_ADDR: u32 = 0x20bc;
pub const SE_AES1_KEY_DATA: u32 = 0x20c0;
pub const SE_AES1_CMAC_RESULT: u32 = 0x20c4;
pub const SE_AES1_SRC_KSLT: u32 = 0x2100;
pub const SE_AES1_TGT_KSLT: u32 = 0x2104;
pub const SE_AES1_KEYMANIFEST: u32 = 0x2114;
pub const SE_AES1_AAD_LEN: u32 = 0x212c;
pub const SE_AES1_CRYPTO_MSG_LEN: u32 = 0x2134;

pub const fn SE_AES_CFG_ENC_MODE(x: u32) -> u32 { field_prep(0xff000000, x) }
pub const SE_AES_ENC_MODE_GMAC: u32 = SE_AES_CFG_ENC_MODE(3);
pub const SE_AES_ENC_MODE_GCM: u32 = SE_AES_CFG_ENC_MODE(4);
pub const SE_AES_ENC_MODE_GCM_FINAL: u32 = SE_AES_CFG_ENC_MODE(5);
pub const SE_AES_ENC_MODE_CMAC: u32 = SE_AES_CFG_ENC_MODE(7);
pub const SE_AES_ENC_MODE_CBC_MAC: u32 = SE_AES_CFG_ENC_MODE(12);
pub const fn SE_AES_CFG_DEC_MODE(x: u32) -> u32 { field_prep(0xff0000, x) }
pub const SE_AES_DEC_MODE_GMAC: u32 = SE_AES_CFG_DEC_MODE(3);
pub const SE_AES_DEC_MODE_GCM: u32 = SE_AES_CFG_DEC_MODE(4);
pub const SE_AES_DEC_MODE_GCM_FINAL: u32 = SE_AES_CFG_DEC_MODE(5);
pub const SE_AES_DEC_MODE_CBC_MAC: u32 = SE_AES_CFG_DEC_MODE(12);
pub const fn SE_AES_CFG_ENC_ALG(x: u32) -> u32 { field_prep(0xf000, x) }
pub const SE_AES_ENC_ALG_NOP: u32 = SE_AES_CFG_ENC_ALG(0);
pub const SE_AES_ENC_ALG_AES_ENC: u32 = SE_AES_CFG_ENC_ALG(1);
pub const SE_AES_ENC_ALG_RNG: u32 = SE_AES_CFG_ENC_ALG(2);
pub const SE_AES_ENC_ALG_SHA: u32 = SE_AES_CFG_ENC_ALG(3);
pub const SE_AES_ENC_ALG_HMAC: u32 = SE_AES_CFG_ENC_ALG(7);
pub const SE_AES_ENC_ALG_KDF: u32 = SE_AES_CFG_ENC_ALG(8);
pub const SE_AES_ENC_ALG_INS: u32 = SE_AES_CFG_ENC_ALG(13);
pub const fn SE_AES_CFG_DEC_ALG(x: u32) -> u32 { field_prep(0xf00, x) }
pub const SE_AES_DEC_ALG_NOP: u32 = SE_AES_CFG_DEC_ALG(0);
pub const SE_AES_DEC_ALG_AES_DEC: u32 = SE_AES_CFG_DEC_ALG(1);
pub const fn SE_AES_CFG_DST(x: u32) -> u32 { field_prep(0x1c, x) }
pub const SE_AES_DST_MEMORY: u32 = SE_AES_CFG_DST(0);
pub const SE_AES_DST_HASH_REG: u32 = SE_AES_CFG_DST(1);
pub const SE_AES_DST_KEYTABLE: u32 = SE_AES_CFG_DST(2);
pub const SE_AES_DST_SRK: u32 = SE_AES_CFG_DST(3);

pub const fn SE_AES_KEY2_INDEX(x: u32) -> u32 { field_prep(0xf0000000, x) }
pub const fn SE_AES_KEY_INDEX(x: u32) -> u32 { field_prep(0x0f000000, x) }
pub const SE_AES_CRYPTO_CFG_SCC_DIS: u32 = 1 << 20;
pub const fn SE_AES_CRYPTO_CFG_CTR_CNTN(x: u32) -> u32 { field_prep(0x7f800, x) }
pub const fn SE_AES_CRYPTO_CFG_IV_MODE(x: u32) -> u32 { field_prep(1 << 10, x) }
pub const SE_AES_IV_MODE_SWIV: u32 = 0;
pub const SE_AES_IV_MODE_HWIV: u32 = 1 << 10;
pub const fn SE_AES_CRYPTO_CFG_CORE_SEL(x: u32) -> u32 { field_prep(1 << 9, x) }
pub const SE_AES_CORE_SEL_DECRYPT: u32 = 0;
pub const SE_AES_CORE_SEL_ENCRYPT: u32 = 1 << 9;
pub const fn SE_AES_CRYPTO_CFG_IV_SEL(x: u32) -> u32 { field_prep(0x180, x) }
pub const SE_AES_IV_SEL_UPDATED: u32 = 0x80;
pub const SE_AES_IV_SEL_REG: u32 = 0x100;
pub const SE_AES_IV_SEL_RANDOM: u32 = 0x180;
pub const fn SE_AES_CRYPTO_CFG_VCTRAM_SEL(x: u32) -> u32 { field_prep(0x60, x) }
pub const SE_AES_VCTRAM_SEL_MEMORY: u32 = 0;
pub const SE_AES_VCTRAM_SEL_TWEAK: u32 = 0x20;
pub const SE_AES_VCTRAM_SEL_AESOUT: u32 = 0x40;
pub const SE_AES_VCTRAM_SEL_PREV_MEM: u32 = 0x60;
pub const fn SE_AES_CRYPTO_CFG_INPUT_SEL(x: u32) -> u32 { field_prep(0x18, x) }
pub const SE_AES_INPUT_SEL_MEMORY: u32 = 0;
pub const SE_AES_INPUT_SEL_RANDOM: u32 = 8;
pub const SE_AES_INPUT_SEL_AESOUT: u32 = 0x10;
pub const SE_AES_INPUT_SEL_LINEAR_CTR: u32 = 0x18;
pub const SE_AES_INPUT_SEL_REG: u32 = 8;
pub const fn SE_AES_CRYPTO_CFG_XOR_POS(x: u32) -> u32 { field_prep(6, x) }
pub const SE_AES_XOR_POS_BYPASS: u32 = 0;
pub const SE_AES_XOR_POS_BOTH: u32 = 2;
pub const SE_AES_XOR_POS_TOP: u32 = 4;
pub const SE_AES_XOR_POS_BOTTOM: u32 = 6;
pub const fn SE_AES_CRYPTO_CFG_HASH_EN(x: u32) -> u32 { field_prep(1, x) }
pub const SE_AES_HASH_DISABLE: u32 = 0;
pub const SE_AES_HASH_ENABLE: u32 = 1;
pub const fn SE_LAST_BLOCK_VAL(x: u32) -> u32 { field_prep(0xfffff, x) }
pub const fn SE_LAST_BLOCK_RES_BITS(x: u32) -> u32 { field_prep(0x7f00000, x) }
pub const SE_AES_OP_LASTBUF: u32 = 1 << 16;
pub const SE_AES_OP_WRSTALL: u32 = 1 << 15;
pub const SE_AES_OP_FINAL: u32 = 1 << 5;
pub const SE_AES_OP_INIT: u32 = 1 << 4;
pub const fn SE_AES_OP_OP(x: u32) -> u32 { x & 7 }
pub const SE_AES_OP_START: u32 = 1;
pub const SE_AES_OP_RESTART_OUT: u32 = 2;
pub const SE_AES_OP_RESTART_IN: u32 = 4;
pub const SE_AES_OP_RESTART_INOUT: u32 = 5;
pub const SE_AES_OP_DUMMY: u32 = 6;
pub const fn SE_KAC_SIZE(x: u32) -> u32 { field_prep(0xc000, x) }
pub const SE_KAC_SIZE_128: u32 = 0;
pub const SE_KAC_SIZE_192: u32 = 0x4000;
pub const SE_KAC_SIZE_256: u32 = 0x8000;
pub const SE_KAC_EXPORTABLE: u32 = 1 << 12;
pub const fn SE_KAC_PURPOSE(x: u32) -> u32 { field_prep(0xf00, x) }
pub const SE_KAC_ENC: u32 = 0;
pub const SE_KAC_CMAC: u32 = 0x100;
pub const SE_KAC_HMAC: u32 = 0x200;
pub const SE_KAC_GCM_KW: u32 = 0x300;
pub const SE_KAC_HMAC_KDK: u32 = 0x600;
pub const SE_KAC_HMAC_KDD: u32 = 0x700;
pub const SE_KAC_HMAC_KDD_KUW: u32 = 0x800;
pub const SE_KAC_XTS: u32 = 0x900;
pub const SE_KAC_GCM: u32 = 0xa00;
pub const SE_KAC_USER_NS: u32 = 3 << 4;
pub const fn SE_AES_KEY_DST_INDEX(x: u32) -> u32 { field_prep(0xf00, x) }
pub const fn SE_ADDR_HI_MSB(x: u32) -> u32 { field_prep(0xff000000, x) }
pub const fn SE_ADDR_HI_SZ(x: u32) -> u32 { field_prep(0xffffff, x) }

pub const HASH_RESULT_REG_COUNT: usize = 50;
pub const CMAC_RESULT_REG_COUNT: usize = 4;
pub const SE_CRYPTO_CTR_REG_COUNT: usize = 4;
pub const SE_MAX_KEYSLOT: u32 = 15;
pub const SE_MAX_MEM_ALLOC: usize = 4 * 1024 * 1024;
pub const TEGRA_AES_RESERVED_KSLT: u32 = 14;
pub const TEGRA_XTS_RESERVED_KSLT: u32 = 15;
pub const SHA_FIRST: u32 = 1;
pub const SHA_INIT: u32 = 2;
pub const SHA_UPDATE: u32 = 4;
pub const SHA_FINAL: u32 = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum se_aes_alg { SE_ALG_CBC, SE_ALG_ECB, SE_ALG_CTR, SE_ALG_XTS, SE_ALG_GMAC, SE_ALG_GCM, SE_ALG_GCM_FINAL, SE_ALG_CMAC, SE_ALG_CBC_MAC }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum se_hash_alg { SE_ALG_RNG_DRBG, SE_ALG_SHA1, SE_ALG_SHA224, SE_ALG_SHA256, SE_ALG_SHA384, SE_ALG_SHA512, SE_ALG_SHA3_224, SE_ALG_SHA3_256, SE_ALG_SHA3_384, SE_ALG_SHA3_512, SE_ALG_SHAKE128, SE_ALG_SHAKE256, SE_ALG_HMAC_SHA224, SE_ALG_HMAC_SHA256, SE_ALG_HMAC_SHA384, SE_ALG_HMAC_SHA512 }

/* External dependency types are intentionally referenced, not implemented here. */
pub enum tegra_se_cmdbuf {}
pub enum skcipher_engine_alg {}
pub enum aead_engine_alg {}
pub enum ahash_engine_alg {}
pub enum host1x_client {}
pub enum host1x_channel {}
pub enum crypto_engine {}
pub enum host1x_syncpt {}
pub enum device {}
pub enum clk {}
pub enum host1x_bo {}
pub enum kref {}
pub type dma_addr_t = usize;

#[repr(C)]
pub struct tegra_se_regs { pub op: u32, pub config: u32, pub last_blk: u32, pub linear_ctr: u32, pub out_addr: u32, pub aad_len: u32, pub cryp_msg_len: u32, pub manifest: u32, pub key_addr: u32, pub key_data: u32, pub key_dst: u32, pub result: u32 }
#[repr(C)]
pub struct tegra_se_hw { pub regs: *const tegra_se_regs, pub init_alg: Option<unsafe extern "C" fn(*mut tegra_se) -> i32>, pub deinit_alg: Option<unsafe extern "C" fn(*mut tegra_se)>, pub support_sm_alg: bool, pub host1x_class: u32, pub kac_ver: u32 }
#[repr(C)]
pub struct tegra_se { pub manifest: Option<unsafe extern "C" fn(u32, u32, u32) -> i32>, pub hw: *const tegra_se_hw, pub client: host1x_client, pub channel: *mut host1x_channel, pub cmdbuf: *mut tegra_se_cmdbuf, pub keybuf: *mut tegra_se_cmdbuf, pub engine: *mut crypto_engine, pub syncpt: *mut host1x_syncpt, pub dev: *mut device, pub clk: *mut clk, pub opcode_addr: u32, pub stream_id: u32, pub syncpt_id: u32, pub base: *mut core::ffi::c_void, pub owner: u32 }
#[repr(C)]
pub struct tegra_se_cmdbuf { pub iova: dma_addr_t, pub addr: *mut u32, pub dev: *mut device, pub ref_: kref, pub bo: host1x_bo, pub size: isize, pub words: u32 }
#[repr(C)]
pub struct tegra_se_datbuf { pub buf: *mut u8, pub addr: dma_addr_t, pub size: isize }

extern "C" {
    pub fn tegra_init_aes(se: *mut tegra_se) -> i32;
    pub fn tegra_init_hash(se: *mut tegra_se) -> i32;
    pub fn tegra_deinit_aes(se: *mut tegra_se);
    pub fn tegra_deinit_hash(se: *mut tegra_se);
    pub fn tegra_key_submit(se: *mut tegra_se, key: *const u8, keylen: u32, alg: u32, keyid: *mut u32) -> i32;
    pub fn tegra_key_submit_reserved(se: *mut tegra_se, key: *const u8, keylen: u32, alg: u32, keyid: *mut u32) -> i32;
    pub fn tegra_key_invalidate(se: *mut tegra_se, keyid: u32, alg: u32);
    pub fn tegra_key_invalidate_reserved(se: *mut tegra_se, keyid: u32, alg: u32);
    pub fn tegra_se_host1x_submit(se: *mut tegra_se, cmdbuf: *mut tegra_se_cmdbuf, size: u32) -> i32;
}

pub unsafe fn tegra_key_submit_reserved_aes(se: *mut tegra_se, key: *const u8, keylen: u32, alg: u32, keyid: *mut u32) -> i32 { *keyid = TEGRA_AES_RESERVED_KSLT; tegra_key_submit_reserved(se, key, keylen, alg, keyid) }
pub unsafe fn tegra_key_submit_reserved_xts(se: *mut tegra_se, key: *const u8, keylen: u32, alg: u32, keyid: *mut u32) -> i32 { *keyid = TEGRA_XTS_RESERVED_KSLT; tegra_key_submit_reserved(se, key, keylen, alg, keyid) }
pub const fn tegra_key_is_reserved(keyid: u32) -> bool { keyid == TEGRA_AES_RESERVED_KSLT || keyid == TEGRA_XTS_RESERVED_KSLT }

pub const fn host1x_opcode_setpayload(payload: u32) -> u32 { (9 << 28) | payload }
pub const fn host1x_opcode_incr_w(offset: u32) -> u32 { (10 << 28) | offset }
pub const fn host1x_opcode_nonincr_w(offset: u32) -> u32 { (11 << 28) | offset }
pub const fn host1x_opcode_incr(offset: u32, count: u32) -> u32 { (1 << 28) | (offset << 16) | count }
pub const fn host1x_opcode_nonincr(offset: u32, count: u32) -> u32 { (2 << 28) | (offset << 16) | count }
pub const fn host1x_uclass_incr_syncpt_cond_f(v: u32) -> u32 { (v & 0xff) << 10 }
pub const fn host1x_uclass_incr_syncpt_indx_f(v: u32) -> u32 { v & 0x3ff }
pub const fn host1x_uclass_wait_syncpt_r() -> u32 { 0x8 }
pub const fn host1x_uclass_incr_syncpt_r() -> u32 { 0 }
pub const fn se_host1x_opcode_incr_w(x: u32) -> u32 { host1x_opcode_incr_w(x / 4) }
pub const fn se_host1x_opcode_nonincr_w(x: u32) -> u32 { host1x_opcode_nonincr_w(x / 4) }
pub const fn se_host1x_opcode_incr(x: u32, y: u32) -> u32 { host1x_opcode_incr(x / 4, y) }
pub const fn se_host1x_opcode_nonincr(x: u32, y: u32) -> u32 { host1x_opcode_nonincr(x / 4, y) }

pub const SE_CFG_AES_ENCRYPT: u32 = SE_AES_ENC_ALG_AES_ENC | SE_AES_DEC_ALG_NOP | SE_AES_DST_MEMORY;
pub const SE_CFG_AES_DECRYPT: u32 = SE_AES_ENC_ALG_NOP | SE_AES_DEC_ALG_AES_DEC | SE_AES_DST_MEMORY;
pub const SE_CFG_GMAC_ENCRYPT: u32 = SE_AES_ENC_ALG_AES_ENC | SE_AES_DEC_ALG_NOP | SE_AES_ENC_MODE_GMAC | SE_AES_DST_MEMORY;
pub const SE_CFG_GMAC_DECRYPT: u32 = SE_AES_ENC_ALG_NOP | SE_AES_DEC_ALG_AES_DEC | SE_AES_DEC_MODE_GMAC | SE_AES_DST_MEMORY;
pub const SE_CFG_GCM_ENCRYPT: u32 = SE_AES_ENC_ALG_AES_ENC | SE_AES_DEC_ALG_NOP | SE_AES_ENC_MODE_GCM | SE_AES_DST_MEMORY;
pub const SE_CFG_GCM_DECRYPT: u32 = SE_AES_ENC_ALG_NOP | SE_AES_DEC_ALG_AES_DEC | SE_AES_DEC_MODE_GCM | SE_AES_DST_MEMORY;
pub const SE_CFG_GCM_FINAL_ENCRYPT: u32 = SE_AES_ENC_ALG_AES_ENC | SE_AES_DEC_ALG_NOP | SE_AES_ENC_MODE_GCM_FINAL | SE_AES_DST_MEMORY;
pub const SE_CFG_GCM_FINAL_DECRYPT: u32 = SE_AES_ENC_ALG_NOP | SE_AES_DEC_ALG_AES_DEC | SE_AES_DEC_MODE_GCM_FINAL | SE_AES_DST_MEMORY;
pub const SE_CFG_CMAC: u32 = SE_AES_ENC_ALG_AES_ENC | SE_AES_ENC_MODE_CMAC | SE_AES_DST_HASH_REG;
pub const SE_CFG_CBC_MAC: u32 = SE_AES_ENC_ALG_AES_ENC | SE_AES_ENC_MODE_CBC_MAC;
pub const SE_CFG_INS: u32 = SE_AES_ENC_ALG_INS | SE_AES_DEC_ALG_NOP;
pub const SE_CRYPTO_CFG_ECB_ENCRYPT: u32 = SE_AES_INPUT_SEL_MEMORY | SE_AES_XOR_POS_BYPASS | SE_AES_CORE_SEL_ENCRYPT;
pub const SE_CRYPTO_CFG_ECB_DECRYPT: u32 = SE_AES_INPUT_SEL_MEMORY | SE_AES_XOR_POS_BYPASS | SE_AES_CORE_SEL_DECRYPT;
pub const SE_CRYPTO_CFG_CBC_ENCRYPT: u32 = SE_AES_INPUT_SEL_MEMORY | SE_AES_VCTRAM_SEL_AESOUT | SE_AES_XOR_POS_TOP | SE_AES_CORE_SEL_ENCRYPT | SE_AES_IV_SEL_REG;
pub const SE_CRYPTO_CFG_CBC_DECRYPT: u32 = SE_AES_INPUT_SEL_MEMORY | SE_AES_VCTRAM_SEL_PREV_MEM | SE_AES_XOR_POS_BOTTOM | SE_AES_CORE_SEL_DECRYPT | SE_AES_IV_SEL_REG;
pub const SE_CRYPTO_CFG_CTR: u32 = SE_AES_INPUT_SEL_LINEAR_CTR | SE_AES_VCTRAM_SEL_MEMORY | SE_AES_XOR_POS_BOTTOM | SE_AES_CORE_SEL_ENCRYPT | SE_AES_CRYPTO_CFG_CTR_CNTN(1) | SE_AES_IV_SEL_REG;
pub const SE_CRYPTO_CFG_XTS_ENCRYPT: u32 = SE_AES_INPUT_SEL_MEMORY | SE_AES_VCTRAM_SEL_TWEAK | SE_AES_XOR_POS_BOTH | SE_AES_CORE_SEL_ENCRYPT | SE_AES_IV_SEL_REG;
pub const SE_CRYPTO_CFG_XTS_DECRYPT: u32 = SE_AES_INPUT_SEL_MEMORY | SE_AES_VCTRAM_SEL_TWEAK | SE_AES_XOR_POS_BOTH | SE_AES_CORE_SEL_DECRYPT | SE_AES_IV_SEL_REG;
pub const SE_CRYPTO_CFG_CBC_MAC: u32 = SE_AES_INPUT_SEL_MEMORY | SE_AES_VCTRAM_SEL_AESOUT | SE_AES_XOR_POS_TOP | SE_AES_CORE_SEL_ENCRYPT | SE_AES_HASH_ENABLE | SE_AES_IV_SEL_REG;

pub unsafe fn se_algname_to_algid(name: *const core::ffi::c_char) -> i32 {
    use core::ffi::CStr;
    match CStr::from_ptr(name).to_bytes() {
        b"cbc(aes)" => se_aes_alg::SE_ALG_CBC as i32, b"ecb(aes)" => se_aes_alg::SE_ALG_ECB as i32,
        b"ctr(aes)" => se_aes_alg::SE_ALG_CTR as i32, b"xts(aes)" => se_aes_alg::SE_ALG_XTS as i32,
        b"cmac(aes)" => se_aes_alg::SE_ALG_CMAC as i32, b"gcm(aes)" => se_aes_alg::SE_ALG_GCM as i32,
        b"ccm(aes)" => se_aes_alg::SE_ALG_CBC_MAC as i32, b"sha1" => se_hash_alg::SE_ALG_SHA1 as i32,
        b"sha224" => se_hash_alg::SE_ALG_SHA224 as i32, b"sha256" => se_hash_alg::SE_ALG_SHA256 as i32,
        b"sha384" => se_hash_alg::SE_ALG_SHA384 as i32, b"sha512" => se_hash_alg::SE_ALG_SHA512 as i32,
        b"sha3-224" => se_hash_alg::SE_ALG_SHA3_224 as i32, b"sha3-256" => se_hash_alg::SE_ALG_SHA3_256 as i32,
        b"sha3-384" => se_hash_alg::SE_ALG_SHA3_384 as i32, b"sha3-512" => se_hash_alg::SE_ALG_SHA3_512 as i32,
        b"hmac(sha224)" => se_hash_alg::SE_ALG_HMAC_SHA224 as i32, b"hmac(sha256)" => se_hash_alg::SE_ALG_HMAC_SHA256 as i32,
        b"hmac(sha384)" => se_hash_alg::SE_ALG_HMAC_SHA384 as i32, b"hmac(sha512)" => se_hash_alg::SE_ALG_HMAC_SHA512 as i32,
        _ => -22,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
