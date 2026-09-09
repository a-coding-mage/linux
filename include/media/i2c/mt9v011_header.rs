/* SPDX-License-Identifier: GPL-2.0-only */
/* mt9v011 sensor
 *
 * Copyright (C) 2011 Hans Verkuil <hverkuil@kernel.org>
 */

#[repr(C)]
pub struct mt9v011_platform_data {
    pub xtal: u32, /* Hz */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
