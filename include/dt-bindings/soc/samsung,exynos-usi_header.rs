/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2021 Linaro Ltd.
 * Author: Sam Protsenko <semen.protsenko@linaro.org>
 *
 * Device Tree bindings for Samsung Exynos USI (Universal Serial Interface).
 */

pub const USI_MODE_NONE: i32 = 0;
pub const USI_MODE_UART: i32 = 1;
pub const USI_MODE_SPI: i32 = 2;
pub const USI_MODE_I2C: i32 = 3;
pub const USI_MODE_I2C1: i32 = 4;
pub const USI_MODE_I2C0_1: i32 = 5;
pub const USI_MODE_UART_I2C1: i32 = 6;

/* Deprecated */
pub const USI_V2_NONE: i32 = USI_MODE_NONE;
pub const USI_V2_UART: i32 = USI_MODE_UART;
pub const USI_V2_SPI: i32 = USI_MODE_SPI;
pub const USI_V2_I2C: i32 = USI_MODE_I2C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
