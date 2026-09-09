/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017-2018 NXP
 *
 * Header file containing the public API for the System Controller (SC)
 * Power Management (PM) function. This includes functions for power state
 * control, clock control, reset control, and wake-up event control.
 *
 * PM_SVC (SVC) Power Management Service
 *
 * Module for the Power Management (PM) service.
 */

// Dependency supplied by linux/firmware/imx/sci.h.

/*
 * This type is used to indicate RPC PM function calls.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImxScPmFunc {
    IMX_SC_PM_FUNC_UNKNOWN = 0,
    IMX_SC_PM_FUNC_SET_SYS_POWER_MODE = 19,
    IMX_SC_PM_FUNC_SET_PARTITION_POWER_MODE = 1,
    IMX_SC_PM_FUNC_GET_SYS_POWER_MODE = 2,
    IMX_SC_PM_FUNC_SET_RESOURCE_POWER_MODE = 3,
    IMX_SC_PM_FUNC_GET_RESOURCE_POWER_MODE = 4,
    IMX_SC_PM_FUNC_REQ_LOW_POWER_MODE = 16,
    IMX_SC_PM_FUNC_SET_CPU_RESUME_ADDR = 17,
    IMX_SC_PM_FUNC_REQ_SYS_IF_POWER_MODE = 18,
    IMX_SC_PM_FUNC_SET_CLOCK_RATE = 5,
    IMX_SC_PM_FUNC_GET_CLOCK_RATE = 6,
    IMX_SC_PM_FUNC_CLOCK_ENABLE = 7,
    IMX_SC_PM_FUNC_SET_CLOCK_PARENT = 14,
    IMX_SC_PM_FUNC_GET_CLOCK_PARENT = 15,
    IMX_SC_PM_FUNC_RESET = 13,
    IMX_SC_PM_FUNC_RESET_REASON = 10,
    IMX_SC_PM_FUNC_BOOT = 8,
    IMX_SC_PM_FUNC_REBOOT = 9,
    IMX_SC_PM_FUNC_REBOOT_PARTITION = 12,
    IMX_SC_PM_FUNC_CPU_START = 11,
}

/*
 * Defines for ALL parameters
 */
pub const IMX_SC_PM_CLK_ALL: u8 = u8::MAX; /* All clocks */

/*
 * Defines for SC PM Power Mode
 */
pub const IMX_SC_PM_PW_MODE_OFF: u8 = 0; /* Power off */
pub const IMX_SC_PM_PW_MODE_STBY: u8 = 1; /* Power in standby */
pub const IMX_SC_PM_PW_MODE_LP: u8 = 2; /* Power in low-power */
pub const IMX_SC_PM_PW_MODE_ON: u8 = 3; /* Power on */

/*
 * Defines for SC PM CLK
 */
pub const IMX_SC_PM_CLK_SLV_BUS: u8 = 0; /* Slave bus clock */
pub const IMX_SC_PM_CLK_MST_BUS: u8 = 1; /* Master bus clock */
pub const IMX_SC_PM_CLK_PER: u8 = 2; /* Peripheral clock */
pub const IMX_SC_PM_CLK_PHY: u8 = 3; /* Phy clock */
pub const IMX_SC_PM_CLK_MISC: u8 = 4; /* Misc clock */
pub const IMX_SC_PM_CLK_MISC0: u8 = 0; /* Misc 0 clock */
pub const IMX_SC_PM_CLK_MISC1: u8 = 1; /* Misc 1 clock */
pub const IMX_SC_PM_CLK_MISC2: u8 = 2; /* Misc 2 clock */
pub const IMX_SC_PM_CLK_MISC3: u8 = 3; /* Misc 3 clock */
pub const IMX_SC_PM_CLK_MISC4: u8 = 4; /* Misc 4 clock */
pub const IMX_SC_PM_CLK_CPU: u8 = 2; /* CPU clock */
pub const IMX_SC_PM_CLK_PLL: u8 = 4; /* PLL */
pub const IMX_SC_PM_CLK_BYPASS: u8 = 4; /* Bypass clock */

/*
 * Defines for SC PM CLK Parent
 */
pub const IMX_SC_PM_PARENT_XTAL: u8 = 0; /* Parent is XTAL. */
pub const IMX_SC_PM_PARENT_PLL0: u8 = 1; /* Parent is PLL0 */
pub const IMX_SC_PM_PARENT_PLL1: u8 = 2; /* Parent is PLL1 or PLL0/2 */
pub const IMX_SC_PM_PARENT_PLL2: u8 = 3; /* Parent in PLL2 or PLL0/4 */
pub const IMX_SC_PM_PARENT_BYPS: u8 = 4; /* Parent is a bypass clock. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
