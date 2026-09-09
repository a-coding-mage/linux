/* SPDX-License-Identifier: GPL-2.0-only OR BSD-3-Clause */
/*
 * Copyright (c) 2020 The Linux Foundation. All rights reserved.
 */

// Dependency supplied by the corresponding qcom,spmi-vadc binding.

pub const SMB139x_1_ADC7_SMB_TEMP: u32 = (SMB139x_1_SID << 8) | ADC7_SMB_TEMP;
pub const SMB139x_1_ADC7_ICHG_SMB: u32 = (SMB139x_1_SID << 8) | ADC7_ICHG_SMB;
pub const SMB139x_1_ADC7_IIN_SMB: u32 = (SMB139x_1_SID << 8) | ADC7_IIN_SMB;

pub const SMB139x_2_ADC7_SMB_TEMP: u32 = (SMB139x_2_SID << 8) | ADC7_SMB_TEMP;
pub const SMB139x_2_ADC7_ICHG_SMB: u32 = (SMB139x_2_SID << 8) | ADC7_ICHG_SMB;
pub const SMB139x_2_ADC7_IIN_SMB: u32 = (SMB139x_2_SID << 8) | ADC7_IIN_SMB;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
