/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Maxim MAX197 A/D Converter Driver
 *
 * Copyright (c) 2012 Savoir-faire Linux Inc.
 *          Vivien Didelot <vivien.didelot@savoirfairelinux.com>
 *
 * For further information, see the Documentation/hwmon/max197.rst file.
 */

/**
 * struct max197_platform_data - MAX197 connectivity info
 * @convert: Function used to start a conversion with control byte ctrl.
 *           It must return the raw data, or a negative error code.
 */
#[repr(C)]
pub struct max197_platform_data {
    pub convert: Option<unsafe extern "C" fn(ctrl: u8) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
