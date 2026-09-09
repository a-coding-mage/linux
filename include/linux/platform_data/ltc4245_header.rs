/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Platform Data for LTC4245 hardware monitor chip
 *
 * Copyright (c) 2010 Ira W. Snyder <iws@ovro.caltech.edu>
 */

// Dependency intent: the C header includes <linux/types.h>.

#[repr(C)]
pub struct ltc4245_platform_data {
    pub use_extra_gpios: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
