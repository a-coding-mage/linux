/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Tesla FSD DTS pinctrl constants
 *
 * Copyright (c) 2016 Samsung Electronics Co., Ltd.
 *	http://www.samsung.com
 * Copyright (c) 2022 Linaro Ltd
 * Author: Krzysztof Kozlowski <krzk@kernel.org>
 */

pub const FSD_PIN_PULL_NONE: u32 = 0;
pub const FSD_PIN_PULL_DOWN: u32 = 1;
pub const FSD_PIN_PULL_UP: u32 = 3;

pub const FSD_PIN_DRV_LV1: u32 = 0;
pub const FSD_PIN_DRV_LV2: u32 = 1;
pub const FSD_PIN_DRV_LV4: u32 = 2;
pub const FSD_PIN_DRV_LV6: u32 = 3;

pub const FSD_PIN_FUNC_INPUT: u32 = 0;
pub const FSD_PIN_FUNC_OUTPUT: u32 = 1;
pub const FSD_PIN_FUNC_2: u32 = 2;
pub const FSD_PIN_FUNC_3: u32 = 3;
pub const FSD_PIN_FUNC_4: u32 = 4;
pub const FSD_PIN_FUNC_5: u32 = 5;
pub const FSD_PIN_FUNC_6: u32 = 6;
pub const FSD_PIN_FUNC_EINT: u32 = 0xf;
pub const FSD_PIN_FUNC_F: u32 = FSD_PIN_FUNC_EINT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
