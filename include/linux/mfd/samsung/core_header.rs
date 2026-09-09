/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (c) 2011 Samsung Electronics Co., Ltd
 *              http://www.samsung.com
 */

/* Macros to represent minimum voltages for LDO/BUCK */
pub const MIN_3000_MV: i32 = 3000000;
pub const MIN_2500_MV: i32 = 2500000;
pub const MIN_2000_MV: i32 = 2000000;
pub const MIN_1800_MV: i32 = 1800000;
pub const MIN_1500_MV: i32 = 1500000;
pub const MIN_1400_MV: i32 = 1400000;
pub const MIN_1000_MV: i32 = 1000000;

pub const MIN_900_MV: i32 = 900000;
pub const MIN_850_MV: i32 = 850000;
pub const MIN_800_MV: i32 = 800000;
pub const MIN_750_MV: i32 = 750000;
pub const MIN_650_MV: i32 = 650000;
pub const MIN_600_MV: i32 = 600000;
pub const MIN_500_MV: i32 = 500000;

/* Ramp delay in uV/us */
pub const RAMP_DELAY_12_MVUS: i32 = 12000;

/* Macros to represent steps for LDO/BUCK */
pub const STEP_50_MV: i32 = 50000;
pub const STEP_25_MV: i32 = 25000;
pub const STEP_12_5_MV: i32 = 12500;
pub const STEP_6_25_MV: i32 = 6250;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sec_device_type {
    S5M8767X,
    S2DOS05,
    S2MPA01,
    S2MPG10,
    S2MPG11,
    S2MPS11X,
    S2MPS13X,
    S2MPS14X,
    S2MPS15X,
    S2MPU02,
    S2MPU05,
    S2MU005,
}

/**
 * struct sec_pmic_dev - s2m/s5m master device for sub-drivers
 * @dev:        Master device of the chip
 * @pdata:      Platform data populated with data from DTS
 *              or board files
 * @regmap_pmic: Regmap associated with PMIC's I2C address
 * @i2c:        I2C client of the main driver
 * @device_type: Type of device, matches enum sec_device_type
 * @irq_base:   Base IRQ number for device, required for IRQs
 * @irq:        Generic IRQ number for device
 * @irq_data:   Runtime data structure for IRQ controller
 * @wakeup:     Whether or not this is a wakeup device
 */
#[repr(C)]
pub struct sec_pmic_dev {
    pub dev: *mut device,
    pub pdata: *mut sec_platform_data,
    pub regmap_pmic: *mut regmap,
    pub i2c: *mut i2c_client,
    pub device_type: i32,
    pub irq: i32,
}

#[repr(C)]
pub struct sec_platform_data {
    pub regulators: *mut sec_regulator_data,
    pub opmode: *mut sec_opmode_data,
    pub num_regulators: i32,
    pub buck_gpios: [i32; 3],
    pub buck_ds: [i32; 3],
    pub buck2_voltage: [u32; 8],
    pub buck2_gpiodvs: bool,
    pub buck3_voltage: [u32; 8],
    pub buck3_gpiodvs: bool,
    pub buck4_voltage: [u32; 8],
    pub buck4_gpiodvs: bool,
    pub buck_default_idx: i32,
    pub buck_ramp_delay: i32,
    pub buck2_ramp_enable: bool,
    pub buck3_ramp_enable: bool,
    pub buck4_ramp_enable: bool,
    pub buck2_init: i32,
    pub buck3_init: i32,
    pub buck4_init: i32,
    /* Whether or not manually set PWRHOLD to low during shutdown. */
    pub manual_poweroff: bool,
    /* Disable the WRSTBI (buck voltage warm reset) when probing? */
    pub disable_wrstbi: bool,
}

/**
 * sec_regulator_data - regulator data
 * @id: regulator id
 * @initdata: regulator init data (contraints, supplies, ...)
 */
#[repr(C)]
pub struct sec_regulator_data {
    pub id: i32,
    pub initdata: *mut regulator_init_data,
    pub reg_node: *mut device_node,
    pub ext_control_gpiod: *mut gpio_desc,
}

/*
 * sec_opmode_data - regulator operation mode data
 * @id: regulator id
 * @mode: regulator operation mode
 */
#[repr(C)]
pub struct sec_opmode_data {
    pub id: i32,
    pub mode: u32,
}

/*
 * samsung regulator operation mode
 * SEC_OPMODE_OFF      Regulator always OFF
 * SEC_OPMODE_ON       Regulator always ON
 * SEC_OPMODE_LOWPOWER Regulator is on in low-power mode
 * SEC_OPMODE_SUSPEND   Regulator is changed by PWREN pin
 *                      If PWREN is high, regulator is on
 *                      If PWREN is low, regulator is off
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sec_opmode {
    SEC_OPMODE_OFF,
    SEC_OPMODE_ON,
    SEC_OPMODE_LOWPOWER,
    SEC_OPMODE_SUSPEND,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
