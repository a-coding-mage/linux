// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP2+ DMA driver
 *
 * Copyright (C) 2003 - 2008 Nokia Corporation
 * Author: Juha Yrjölä <juha.yrjola@nokia.com>
 * DMA channel linking for 1610 by Samuel Ortiz <samuel.ortiz@nokia.com>
 * Graphics DMA and LCD DMA graphics tranformations
 * by Imre Deak <imre.deak@nokia.com>
 * OMAP2/3 support Copyright (C) 2004-2007 Texas Instruments, Inc.
 * Some functions based on earlier dma-omap.c Copyright (C) 2001 RidgeRun, Inc.
 *
 * Copyright (C) 2009 Texas Instruments
 * Added OMAP4 support - Santosh Shilimkar <santosh.shilimkar@ti.com>
 *
 * Copyright (C) 2010 Texas Instruments Incorporated - https://www.ti.com/
 * Converted DMA library into platform driver
 *	- G, Manjunath Kondaiah <manjugk@ti.com>
 */

// C headers and symbols supplied by the surrounding kernel translation unit.

extern "C" {
    fn cpu_is_omap2420() -> bool;
    fn cpu_is_omap2430() -> bool;
    fn cpu_class_is_omap2() -> bool;
    fn cpu_is_omap34xx() -> bool;
    fn omap_type() -> u32;
    fn soc_is_omap24xx() -> bool;
    fn soc_is_omap242x() -> bool;
    fn soc_is_omap34xx() -> bool;
    fn set_dma_errata(errata: u32);
}

const DMA_ERRATA_IFRAME_BUFFERING: u32 = 1 << 0;
const DMA_ERRATA_PARALLEL_CHANNELS: u32 = 1 << 1;
const DMA_ERRATA_I378: u32 = 1 << 2;
const DMA_ERRATA_I541: u32 = 1 << 3;
const DMA_ERRATA_I88: u32 = 1 << 4;
const DMA_ERRATA_3_3: u32 = 1 << 5;
const DMA_ROMCODE_BUG: u32 = 1 << 6;
const OMAP2430_REV_ES1_0: u32 = 0;
const OMAP3430_REV_ES1_0: u32 = 0;
const OMAP2_DEVICE_TYPE_GP: u32 = 0;

// Register indices, capability flags, and structure definitions are provided by linux/omap-dma.h.
extern "C" {
    static mut reg_map: [omap_dma_reg; 32];
    static mut dma_attr: omap_dma_dev_attr;
    static mut dma_plat_info: omap_system_dma_plat_info;
}

#[repr(C)]
pub struct omap_dma_reg { pub offset: u32, pub channel_offset: u32, pub flags: u32 }
#[repr(C)]
pub struct dma_slave_map { pub slave: *const u8, pub request: *const u8, pub param: u32 }
#[repr(C)]
pub struct omap_dma_dev_attr { pub dev_caps: u32, pub lch_count: u32 }
#[repr(C)]
pub struct omap_system_dma_plat_info {
    pub reg_map: *const omap_dma_reg,
    pub channel_stride: u32,
    pub dma_attr: *mut omap_dma_dev_attr,
    pub errata: u32,
    pub slave_map: *const dma_slave_map,
    pub slavecnt: usize,
}

#[allow(non_upper_case_globals)]
static REG_MAP: [omap_dma_reg; 32] = [
    omap_dma_reg { offset: 0x0000, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0078, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0008, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x000c, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0010, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0014, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0018, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x001c, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0020, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0024, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0028, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x002c, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0064, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x006c, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0070, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0074, channel_offset: 0x00, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0080, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0084, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0088, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x008c, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0090, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0094, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x0098, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x00a4, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x00a8, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x00ac, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x00b0, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x00b4, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x00b8, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x009c, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x00a0, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
    omap_dma_reg { offset: 0x00bc, channel_offset: 0x60, flags: OMAP_DMA_REG_32BIT },
];

const OMAP_DMA_REG_32BIT: u32 = 0;
const RESERVE_CHANNEL: u32 = 1 << 0;
const DMA_LINKED_LCH: u32 = 1 << 1;
const GLOBAL_PRIORITY: u32 = 1 << 2;
const IS_CSSA_32: u32 = 1 << 3;
const IS_CDSA_32: u32 = 1 << 4;
const IS_RW_PRIORITY: u32 = 1 << 5;
const HS_CHANNELS_RESERVED: u32 = 1 << 6;

static OMAP24XX_SDMA_DT_MAP: [dma_slave_map; 6] = [
    dma_slave_map { slave: b"musb-hdrc.1.auto\0".as_ptr(), request: b"dmareq0\0".as_ptr(), param: 2 },
    dma_slave_map { slave: b"musb-hdrc.1.auto\0".as_ptr(), request: b"dmareq1\0".as_ptr(), param: 3 },
    dma_slave_map { slave: b"musb-hdrc.1.auto\0".as_ptr(), request: b"dmareq2\0".as_ptr(), param: 14 },
    dma_slave_map { slave: b"musb-hdrc.1.auto\0".as_ptr(), request: b"dmareq3\0".as_ptr(), param: 15 },
    dma_slave_map { slave: b"musb-hdrc.1.auto\0".as_ptr(), request: b"dmareq4\0".as_ptr(), param: 16 },
    dma_slave_map { slave: b"musb-hdrc.1.auto\0".as_ptr(), request: b"dmareq5\0".as_ptr(), param: 64 },
];

unsafe fn configure_dma_errata() -> u32 {
    let mut errata = 0;
    if cpu_is_omap2420() || (cpu_is_omap2430() && omap_type() == OMAP2430_REV_ES1_0) {
        set_dma_errata(DMA_ERRATA_IFRAME_BUFFERING); set_dma_errata(DMA_ERRATA_PARALLEL_CHANNELS);
    }
    if cpu_class_is_omap2() { set_dma_errata(DMA_ERRATA_I378); }
    if cpu_is_omap34xx() { set_dma_errata(DMA_ERRATA_I541); }
    if omap_type() == OMAP3430_REV_ES1_0 { set_dma_errata(DMA_ERRATA_I88); }
    set_dma_errata(DMA_ERRATA_3_3);
    if cpu_is_omap34xx() && omap_type() != OMAP2_DEVICE_TYPE_GP { set_dma_errata(DMA_ROMCODE_BUG); }
    errata
}

unsafe fn omap2_system_dma_init() -> i32 {
    dma_plat_info.errata = configure_dma_errata();
    if soc_is_omap24xx() { dma_plat_info.slave_map = OMAP24XX_SDMA_DT_MAP.as_ptr(); dma_plat_info.slavecnt = OMAP24XX_SDMA_DT_MAP.len(); }
    if !soc_is_omap242x() { dma_attr.dev_caps |= IS_RW_PRIORITY; }
    if soc_is_omap34xx() && omap_type() != OMAP2_DEVICE_TYPE_GP { dma_attr.dev_caps |= HS_CHANNELS_RESERVED; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
