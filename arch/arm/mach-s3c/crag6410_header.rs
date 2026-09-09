/* SPDX-License-Identifier: GPL-2.0 */
/* Cragganmore 6410 shared definitions
 *
 * Copyright 2011 Wolfson Microelectronics plc
 *	Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// Dependency provided by gpio-samsung.h in the original C header.

pub const GLENFARCLAS_PMIC_IRQ_BASE: i32 = IRQ_BOARD_START;
pub const BANFF_PMIC_IRQ_BASE: i32 = IRQ_BOARD_START + 64;

pub const PCA935X_GPIO_BASE: i32 = GPIO_BOARD_START;
pub const CODEC_GPIO_BASE: i32 = GPIO_BOARD_START + 8;
pub const GLENFARCLAS_PMIC_GPIO_BASE: i32 = GPIO_BOARD_START + 32;
pub const BANFF_PMIC_GPIO_BASE: i32 = GPIO_BOARD_START + 64;
pub const MMGPIO_GPIO_BASE: i32 = GPIO_BOARD_START + 96;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
