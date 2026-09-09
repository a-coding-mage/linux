/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This file traditionally describes the i8237 PC style DMA controller.
 * Most architectures don't have these any more and can get the minimal
 * implementation from kernel/dma.c by not defining MAX_DMA_CHANNELS.
 *
 * Some code relies on seeing MAX_DMA_ADDRESS though.
 */

// Supplied by the surrounding target environment.
pub const MAX_DMA_ADDRESS: usize = PAGE_OFFSET;

unsafe extern "C" {
    pub fn request_dma(dmanr: ::core::ffi::c_uint, device_id: *const ::core::ffi::c_char)
        -> ::core::ffi::c_int;
    pub fn free_dma(dmanr: ::core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
