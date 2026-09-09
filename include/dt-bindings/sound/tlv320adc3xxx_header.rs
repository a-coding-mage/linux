/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Devicetree bindings definitions for tlv320adc3xxx driver.
 *
 * Copyright (C) 2021 Axis Communications AB
 */

pub const ADC3XXX_GPIO_DISABLED: u32 = 0; /* I/O buffers powered down */
pub const ADC3XXX_GPIO_INPUT: u32 = 1; /* Various non-GPIO inputs */
pub const ADC3XXX_GPIO_GPI: u32 = 2; /* General purpose input */
pub const ADC3XXX_GPIO_GPO: u32 = 3; /* General purpose output */
pub const ADC3XXX_GPIO_CLKOUT: u32 = 4; /* Source set in reg. CLKOUT_MUX */
pub const ADC3XXX_GPIO_INT1: u32 = 5; /* INT1 output */
pub const ADC3XXX_GPIO_INT2: u32 = 6; /* INT2 output */
/* value 7 is reserved */
pub const ADC3XXX_GPIO_SECONDARY_BCLK: u32 = 8; /* Codec interface secondary BCLK */
pub const ADC3XXX_GPIO_SECONDARY_WCLK: u32 = 9; /* Codec interface secondary WCLK */
pub const ADC3XXX_GPIO_ADC_MOD_CLK: u32 = 10; /* Clock output for digital mics */
/* values 11-15 reserved */

pub const ADC3XXX_MICBIAS_OFF: u32 = 0; /* Micbias pin powered off */
pub const ADC3XXX_MICBIAS_2_0V: u32 = 1; /* Micbias pin set to 2.0V */
pub const ADC3XXX_MICBIAS_2_5V: u32 = 2; /* Micbias pin set to 2.5V */
pub const ADC3XXX_MICBIAS_AVDD: u32 = 3; /* Use AVDD voltage for micbias pin */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
