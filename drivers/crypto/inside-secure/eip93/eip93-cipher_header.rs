/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2019 - 2021
 *
 * Richard van Schagen <vschagen@icloud.com>
 * Christian Marangi <ansuelsmth@gmail.com>
 */

// Dependency declarations from eip93-main.h are supplied externally.

#[repr(C)]
pub struct eip93_crypto_ctx {
    pub eip93: *mut eip93_device,
    pub flags: u32,
    pub sa_record: *mut sa_record,
    pub sa_nonce: u32,
    pub blksize: i32,
    pub sa_record_base: dma_addr_t,
    /* AEAD specific */
    pub authsize: u32,
    pub assoclen: u32,
    pub set_assoc: bool,
    pub type_: eip93_alg_type,
}

#[repr(C)]
pub struct eip93_cipher_reqctx {
    pub desc_flags: u16,
    pub flags: u16,
    pub blksize: u32,
    pub ivsize: u32,
    pub textsize: u32,
    pub assoclen: u32,
    pub authsize: u32,
    pub sa_record_base: dma_addr_t,
    pub sa_state: *mut sa_state,
    pub sa_state_base: dma_addr_t,
    pub cdesc: *mut eip93_descriptor,
    pub sg_src: *mut scatterlist,
    pub sg_dst: *mut scatterlist,
    pub src_nents: i32,
    pub dst_nents: i32,
    pub sa_state_ctr: *mut sa_state,
    pub sa_state_ctr_base: dma_addr_t,
}

extern "C" {
    pub fn check_valid_request(rctx: *mut eip93_cipher_reqctx) -> i32;

    pub fn eip93_unmap_dma(
        eip93: *mut eip93_device,
        rctx: *mut eip93_cipher_reqctx,
        reqsrc: *mut scatterlist,
        reqdst: *mut scatterlist,
    );

    pub fn eip93_skcipher_handle_result(async_: *mut crypto_async_request, err: i32);

    pub fn eip93_send_req(
        async_: *mut crypto_async_request,
        reqiv: *const u8,
        rctx: *mut eip93_cipher_reqctx,
    ) -> i32;

    pub fn eip93_handle_result(
        eip93: *mut eip93_device,
        rctx: *mut eip93_cipher_reqctx,
        reqiv: *mut u8,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
