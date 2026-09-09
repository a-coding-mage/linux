/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

/* \file cc_buffer_mgr.h
 * Buffer Manager
 */

// C header guard: __CC_BUFFER_MGR_H__
// C dependencies: <crypto/algapi.h> and "cc_driver.h"

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cc_req_dma_buf_type {
    CC_DMA_BUF_NULL = 0,
    CC_DMA_BUF_DLLI,
    CC_DMA_BUF_MLLI,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cc_sg_cpy_direct {
    CC_SG_TO_BUF = 0,
    CC_SG_FROM_BUF = 1,
}

#[repr(C)]
pub struct cc_mlli {
    pub sram_addr: u32,
    pub mapped_nents: ::core::ffi::c_uint,
    pub nents: ::core::ffi::c_uint, // sg nents
    pub mlli_nents: ::core::ffi::c_uint, // mlli nents might be different than the above
}

#[repr(C)]
pub struct mlli_params {
    pub curr_pool: *mut dma_pool,
    pub mlli_virt_addr: *mut ::core::ffi::c_void,
    pub mlli_dma_addr: dma_addr_t,
    pub mlli_len: u32,
}

unsafe extern "C" {
    pub fn cc_buffer_mgr_init(drvdata: *mut cc_drvdata) -> ::core::ffi::c_int;

    pub fn cc_buffer_mgr_fini(drvdata: *mut cc_drvdata) -> ::core::ffi::c_int;

    pub fn cc_map_cipher_request(
        drvdata: *mut cc_drvdata,
        ctx: *mut ::core::ffi::c_void,
        ivsize: ::core::ffi::c_uint,
        nbytes: ::core::ffi::c_uint,
        info: *mut ::core::ffi::c_void,
        src: *mut scatterlist,
        dst: *mut scatterlist,
        flags: gfp_t,
    ) -> ::core::ffi::c_int;

    pub fn cc_unmap_cipher_request(
        dev: *mut device,
        ctx: *mut ::core::ffi::c_void,
        ivsize: ::core::ffi::c_uint,
        src: *mut scatterlist,
        dst: *mut scatterlist,
    );

    pub fn cc_map_aead_request(
        drvdata: *mut cc_drvdata,
        req: *mut aead_request,
    ) -> ::core::ffi::c_int;

    pub fn cc_unmap_aead_request(dev: *mut device, req: *mut aead_request);

    pub fn cc_map_hash_request_final(
        drvdata: *mut cc_drvdata,
        ctx: *mut ::core::ffi::c_void,
        src: *mut scatterlist,
        nbytes: ::core::ffi::c_uint,
        do_update: bool,
        flags: gfp_t,
    ) -> ::core::ffi::c_int;

    pub fn cc_map_hash_request_update(
        drvdata: *mut cc_drvdata,
        ctx: *mut ::core::ffi::c_void,
        src: *mut scatterlist,
        nbytes: ::core::ffi::c_uint,
        block_size: ::core::ffi::c_uint,
        flags: gfp_t,
    ) -> ::core::ffi::c_int;

    pub fn cc_unmap_hash_request(
        dev: *mut device,
        ctx: *mut ::core::ffi::c_void,
        src: *mut scatterlist,
        do_revert: bool,
    );

    pub fn cc_copy_sg_portion(
        dev: *mut device,
        dest: *mut u8,
        sg: *mut scatterlist,
        to_skip: u32,
        end: u32,
        direct: cc_sg_cpy_direct,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
