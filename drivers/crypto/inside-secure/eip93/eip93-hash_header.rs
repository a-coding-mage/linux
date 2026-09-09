/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2019 - 2021
 *
 * Richard van Schagen <vschagen@icloud.com>
 * Christian Marangi <ansuelsmth@gmail.com>
 */

/* Dependencies supplied by crypto/sha2.h, eip93-main.h, and eip93-regs.h. */

#[repr(C)]
pub struct eip93_hash_ctx {
    pub eip93: *mut eip93_device,
    pub flags: u32,

    pub ipad: [u8; SHA256_BLOCK_SIZE],
    pub opad: [u8; SHA256_DIGEST_SIZE],
}

#[repr(C, align(4))]
pub struct eip93_hash_reqctx {
    /* Placement is important for DMA align */
    pub records: eip93_hash_reqctx_records,

    pub sa_record_base: dma_addr_t,
    pub sa_record_hmac_base: dma_addr_t,
    pub sa_state_base: dma_addr_t,

    /* Don't enable HASH_FINALIZE when last block is sent */
    pub partial_hash: bool,

    /* Set to signal interrupt is for final packet */
    pub finalize: bool,

    /*
     * EIP93 requires data to be accumulated in block of 64 bytes
     * for intermediate hash calculation.
     */
    pub len: u64,
    pub data_used: u32,

    pub data: [u8; SHA256_BLOCK_SIZE],
    pub data_dma: dma_addr_t,

    pub blocks: list_head,
}

#[repr(C)]
pub struct eip93_hash_reqctx_records {
    pub sa_record: sa_record,
    pub sa_record_hmac: sa_record,
    pub sa_state: sa_state,
}

#[repr(C)]
pub struct mkt_hash_block {
    pub list: list_head,
    pub data: [u8; SHA256_BLOCK_SIZE],
    pub data_dma: dma_addr_t,
}

#[repr(C)]
pub struct eip93_hash_export_state {
    pub len: u64,
    pub data_used: u32,

    pub state_len: [u32; 2],
    pub state_hash: [u8; SHA256_DIGEST_SIZE],

    pub data: [u8; SHA256_BLOCK_SIZE],
}

pub unsafe extern "C" fn eip93_hash_handle_result(
    async_: *mut crypto_async_request,
    err: i32,
);

extern "C" {
    pub static mut eip93_alg_md5: eip93_alg_template;
    pub static mut eip93_alg_sha1: eip93_alg_template;
    pub static mut eip93_alg_sha224: eip93_alg_template;
    pub static mut eip93_alg_sha256: eip93_alg_template;
    pub static mut eip93_alg_hmac_md5: eip93_alg_template;
    pub static mut eip93_alg_hmac_sha1: eip93_alg_template;
    pub static mut eip93_alg_hmac_sha224: eip93_alg_template;
    pub static mut eip93_alg_hmac_sha256: eip93_alg_template;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
