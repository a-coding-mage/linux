/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 Broadcom Corporation
 * Copyright 2013 Linaro Limited
 */

/*
 * This file defines the values used to specify clocks provided by
 * the clock control units (CCUs) on Broadcom BCM21664 family SoCs.
 */

/* bcm21664 CCU device tree "compatible" strings */
pub const BCM21664_DT_ROOT_CCU_COMPAT: &str = "brcm,bcm21664-root-ccu";
pub const BCM21664_DT_AON_CCU_COMPAT: &str = "brcm,bcm21664-aon-ccu";
pub const BCM21664_DT_MASTER_CCU_COMPAT: &str = "brcm,bcm21664-master-ccu";
pub const BCM21664_DT_SLAVE_CCU_COMPAT: &str = "brcm,bcm21664-slave-ccu";

/* root CCU clock ids */

pub const BCM21664_ROOT_CCU_FRAC_1M: u32 = 0;
pub const BCM21664_ROOT_CCU_CLOCK_COUNT: u32 = 1;

/* aon CCU clock ids */

pub const BCM21664_AON_CCU_HUB_TIMER: u32 = 0;
pub const BCM21664_AON_CCU_CLOCK_COUNT: u32 = 1;

/* master CCU clock ids */

pub const BCM21664_MASTER_CCU_SDIO1: u32 = 0;
pub const BCM21664_MASTER_CCU_SDIO2: u32 = 1;
pub const BCM21664_MASTER_CCU_SDIO3: u32 = 2;
pub const BCM21664_MASTER_CCU_SDIO4: u32 = 3;
pub const BCM21664_MASTER_CCU_SDIO1_SLEEP: u32 = 4;
pub const BCM21664_MASTER_CCU_SDIO2_SLEEP: u32 = 5;
pub const BCM21664_MASTER_CCU_SDIO3_SLEEP: u32 = 6;
pub const BCM21664_MASTER_CCU_SDIO4_SLEEP: u32 = 7;
pub const BCM21664_MASTER_CCU_CLOCK_COUNT: u32 = 8;

/* slave CCU clock ids */

pub const BCM21664_SLAVE_CCU_UARTB: u32 = 0;
pub const BCM21664_SLAVE_CCU_UARTB2: u32 = 1;
pub const BCM21664_SLAVE_CCU_UARTB3: u32 = 2;
pub const BCM21664_SLAVE_CCU_BSC1: u32 = 3;
pub const BCM21664_SLAVE_CCU_BSC2: u32 = 4;
pub const BCM21664_SLAVE_CCU_BSC3: u32 = 5;
pub const BCM21664_SLAVE_CCU_BSC4: u32 = 6;
pub const BCM21664_SLAVE_CCU_CLOCK_COUNT: u32 = 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
