/* SPDX-License-Identifier: GPL-2.0
 *
 * SH4 CPU-specific DMA definitions, used by both DMA drivers
 *
 * Copyright (C) 2010 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 */

/* SH7751/7760/7780 DMA IRQ sources */

#[cfg(CONFIG_CPU_SH4A)]
pub const DMAOR_INIT: u32 = DMAOR_DME;

#[cfg(all(CONFIG_CPU_SH4A, CONFIG_CPU_SUBTYPE_SH7343))]
pub const CHCR_TS_LOW_MASK: u32 = 0x00000018;
#[cfg(all(CONFIG_CPU_SH4A, CONFIG_CPU_SUBTYPE_SH7343))]
pub const CHCR_TS_LOW_SHIFT: u32 = 3;
#[cfg(all(CONFIG_CPU_SH4A, CONFIG_CPU_SUBTYPE_SH7343))]
pub const CHCR_TS_HIGH_MASK: u32 = 0;
#[cfg(all(CONFIG_CPU_SH4A, CONFIG_CPU_SUBTYPE_SH7343))]
pub const CHCR_TS_HIGH_SHIFT: u32 = 0;

#[cfg(all(
    CONFIG_CPU_SH4A,
    any(
        CONFIG_CPU_SUBTYPE_SH7722,
        CONFIG_CPU_SUBTYPE_SH7723,
        CONFIG_CPU_SUBTYPE_SH7724,
        CONFIG_CPU_SUBTYPE_SH7730,
        CONFIG_CPU_SUBTYPE_SH7786
    )
))]
pub const CHCR_TS_LOW_MASK: u32 = 0x00000018;
#[cfg(all(
    CONFIG_CPU_SH4A,
    any(
        CONFIG_CPU_SUBTYPE_SH7722,
        CONFIG_CPU_SUBTYPE_SH7723,
        CONFIG_CPU_SUBTYPE_SH7724,
        CONFIG_CPU_SUBTYPE_SH7730,
        CONFIG_CPU_SUBTYPE_SH7786
    )
))]
pub const CHCR_TS_LOW_SHIFT: u32 = 3;
#[cfg(all(
    CONFIG_CPU_SH4A,
    any(
        CONFIG_CPU_SUBTYPE_SH7722,
        CONFIG_CPU_SUBTYPE_SH7723,
        CONFIG_CPU_SUBTYPE_SH7724,
        CONFIG_CPU_SUBTYPE_SH7730,
        CONFIG_CPU_SUBTYPE_SH7786
    )
))]
pub const CHCR_TS_HIGH_MASK: u32 = 0x00300000;
#[cfg(all(
    CONFIG_CPU_SH4A,
    any(
        CONFIG_CPU_SUBTYPE_SH7722,
        CONFIG_CPU_SUBTYPE_SH7723,
        CONFIG_CPU_SUBTYPE_SH7724,
        CONFIG_CPU_SUBTYPE_SH7730,
        CONFIG_CPU_SUBTYPE_SH7786
    )
))]
pub const CHCR_TS_HIGH_SHIFT: u32 = 20 - 2;

#[cfg(all(
    CONFIG_CPU_SH4A,
    any(
        CONFIG_CPU_SUBTYPE_SH7757,
        CONFIG_CPU_SUBTYPE_SH7763,
        CONFIG_CPU_SUBTYPE_SH7780,
        CONFIG_CPU_SUBTYPE_SH7785
    )
))]
pub const CHCR_TS_LOW_MASK: u32 = 0x00000018;
#[cfg(all(
    CONFIG_CPU_SH4A,
    any(
        CONFIG_CPU_SUBTYPE_SH7757,
        CONFIG_CPU_SUBTYPE_SH7763,
        CONFIG_CPU_SUBTYPE_SH7780,
        CONFIG_CPU_SUBTYPE_SH7785
    )
))]
pub const CHCR_TS_LOW_SHIFT: u32 = 3;
#[cfg(all(
    CONFIG_CPU_SH4A,
    any(
        CONFIG_CPU_SUBTYPE_SH7757,
        CONFIG_CPU_SUBTYPE_SH7763,
        CONFIG_CPU_SUBTYPE_SH7780,
        CONFIG_CPU_SUBTYPE_SH7785
    )
))]
pub const CHCR_TS_HIGH_MASK: u32 = 0x00100000;
#[cfg(all(
    CONFIG_CPU_SH4A,
    any(
        CONFIG_CPU_SUBTYPE_SH7757,
        CONFIG_CPU_SUBTYPE_SH7763,
        CONFIG_CPU_SUBTYPE_SH7780,
        CONFIG_CPU_SUBTYPE_SH7785
    )
))]
pub const CHCR_TS_HIGH_SHIFT: u32 = 20 - 2;

