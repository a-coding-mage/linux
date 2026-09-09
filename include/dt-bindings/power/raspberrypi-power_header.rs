/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright © 2015 Broadcom
 */

/* These power domain indices are the firmware interface's indices
 * minus one.
 */
pub const RPI_POWER_DOMAIN_I2C0: u32 = 0;
pub const RPI_POWER_DOMAIN_I2C1: u32 = 1;
pub const RPI_POWER_DOMAIN_I2C2: u32 = 2;
pub const RPI_POWER_DOMAIN_VIDEO_SCALER: u32 = 3;
pub const RPI_POWER_DOMAIN_VPU1: u32 = 4;
pub const RPI_POWER_DOMAIN_HDMI: u32 = 5;
pub const RPI_POWER_DOMAIN_USB: u32 = 6;
pub const RPI_POWER_DOMAIN_VEC: u32 = 7;
pub const RPI_POWER_DOMAIN_JPEG: u32 = 8;
pub const RPI_POWER_DOMAIN_H264: u32 = 9;
pub const RPI_POWER_DOMAIN_V3D: u32 = 10;
pub const RPI_POWER_DOMAIN_ISP: u32 = 11;
pub const RPI_POWER_DOMAIN_UNICAM0: u32 = 12;
pub const RPI_POWER_DOMAIN_UNICAM1: u32 = 13;
pub const RPI_POWER_DOMAIN_CCP2RX: u32 = 14;
pub const RPI_POWER_DOMAIN_CSI2: u32 = 15;
pub const RPI_POWER_DOMAIN_CPI: u32 = 16;
pub const RPI_POWER_DOMAIN_DSI0: u32 = 17;
pub const RPI_POWER_DOMAIN_DSI1: u32 = 18;
pub const RPI_POWER_DOMAIN_TRANSPOSER: u32 = 19;
pub const RPI_POWER_DOMAIN_CCP2TX: u32 = 20;
pub const RPI_POWER_DOMAIN_CDP: u32 = 21;
pub const RPI_POWER_DOMAIN_ARM: u32 = 22;

pub const RPI_POWER_DOMAIN_COUNT: u32 = 23;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
