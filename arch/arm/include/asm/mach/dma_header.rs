/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/mach/dma.h
 *
 *  Copyright (C) 1998-2000 Russell King
 *
 *  This header file describes the interface between the generic DMA handler
 *  (dma.c) and the architecture-specific DMA backends (dma-*.c)
 */

pub struct dma_struct;
pub type dma_t = dma_struct;

#[repr(C)]
pub struct dma_ops {
    pub request: Option<unsafe extern "C" fn(unsigned_int: ::core::ffi::c_uint, dma: *mut dma_t) -> ::core::ffi::c_int>, // optional
    pub free: Option<unsafe extern "C" fn(unsigned_int: ::core::ffi::c_uint, dma: *mut dma_t)>, // optional
    pub enable: Option<unsafe extern "C" fn(unsigned_int: ::core::ffi::c_uint, dma: *mut dma_t)>, // mandatory
    pub disable: Option<unsafe extern "C" fn(unsigned_int: ::core::ffi::c_uint, dma: *mut dma_t)>, // mandatory
    pub residue: Option<unsafe extern "C" fn(unsigned_int: ::core::ffi::c_uint, dma: *mut dma_t) -> ::core::ffi::c_int>, // optional
    pub setspeed: Option<unsafe extern "C" fn(unsigned_int: ::core::ffi::c_uint, dma: *mut dma_t, speed: ::core::ffi::c_int) -> ::core::ffi::c_int>, // optional
    pub type_: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct dma_struct {
    pub addr: *mut ::core::ffi::c_void, // single DMA address
    pub count: ::core::ffi::c_ulong, // single DMA size
    pub buf: scatterlist, // single DMA
    pub sgcount: ::core::ffi::c_int, // number of DMA SG
    pub sg: *mut scatterlist, // DMA Scatter-Gather List

    // C bit-fields: unsigned int active:1; unsigned int invalid:1;
    pub active: ::core::ffi::c_uint, // Transfer active
    pub invalid: ::core::ffi::c_uint, // Address/Count changed

    pub dma_mode: ::core::ffi::c_uint, // DMA mode
    pub speed: ::core::ffi::c_int, // DMA speed

    pub lock: ::core::ffi::c_uint, // Device is allocated
    pub device_id: *const ::core::ffi::c_char, // Device name

    pub d_ops: *const dma_ops,
}

/*
 * isa_dma_add - add an ISA-style DMA channel
 */
unsafe extern "C" {
    pub fn isa_dma_add(channel: ::core::ffi::c_uint, dma: *mut dma_t) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
