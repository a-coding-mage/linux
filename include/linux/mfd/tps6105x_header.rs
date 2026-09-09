/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2011 ST-Ericsson SA
 * Written on behalf of Linaro for ST-Ericsson
 *
 * Author: Linus Walleij <linus.walleij@linaro.org>
 */

// Dependencies supplied by the corresponding Linux/Rust bindings are intentionally
// referenced here rather than implemented in this translation.

/*
 * Register definitions to all subdrivers
 */
pub const TPS6105X_REG_0: u32 = 0x00;
pub const TPS6105X_REG0_MODE_SHIFT: u32 = 6;
pub const TPS6105X_REG0_MODE_MASK: u32 = 0x03 << 6;
/* These defines for both reg0 and reg1 */
pub const TPS6105X_REG0_MODE_SHUTDOWN: u32 = 0x00;
pub const TPS6105X_REG0_MODE_TORCH: u32 = 0x01;
pub const TPS6105X_REG0_MODE_TORCH_FLASH: u32 = 0x02;
pub const TPS6105X_REG0_MODE_VOLTAGE: u32 = 0x03;
pub const TPS6105X_REG0_VOLTAGE_SHIFT: u32 = 4;
pub const TPS6105X_REG0_VOLTAGE_MASK: u32 = 3 << 4;
pub const TPS6105X_REG0_VOLTAGE_450: u32 = 0;
pub const TPS6105X_REG0_VOLTAGE_500: u32 = 1;
pub const TPS6105X_REG0_VOLTAGE_525: u32 = 2;
pub const TPS6105X_REG0_VOLTAGE_500_2: u32 = 3;
pub const TPS6105X_REG0_DIMMING_SHIFT: u32 = 3;
pub const TPS6105X_REG0_TORCHC_SHIFT: u32 = 0;
pub const TPS6105X_REG0_TORCHC_MASK: u32 = 7 << 0;
pub const TPS6105X_REG0_TORCHC_0: u32 = 0x00;
pub const TPS6105X_REG0_TORCHC_50: u32 = 0x01;
pub const TPS6105X_REG0_TORCHC_75: u32 = 0x02;
pub const TPS6105X_REG0_TORCHC_100: u32 = 0x03;
pub const TPS6105X_REG0_TORCHC_150: u32 = 0x04;
pub const TPS6105X_REG0_TORCHC_200: u32 = 0x05;
pub const TPS6105X_REG0_TORCHC_250_400: u32 = 0x06;
pub const TPS6105X_REG0_TORCHC_250_500: u32 = 0x07;
pub const TPS6105X_REG_1: u32 = 0x01;
pub const TPS6105X_REG1_MODE_SHIFT: u32 = 6;
pub const TPS6105X_REG1_MODE_MASK: u32 = 0x03 << 6;
pub const TPS6105X_REG1_MODE_SHUTDOWN: u32 = 0x00;
pub const TPS6105X_REG1_MODE_TORCH: u32 = 0x01;
pub const TPS6105X_REG1_MODE_TORCH_FLASH: u32 = 0x02;
pub const TPS6105X_REG1_MODE_VOLTAGE: u32 = 0x03;
pub const TPS6105X_REG_2: u32 = 0x02;
pub const TPS6105X_REG_3: u32 = 0x03;

/**
 * enum tps6105x_mode - desired mode for the TPS6105x
 * @TPS6105X_MODE_SHUTDOWN: this instance is inactive, not used for anything
 * @TPS6105X_MODE_TORCH: this instance is used as a LED, usually a while
 *	LED, for example as backlight or flashlight. If this is set, the
 *	TPS6105X will register to the LED framework
 * @TPS6105X_MODE_TORCH_FLASH: this instance is used as a flashgun, usually
 *	in a camera
 * @TPS6105X_MODE_VOLTAGE: this instance is used as a voltage regulator and
 *	will register to the regulator framework
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tps6105x_mode {
    TPS6105X_MODE_SHUTDOWN,
    TPS6105X_MODE_TORCH,
    TPS6105X_MODE_TORCH_FLASH,
    TPS6105X_MODE_VOLTAGE,
}

/**
 * struct tps6105x_platform_data - TPS61905x platform data
 * @mode: what mode this instance shall be operated in,
 *	this is not selectable at runtime
 * @regulator_data: initialization data for the voltage
 *	regulator if used as a voltage source
 */
#[repr(C)]
pub struct tps6105x_platform_data {
    pub mode: tps6105x_mode,
    pub regulator_data: *mut regulator_init_data,
}

/**
 * struct tps6105x - state holder for the TPS6105x drivers
 * @pdata: associated platform data
 * @client: corresponding I2C client
 * @regulator: regulator device if used in voltage mode
 * @regmap: used for i2c communcation on accessing registers
 */
#[repr(C)]
pub struct tps6105x {
    pub pdata: *mut tps6105x_platform_data,
    pub client: *mut i2c_client,
    pub regulator: *mut regulator_dev,
    pub regmap: *mut regmap,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
