/* SPDX-License-Identifier: GPL-2.0
 *
 * Common header for the legacy SH DMA driver and the new dmaengine driver
 *
 * extracted from arch/sh/include/asm/dma-sh.h:
 *
 * Copyright (C) 2000  Takashi YOSHII
 * Copyright (C) 2003  Paul Mundt
 */

/* DMA registers */
pub const SAR: u32 = 0x00; /* Source Address Register */
pub const DAR: u32 = 0x04; /* Destination Address Register */
pub const TCR: u32 = 0x08; /* Transfer Count Register */
pub const CHCR: u32 = 0x0C; /* Channel Control Register */
pub const DMAOR: u32 = 0x40; /* DMA Operation Register */

/* DMAOR definitions */
pub const DMAOR_AE: u32 = 0x00000004; /* Address Error Flag */
pub const DMAOR_NMIF: u32 = 0x00000002;
pub const DMAOR_DME: u32 = 0x00000001; /* DMA Master Enable */

/* Definitions for the SuperH DMAC */
pub const REQ_L: u32 = 0x00000000;
pub const REQ_E: u32 = 0x00080000;
pub const RACK_H: u32 = 0x00000000;
pub const RACK_L: u32 = 0x00040000;
pub const ACK_R: u32 = 0x00000000;
pub const ACK_W: u32 = 0x00020000;
pub const ACK_H: u32 = 0x00000000;
pub const ACK_L: u32 = 0x00010000;
pub const DM_INC: u32 = 0x00004000; /* Destination addresses are incremented */
pub const DM_DEC: u32 = 0x00008000; /* Destination addresses are decremented */
pub const DM_FIX: u32 = 0x0000c000; /* Destination address is fixed */
pub const SM_INC: u32 = 0x00001000; /* Source addresses are incremented */
pub const SM_DEC: u32 = 0x00002000; /* Source addresses are decremented */
pub const SM_FIX: u32 = 0x00003000; /* Source address is fixed */
pub const RS_IN: u32 = 0x00000200;
pub const RS_OUT: u32 = 0x00000300;
pub const RS_AUTO: u32 = 0x00000400; /* Auto Request */
pub const RS_ERS: u32 = 0x00000800; /* DMA extended resource selector */
pub const TS_BLK: u32 = 0x00000040;
pub const TM_BUR: u32 = 0x00000020;
pub const CHCR_DE: u32 = 0x00000001; /* DMA Enable */
pub const CHCR_TE: u32 = 0x00000002; /* Transfer End Flag */
pub const CHCR_IE: u32 = 0x00000004; /* Interrupt Enable */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
