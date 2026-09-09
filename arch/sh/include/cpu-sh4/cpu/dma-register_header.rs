/* SPDX-License-Identifier: GPL-2.0
 *
 * SH4 CPU-specific DMA definitions, used by both DMA drivers
 *
 * Copyright (C) 2010 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 */

/* SH7751/7760/7780 DMA IRQ sources */

#[cfg(feature = "CONFIG_CPU_SH4A")]
pub const DMAOR_INIT: u32 = DMAOR_DME;

#[cfg(all(feature = "CONFIG_CPU_SH4A", feature = "CONFIG_CPU_SUBTYPE_SH7343"))]
pub const CHCR_TS_LOW_MASK: u32 = 0x00000018;
#[cfg(all(feature = "CONFIG_CPU_SH4A", feature = "CONFIG_CPU_SUBTYPE_SH7343"))]
pub const CHCR_TS_LOW_SHIFT: u32 = 3;
#[cfg(all(feature = "CONFIG_CPU_SH4A", feature = "CONFIG_CPU_SUBTYPE_SH7343"))]
pub const CHCR_TS_HIGH_MASK: u32 = 0;
#[cfg(all(feature = "CONFIG_CPU_SH4A", feature = "CONFIG_CPU_SUBTYPE_SH7343"))]
pub const CHCR_TS_HIGH_SHIFT: u32 = 0;

#[cfg(all(
    feature = "CONFIG_CPU_SH4A",
    any(
        feature = "CONFIG_CPU_SUBTYPE_SH7722",
        feature = "CONFIG_CPU_SUBTYPE_SH7723",
        feature = "CONFIG_CPU_SUBTYPE_SH7724",
        feature = "CONFIG_CPU_SUBTYPE_SH7730",
        feature = "CONFIG_CPU_SUBTYPE_SH7786"
    )
))]
pub const CHCR_TS_LOW_MASK: u32 = 0x00000018;
#[cfg(all(
    feature = "CONFIG_CPU_SH4A",
    any(
        feature = "CONFIG_CPU_SUBTYPE_SH7722",
        feature = "CONFIG_CPU_SUBTYPE_SH7723",
        feature = "CONFIG_CPU_SUBTYPE_SH7724",
        feature = "CONFIG_CPU_SUBTYPE_SH7730",
        feature = "CONFIG_CPU_SUBTYPE_SH7786"
    )
))]
pub const CHCR_TS_LOW_SHIFT: u32 = 3;
#[cfg(all(
    feature = "CONFIG_CPU_SH4A",
    any(
        feature = "CONFIG_CPU_SUBTYPE_SH7722",
        feature = "CONFIG_CPU_SUBTYPE_SH7723",
        feature = "CONFIG_CPU_SUBTYPE_SH7724",
        feature = "CONFIG_CPU_SUBTYPE_SH7730",
        feature = "CONFIG_CPU_SUBTYPE_SH7786"
    )
))]
pub const CHCR_TS_HIGH_MASK: u32 = 0x00300000;
#[cfg(all(
    feature = "CONFIG_CPU_SH4A",
    any(
        feature = "CONFIG_CPU_SUBTYPE_SH7722",
        feature = "CONFIG_CPU_SUBTYPE_SH7723",
        feature = "CONFIG_CPU_SUBTYPE_SH7724",
        feature = "CONFIG_CPU_SUBTYPE_SH7730",
        feature = "CONFIG_CPU_SUBTYPE_SH7786"
    )
))]
pub const CHCR_TS_HIGH_SHIFT: u32 = 20 - 2;

