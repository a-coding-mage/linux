/* SPDX-License-Identifier: GPL-2.0 */
/*
 * CS42L43 core driver external data
 *
 * Copyright (C) 2022-2023 Cirrus Logic, Inc. and
 *                         Cirrus Logic International Semiconductor Ltd.
 */

// C header dependencies are supplied by the surrounding kernel translation.

pub const CS42L43_N_SUPPLIES: usize = 3;

pub struct device;
pub struct gpio_desc;
pub struct sdw_slave;
pub struct regmap;
pub struct regulator;
pub struct regmap_irq_chip;
pub struct regmap_irq_chip_data;
pub struct regmap_bulk_data;
pub struct work_struct;
pub struct completion;
pub struct mutex;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs42l43_irq_numbers {
    CS42L43_PLL_LOST_LOCK,
    CS42L43_PLL_READY,

    CS42L43_HP_STARTUP_DONE,
    CS42L43_HP_SHUTDOWN_DONE,
    CS42L43_HSDET_DONE,
    CS42L43_TIPSENSE_UNPLUG_DB,
    CS42L43_TIPSENSE_PLUG_DB,
    CS42L43_RINGSENSE_UNPLUG_DB,
    CS42L43_RINGSENSE_PLUG_DB,
    CS42L43_TIPSENSE_UNPLUG_PDET,
    CS42L43_TIPSENSE_PLUG_PDET,
    CS42L43_RINGSENSE_UNPLUG_PDET,
    CS42L43_RINGSENSE_PLUG_PDET,

    CS42L43_HS2_BIAS_SENSE,
    CS42L43_HS1_BIAS_SENSE,
    CS42L43_DC_DETECT1_FALSE,
    CS42L43_DC_DETECT1_TRUE,
    CS42L43_HSBIAS_CLAMPED,
    CS42L43_HS3_4_BIAS_SENSE,

    CS42L43_AMP2_CLK_STOP_FAULT,
    CS42L43_AMP1_CLK_STOP_FAULT,
    CS42L43_AMP2_VDDSPK_FAULT,
    CS42L43_AMP1_VDDSPK_FAULT,
    CS42L43_AMP2_SHUTDOWN_DONE,
    CS42L43_AMP1_SHUTDOWN_DONE,
    CS42L43_AMP2_STARTUP_DONE,
    CS42L43_AMP1_STARTUP_DONE,
    CS42L43_AMP2_THERM_SHDN,
    CS42L43_AMP1_THERM_SHDN,
    CS42L43_AMP2_THERM_WARN,
    CS42L43_AMP1_THERM_WARN,
    CS42L43_AMP2_SCDET,
    CS42L43_AMP1_SCDET,

    CS42L43_GPIO3_FALL,
    CS42L43_GPIO3_RISE,
    CS42L43_GPIO2_FALL,
    CS42L43_GPIO2_RISE,
    CS42L43_GPIO1_FALL,
    CS42L43_GPIO1_RISE,

    CS42L43_HP_ILIMIT,
    CS42L43_HP_LOADDET_DONE,
}

#[repr(C)]
pub struct cs42l43 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub sdw: *mut sdw_slave,

    pub vdd_p: *mut regulator,
    pub vdd_d: *mut regulator,
    pub core_supplies: [regmap_bulk_data; CS42L43_N_SUPPLIES],

    pub reset: *mut gpio_desc,

    pub irq: i32,
    pub irq_chip: regmap_irq_chip,
    pub irq_data: *mut regmap_irq_chip_data,

    pub boot_work: work_struct,
    pub device_detach: completion,
    pub firmware_download: completion,
    pub firmware_error: i32,

    pub sdw_freq: u32,
    /* Lock to gate control of the PLL and its sources. */
    pub pll_lock: mutex,

    pub sdw_pll_active: bool,
    pub hw_lock: bool,
    pub variant_id: i64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
