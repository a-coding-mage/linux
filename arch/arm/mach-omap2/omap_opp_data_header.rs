/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP SoC specific OPP Data helpers
 *
 * Copyright (C) 2009-2010 Texas Instruments Incorporated - https://www.ti.com/
 *\tNishanth Menon
 *\tKevin Hilman
 * Copyright (C) 2010 Nokia Corporation.
 *      Eduardo Valentin
 */

// C dependency: "omap_hwmod.h"
// C dependency: "voltage.h"

/*
 * *BIG FAT WARNING*:
 * USE the following ONLY in opp data initialization common to an SoC.
 * DO NOT USE these in board files/pm core etc.
 */

/**
 * struct omap_opp_def - OMAP OPP Definition
 * @hwmod_name:\tName of the hwmod for this domain
 * @freq:\tFrequency in hertz corresponding to this OPP
 * @u_volt:\tNominal voltage in microvolts corresponding to this OPP
 * @default_available:\tTrue/false - is this OPP available by default
 *
 * OMAP SOCs have a standard set of tuples consisting of frequency and voltage
 * pairs that the device will support per voltage domain. This is called
 * Operating Points or OPP. The actual definitions of OMAP Operating Points
 * varies over silicon within the same family of devices. For a specific
 * domain, you can have a set of {frequency, voltage} pairs and this is denoted
 * by an array of omap_opp_def. As the kernel boots and more information is
 * available, a set of these are activated based on the precise nature of
 * device the kernel boots up on. It is interesting to remember that each IP
 * which belongs to a voltage domain may define their own set of OPPs on top
 * of this - but this is handled by the appropriate driver.
 */
#[repr(C)]
pub struct omap_opp_def {
    pub hwmod_name: *mut core::ffi::c_char,
    pub freq: core::ffi::c_ulong,
    pub u_volt: core::ffi::c_ulong,
    pub default_available: bool,
}

/*
 * Initialization wrapper used to define an OPP for OMAP variants.
 */
#[macro_export]
macro_rules! OPP_INITIALIZER {
    ($hwmod_name:expr, $enabled:expr, $freq:expr, $uv:expr) => {
        omap_opp_def {
            hwmod_name: $hwmod_name,
            default_available: $enabled,
            freq: $freq,
            u_volt: $uv,
        }
    };
}

/*
 * Initialization wrapper used to define SmartReflex process data
 * XXX Is this needed?  Just use C99 initializers in data files?
 */
#[macro_export]
macro_rules! VOLT_DATA_DEFINE {
    ($v_nom:expr, $efuse_offs:expr, $errminlimit:expr, $errgain:expr) => {
        omap_volt_data {
            volt_nominal: $v_nom,
            sr_efuse_offs: $efuse_offs,
            sr_errminlimit: $errminlimit,
            vp_errgain: $errgain,
        }
    };
}

extern "C" {
    pub static mut omap34xx_vddmpu_volt_data: [omap_volt_data; 0];
    pub static mut omap34xx_vddcore_volt_data: [omap_volt_data; 0];
    pub static mut omap36xx_vddmpu_volt_data: [omap_volt_data; 0];
    pub static mut omap36xx_vddcore_volt_data: [omap_volt_data; 0];

    pub static mut omap443x_vdd_mpu_volt_data: [omap_volt_data; 0];
    pub static mut omap443x_vdd_iva_volt_data: [omap_volt_data; 0];
    pub static mut omap443x_vdd_core_volt_data: [omap_volt_data; 0];
    pub static mut omap446x_vdd_mpu_volt_data: [omap_volt_data; 0];
    pub static mut omap446x_vdd_iva_volt_data: [omap_volt_data; 0];
    pub static mut omap446x_vdd_core_volt_data: [omap_volt_data; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
