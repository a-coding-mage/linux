/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for Samsung audio subsystem
 * clock controller.
 *
 * The constants defined in this header are being used in dts
 * and exynos audss driver.
 */

pub const EXYNOS_MOUT_AUDSS: u32 = 0;
pub const EXYNOS_MOUT_I2S: u32 = 1;
pub const EXYNOS_DOUT_SRP: u32 = 2;
pub const EXYNOS_DOUT_AUD_BUS: u32 = 3;
pub const EXYNOS_DOUT_I2S: u32 = 4;
pub const EXYNOS_SRP_CLK: u32 = 5;
pub const EXYNOS_I2S_BUS: u32 = 6;
pub const EXYNOS_SCLK_I2S: u32 = 7;
pub const EXYNOS_PCM_BUS: u32 = 8;
pub const EXYNOS_SCLK_PCM: u32 = 9;
pub const EXYNOS_ADMA: u32 = 10;

pub const EXYNOS_AUDSS_MAX_CLKS: u32 = 11;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