#[cfg(all(
    feature = "CONFIG_CPU_SH4A",
    any(
        feature = "CONFIG_CPU_SUBTYPE_SH7757",
        feature = "CONFIG_CPU_SUBTYPE_SH7763",
        feature = "CONFIG_CPU_SUBTYPE_SH7780",
        feature = "CONFIG_CPU_SUBTYPE_SH7785"
    )
))]
pub const CHCR_TS_LOW_MASK: u32 = 0x00000018;
#[cfg(all(
    feature = "CONFIG_CPU_SH4A",
    any(
        feature = "CONFIG_CPU_SUBTYPE_SH7757",
        feature = "CONFIG_CPU_SUBTYPE_SH7763",
        feature = "CONFIG_CPU_SUBTYPE_SH7780",
        feature = "CONFIG_CPU_SUBTYPE_SH7785"
    )
))]
pub const CHCR_TS_LOW_SHIFT: u32 = 3;
#[cfg(all(
    feature = "CONFIG_CPU_SH4A",
    any(
        feature = "CONFIG_CPU_SUBTYPE_SH7757",
        feature = "CONFIG_CPU_SUBTYPE_SH7763",
        feature = "CONFIG_CPU_SUBTYPE_SH7780",
        feature = "CONFIG_CPU_SUBTYPE_SH7785"
    )
))]
pub const CHCR_TS_HIGH_MASK: u32 = 0x00100000;
#[cfg(all(
    feature = "CONFIG_CPU_SH4A",
    any(
        feature = "CONFIG_CPU_SUBTYPE_SH7757",
        feature = "CONFIG_CPU_SUBTYPE_SH7763",
        feature = "CONFIG_CPU_SUBTYPE_SH7780",
        feature = "CONFIG_CPU_SUBTYPE_SH7785"
    )
))]
pub const CHCR_TS_HIGH_SHIFT: u32 = 20 - 2;

#[cfg(feature = "CONFIG_CPU_SH4A")]
pub const XMIT_SZ_8BIT: usize = 0;
#[cfg(feature = "CONFIG_CPU_SH4A")]
pub const XMIT_SZ_16BIT: usize = 1;
#[cfg(feature = "CONFIG_CPU_SH4A")]
pub const XMIT_SZ_32BIT: usize = 2;
#[cfg(feature = "CONFIG_CPU_SH4A")]
pub const XMIT_SZ_64BIT: usize = 7;
#[cfg(feature = "CONFIG_CPU_SH4A")]
pub const XMIT_SZ_128BIT: usize = 3;
#[cfg(feature = "CONFIG_CPU_SH4A")]
pub const XMIT_SZ_256BIT: usize = 4;
#[cfg(feature = "CONFIG_CPU_SH4A")]
pub const XMIT_SZ_128BIT_BLK: usize = 0xb;
#[cfg(feature = "CONFIG_CPU_SH4A")]
pub const XMIT_SZ_256BIT_BLK: usize = 0xc;

#[cfg(feature = "CONFIG_CPU_SH4A")]
pub const TS_SHIFT: [u32; 13] = [0, 1, 2, 4, 3, 5, 0, 4, 5, 0, 0, 4, 5];

#[cfg(feature = "CONFIG_CPU_SH4A")]
#[inline]
pub const fn TS_INDEX2VAL(i: u32) -> u32 {
    ((i & 3) << CHCR_TS_LOW_SHIFT) | ((i & 0xc) << CHCR_TS_HIGH_SHIFT)
}

#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
pub const DMAOR_INIT: u32 = 0x8000 | DMAOR_DME;
#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
pub const CHCR_TS_LOW_MASK: u32 = 0x70;
#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
pub const CHCR_TS_LOW_SHIFT: u32 = 4;
#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
pub const CHCR_TS_HIGH_MASK: u32 = 0;
#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
pub const CHCR_TS_HIGH_SHIFT: u32 = 0;

#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
pub const XMIT_SZ_8BIT: usize = 1;
#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
pub const XMIT_SZ_16BIT: usize = 2;
#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
pub const XMIT_SZ_32BIT: usize = 3;
#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
pub const XMIT_SZ_64BIT: usize = 0;
#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
pub const XMIT_SZ_256BIT: usize = 4;

#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
pub const TS_SHIFT: [u32; 5] = [3, 0, 1, 2, 5];

#[cfg(not(feature = "CONFIG_CPU_SH4A"))]
#[inline]
pub const fn TS_INDEX2VAL(i: u32) -> u32 {
    (i & 7) << CHCR_TS_LOW_SHIFT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
