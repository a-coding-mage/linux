/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018 Samsung Electronics Co., Ltd.
 *	      http://www.samsung.com/
 *
 * Exynos - CHIPID support
 */

pub const EXYNOS_CHIPID_REG_PRO_ID: u32 = 0x00;
pub const EXYNOS_REV_PART_MASK: u32 = 0xf;
pub const EXYNOS_REV_PART_SHIFT: u32 = 4;
pub const EXYNOS_MASK: u32 = 0xfffff000;

pub const EXYNOS_CHIPID_REG_PKG_ID: u32 = 0x04;
/* Bit field definitions for EXYNOS_CHIPID_REG_PKG_ID register */
pub const EXYNOS5422_IDS_OFFSET: u32 = 24;
pub const EXYNOS5422_IDS_MASK: u32 = 0xff;
pub const EXYNOS5422_USESG_OFFSET: u32 = 3;
pub const EXYNOS5422_USESG_MASK: u32 = 0x01;
pub const EXYNOS5422_SG_OFFSET: u32 = 0;
pub const EXYNOS5422_SG_MASK: u32 = 0x07;
pub const EXYNOS5422_TABLE_OFFSET: u32 = 8;
pub const EXYNOS5422_TABLE_MASK: u32 = 0x03;
pub const EXYNOS5422_SG_A_OFFSET: u32 = 17;
pub const EXYNOS5422_SG_A_MASK: u32 = 0x0f;
pub const EXYNOS5422_SG_B_OFFSET: u32 = 21;
pub const EXYNOS5422_SG_B_MASK: u32 = 0x03;
pub const EXYNOS5422_SG_BSIGN_OFFSET: u32 = 23;
pub const EXYNOS5422_SG_BSIGN_MASK: u32 = 0x01;
pub const EXYNOS5422_BIN2_OFFSET: u32 = 12;
pub const EXYNOS5422_BIN2_MASK: u32 = 0x01;

pub const EXYNOS_CHIPID_REG_LOT_ID: u32 = 0x14;

pub const EXYNOS_CHIPID_REG_AUX_INFO: u32 = 0x1c;
/* Bit field definitions for EXYNOS_CHIPID_REG_AUX_INFO register */
pub const EXYNOS5422_TMCB_OFFSET: u32 = 0;
pub const EXYNOS5422_TMCB_MASK: u32 = 0x7f;
pub const EXYNOS5422_ARM_UP_OFFSET: u32 = 8;
pub const EXYNOS5422_ARM_UP_MASK: u32 = 0x03;
pub const EXYNOS5422_ARM_DN_OFFSET: u32 = 10;
pub const EXYNOS5422_ARM_DN_MASK: u32 = 0x03;
pub const EXYNOS5422_KFC_UP_OFFSET: u32 = 12;
pub const EXYNOS5422_KFC_UP_MASK: u32 = 0x03;
pub const EXYNOS5422_KFC_DN_OFFSET: u32 = 14;
pub const EXYNOS5422_KFC_DN_MASK: u32 = 0x03;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
