/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Driver for the High Speed UART DMA
 *
 * Copyright (C) 2015 Intel Corporation
 *
 * Partially based on the bits found in drivers/tty/serial/mfd.c.
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

pub const HSU_CH_SR: usize = 0x00; // channel status
pub const HSU_CH_CR: usize = 0x04; // channel control
pub const HSU_CH_DCR: usize = 0x08; // descriptor control
pub const HSU_CH_BSR: usize = 0x10; // FIFO buffer size
pub const HSU_CH_MTSR: usize = 0x14; // minimum transfer size
pub const fn HSU_CH_DxSAR(x: usize) -> usize { 0x20 + 8 * x } // desc start addr
pub const fn HSU_CH_DxTSR(x: usize) -> usize { 0x24 + 8 * x } // desc transfer size
pub const HSU_CH_D0SAR: usize = 0x20; // desc 0 start addr
pub const HSU_CH_D0TSR: usize = 0x24; // desc 0 transfer size
pub const HSU_CH_D1SAR: usize = 0x28;
pub const HSU_CH_D1TSR: usize = 0x2c;
pub const HSU_CH_D2SAR: usize = 0x30;
pub const HSU_CH_D2TSR: usize = 0x34;
pub const HSU_CH_D3SAR: usize = 0x38;
pub const HSU_CH_D3TSR: usize = 0x3c;

pub const HSU_DMA_CHAN_NR_DESC: usize = 4;
pub const HSU_DMA_CHAN_LENGTH: usize = 0x40;

pub const fn HSU_CH_SR_DESCTO(x: u32) -> u32 { 1u32 << (8 + x) }
pub const HSU_CH_SR_DESCTO_ANY: u32 = 0x00000f00;
pub const HSU_CH_SR_CHE: u32 = 1u32 << 15;
pub const fn HSU_CH_SR_DESCE(x: u32) -> u32 { 1u32 << (16 + x) }
pub const HSU_CH_SR_DESCE_ANY: u32 = 0x000f0000;
pub const HSU_CH_SR_CDESC_ANY: u32 = 0xc0000000;

pub const HSU_CH_CR_CHA: u32 = 1u32 << 0;
pub const HSU_CH_CR_CHD: u32 = 1u32 << 1;

pub const fn HSU_CH_DCR_DESCA(x: u32) -> u32 { 1u32 << x }
pub const fn HSU_CH_DCR_CHSOD(x: u32) -> u32 { 1u32 << (8 + x) }
pub const HSU_CH_DCR_CHSOTO: u32 = 1u32 << 14;
pub const HSU_CH_DCR_CHSOE: u32 = 1u32 << 15;
pub const fn HSU_CH_DCR_CHDI(x: u32) -> u32 { 1u32 << (16 + x) }
pub const HSU_CH_DCR_CHEI: u32 = 1u32 << 23;
pub const fn HSU_CH_DCR_CHTOI(x: u32) -> u32 { 1u32 << (24 + x) }

pub const HSU_CH_DxTSR_MASK: u32 = 0x0000ffff;
pub const fn HSU_CH_DxTSR_TSR(x: u32) -> u32 { x & HSU_CH_DxTSR_MASK }

#[repr(C)]
pub struct hsu_dma_sg {
    pub addr: dma_addr_t,
    pub len: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct hsu_dma_desc {
    pub vdesc: virt_dma_desc,
    pub direction: dma_transfer_direction,
    pub sg: *mut hsu_dma_sg,
    pub nents: ::core::ffi::c_uint,
    pub length: usize,
    pub active: ::core::ffi::c_uint,
    pub status: dma_status,
}

#[inline]
pub unsafe fn to_hsu_dma_desc(vdesc: *mut virt_dma_desc) -> *mut hsu_dma_desc {
    container_of!(vdesc, hsu_dma_desc, vdesc)
}

#[repr(C)]
pub struct hsu_dma_chan {
    pub vchan: virt_dma_chan,
    pub reg: *mut ::core::ffi::c_void,
    // hardware configuration
    pub direction: dma_transfer_direction,
    pub config: dma_slave_config,
    pub desc: *mut hsu_dma_desc,
}

#[inline]
pub unsafe fn to_hsu_dma_chan(chan: *mut dma_chan) -> *mut hsu_dma_chan {
    container_of!(chan, hsu_dma_chan, vchan.chan)
}

#[inline]
pub unsafe fn hsu_chan_readl(hsuc: *mut hsu_dma_chan, offset: isize) -> u32 {
    readl((*hsuc).reg.offset(offset))
}

#[inline]
pub unsafe fn hsu_chan_writel(hsuc: *mut hsu_dma_chan, offset: isize, value: u32) {
    writel(value, (*hsuc).reg.offset(offset));
}

#[repr(C)]
pub struct hsu_dma {
    pub dma: dma_device,
    // channels
    pub chan: *mut hsu_dma_chan,
    pub nr_channels: u16,
}

#[inline]
pub unsafe fn to_hsu_dma(ddev: *mut dma_device) -> *mut hsu_dma {
    container_of!(ddev, hsu_dma, dma)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
