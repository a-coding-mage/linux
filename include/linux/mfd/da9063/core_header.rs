/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Definitions for DA9063 MFD driver
 *
 * Copyright 2012 Dialog Semiconductor Ltd.
 *
 * Author: Michal Hajduk, Dialog Semiconductor
 * Author: Krystian Garbaciak, Dialog Semiconductor
 */

/* Dependencies supplied by the surrounding kernel translation. */

/* DA9063 modules */
pub const DA9063_DRVNAME_CORE: &str = "da9063-core";
pub const DA9063_DRVNAME_REGULATORS: &str = "da9063-regulators";
pub const DA9063_DRVNAME_LEDS: &str = "da9063-leds";
pub const DA9063_DRVNAME_WATCHDOG: &str = "da9063-watchdog";
pub const DA9063_DRVNAME_HWMON: &str = "da9063-hwmon";
pub const DA9063_DRVNAME_ONKEY: &str = "da9063-onkey";
pub const DA9063_DRVNAME_RTC: &str = "da9063-rtc";
pub const DA9063_DRVNAME_VIBRATION: &str = "da9063-vibration";

pub const PMIC_CHIP_ID_DA9063: u32 = 0x61;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da9063_type {
    PMIC_TYPE_DA9063 = 0,
    PMIC_TYPE_DA9063L,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da9063_variant_codes {
    PMIC_DA9063_AD = 0x3,
    PMIC_DA9063_BB = 0x5,
    PMIC_DA9063_CA = 0x6,
    PMIC_DA9063_DA = 0x7,
    PMIC_DA9063_EA = 0x8,
}

/* Interrupts */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da9063_irqs {
    DA9063_IRQ_ONKEY = 0,
    DA9063_IRQ_ALARM,
    DA9063_IRQ_TICK,
    DA9063_IRQ_ADC_RDY,
    DA9063_IRQ_SEQ_RDY,
    DA9063_IRQ_WAKE,
    DA9063_IRQ_TEMP,
    DA9063_IRQ_COMP_1V2,
    DA9063_IRQ_LDO_LIM,
    DA9063_IRQ_REG_UVOV,
    DA9063_IRQ_DVC_RDY,
    DA9063_IRQ_VDD_MON,
    DA9063_IRQ_WARN,
    DA9063_IRQ_GPI0,
    DA9063_IRQ_GPI1,
    DA9063_IRQ_GPI2,
    DA9063_IRQ_GPI3,
    DA9063_IRQ_GPI4,
    DA9063_IRQ_GPI5,
    DA9063_IRQ_GPI6,
    DA9063_IRQ_GPI7,
    DA9063_IRQ_GPI8,
    DA9063_IRQ_GPI9,
    DA9063_IRQ_GPI10,
    DA9063_IRQ_GPI11,
    DA9063_IRQ_GPI12,
    DA9063_IRQ_GPI13,
    DA9063_IRQ_GPI14,
    DA9063_IRQ_GPI15,
}

#[repr(C)]
pub struct da9063 {
    /* Device */
    pub dev: *mut device,
    pub type_: da9063_type,
    pub variant_code: u8,
    pub flags: u32,
    pub use_sw_pm: bool,

    /* Control interface */
    pub regmap: *mut regmap,

    /* Interrupts */
    pub chip_irq: i32,
    pub irq_base: u32,
    pub regmap_irq: *mut regmap_irq_chip_data,
}

extern "C" {
    pub fn da9063_device_init(da9063: *mut da9063, irq: u32) -> i32;
    pub fn da9063_irq_init(da9063: *mut da9063) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
