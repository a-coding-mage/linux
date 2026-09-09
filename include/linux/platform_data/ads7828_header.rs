/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TI ADS7828 A/D Converter platform data definition
 *
 * Copyright (c) 2012 Savoir-faire Linux Inc.
 *          Vivien Didelot <vivien.didelot@savoirfairelinux.com>
 *
 * For further information, see the Documentation/hwmon/ads7828.rst file.
 */

/**
 * struct ads7828_platform_data - optional ADS7828 connectivity info
 * @diff_input:        Differential input mode.
 * @ext_vref:          Use an external voltage reference.
 * @vref_mv:           Voltage reference value, if external.
 */
#[repr(C)]
pub struct ads7828_platform_data {
    pub diff_input: bool,
    pub ext_vref: bool,
    pub vref_mv: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
