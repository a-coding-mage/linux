/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * This header provides constants for rt4831 backlight bindings.
 *
 * Copyright (C) 2020, Richtek Technology Corp.
 * Author: ChiYuan Huang <cy_huang@richtek.com>
 */

pub const RT4831_BLOVPLVL_17V: i32 = 0;
pub const RT4831_BLOVPLVL_21V: i32 = 1;
pub const RT4831_BLOVPLVL_25V: i32 = 2;
pub const RT4831_BLOVPLVL_29V: i32 = 3;

pub const RT4831_BLED_CH1EN: i32 = 1 << 0;
pub const RT4831_BLED_CH2EN: i32 = 1 << 1;
pub const RT4831_BLED_CH3EN: i32 = 1 << 2;
pub const RT4831_BLED_CH4EN: i32 = 1 << 3;
pub const RT4831_BLED_ALLCHEN: i32 = (1 << 4) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
