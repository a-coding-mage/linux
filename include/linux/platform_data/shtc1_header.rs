/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014 Sensirion AG, Switzerland
 * Author: Johannes Winkelmann <johannes.winkelmann@sensirion.com>
 */

#[repr(C)]
pub struct shtc1_platform_data {
    pub blocking_io: bool,
    pub high_precision: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
