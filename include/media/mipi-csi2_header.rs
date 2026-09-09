/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MIPI CSI-2 Data Types
 *
 * Copyright (C) 2022 Laurent Pinchart <laurent.pinchart@ideasonboard.com>
 */

/* Short packet data types */
pub const MIPI_CSI2_DT_FS: u8 = 0x00;
pub const MIPI_CSI2_DT_FE: u8 = 0x01;
pub const MIPI_CSI2_DT_LS: u8 = 0x02;
pub const MIPI_CSI2_DT_LE: u8 = 0x03;
pub const fn MIPI_CSI2_DT_GENERIC_SHORT(n: u8) -> u8 { 0x08 + n } /* 0..7 */

/* Long packet data types */
pub const MIPI_CSI2_DT_NULL: u8 = 0x10;
pub const MIPI_CSI2_DT_BLANKING: u8 = 0x11;
pub const MIPI_CSI2_DT_EMBEDDED_8B: u8 = 0x12;
pub const fn MIPI_CSI2_DT_GENERIC_LONG(n: u8) -> u8 { 0x13 + n - 1 } /* 1..4 */
pub const MIPI_CSI2_DT_YUV420_8B: u8 = 0x18;
pub const MIPI_CSI2_DT_YUV420_10B: u8 = 0x19;
pub const MIPI_CSI2_DT_YUV420_8B_LEGACY: u8 = 0x1a;
pub const MIPI_CSI2_DT_YUV420_8B_CS: u8 = 0x1c;
pub const MIPI_CSI2_DT_YUV420_10B_CS: u8 = 0x1d;
pub const MIPI_CSI2_DT_YUV422_8B: u8 = 0x1e;
pub const MIPI_CSI2_DT_YUV422_10B: u8 = 0x1f;
pub const MIPI_CSI2_DT_RGB444: u8 = 0x20;
pub const MIPI_CSI2_DT_RGB555: u8 = 0x21;
pub const MIPI_CSI2_DT_RGB565: u8 = 0x22;
pub const MIPI_CSI2_DT_RGB666: u8 = 0x23;
pub const MIPI_CSI2_DT_RGB888: u8 = 0x24;
pub const MIPI_CSI2_DT_RAW28: u8 = 0x26;
pub const MIPI_CSI2_DT_RAW24: u8 = 0x27;
pub const MIPI_CSI2_DT_RAW6: u8 = 0x28;
pub const MIPI_CSI2_DT_RAW7: u8 = 0x29;
pub const MIPI_CSI2_DT_RAW8: u8 = 0x2a;
pub const MIPI_CSI2_DT_RAW10: u8 = 0x2b;
pub const MIPI_CSI2_DT_RAW12: u8 = 0x2c;
pub const MIPI_CSI2_DT_RAW14: u8 = 0x2d;
pub const MIPI_CSI2_DT_RAW16: u8 = 0x2e;
pub const MIPI_CSI2_DT_RAW20: u8 = 0x2f;
pub const fn MIPI_CSI2_DT_USER_DEFINED(n: u8) -> u8 { 0x30 + n } /* 0..7 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
