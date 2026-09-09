/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Rockchip General Register Files definitions
 *
 * Copyright (c) 2018, Collabora Ltd.
 * Author: Enric Balletbo i Serra <enric.balletbo@collabora.com>
 */

/* PMU GRF Registers */
pub const RK3399_PMUGRF_OS_REG2: u32 = 0x308;
pub const RK3399_PMUGRF_OS_REG2_DDRTYPE: u32 = 0xe000;
pub const RK3399_PMUGRF_OS_REG2_BW_CH0: u32 = 0x000c;
pub const RK3399_PMUGRF_OS_REG2_BW_CH1: u32 = 0x000c0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
