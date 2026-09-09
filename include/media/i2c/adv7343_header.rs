/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ADV7343 header file
 *
 * Copyright (C) 2009 Texas Instruments Incorporated - http://www.ti.com/
 */

// C header guard: ADV7343_H

pub const ADV7343_COMPOSITE_ID: u32 = 0;
pub const ADV7343_COMPONENT_ID: u32 = 1;
pub const ADV7343_SVIDEO_ID: u32 = 2;

/**
 * struct adv7343_power_mode - power mode configuration.
 * @sleep_mode: on enable the current consumption is reduced to micro ampere
 *\tlevel. All DACs and the internal PLL circuit are disabled.
 *\tRegisters can be read from and written in sleep mode.
 * @pll_control: PLL and oversampling control. This control allows internal
 *\t\t PLL 1 circuit to be powered down and the oversampling to be
 *\t\t switched off.
 * @dac: array to configure power on/off DAC's 1..6
 *
 * Power mode register (Register 0x0), for more info refer REGISTER MAP ACCESS
 * section of datasheet[1], table 17 page no 30.
 *
 * [1] http://www.analog.com/static/imported-files/data_sheets/ADV7342_7343.pdf
 */
#[repr(C)]
pub struct adv7343_power_mode {
    pub sleep_mode: bool,
    pub pll_control: bool,
    pub dac: [u32; 6],
}

/**
 * struct adv7343_sd_config - SD Only Output Configuration.
 * @sd_dac_out: array configuring SD DAC Outputs 1 and 2
 */
#[repr(C)]
pub struct adv7343_sd_config {
    /* SD only Output Configuration */
    pub sd_dac_out: [u32; 2],
}

/**
 * struct adv7343_platform_data - Platform data values and access functions.
 * @mode_config: Configuration for power mode.
 * @sd_config: SD Only Configuration.
 */
#[repr(C)]
pub struct adv7343_platform_data {
    pub mode_config: adv7343_power_mode,
    pub sd_config: adv7343_sd_config,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
