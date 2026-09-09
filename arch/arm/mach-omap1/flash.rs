// SPDX-License-Identifier: GPL-2.0-only
/*
 * Flash support for OMAP1
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/io.h, linux/mtd/mtd.h, linux/mtd/map.h,
// linux/soc/ti/omap1-io.h, tc.h, and flash.h

use core::ffi::c_int;

extern "C" {
    pub fn omap_readl(address: u32) -> u32;
    pub fn omap_writel(value: u32, address: u32);
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

// Supplied by the OMAP1 IO and EMIFS declarations.
extern "C" {
    static EMIFS_CONFIG: u32;
}

// Supplied by the OMAP1 EMIFS declarations.
const OMAP_EMIFS_CONFIG_WP: u32 = 0;

pub unsafe fn omap1_set_vpp(pdev: *mut platform_device, enable: c_int) {
    let mut l: u32;

    l = omap_readl(EMIFS_CONFIG as u32);
    if enable != 0 {
        l |= OMAP_EMIFS_CONFIG_WP;
    } else {
        l &= !OMAP_EMIFS_CONFIG_WP;
    }
    omap_writel(l, EMIFS_CONFIG as u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
