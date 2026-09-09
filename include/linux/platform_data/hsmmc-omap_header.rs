/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MMC definitions for OMAP2
 *
 * Copyright (C) 2006 Nokia Corporation
 */

/*
 * struct omap_hsmmc_dev_attr.flags possibilities
 *
 * OMAP_HSMMC_SUPPORTS_DUAL_VOLT: Some HSMMC controller instances can
 *    operate with either 1.8Vdc or 3.0Vdc card voltages; this flag
 *    should be set if this is the case.  See for example Section 22.5.3
 *    "MMC/SD/SDIO1 Bus Voltage Selection" of the OMAP34xx Multimedia
 *    Device Silicon Revision 3.1.x Revision ZR (July 2011) (SWPU223R).
 *
 * OMAP_HSMMC_BROKEN_MULTIBLOCK_READ: Multiple-block read transfers
 *    don't work correctly on some MMC controller instances on some
 *    OMAP3 SoCs; this flag should be set if this is the case.  See
 *    for example Advisory 2.1.1.128 "MMC: Multiple Block Read
 *    Operation Issue" in _OMAP3530/3525/3515/3503 Silicon Errata_
 *    Revision F (October 2010) (SPRZ278F).
 */
pub const OMAP_HSMMC_SUPPORTS_DUAL_VOLT: u32 = 1 << 0;
pub const OMAP_HSMMC_BROKEN_MULTIBLOCK_READ: u32 = 1 << 1;
pub const OMAP_HSMMC_SWAKEUP_MISSING: u32 = 1 << 2;

#[repr(C)]
pub struct omap_hsmmc_dev_attr {
    pub flags: u8,
}

#[repr(C)]
pub struct mmc_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct omap_hsmmc_platform_data {
    /* back-link to device */
    pub dev: *mut device,

    /* set if your board has components or wiring that limits the
     * maximum frequency on the MMC bus */
    pub max_freq: ::core::ffi::c_uint,

    /* Integrating attributes from the omap_hwmod layer */
    pub controller_flags: u8,

    /* Register offset deviation */
    pub reg_offset: u16,

    /*
     * 4/8 wires and any additional host capabilities
     * need to OR'd all capabilities (ref. linux/mmc/host.h)
     */
    pub caps: u32, /* Used for the MMC driver on 2430 and later */
    pub pm_caps: u32, /* PM capabilities of the mmc */

    /* nonremovable e.g. eMMC */
    pub nonremovable: u32,

    /* eMMC does not handle power off when not in sleep state */
    pub no_regulator_off_init: u32,

    /* we can put the features above into this variable */
    pub features: ::core::ffi::c_uint,

    /* string specifying a particular variant of hardware */
    pub version: *mut ::core::ffi::c_char,

    pub name: *const ::core::ffi::c_char,
    pub ocr_mask: u32,
}

pub const HSMMC_HAS_PBIAS: u32 = 1 << 0;
pub const HSMMC_HAS_UPDATED_RESET: u32 = 1 << 1;
pub const HSMMC_HAS_HSPE_SUPPORT: u32 = 1 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
