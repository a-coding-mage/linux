/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2016 Pengutronix, <kernel@pengutronix.de>
 */

// The C header guard `__SOC_IMX_CPUIDLE_H__` is omitted in Rust.

// C condition: defined(CONFIG_CPU_IDLE) && defined(CONFIG_SOC_IMX6Q)
#[cfg(all(CONFIG_CPU_IDLE, CONFIG_SOC_IMX6Q))]
extern "C" {
    pub fn imx6q_cpuidle_fec_irqs_used();
    pub fn imx6q_cpuidle_fec_irqs_unused();
}

// Fallback when CONFIG_CPU_IDLE or CONFIG_SOC_IMX6Q is not enabled.
#[cfg(not(all(CONFIG_CPU_IDLE, CONFIG_SOC_IMX6Q)))]
#[inline]
pub fn imx6q_cpuidle_fec_irqs_used() {}

#[cfg(not(all(CONFIG_CPU_IDLE, CONFIG_SOC_IMX6Q)))]
#[inline]
pub fn imx6q_cpuidle_fec_irqs_unused() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
