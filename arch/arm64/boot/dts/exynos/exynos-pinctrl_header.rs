/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Samsung Exynos DTS pinctrl constants
 *
 * Copyright (c) 2016 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 * Copyright (c) 2022 Linaro Ltd
 * Author: Krzysztof Kozlowski <krzk@kernel.org>
 */

pub const EXYNOS_PIN_PULL_NONE: u32 = 0;
pub const EXYNOS_PIN_PULL_DOWN: u32 = 1;
pub const EXYNOS_PIN_PULL_UP: u32 = 3;

/* Pin function in power down mode */
pub const EXYNOS_PIN_PDN_OUT0: u32 = 0;
pub const EXYNOS_PIN_PDN_OUT1: u32 = 1;
pub const EXYNOS_PIN_PDN_INPUT: u32 = 2;
pub const EXYNOS_PIN_PDN_PREV: u32 = 3;

/*
 * Drive strengths for Exynos5410, Exynos542x, Exynos5800, Exynos7885, Exynos850
 * (except GPIO_HSI block), ExynosAutov9 (FSI0, PERIC1)
 */
pub const EXYNOS5420_PIN_DRV_LV1: u32 = 0;
pub const EXYNOS5420_PIN_DRV_LV2: u32 = 1;
pub const EXYNOS5420_PIN_DRV_LV3: u32 = 2;
pub const EXYNOS5420_PIN_DRV_LV4: u32 = 3;

/* Drive strengths for Exynos5433 */
pub const EXYNOS5433_PIN_DRV_FAST_SR1: u32 = 0;
pub const EXYNOS5433_PIN_DRV_FAST_SR2: u32 = 1;
pub const EXYNOS5433_PIN_DRV_FAST_SR3: u32 = 2;
pub const EXYNOS5433_PIN_DRV_FAST_SR4: u32 = 3;
pub const EXYNOS5433_PIN_DRV_FAST_SR5: u32 = 4;
pub const EXYNOS5433_PIN_DRV_FAST_SR6: u32 = 5;
pub const EXYNOS5433_PIN_DRV_SLOW_SR1: u32 = 8;
pub const EXYNOS5433_PIN_DRV_SLOW_SR2: u32 = 9;
pub const EXYNOS5433_PIN_DRV_SLOW_SR3: u32 = 0xa;
pub const EXYNOS5433_PIN_DRV_SLOW_SR4: u32 = 0xb;
pub const EXYNOS5433_PIN_DRV_SLOW_SR5: u32 = 0xc;
pub const EXYNOS5433_PIN_DRV_SLOW_SR6: u32 = 0xf;

/* Drive strengths for Exynos7 (except FSYS1) */
pub const EXYNOS7_PIN_DRV_LV1: u32 = 0;
pub const EXYNOS7_PIN_DRV_LV2: u32 = 2;
pub const EXYNOS7_PIN_DRV_LV3: u32 = 1;
pub const EXYNOS7_PIN_DRV_LV4: u32 = 3;

/* Drive strengths for Exynos7 FSYS1 block */
pub const EXYNOS7_FSYS1_PIN_DRV_LV1: u32 = 0;
pub const EXYNOS7_FSYS1_PIN_DRV_LV2: u32 = 4;
pub const EXYNOS7_FSYS1_PIN_DRV_LV3: u32 = 2;
pub const EXYNOS7_FSYS1_PIN_DRV_LV4: u32 = 6;
pub const EXYNOS7_FSYS1_PIN_DRV_LV5: u32 = 1;
pub const EXYNOS7_FSYS1_PIN_DRV_LV6: u32 = 5;

/* Drive strengths for Exynos850 GPIO_HSI block */
pub const EXYNOS850_HSI_PIN_DRV_LV1: u32 = 0; /* 1x   */
pub const EXYNOS850_HSI_PIN_DRV_LV1_5: u32 = 1; /* 1.5x */
pub const EXYNOS850_HSI_PIN_DRV_LV2: u32 = 2; /* 2x   */
pub const EXYNOS850_HSI_PIN_DRV_LV2_5: u32 = 3; /* 2.5x */
pub const EXYNOS850_HSI_PIN_DRV_LV3: u32 = 4; /* 3x   */
pub const EXYNOS850_HSI_PIN_DRV_LV4: u32 = 5; /* 4x   */

pub const EXYNOS_PIN_FUNC_INPUT: u32 = 0;
pub const EXYNOS_PIN_FUNC_OUTPUT: u32 = 1;
pub const EXYNOS_PIN_FUNC_2: u32 = 2;
pub const EXYNOS_PIN_FUNC_3: u32 = 3;
pub const EXYNOS_PIN_FUNC_4: u32 = 4;
pub const EXYNOS_PIN_FUNC_5: u32 = 5;
pub const EXYNOS_PIN_FUNC_6: u32 = 6;
pub const EXYNOS_PIN_FUNC_EINT: u32 = 0xf;
pub const EXYNOS_PIN_FUNC_F: u32 = EXYNOS_PIN_FUNC_EINT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
