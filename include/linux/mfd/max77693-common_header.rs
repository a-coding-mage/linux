/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Common data shared between Maxim 77693, 77705 and 77843 drivers
 *
 * Copyright (C) 2015 Samsung Electronics
 */

/* C header guard: __LINUX_MFD_MAX77693_COMMON_H */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum max77693_types {
    TYPE_MAX77693_UNKNOWN,
    TYPE_MAX77693,
    TYPE_MAX77705,
    TYPE_MAX77843,

    TYPE_MAX77693_NUM,
}

/*
 * Shared also with max77843.
 */
#[repr(C)]
pub struct max77693_dev {
    pub dev: *mut device,
    pub i2c: *mut i2c_client, // 0xCC , PMIC, Charger, Flash LED
    pub i2c_muic: *mut i2c_client, // 0x4A , MUIC
    pub i2c_haptic: *mut i2c_client, // MAX77693: 0x90 , Haptic
    pub i2c_chg: *mut i2c_client, // MAX77843: 0xD2, Charger

    pub type_: max77693_types,

    pub regmap: *mut regmap,
    pub regmap_muic: *mut regmap,
    pub regmap_haptic: *mut regmap, // Only MAX77693
    pub regmap_chg: *mut regmap, // Only MAX77843
    pub regmap_leds: *mut regmap, // Only MAX77705

    pub irq_data_led: *mut regmap_irq_chip_data,
    pub irq_data_topsys: *mut regmap_irq_chip_data,
    pub irq_data_chg: *mut regmap_irq_chip_data, // Only MAX77693
    pub irq_data_muic: *mut regmap_irq_chip_data,

    pub irq: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
