/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * AD7887 SPI ADC driver
 *
 * Copyright 2010 Analog Devices Inc.
 */

/**
 * struct ad7887_platform_data - AD7887 ADC driver platform data
 * @en_dual: Whether to use dual channel mode. If set to true AIN1 becomes the
 *\tsecond input channel, and Vref is internally connected to Vdd. If set to
 *\tfalse the device is used in single channel mode and AIN1/Vref is used as
 *\tVREF input.
 */
#[repr(C)]
pub struct ad7887_platform_data {
    pub en_dual: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
