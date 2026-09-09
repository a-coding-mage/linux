/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 Texas Instruments Inc
 *
 * Copyright 2013 Cisco Systems, Inc. and/or its affiliates.
 *
 * Contributors:
 *     Hans Verkuil <hverkuil@kernel.org>
 *     Lad, Prabhakar <prabhakar.lad@ti.com>
 *     Martin Bugge <marbugge@cisco.com>
 */

/**
 * struct ths7303_platform_data - Platform dependent data
 * @ch_1: Bias value for channel one.
 * @ch_2: Bias value for channel two.
 * @ch_3: Bias value for channel three.
 */
#[repr(C)]
pub struct ths7303_platform_data {
    pub ch_1: u8,
    pub ch_2: u8,
    pub ch_3: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
