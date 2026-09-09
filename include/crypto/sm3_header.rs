/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SM3 hash algorithm
 *
 * Copyright (C) 2017 ARM Limited or its affiliates.
 * Copyright (C) 2017 Gilad Ben-Yossef <gilad@benyossef.com>
 * Copyright (C) 2021 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
 */

// Dependency intent: the C header includes <linux/types.h> for u8, u32, and u64.

pub const SM3_DIGEST_SIZE: usize = 32;
pub const SM3_BLOCK_SIZE: usize = 64;

pub const SM3_IVA: u32 = 0x7380_166f;
pub const SM3_IVB: u32 = 0x4914_b2b9;
pub const SM3_IVC: u32 = 0x1724_42d7;
pub const SM3_IVD: u32 = 0xda8a_0600;
pub const SM3_IVE: u32 = 0xa96f_30bc;
pub const SM3_IVF: u32 = 0x1631_38aa;
pub const SM3_IVG: u32 = 0xe38d_ee4d;
pub const SM3_IVH: u32 = 0xb0fb_0e4e;

/// State for the SM3 compression function.
#[repr(C)]
pub struct sm3_block_state {
    pub h: [u32; SM3_DIGEST_SIZE / 4],
}

/// Context for hashing a message with SM3.
///
/// `state`: the compression function state
/// `bytecount`: number of bytes processed so far
/// `buf`: partial block buffer; `bytecount % SM3_BLOCK_SIZE` bytes are valid
#[repr(C, align(8))]
pub struct sm3_ctx {
    pub state: sm3_block_state,
    pub bytecount: u64,
    pub buf: [u8; SM3_BLOCK_SIZE],
}

/// Initialize an SM3 context for a new message.
///
/// If incremental computation is not needed, consider `sm3` instead.
extern "C" {
    pub fn sm3_init(ctx: *mut sm3_ctx);

    /// Update an SM3 context with message data.
    ///
    /// The context must have been initialized. This can be called any number
    /// of times.
    pub fn sm3_update(ctx: *mut sm3_ctx, data: *const u8, len: usize);

    /// Finish computing an SM3 message digest.
    ///
    /// After finishing, this zeroizes `ctx`.
    pub fn sm3_final(ctx: *mut sm3_ctx, out: *mut u8);

    /// Compute an SM3 message digest in one shot.
    pub fn sm3(data: *const u8, len: usize, out: *mut u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
