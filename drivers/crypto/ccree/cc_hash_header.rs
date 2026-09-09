/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

/* \file cc_hash.h
 * ARM CryptoCell Hash Crypto API
 */

/* Dependency: cc_buffer_mgr.h */

pub const HMAC_IPAD_CONST: u32 = 0x3636_3636;
pub const HMAC_OPAD_CONST: u32 = 0x5C5C_5C5C;
pub const HASH_LEN_SIZE_712: usize = 16;
pub const HASH_LEN_SIZE_630: usize = 8;
pub const HASH_MAX_LEN_SIZE: usize = HASH_LEN_SIZE_712;
pub const CC_MAX_HASH_DIGEST_SIZE: usize = SHA512_DIGEST_SIZE;
pub const CC_MAX_HASH_BLCK_SIZE: usize = SHA512_BLOCK_SIZE;

pub const XCBC_MAC_K1_OFFSET: usize = 0;
pub const XCBC_MAC_K2_OFFSET: usize = 16;
pub const XCBC_MAC_K3_OFFSET: usize = 32;

pub const CC_EXPORT_MAGIC: u32 = 0xC2EE_1070;

/* this struct was taken from drivers/crypto/nx/nx-aes-xcbc.c and it is used
 * for xcbc/cmac statesize
 */
#[repr(C)]
pub struct aeshash_state {
    pub state: [u8; AES_BLOCK_SIZE],
    pub count: core::ffi::c_uint,
    pub buffer: [u8; AES_BLOCK_SIZE],
}

/* ahash state */
#[repr(C)]
pub struct ahash_req_ctx {
    pub buffers: [[u8; CC_MAX_HASH_BLCK_SIZE]; 2],
    pub digest_result_buff: [u8; CC_MAX_HASH_DIGEST_SIZE],
    pub digest_buff: [u8; CC_MAX_HASH_DIGEST_SIZE],
    pub opad_digest_buff: [u8; CC_MAX_HASH_DIGEST_SIZE],
    pub digest_bytes_len: [u8; HASH_MAX_LEN_SIZE],
    pub gen_ctx: async_gen_req_ctx,
    pub data_dma_buf_type: cc_req_dma_buf_type,
    pub opad_digest_dma_addr: dma_addr_t,
    pub digest_buff_dma_addr: dma_addr_t,
    pub digest_bytes_len_dma_addr: dma_addr_t,
    pub digest_result_dma_addr: dma_addr_t,
    pub buf_cnt: [u32; 2],
    pub buff_index: u32,
    pub xcbc_count: u32, /* count xcbc update operatations */
    pub buff_sg: [scatterlist; 2],
    pub curr_sg: *mut scatterlist,
    pub in_nents: u32,
    pub mlli_nents: u32,
    pub mlli_params: mlli_params,
}

pub unsafe inline fn cc_hash_buf_cnt(state: *mut ahash_req_ctx) -> *mut u32 {
    (*state).buf_cnt.as_mut_ptr().add((*state).buff_index as usize)
}

pub unsafe inline fn cc_hash_buf(state: *mut ahash_req_ctx) -> *mut u8 {
    (*state).buffers[(*state).buff_index as usize].as_mut_ptr()
}

pub unsafe inline fn cc_next_buf_cnt(state: *mut ahash_req_ctx) -> *mut u32 {
    (*state)
        .buf_cnt
        .as_mut_ptr()
        .add(((*state).buff_index ^ 1) as usize)
}

pub unsafe inline fn cc_next_buf(state: *mut ahash_req_ctx) -> *mut u8 {
    (*state).buffers[((*state).buff_index ^ 1) as usize].as_mut_ptr()
}

extern "C" {
    pub fn cc_hash_alloc(drvdata: *mut cc_drvdata) -> i32;
    pub fn cc_init_hash_sram(drvdata: *mut cc_drvdata) -> i32;
    pub fn cc_hash_free(drvdata: *mut cc_drvdata) -> i32;

    /**
     * cc_digest_len_addr() - Gets the initial digest length
     *
     * @drvdata: Associated device driver context
     * @mode: The Hash mode. Supported modes: MD5/SHA1/SHA224/SHA256/SHA384/SHA512
     *
     * Return:
     * Returns the address of the initial digest length in SRAM
     */
    pub fn cc_digest_len_addr(drvdata: *mut core::ffi::c_void, mode: u32) -> u32;

    /**
     * cc_larval_digest_addr() - Gets the address of the initial digest in SRAM
     * according to the given hash mode
     *
     * @drvdata: Associated device driver context
     * @mode: The Hash mode. Supported modes: MD5/SHA1/SHA224/SHA256/SHA384/SHA512
     *
     * Return:
     * The address of the initial digest in SRAM
     */
    pub fn cc_larval_digest_addr(drvdata: *mut core::ffi::c_void, mode: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
