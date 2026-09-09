/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Samsung S5PV210 DTS pinctrl constants
 *
 * Copyright (c) 2016 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 * Copyright (c) 2022 Linaro Ltd
 * Author: Krzysztof Kozlowski <krzk@kernel.org>
 */

// C header guard: __DTS_ARM_SAMSUNG_S5PV210_PINCTRL_H__

pub const S5PV210_PIN_PULL_NONE: u32 = 0;
pub const S5PV210_PIN_PULL_DOWN: u32 = 1;
pub const S5PV210_PIN_PULL_UP: u32 = 2;

/* Pin function in power down mode */
pub const S5PV210_PIN_PDN_OUT0: u32 = 0;
pub const S5PV210_PIN_PDN_OUT1: u32 = 1;
pub const S5PV210_PIN_PDN_INPUT: u32 = 2;
pub const S5PV210_PIN_PDN_PREV: u32 = 3;

pub const S5PV210_PIN_DRV_LV1: u32 = 0;
pub const S5PV210_PIN_DRV_LV2: u32 = 2;
pub const S5PV210_PIN_DRV_LV3: u32 = 1;
pub const S5PV210_PIN_DRV_LV4: u32 = 3;

pub const S5PV210_PIN_FUNC_INPUT: u32 = 0;
pub const S5PV210_PIN_FUNC_OUTPUT: u32 = 1;
pub const S5PV210_PIN_FUNC_2: u32 = 2;
pub const S5PV210_PIN_FUNC_3: u32 = 3;
pub const S5PV210_PIN_FUNC_4: u32 = 4;
pub const S5PV210_PIN_FUNC_5: u32 = 5;
pub const S5PV210_PIN_FUNC_6: u32 = 6;
pub const S5PV210_PIN_FUNC_EINT: u32 = 0xf;
pub const S5PV210_PIN_FUNC_F: u32 = S5PV210_PIN_FUNC_EINT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
