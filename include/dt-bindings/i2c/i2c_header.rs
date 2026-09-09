/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This header provides constants for I2C bindings
 *
 * Copyright (C) 2015 by Sang Engineering
 * Copyright (C) 2015 by Renesas Electronics Corporation
 *
 * Wolfram Sang <wsa@sang-engineering.com>
 */

// Translated from the C header guard: _DT_BINDINGS_I2C_I2C_H.

pub const I2C_TEN_BIT_ADDRESS: u32 = 1u32 << 31;
pub const I2C_OWN_SLAVE_ADDRESS: u32 = 1u32 << 30;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
