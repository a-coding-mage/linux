/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Omnivision OV2659 CMOS Image Sensor driver
 *
 * Copyright (C) 2015 Texas Instruments, Inc.
 *
 * Benoit Parrot <bparrot@ti.com>
 * Lad, Prabhakar <prabhakar.csengg@gmail.com>
 */

/**
 * struct ov2659_platform_data - ov2659 driver platform data
 * @link_frequency: target pixel clock frequency
 */
#[repr(C)]
pub struct ov2659_platform_data {
    pub link_frequency: i64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
