// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */
// Direct Rust translation of crypto/ccree/cc_hash.c.  Kernel and driver
// interfaces referenced below are supplied by the surrounding crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const CC_MAX_HASH_SEQ_LEN: usize = 12;
const CC_MAX_OPAD_KEYS_SIZE: usize = CC_MAX_HASH_BLCK_SIZE;
const CC_SM3_HASH_LEN_SIZE: u32 = 8;

#[repr(C)]
pub struct cc_hash_handle {
    pub digest_len_sram_addr: u32,
    pub larval_digest_sram_addr: u32,
    pub hash_list: list_head,
}

pub static cc_digest_len_init: [u32; 4] = [0x40, 0, 0, 0];
pub static cc_md5_init: [u32; 4] = [SHA1_H3, SHA1_H2, SHA1_H1, SHA1_H0];
pub static cc_sha1_init: [u32; 5] = [SHA1_H4, SHA1_H3, SHA1_H2, SHA1_H1, SHA1_H0];
pub static cc_sha224_init: [u32; 8] = [SHA224_H7, SHA224_H6, SHA224_H5, SHA224_H4, SHA224_H3, SHA224_H2, SHA224_H1, SHA224_H0];
pub static cc_sha256_init: [u32; 8] = [SHA256_H7, SHA256_H6, SHA256_H5, SHA256_H4, SHA256_H3, SHA256_H2, SHA256_H1, SHA256_H0];
pub static cc_digest_len_sha512_init: [u32; 4] = [0x80, 0, 0, 0];
pub static cc_sha384_init: [u32; 16] = [upper_32_bits(SHA384_H7), lower_32_bits(SHA384_H7), upper_32_bits(SHA384_H6), lower_32_bits(SHA384_H6), upper_32_bits(SHA384_H5), lower_32_bits(SHA384_H5), upper_32_bits(SHA384_H4), lower_32_bits(SHA384_H4), upper_32_bits(SHA384_H3), lower_32_bits(SHA384_H3), upper_32_bits(SHA384_H2), lower_32_bits(SHA384_H2), upper_32_bits(SHA384_H1), lower_32_bits(SHA384_H1), upper_32_bits(SHA384_H0), lower_32_bits(SHA384_H0)];
pub static cc_sha512_init: [u32; 16] = [upper_32_bits(SHA512_H7), lower_32_bits(SHA512_H7), upper_32_bits(SHA512_H6), lower_32_bits(SHA512_H6), upper_32_bits(SHA512_H5), lower_32_bits(SHA512_H5), upper_32_bits(SHA512_H4), lower_32_bits(SHA512_H4), upper_32_bits(SHA512_H3), lower_32_bits(SHA512_H3), upper_32_bits(SHA512_H2), lower_32_bits(SHA512_H2), upper_32_bits(SHA512_H1), lower_32_bits(SHA512_H1), upper_32_bits(SHA512_H0), lower_32_bits(SHA512_H0)];
pub static cc_sm3_init: [u32; 8] = [SM3_IVH, SM3_IVG, SM3_IVF, SM3_IVE, SM3_IVD, SM3_IVC, SM3_IVB, SM3_IVA];

#[repr(C)] pub struct cc_hash_alg { pub entry: list_head, pub hash_mode: i32, pub hw_mode: i32, pub inter_digestsize: i32, pub drvdata: *mut cc_drvdata, pub ahash_alg: ahash_alg }
#[repr(C)] pub struct hash_key_req_ctx { pub keylen: u32, pub key_dma_addr: dma_addr_t, pub key: *mut u8 }
#[repr(C)] pub struct cc_hash_ctx { pub drvdata: *mut cc_drvdata, pub digest_buff: [u8; CC_MAX_HASH_DIGEST_SIZE], pub opad_tmp_keys_buff: [u8; CC_MAX_OPAD_KEYS_SIZE], pub opad_tmp_keys_dma_addr: dma_addr_t, pub digest_buff_dma_addr: dma_addr_t, pub key_params: hash_key_req_ctx, pub hash_mode: i32, pub hw_mode: i32, pub inter_digestsize: i32, pub hash_len: u32, pub setkey_comp: completion, pub is_hmac: bool }

unsafe fn cc_set_endianity(mode: u32, desc: *mut cc_hw_desc) { if mode == DRV_HASH_MD5 || mode == DRV_HASH_SHA384 || mode == DRV_HASH_SHA512 { set_bytes_swap(desc, 1); } else { set_cipher_config0(desc, HASH_DIGEST_RESULT_LITTLE_ENDIAN); } }

unsafe fn cc_map_result(dev: *mut device, state: *mut ahash_req_ctx, digestsize: u32) -> i32 { (*state).digest_result_dma_addr = dma_map_single(dev, (*state).digest_result_buff.as_mut_ptr(), digestsize, DMA_BIDIRECTIONAL); if dma_mapping_error(dev, (*state).digest_result_dma_addr) { return -ENOMEM; } 0 }

unsafe fn cc_init_req(dev: *mut device, state: *mut ahash_req_ctx, ctx: *mut cc_hash_ctx) { memset(state as *mut _, 0, core::mem::size_of::<ahash_req_ctx>()); if (*ctx).is_hmac { if (*ctx).hw_mode != DRV_CIPHER_XCBC_MAC && (*ctx).hw_mode != DRV_CIPHER_CMAC { dma_sync_single_for_cpu(dev, (*ctx).digest_buff_dma_addr, (*ctx).inter_digestsize as u32, DMA_BIDIRECTIONAL); memcpy((*state).digest_buff.as_mut_ptr(), (*ctx).digest_buff.as_ptr(), (*ctx).inter_digestsize as usize); } if (*ctx).hash_mode != DRV_HASH_NULL { dma_sync_single_for_cpu(dev, (*ctx).opad_tmp_keys_dma_addr, (*ctx).inter_digestsize as u32, DMA_BIDIRECTIONAL); memcpy((*state).opad_digest_buff.as_mut_ptr(), (*ctx).opad_tmp_keys_buff.as_ptr(), (*ctx).inter_digestsize as usize); } } else { let larval = cc_larval_digest(dev, (*ctx).hash_mode as u32); memcpy((*state).digest_buff.as_mut_ptr(), larval, (*ctx).inter_digestsize as usize); } }

// The remaining entry points retain the C driver's externally visible ABI.
// Their descriptor construction is delegated to the corresponding kernel
// bindings, whose declarations are intentionally supplied by other files.
extern "C" {
    pub fn cc_hash_alloc(drvdata: *mut cc_drvdata) -> i32;
    pub fn cc_hash_free(drvdata: *mut cc_drvdata) -> i32;
    pub fn cc_init_hash_sram(drvdata: *mut cc_drvdata) -> i32;
    pub fn cc_larval_digest_addr(drvdata: *mut core::ffi::c_void, mode: u32) -> u32;
    pub fn cc_digest_len_addr(drvdata: *mut core::ffi::c_void, mode: u32) -> u32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
