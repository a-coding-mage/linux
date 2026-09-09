/* SPDX-License-Identifier: GPL-2.0 */

// Types and symbols from the Linux MMC host and OMAP platform-data headers
// are supplied by the surrounding translation.

pub const OMAP15XX_NR_MMC: i32 = 1;
pub const OMAP16XX_NR_MMC: i32 = 2;
pub const OMAP1_MMC_SIZE: u32 = 0x080;
pub const OMAP1_MMC1_BASE: u32 = 0xfffb7800;
pub const OMAP1_MMC2_BASE: u32 = 0xfffb7c00; // omap16xx only

// Equivalent to the source build-time condition IS_ENABLED(CONFIG_MMC_OMAP).
#[cfg(feature = "CONFIG_MMC_OMAP")]
extern "C" {
    pub fn omap1_init_mmc(
        mmc_data: *mut *mut omap_mmc_platform_data,
        nr_controllers: i32,
    );
}

#[cfg(not(feature = "CONFIG_MMC_OMAP"))]
#[inline]
pub unsafe fn omap1_init_mmc(
    _mmc_data: *mut *mut omap_mmc_platform_data,
    _nr_controllers: i32,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
