/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2021  Angelo Dureghello <angelo@kernel-space.org>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * version 2 as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

// Original C header guard: _CAN_PLATFORM_FLEXCAN_H

#[repr(C)]
pub struct flexcan_platform_data {
	pub clock_frequency: u32,
	pub clk_src: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
