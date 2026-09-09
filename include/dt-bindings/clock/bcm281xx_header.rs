/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 Broadcom Corporation
 * Copyright 2013 Linaro Limited
 */

/*
 * This file defines the values used to specify clocks provided by
 * the clock control units (CCUs) on Broadcom BCM281XX family SoCs.
 */

/*
 * These are the bcm281xx CCU device tree "compatible" strings.
 * We're stuck with using "bcm11351" in the string because wild
 * cards aren't allowed, and that name was the first one defined
 * in this family of devices.
 */
pub const BCM281XX_DT_ROOT_CCU_COMPAT: &str = "brcm,bcm11351-root-ccu";
pub const BCM281XX_DT_AON_CCU_COMPAT: &str = "brcm,bcm11351-aon-ccu";
pub const BCM281XX_DT_HUB_CCU_COMPAT: &str = "brcm,bcm11351-hub-ccu";
pub const BCM281XX_DT_MASTER_CCU_COMPAT: &str = "brcm,bcm11351-master-ccu";
pub const BCM281XX_DT_SLAVE_CCU_COMPAT: &str = "brcm,bcm11351-slave-ccu";

/* root CCU clock ids */

pub const BCM281XX_ROOT_CCU_FRAC_1M: u32 = 0;
pub const BCM281XX_ROOT_CCU_CLOCK_COUNT: u32 = 1;

/* aon CCU clock ids */

pub const BCM281XX_AON_CCU_HUB_TIMER: u32 = 0;
pub const BCM281XX_AON_CCU_PMU_BSC: u32 = 1;
pub const BCM281XX_AON_CCU_PMU_BSC_VAR: u32 = 2;
pub const BCM281XX_AON_CCU_CLOCK_COUNT: u32 = 3;

/* hub CCU clock ids */

pub const BCM281XX_HUB_CCU_TMON_1M: u32 = 0;
pub const BCM281XX_HUB_CCU_CLOCK_COUNT: u32 = 1;

/* master CCU clock ids */

pub const BCM281XX_MASTER_CCU_SDIO1: u32 = 0;
pub const BCM281XX_MASTER_CCU_SDIO2: u32 = 1;
pub const BCM281XX_MASTER_CCU_SDIO3: u32 = 2;
pub const BCM281XX_MASTER_CCU_SDIO4: u32 = 3;
pub const BCM281XX_MASTER_CCU_USB_IC: u32 = 4;
pub const BCM281XX_MASTER_CCU_HSIC2_48M: u32 = 5;
pub const BCM281XX_MASTER_CCU_HSIC2_12M: u32 = 6;
pub const BCM281XX_MASTER_CCU_CLOCK_COUNT: u32 = 7;

/* slave CCU clock ids */

pub const BCM281XX_SLAVE_CCU_UARTB: u32 = 0;
pub const BCM281XX_SLAVE_CCU_UARTB2: u32 = 1;
pub const BCM281XX_SLAVE_CCU_UARTB3: u32 = 2;
pub const BCM281XX_SLAVE_CCU_UARTB4: u32 = 3;
pub const BCM281XX_SLAVE_CCU_SSP0: u32 = 4;
pub const BCM281XX_SLAVE_CCU_SSP2: u32 = 5;
pub const BCM281XX_SLAVE_CCU_BSC1: u32 = 6;
pub const BCM281XX_SLAVE_CCU_BSC2: u32 = 7;
pub const BCM281XX_SLAVE_CCU_BSC3: u32 = 8;
pub const BCM281XX_SLAVE_CCU_PWM: u32 = 9;
pub const BCM281XX_SLAVE_CCU_CLOCK_COUNT: u32 = 10;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