#[cfg(CONFIG_CPU_SH4A)]
pub const XMIT_SZ_8BIT: usize = 0;
#[cfg(CONFIG_CPU_SH4A)]
pub const XMIT_SZ_16BIT: usize = 1;
#[cfg(CONFIG_CPU_SH4A)]
pub const XMIT_SZ_32BIT: usize = 2;
#[cfg(CONFIG_CPU_SH4A)]
pub const XMIT_SZ_64BIT: usize = 7;
#[cfg(CONFIG_CPU_SH4A)]
pub const XMIT_SZ_128BIT: usize = 3;
#[cfg(CONFIG_CPU_SH4A)]
pub const XMIT_SZ_256BIT: usize = 4;
#[cfg(CONFIG_CPU_SH4A)]
pub const XMIT_SZ_128BIT_BLK: usize = 0xb;
#[cfg(CONFIG_CPU_SH4A)]
pub const XMIT_SZ_256BIT_BLK: usize = 0xc;

#[cfg(CONFIG_CPU_SH4A)]
pub const TS_SHIFT: [u32; 13] = [0, 1, 2, 4, 3, 5, 0, 4, 5, 0, 0, 4, 5];

#[cfg(CONFIG_CPU_SH4A)]
#[inline]
pub const fn TS_INDEX2VAL(i: u32) -> u32 {
    ((i & 3) << CHCR_TS_LOW_SHIFT) | ((i & 0xc) << CHCR_TS_HIGH_SHIFT)
}

#[cfg(not(CONFIG_CPU_SH4A))]
pub const DMAOR_INIT: u32 = 0x8000 | DMAOR_DME;
#[cfg(not(CONFIG_CPU_SH4A))]
pub const CHCR_TS_LOW_MASK: u32 = 0x70;
#[cfg(not(CONFIG_CPU_SH4A))]
pub const CHCR_TS_LOW_SHIFT: u32 = 4;
#[cfg(not(CONFIG_CPU_SH4A))]
pub const CHCR_TS_HIGH_MASK: u32 = 0;
#[cfg(not(CONFIG_CPU_SH4A))]
pub const CHCR_TS_HIGH_SHIFT: u32 = 0;

#[cfg(not(CONFIG_CPU_SH4A))]
pub const XMIT_SZ_8BIT: usize = 1;
#[cfg(not(CONFIG_CPU_SH4A))]
pub const XMIT_SZ_16BIT: usize = 2;
#[cfg(not(CONFIG_CPU_SH4A))]
pub const XMIT_SZ_32BIT: usize = 3;
#[cfg(not(CONFIG_CPU_SH4A))]
pub const XMIT_SZ_64BIT: usize = 0;
#[cfg(not(CONFIG_CPU_SH4A))]
pub const XMIT_SZ_256BIT: usize = 4;

#[cfg(not(CONFIG_CPU_SH4A))]
pub const TS_SHIFT: [u32; 5] = [3, 0, 1, 2, 5];

#[cfg(not(CONFIG_CPU_SH4A))]
#[inline]
pub const fn TS_INDEX2VAL(i: u32) -> u32 {
    (i & 7) << CHCR_TS_LOW_SHIFT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
