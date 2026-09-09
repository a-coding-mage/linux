/*
 * Copyright (c) 2015 NVIDIA Corporation. All rights reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sub license,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

pub const SCDC_SINK_VERSION: u8 = 0x01;
pub const SCDC_SOURCE_VERSION: u8 = 0x02;

pub const SCDC_UPDATE_0: u8 = 0x10;
pub const SCDC_READ_REQUEST_TEST: u8 = 1 << 2;
pub const SCDC_CED_UPDATE: u8 = 1 << 1;
pub const SCDC_STATUS_UPDATE: u8 = 1 << 0;

pub const SCDC_UPDATE_1: u8 = 0x11;

pub const SCDC_TMDS_CONFIG: u8 = 0x20;
pub const SCDC_TMDS_BIT_CLOCK_RATIO_BY_40: u8 = 1 << 1;
pub const SCDC_TMDS_BIT_CLOCK_RATIO_BY_10: u8 = 0 << 1;
pub const SCDC_SCRAMBLING_ENABLE: u8 = 1 << 0;

pub const SCDC_SCRAMBLER_STATUS: u8 = 0x21;
pub const SCDC_SCRAMBLING_STATUS: u8 = 1 << 0;

pub const SCDC_CONFIG_0: u8 = 0x30;
pub const SCDC_READ_REQUEST_ENABLE: u8 = 1 << 0;

pub const SCDC_STATUS_FLAGS_0: u8 = 0x40;
pub const SCDC_CH2_LOCK: u8 = 1 << 3;
pub const SCDC_CH1_LOCK: u8 = 1 << 2;
pub const SCDC_CH0_LOCK: u8 = 1 << 1;
pub const SCDC_CH_LOCK_MASK: u8 = SCDC_CH2_LOCK | SCDC_CH1_LOCK | SCDC_CH0_LOCK;
pub const SCDC_CLOCK_DETECT: u8 = 1 << 0;

pub const SCDC_STATUS_FLAGS_1: u8 = 0x41;

pub const SCDC_ERR_DET_0_L: u8 = 0x50;
pub const SCDC_ERR_DET_0_H: u8 = 0x51;
pub const SCDC_ERR_DET_1_L: u8 = 0x52;
pub const SCDC_ERR_DET_1_H: u8 = 0x53;
pub const SCDC_ERR_DET_2_L: u8 = 0x54;
pub const SCDC_ERR_DET_2_H: u8 = 0x55;
pub const SCDC_CHANNEL_VALID: u8 = 1 << 7;

pub const SCDC_ERR_DET_CHECKSUM: u8 = 0x56;

pub const SCDC_TEST_CONFIG_0: u8 = 0xc0;
pub const SCDC_TEST_READ_REQUEST: u8 = 1 << 7;
#[inline]
pub const fn SCDC_TEST_READ_REQUEST_DELAY(x: u8) -> u8 { x & 0x7f }

pub const SCDC_MANUFACTURER_IEEE_OUI: u8 = 0xd0;
pub const SCDC_MANUFACTURER_IEEE_OUI_SIZE: u8 = 3;

pub const SCDC_DEVICE_ID: u8 = 0xd3;
pub const SCDC_DEVICE_ID_SIZE: u8 = 8;

pub const SCDC_DEVICE_HARDWARE_REVISION: u8 = 0xdb;
#[inline]
pub const fn SCDC_GET_DEVICE_HARDWARE_REVISION_MAJOR(x: u8) -> u8 { (x >> 4) & 0xf }
#[inline]
pub const fn SCDC_GET_DEVICE_HARDWARE_REVISION_MINOR(x: u8) -> u8 { (x >> 0) & 0xf }

pub const SCDC_DEVICE_SOFTWARE_MAJOR_REVISION: u8 = 0xdc;
pub const SCDC_DEVICE_SOFTWARE_MINOR_REVISION: u8 = 0xdd;

pub const SCDC_MANUFACTURER_SPECIFIC: u8 = 0xde;
pub const SCDC_MANUFACTURER_SPECIFIC_SIZE: u8 = 34;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
