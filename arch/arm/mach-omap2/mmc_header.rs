/* SPDX-License-Identifier: GPL-2.0 */

pub const OMAP24XX_NR_MMC: usize = 2;
pub const OMAP2420_MMC_SIZE: usize = OMAP1_MMC_SIZE;
pub const OMAP2_MMC1_BASE: usize = 0x4809c000;

pub const OMAP4_MMC_REG_OFFSET: usize = 0x100;

#[repr(C)]
pub struct omap_hwmod {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SOC_OMAP2420")]
unsafe extern "C" {
    pub fn omap_msdi_reset(oh: *mut omap_hwmod) -> i32;
}

#[cfg(not(feature = "CONFIG_SOC_OMAP2420"))]
#[inline]
pub unsafe fn omap_msdi_reset(_oh: *mut omap_hwmod) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
