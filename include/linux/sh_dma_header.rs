/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Header for the new SH dmaengine driver
 *
 * Copyright (C) 2010 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 */

// C dependencies supplied by the surrounding translation unit:
// linux/dmaengine.h, linux/list.h, linux/shdma-base.h, linux/types.h

use core::ffi::c_char;

#[repr(C)]
pub struct device;

/* Used by slave DMA clients to request DMA to/from a specific peripheral */
#[repr(C)]
pub struct sh_dmae_slave {
    pub shdma_slave: shdma_slave,
}

/*
 * Supplied by platforms to specify, how a DMA channel has to be configured for
 * a certain peripheral
 */
#[repr(C)]
pub struct sh_dmae_slave_config {
    pub slave_id: core::ffi::c_int,
    pub addr: dma_addr_t,
    pub chcr: u32,
    pub mid_rid: c_char,
}

#[repr(C)]
pub struct sh_dmae_channel {
    pub offset: u32,
    pub dmars: u32,
    pub chclr_offset: u32,
    pub dmars_bit: u8,
    pub chclr_bit: u8,
}

#[repr(C)]
pub struct sh_dmae_pdata {
    pub slave: *const sh_dmae_slave_config,
    pub slave_num: core::ffi::c_int,
    pub channel: *const sh_dmae_channel,
    pub channel_num: core::ffi::c_int,
    pub ts_low_shift: u32,
    pub ts_low_mask: u32,
    pub ts_high_shift: u32,
    pub ts_high_mask: u32,
    pub ts_shift: *const u32,
    pub ts_shift_num: core::ffi::c_int,
    pub dmaor_init: u16,
    pub chcr_offset: u32,
    pub chcr_ie_bit: u32,

    /* C unsigned-int bitfields, packed into one 32-bit storage unit. */
    pub feature_flags: u32,
}

pub const SH_DMAE_PDATA_DMAOR_IS_32BIT: u32 = 1 << 0;
pub const SH_DMAE_PDATA_NEEDS_TEND_SET: u32 = 1 << 1;
pub const SH_DMAE_PDATA_NO_DMARS: u32 = 1 << 2;
pub const SH_DMAE_PDATA_CHCLR_PRESENT: u32 = 1 << 3;
pub const SH_DMAE_PDATA_CHCLR_BITWISE: u32 = 1 << 4;
pub const SH_DMAE_PDATA_SLAVE_ONLY: u32 = 1 << 5;

/* DMAOR definitions */
pub const DMAOR_AE: u32 = 0x00000004; /* Address Error Flag */
pub const DMAOR_NMIF: u32 = 0x00000002;
pub const DMAOR_DME: u32 = 0x00000001; /* DMA Master Enable */

/* Definitions for the SuperH DMAC */
pub const DM_INC: u32 = 0x00004000; /* Destination addresses are incremented */
pub const DM_DEC: u32 = 0x00008000; /* Destination addresses are decremented */
pub const DM_FIX: u32 = 0x0000c000; /* Destination address is fixed */
pub const SM_INC: u32 = 0x00001000; /* Source addresses are incremented */
pub const SM_DEC: u32 = 0x00002000; /* Source addresses are decremented */
pub const SM_FIX: u32 = 0x00003000; /* Source address is fixed */
pub const RS_AUTO: u32 = 0x00000400; /* Auto Request */
pub const RS_ERS: u32 = 0x00000800; /* DMA extended resource selector */
pub const CHCR_DE: u32 = 0x00000001; /* DMA Enable */
pub const CHCR_TE: u32 = 0x00000002; /* Transfer End Flag */
pub const CHCR_IE: u32 = 0x00000004; /* Interrupt Enable */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
