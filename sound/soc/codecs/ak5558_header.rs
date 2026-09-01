/* SPDX-License-Identifier: GPL-2.0
 *
 * Audio driver header for AK5558
 *
 * Copyright (C) 2016 Asahi Kasei Microdevices Corporation
 * Copyright 2018 NXP
 */

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    ((!0u32) << l) & ((!0u32) >> (u32::BITS - 1 - h))
}

pub const AK5558_00_POWER_MANAGEMENT1: u32 = 0x00;
pub const AK5558_01_POWER_MANAGEMENT2: u32 = 0x01;
pub const AK5558_02_CONTROL1: u32 = 0x02;
pub const AK5558_03_CONTROL2: u32 = 0x03;
pub const AK5558_04_CONTROL3: u32 = 0x04;
pub const AK5558_05_DSD: u32 = 0x05;

/* AK5558_02_CONTROL1 fields */
pub const AK5558_DIF: u32 = GENMASK(1, 1);
pub const AK5558_DIF_MSB_MODE: u32 = 0 << 1;
pub const AK5558_DIF_I2S_MODE: u32 = 1 << 1;

pub const AK5558_BITS: u32 = GENMASK(2, 2);
pub const AK5558_DIF_24BIT_MODE: u32 = 0 << 2;
pub const AK5558_DIF_32BIT_MODE: u32 = 1 << 2;

pub const AK5558_CKS: u32 = GENMASK(6, 3);
pub const AK5558_CKS_128FS_192KHZ: u32 = 0 << 3;
pub const AK5558_CKS_192FS_192KHZ: u32 = 1 << 3;
pub const AK5558_CKS_256FS_48KHZ: u32 = 2 << 3;
pub const AK5558_CKS_256FS_96KHZ: u32 = 3 << 3;
pub const AK5558_CKS_384FS_96KHZ: u32 = 4 << 3;
pub const AK5558_CKS_384FS_48KHZ: u32 = 5 << 3;
pub const AK5558_CKS_512FS_48KHZ: u32 = 6 << 3;
pub const AK5558_CKS_768FS_48KHZ: u32 = 7 << 3;
pub const AK5558_CKS_64FS_384KHZ: u32 = 8 << 3;
pub const AK5558_CKS_32FS_768KHZ: u32 = 9 << 3;
pub const AK5558_CKS_96FS_384KHZ: u32 = 10 << 3;
pub const AK5558_CKS_48FS_768KHZ: u32 = 11 << 3;
pub const AK5558_CKS_64FS_768KHZ: u32 = 12 << 3;
pub const AK5558_CKS_1024FS_16KHZ: u32 = 13 << 3;
pub const AK5558_CKS_AUTO: u32 = 15 << 3;

/* AK5558_03_CONTROL2 fields */
pub const AK5558_MODE_BITS: u32 = GENMASK(6, 5);
pub const AK5558_MODE_NORMAL: u32 = 0 << 5;
pub const AK5558_MODE_TDM128: u32 = 1 << 5;
pub const AK5558_MODE_TDM256: u32 = 2 << 5;
pub const AK5558_MODE_TDM512: u32 = 3 << 5;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
