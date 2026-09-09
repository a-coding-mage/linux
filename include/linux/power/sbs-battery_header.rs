/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Gas Gauge driver for SBS Compliant Gas Gauges
 *
 * Copyright (c) 2010, NVIDIA Corporation.
 */

// Dependencies supplied by the Linux power-supply and types interfaces are
// intentionally left external to this translation.

/**
 * struct sbs_platform_data - platform data for sbs devices
 * @i2c_retry_count:             # of times to retry on i2c IO failure
 * @poll_retry_count:            # of times to retry looking for new status after
 *                               external change notification
 */
#[repr(C)]
pub struct sbs_platform_data {
    pub i2c_retry_count: u32,
    pub poll_retry_count: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
