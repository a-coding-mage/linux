/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ALSA SoC Audio Layer - Rockchip SPDIF transceiver driver
 *
 * Copyright (c) 2015-2026 Collabora Ltd.
 * Author: Sjoerd Simons <sjoerd.simons@collabora.co.uk>
 */

/*
 * Original C header guard: _ROCKCHIP_SPDIF_H
 */

const fn bit(n: u32) -> u32 {
    1u32 << n
}

const fn genmask(high: u32, low: u32) -> u32 {
    u32::MAX.wrapping_shl(low) & u32::MAX.wrapping_shr(31u32.wrapping_sub(high))
}

const fn field_prep(mask: u32, val: u32) -> u32 {
    (val << mask.trailing_zeros()) & mask
}

/*
 * CFGR
 * transfer configuration register
*/
pub const SPDIF_CFGR_CLK_DIV_MASK: u32 = genmask(23, 16);
pub const fn SPDIF_CFGR_CLK_DIV(x: u32) -> u32 {
    field_prep(SPDIF_CFGR_CLK_DIV_MASK, x.wrapping_sub(1))
}

pub const SPDIF_CFGR_CLR_MASK: u32 = bit(7);
pub const SPDIF_CFGR_CLR_EN: u32 = field_prep(SPDIF_CFGR_CLR_MASK, 1);
pub const SPDIF_CFGR_CLR_DIS: u32 = field_prep(SPDIF_CFGR_CLR_MASK, 0);

pub const SPDIF_CFGR_CSE_MASK: u32 = bit(6);
pub const SPDIF_CFGR_CSE_EN: u32 = field_prep(SPDIF_CFGR_CSE_MASK, 1);
pub const SPDIF_CFGR_CSE_DIS: u32 = field_prep(SPDIF_CFGR_CSE_MASK, 0);

pub const SPDIF_CFGR_ADJ_MASK: u32 = bit(3);
pub const SPDIF_CFGR_ADJ_LEFT_J: u32 = field_prep(SPDIF_CFGR_ADJ_MASK, 1);
pub const SPDIF_CFGR_ADJ_RIGHT_J: u32 = field_prep(SPDIF_CFGR_ADJ_MASK, 0);

pub const SPDIF_CFGR_HALFWORD_MASK: u32 = bit(2);
pub const SPDIF_CFGR_HALFWORD_DISABLE: u32 = field_prep(SPDIF_CFGR_HALFWORD_MASK, 0);
pub const SPDIF_CFGR_HALFWORD_ENABLE: u32 = field_prep(SPDIF_CFGR_HALFWORD_MASK, 1);

pub const SDPIF_CFGR_VDW_MASK: u32 = genmask(1, 0);
pub const fn SPDIF_CFGR_VDW(x: u32) -> u32 {
    field_prep(SDPIF_CFGR_VDW_MASK, x)
}

pub const SPDIF_CFGR_VDW_16: u32 = SPDIF_CFGR_VDW(0x0);
pub const SPDIF_CFGR_VDW_20: u32 = SPDIF_CFGR_VDW(0x1);
pub const SPDIF_CFGR_VDW_24: u32 = SPDIF_CFGR_VDW(0x2);

/*
 * DMACR
 * DMA control register
*/
pub const SPDIF_DMACR_TDE_MASK: u32 = bit(5);
pub const SPDIF_DMACR_TDE_DISABLE: u32 = field_prep(SPDIF_DMACR_TDE_MASK, 0);
pub const SPDIF_DMACR_TDE_ENABLE: u32 = field_prep(SPDIF_DMACR_TDE_MASK, 1);

pub const SPDIF_DMACR_TDL_MASK: u32 = genmask(4, 0);
pub const fn SPDIF_DMACR_TDL(x: u32) -> u32 {
    field_prep(SPDIF_DMACR_TDL_MASK, x)
}

/*
 * XFER
 * Transfer control register
*/
pub const SPDIF_XFER_TXS_MASK: u32 = bit(0);
pub const SPDIF_XFER_TXS_STOP: u32 = field_prep(SPDIF_XFER_TXS_MASK, 0);
pub const SPDIF_XFER_TXS_START: u32 = field_prep(SPDIF_XFER_TXS_MASK, 1);

pub const SPDIF_CFGR: u32 = 0x0000;
pub const SPDIF_SDBLR: u32 = 0x0004;
pub const SPDIF_DMACR: u32 = 0x0008;
pub const SPDIF_INTCR: u32 = 0x000c;
pub const SPDIF_INTSR: u32 = 0x0010;
pub const SPDIF_XFER: u32 = 0x0018;
pub const SPDIF_SMPDR: u32 = 0x0020;
pub const fn SPDIF_VLDFRn(x: u32) -> u32 {
    0x0060 + x * 4
}
pub const fn SPDIF_USRDRn(x: u32) -> u32 {
    0x0090 + x * 4
}
pub const fn SPDIF_CHNSRn(x: u32) -> u32 {
    0x00c0 + x * 4
}
pub const SPDIF_VERSION: u32 = 0x01c0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
