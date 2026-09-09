/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * DA9055 ALSA Soc codec driver
 *
 * Copyright (c) 2012 Dialog Semiconductor
 *
 * Tested on (Samsung SMDK6410 board + DA9055 EVB) using I2S and I2C
 * Written by David Chen <david.chen@diasemi.com> and
 * Ashish Chavan <ashish.chavan@kpitcummins.com>
 */

#[repr(C)]
pub enum da9055_micbias_voltage {
    DA9055_MICBIAS_1_6V = 0,
    DA9055_MICBIAS_1_8V = 1,
    DA9055_MICBIAS_2_1V = 2,
    DA9055_MICBIAS_2_2V = 3,
}

#[repr(C)]
pub struct da9055_platform_data {
    /* Selects which of the two MicBias pins acts as the bias source */
    pub micbias_source: bool,
    /* Selects the micbias voltage */
    pub micbias: da9055_micbias_voltage,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
