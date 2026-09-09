/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2011 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 *
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *	http://armlinux.simtec.co.uk/
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * S3C Platform - SDHCI (HSMMC) platform data definitions
 */

// C header guard: __PLAT_S3C_SDHCI_H
// C dependencies: linux/platform_data/mmc-sdhci-s3c.h and devs.h

extern "C" {
    pub fn s3c_sdhci_set_platdata(
        pd: *mut s3c_sdhci_platdata,
        set: *mut s3c_sdhci_platdata,
    );

    /**
     * s3c_sdhci0_set_platdata - Set platform data for S3C SDHCI device.
     * @pd: Platform data to register to device.
     *
     * Register the given platform data for use withe S3C SDHCI device.
     * The call will copy the platform data, so the board definitions can
     * make the structure itself __initdata.
     */
    pub fn s3c_sdhci0_set_platdata(pd: *mut s3c_sdhci_platdata);
    pub fn s3c_sdhci1_set_platdata(pd: *mut s3c_sdhci_platdata);
    pub fn s3c_sdhci2_set_platdata(pd: *mut s3c_sdhci_platdata);
    pub fn s3c_sdhci3_set_platdata(pd: *mut s3c_sdhci_platdata);

    /* Default platform data, exported so that per-cpu initialisation can
     * set the correct one when there are more than one cpu type selected.
     */
    pub static mut s3c_hsmmc0_def_platdata: s3c_sdhci_platdata;
    pub static mut s3c_hsmmc1_def_platdata: s3c_sdhci_platdata;
    pub static mut s3c_hsmmc2_def_platdata: s3c_sdhci_platdata;
    pub static mut s3c_hsmmc3_def_platdata: s3c_sdhci_platdata;

    /* Helper function availability */
    pub fn s3c64xx_setup_sdhci0_cfg_gpio(dev: *mut platform_device, w: ::core::ffi::c_int);
    pub fn s3c64xx_setup_sdhci1_cfg_gpio(dev: *mut platform_device, w: ::core::ffi::c_int);
    pub fn s3c64xx_setup_sdhci2_cfg_gpio(dev: *mut platform_device, w: ::core::ffi::c_int);
}

// S3C64XX SDHCI setup. CONFIG_S3C64XX_SETUP_SDHCI and the CONFIG_S3C_DEV_HSMMC*
// conditions are build-time C configuration options preserved below as cfgs.
#[cfg(CONFIG_S3C64XX_SETUP_SDHCI)]
pub unsafe fn s3c6400_default_sdhci0() {
    #[cfg(CONFIG_S3C_DEV_HSMMC)]
    { s3c_hsmmc0_def_platdata.cfg_gpio = Some(s3c64xx_setup_sdhci0_cfg_gpio); }
}

#[cfg(CONFIG_S3C64XX_SETUP_SDHCI)]
pub unsafe fn s3c6400_default_sdhci1() {
    #[cfg(CONFIG_S3C_DEV_HSMMC1)]
    { s3c_hsmmc1_def_platdata.cfg_gpio = Some(s3c64xx_setup_sdhci1_cfg_gpio); }
}

#[cfg(CONFIG_S3C64XX_SETUP_SDHCI)]
pub unsafe fn s3c6400_default_sdhci2() {
    #[cfg(CONFIG_S3C_DEV_HSMMC2)]
    { s3c_hsmmc2_def_platdata.cfg_gpio = Some(s3c64xx_setup_sdhci2_cfg_gpio); }
}

#[cfg(CONFIG_S3C64XX_SETUP_SDHCI)]
pub unsafe fn s3c6410_default_sdhci0() {
    #[cfg(CONFIG_S3C_DEV_HSMMC)]
    { s3c_hsmmc0_def_platdata.cfg_gpio = Some(s3c64xx_setup_sdhci0_cfg_gpio); }
}

#[cfg(CONFIG_S3C64XX_SETUP_SDHCI)]
pub unsafe fn s3c6410_default_sdhci1() {
    #[cfg(CONFIG_S3C_DEV_HSMMC1)]
    { s3c_hsmmc1_def_platdata.cfg_gpio = Some(s3c64xx_setup_sdhci1_cfg_gpio); }
}

#[cfg(CONFIG_S3C64XX_SETUP_SDHCI)]
pub unsafe fn s3c6410_default_sdhci2() {
    #[cfg(CONFIG_S3C_DEV_HSMMC2)]
    { s3c_hsmmc2_def_platdata.cfg_gpio = Some(s3c64xx_setup_sdhci2_cfg_gpio); }
}

#[cfg(not(CONFIG_S3C64XX_SETUP_SDHCI))]
pub unsafe fn s3c6410_default_sdhci0() {}
#[cfg(not(CONFIG_S3C64XX_SETUP_SDHCI))]
pub unsafe fn s3c6410_default_sdhci1() {}
#[cfg(not(CONFIG_S3C64XX_SETUP_SDHCI))]
pub unsafe fn s3c6410_default_sdhci2() {}
#[cfg(not(CONFIG_S3C64XX_SETUP_SDHCI))]
pub unsafe fn s3c6400_default_sdhci0() {}
#[cfg(not(CONFIG_S3C64XX_SETUP_SDHCI))]
pub unsafe fn s3c6400_default_sdhci1() {}
#[cfg(not(CONFIG_S3C64XX_SETUP_SDHCI))]
pub unsafe fn s3c6400_default_sdhci2() {}

pub unsafe fn s3c_sdhci_setname(id: ::core::ffi::c_int, name: *mut ::core::ffi::c_char) {
    match id {
        #[cfg(CONFIG_S3C_DEV_HSMMC)]
        0 => { s3c_device_hsmmc0.name = name; }
        #[cfg(CONFIG_S3C_DEV_HSMMC1)]
        1 => { s3c_device_hsmmc1.name = name; }
        #[cfg(CONFIG_S3C_DEV_HSMMC2)]
        2 => { s3c_device_hsmmc2.name = name; }
        #[cfg(CONFIG_S3C_DEV_HSMMC3)]
        3 => { s3c_device_hsmmc3.name = name; }
        _ => {}
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
