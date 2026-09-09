/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * nomadik.h
 *
 * Copyright (C) ST-Ericsson SA 2013
 * Author: Gabriel Fernandez <gabriel.fernandez@st.com> for ST-Ericsson.
 */

pub const INPUT_NOPULL: i32 = 0;
pub const INPUT_PULLUP: i32 = 1;
pub const INPUT_PULLDOWN: i32 = 2;

pub const OUTPUT_LOW: i32 = 0;
pub const OUTPUT_HIGH: i32 = 1;
pub const DIR_OUTPUT: i32 = 2;

pub const SLPM_DISABLED: i32 = 0;
pub const SLPM_ENABLED: i32 = 1;

pub const SLPM_INPUT_NOPULL: i32 = 0;
pub const SLPM_INPUT_PULLUP: i32 = 1;
pub const SLPM_INPUT_PULLDOWN: i32 = 2;
pub const SLPM_DIR_INPUT: i32 = 3;

pub const SLPM_OUTPUT_LOW: i32 = 0;
pub const SLPM_OUTPUT_HIGH: i32 = 1;
pub const SLPM_DIR_OUTPUT: i32 = 2;

pub const SLPM_WAKEUP_DISABLE: i32 = 0;
pub const SLPM_WAKEUP_ENABLE: i32 = 1;

pub const GPIOMODE_DISABLED: i32 = 0;
pub const GPIOMODE_ENABLED: i32 = 1;

pub const SLPM_PDIS_DISABLED: i32 = 0;
pub const SLPM_PDIS_ENABLED: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
