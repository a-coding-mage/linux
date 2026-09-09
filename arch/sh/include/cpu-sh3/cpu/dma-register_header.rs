/* SPDX-License-Identifier: GPL-2.0
 *
 * SH3 CPU-specific DMA definitions, used by both DMA drivers
 *
 * Copyright (C) 2010 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 */

pub const CHCR_TS_LOW_MASK: u32 = 0x18;
pub const CHCR_TS_LOW_SHIFT: u32 = 3;
pub const CHCR_TS_HIGH_MASK: u32 = 0;
pub const CHCR_TS_HIGH_SHIFT: u32 = 0;

// DMAOR_DME is supplied by the corresponding DMA register definitions.
pub const DMAOR_INIT: u32 = DMAOR_DME;

/*
 * The SuperH DMAC supports a number of transmit sizes, we list them here,
 * with their respective values as they appear in the CHCR registers.
 */
#[repr(i32)]
pub enum XmitSize {
    XMIT_SZ_8BIT,
    XMIT_SZ_16BIT,
    XMIT_SZ_32BIT,
    XMIT_SZ_128BIT,
}

/* log2(size / 8) - used to calculate number of transfers */
pub const TS_SHIFT: [u32; 4] = [
    0, // XMIT_SZ_8BIT
    1, // XMIT_SZ_16BIT
    2, // XMIT_SZ_32BIT
    4, // XMIT_SZ_128BIT
];

#[inline]
pub const fn TS_INDEX2VAL(i: u32) -> u32 {
    (i & 3).wrapping_shl(CHCR_TS_LOW_SHIFT)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
