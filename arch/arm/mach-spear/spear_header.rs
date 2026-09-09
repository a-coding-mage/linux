/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SPEAr3xx/6xx Machine family specific definition
 *
 * Copyright (C) 2009,2012 ST Microelectronics
 * Rajeev Kumar<rajeev-dlh.kumar@st.com>
 * Viresh Kumar <vireshk@kernel.org>
 */

/* The following declarations are conditional on CONFIG_ARCH_SPEAR3XX or
 * CONFIG_ARCH_SPEAR6XX in the original header. */
#[cfg(any(feature = "CONFIG_ARCH_SPEAR3XX", feature = "CONFIG_ARCH_SPEAR6XX"))]

/* ICM1 - Low speed connection */
pub const SPEAR_ICM1_2_BASE: usize = 0xD0000000;
pub const VA_SPEAR_ICM1_2_BASE: usize = 0xFD000000;
pub const SPEAR_ICM1_UART_BASE: usize = 0xD0000000;
pub const VA_SPEAR_ICM1_UART_BASE: usize =
    VA_SPEAR_ICM1_2_BASE - SPEAR_ICM1_2_BASE + SPEAR_ICM1_UART_BASE;
pub const SPEAR3XX_ICM1_SSP_BASE: usize = 0xD0100000;

/* ML-1, 2 - Multi Layer CPU Subsystem */
pub const SPEAR_ICM3_ML1_2_BASE: usize = 0xF0000000;
pub const VA_SPEAR6XX_ML_CPU_BASE: usize = 0xF0000000;

/* ICM3 - Basic Subsystem */
pub const SPEAR_ICM3_SMI_CTRL_BASE: usize = 0xFC000000;
pub const VA_SPEAR_ICM3_SMI_CTRL_BASE: usize = 0xFC000000;
pub const SPEAR_ICM3_DMA_BASE: usize = 0xFC400000;
pub const SPEAR_ICM3_SYS_CTRL_BASE: usize = 0xFCA00000;
pub const VA_SPEAR_ICM3_SYS_CTRL_BASE: usize =
    VA_SPEAR_ICM3_SMI_CTRL_BASE - SPEAR_ICM3_SMI_CTRL_BASE + SPEAR_ICM3_SYS_CTRL_BASE;
pub const SPEAR_ICM3_MISC_REG_BASE: usize = 0xFCA80000;
pub const VA_SPEAR_ICM3_MISC_REG_BASE: usize =
    VA_SPEAR_ICM3_SMI_CTRL_BASE - SPEAR_ICM3_SMI_CTRL_BASE + SPEAR_ICM3_MISC_REG_BASE;

/* Debug uart for linux, will be used for debug and uncompress messages */
#[cfg(any(feature = "CONFIG_ARCH_SPEAR3XX", feature = "CONFIG_ARCH_SPEAR6XX"))]
pub const SPEAR_DBG_UART_BASE: usize = SPEAR_ICM1_UART_BASE;

/* Sysctl base for spear platform */
pub const SPEAR_SYS_CTRL_BASE: usize = SPEAR_ICM3_SYS_CTRL_BASE;
pub const VA_SPEAR_SYS_CTRL_BASE: usize = VA_SPEAR_ICM3_SYS_CTRL_BASE;

/* SPEAr320 Macros */
pub const SPEAR320_SOC_CONFIG_BASE: usize = 0xB3000000;
pub const VA_SPEAR320_SOC_CONFIG_BASE: usize = 0xFE000000;

/* The following declarations are conditional on CONFIG_ARCH_SPEAR13XX in the
 * original header. */
#[cfg(feature = "CONFIG_ARCH_SPEAR13XX")]

pub const PERIP_GRP2_BASE: usize = 0xB3000000;
pub const VA_PERIP_GRP2_BASE: usize = 0xF9000000;
pub const MCIF_SDHCI_BASE: usize = 0xB3000000;
pub const SYSRAM0_BASE: usize = 0xB3800000;
pub const VA_SYSRAM0_BASE: usize = 0xF9800000;
pub const SYS_LOCATION: usize = VA_SYSRAM0_BASE + 0x600;

pub const PERIP_GRP1_BASE: usize = 0xE0000000;
pub const VA_PERIP_GRP1_BASE: usize = 0xFD000000;
pub const UART_BASE: usize = 0xE0000000;
pub const VA_UART_BASE: usize = 0xFD000000;
pub const SSP_BASE: usize = 0xE0100000;
pub const MISC_BASE: usize = 0xE0700000;
pub const VA_MISC_BASE: usize = 0xFD700000;

pub const A9SM_AND_MPMC_BASE: usize = 0xEC000000;
pub const VA_A9SM_AND_MPMC_BASE: usize = 0xFC000000;

/* A9SM peripheral offsets */
pub const A9SM_PERIP_BASE: usize = 0xEC800000;
pub const VA_A9SM_PERIP_BASE: usize = 0xFC800000;
pub const VA_SCU_BASE: usize = VA_A9SM_PERIP_BASE + 0x00;

pub const L2CC_BASE: usize = 0xED000000;
pub const VA_L2CC_BASE: usize = 0xFB000000;

/* others */
pub const MCIF_CF_BASE: usize = 0xB2800000;

/* Debug uart for linux, will be used for debug and uncompress messages */
#[cfg(feature = "CONFIG_ARCH_SPEAR13XX")]
pub const SPEAR_DBG_UART_BASE: usize = UART_BASE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
