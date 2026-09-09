/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2022 MediaTek Inc.
 */

// Dependencies supplied by the surrounding translation unit:
// linux/reset-controller.h, linux/types.h

pub const RST_NR_PER_BANK: u32 = 32;

/* Infra global controller reset set register */
pub const INFRA_RST0_SET_OFFSET: u32 = 0x120;
pub const INFRA_RST1_SET_OFFSET: u32 = 0x130;
pub const INFRA_RST2_SET_OFFSET: u32 = 0x140;
pub const INFRA_RST3_SET_OFFSET: u32 = 0x150;
pub const INFRA_RST4_SET_OFFSET: u32 = 0x730;

/**
 * enum mtk_reset_version - Version of MediaTek clock reset controller.
 * @MTK_RST_SIMPLE: Use the same registers for bit set and clear.
 * @MTK_RST_SET_CLR: Use separate registers for bit set and clear.
 * @MTK_RST_MAX: Total quantity of version for MediaTek clock reset controller.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mtk_reset_version {
	MTK_RST_SIMPLE = 0,
	MTK_RST_SET_CLR,
	MTK_RST_MAX,
}

/**
 * struct mtk_clk_rst_desc - Description of MediaTek clock reset.
 * @version: Reset version which is defined in enum mtk_reset_version.
 * @rst_bank_ofs: Pointer to an array containing base offsets of the reset register.
 * @rst_bank_nr: Quantity of reset bank.
 * @rst_idx_map:Pointer to an array containing ids if input argument is index.
 *		This array is not necessary if our input argument does not mean index.
 * @rst_idx_map_nr: Quantity of reset index map.
 */
#[repr(C)]
pub struct mtk_clk_rst_desc {
	pub version: mtk_reset_version,
	pub rst_bank_ofs: *mut u16,
	pub rst_bank_nr: u32,
	pub rst_idx_map: *mut u16,
	pub rst_idx_map_nr: u32,
}

/**
 * struct mtk_clk_rst_data - Data of MediaTek clock reset controller.
 * @regmap: Pointer to base address of reset register address.
 * @rcdev: Reset controller device.
 * @desc: Pointer to description of the reset controller.
 */
#[repr(C)]
pub struct mtk_clk_rst_data {
	pub regmap: *mut regmap,
	pub rcdev: reset_controller_dev,
	pub desc: *const mtk_clk_rst_desc,
}

/**
 * mtk_register_reset_controller - Register mediatek clock reset controller with device
 * @np: Pointer to device.
 * @desc: Constant pointer to description of clock reset.
 *
 * Return: 0 on success and errorno otherwise.
 */
unsafe extern "C" {
	pub fn mtk_register_reset_controller_with_dev(
		dev: *mut device,
		desc: *const mtk_clk_rst_desc,
	) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
