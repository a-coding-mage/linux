/*
 * Hardware definitions for TI OMAP processors and boards
 *
 * NOTE: Please put device driver specific defines into a separate header
 *       file for each driver.
 *
 * Copyright (C) 2001 RidgeRun, Inc.
 * Author: RidgeRun, Inc. Greg Lonnon <glonnon@ridgerun.com>
 *
 * Reorganized for Linux-2.6 by Tony Lindgren <tony@atomide.com>
 *                          and Dirk Behme <dirk.behme@de.bosch.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 */

// C dependencies: linux/sizes.h, linux/soc/ti/omap1-io.h,
// asm/types.h, linux/soc/ti/omap1-soc.h, tc.h, and serial.h.

// Almost all documentation for chip and board memory maps assumes BM is
// clear. Most development boards have a switch to control booting from NOR
// flash rather than mask ROM, interchanging physical CS0 and CS3 addresses.
#[inline]
pub unsafe fn omap_cs0m_phys() -> u32 {
    if (omap_readl(EMIFS_CONFIG) & OMAP_EMIFS_CONFIG_BM) != 0 {
        OMAP_CS3_PHYS
    } else {
        0
    }
}

#[inline]
pub unsafe fn omap_cs3_phys() -> u32 {
    if (omap_readl(EMIFS_CONFIG) & OMAP_EMIFS_CONFIG_BM) != 0 {
        0
    } else {
        OMAP_CS3_PHYS
    }
}

pub const OMAP1_IO_OFFSET: usize = 0x00f00000;
// C macro: IOMEM((pa) - OMAP1_IO_OFFSET)
#[inline]
pub const fn OMAP1_IO_ADDRESS(pa: usize) -> usize {
    pa - OMAP1_IO_OFFSET
}

pub const OMAP_MPU_TIMER1_BASE: u32 = 0xfffec500;
pub const OMAP_MPU_TIMER2_BASE: u32 = 0xfffec600;
pub const OMAP_MPU_TIMER3_BASE: u32 = 0xfffec700;
pub const MPU_TIMER_FREE: u32 = 1 << 6;
pub const MPU_TIMER_CLOCK_ENABLE: u32 = 1 << 5;
pub const MPU_TIMER_AR: u32 = 1 << 1;
pub const MPU_TIMER_ST: u32 = 1 << 0;

pub const OMAP_MPU_WATCHDOG_BASE: u32 = 0xfffec800;
pub const OMAP_WDT_TIMER: u32 = OMAP_MPU_WATCHDOG_BASE + 0x0;
pub const OMAP_WDT_LOAD_TIM: u32 = OMAP_MPU_WATCHDOG_BASE + 0x4;
pub const OMAP_WDT_READ_TIM: u32 = OMAP_MPU_WATCHDOG_BASE + 0x4;
pub const OMAP_WDT_TIMER_MODE: u32 = OMAP_MPU_WATCHDOG_BASE + 0x8;

// #ifdef CONFIG_ARCH_OMAP1
pub const OMAP_IH1_BASE: u32 = 0xfffecb00;
pub const OMAP_IH2_BASE: u32 = 0xfffe0000;
pub const OMAP_IH2_0_BASE: u32 = 0xfffe0000;
pub const OMAP_IH2_1_BASE: u32 = 0xfffe0100;
pub const OMAP_IH2_2_BASE: u32 = 0xfffe0200;
pub const OMAP_IH2_3_BASE: u32 = 0xfffe0300;

pub const OMAP_IH1_ITR: u32 = OMAP_IH1_BASE + 0x00;
pub const OMAP_IH1_MIR: u32 = OMAP_IH1_BASE + 0x04;
pub const OMAP_IH1_SIR_IRQ: u32 = OMAP_IH1_BASE + 0x10;
pub const OMAP_IH1_SIR_FIQ: u32 = OMAP_IH1_BASE + 0x14;
pub const OMAP_IH1_CONTROL: u32 = OMAP_IH1_BASE + 0x18;
pub const OMAP_IH1_ILR0: u32 = OMAP_IH1_BASE + 0x1c;
pub const OMAP_IH1_ISR: u32 = OMAP_IH1_BASE + 0x9c;

pub const OMAP_IH2_ITR: u32 = OMAP_IH2_BASE + 0x00;
pub const OMAP_IH2_MIR: u32 = OMAP_IH2_BASE + 0x04;
pub const OMAP_IH2_SIR_IRQ: u32 = OMAP_IH2_BASE + 0x10;
pub const OMAP_IH2_SIR_FIQ: u32 = OMAP_IH2_BASE + 0x14;
pub const OMAP_IH2_CONTROL: u32 = OMAP_IH2_BASE + 0x18;
pub const OMAP_IH2_ILR0: u32 = OMAP_IH2_BASE + 0x1c;
pub const OMAP_IH2_ISR: u32 = OMAP_IH2_BASE + 0x9c;

pub const OMAP_IH2_0_ITR: u32 = OMAP_IH2_0_BASE + 0x00;
pub const OMAP_IH2_0_MIR: u32 = OMAP_IH2_0_BASE + 0x04;
pub const OMAP_IH2_0_SIR_IRQ: u32 = OMAP_IH2_0_BASE + 0x10;
pub const OMAP_IH2_0_SIR_FIQ: u32 = OMAP_IH2_0_BASE + 0x14;
pub const OMAP_IH2_0_CONTROL: u32 = OMAP_IH2_0_BASE + 0x18;
pub const OMAP_IH2_0_ILR0: u32 = OMAP_IH2_0_BASE + 0x1c;
pub const OMAP_IH2_0_ISR: u32 = OMAP_IH2_0_BASE + 0x9c;

pub const OMAP_IH2_1_ITR: u32 = OMAP_IH2_1_BASE + 0x00;
pub const OMAP_IH2_1_MIR: u32 = OMAP_IH2_1_BASE + 0x04;
pub const OMAP_IH2_1_SIR_IRQ: u32 = OMAP_IH2_1_BASE + 0x10;
pub const OMAP_IH2_1_SIR_FIQ: u32 = OMAP_IH2_1_BASE + 0x14;
pub const OMAP_IH2_1_CONTROL: u32 = OMAP_IH2_1_BASE + 0x18;
pub const OMAP_IH2_1_ILR1: u32 = OMAP_IH2_1_BASE + 0x1c;
pub const OMAP_IH2_1_ISR: u32 = OMAP_IH2_1_BASE + 0x9c;

pub const OMAP_IH2_2_ITR: u32 = OMAP_IH2_2_BASE + 0x00;
pub const OMAP_IH2_2_MIR: u32 = OMAP_IH2_2_BASE + 0x04;
pub const OMAP_IH2_2_SIR_IRQ: u32 = OMAP_IH2_2_BASE + 0x10;
pub const OMAP_IH2_2_SIR_FIQ: u32 = OMAP_IH2_2_BASE + 0x14;
pub const OMAP_IH2_2_CONTROL: u32 = OMAP_IH2_2_BASE + 0x18;
pub const OMAP_IH2_2_ILR2: u32 = OMAP_IH2_2_BASE + 0x1c;
pub const OMAP_IH2_2_ISR: u32 = OMAP_IH2_2_BASE + 0x9c;

pub const OMAP_IH2_3_ITR: u32 = OMAP_IH2_3_BASE + 0x00;
pub const OMAP_IH2_3_MIR: u32 = OMAP_IH2_3_BASE + 0x04;
pub const OMAP_IH2_3_SIR_IRQ: u32 = OMAP_IH2_3_BASE + 0x10;
pub const OMAP_IH2_3_SIR_FIQ: u32 = OMAP_IH2_3_BASE + 0x14;
pub const OMAP_IH2_3_CONTROL: u32 = OMAP_IH2_3_BASE + 0x18;
pub const OMAP_IH2_3_ILR3: u32 = OMAP_IH2_3_BASE + 0x1c;
pub const OMAP_IH2_3_ISR: u32 = OMAP_IH2_3_BASE + 0x9c;

pub const IRQ_ITR_REG_OFFSET: u32 = 0x00;
pub const IRQ_MIR_REG_OFFSET: u32 = 0x04;
pub const IRQ_SIR_IRQ_REG_OFFSET: u32 = 0x10;
pub const IRQ_SIR_FIQ_REG_OFFSET: u32 = 0x14;
pub const IRQ_CONTROL_REG_OFFSET: u32 = 0x18;
pub const IRQ_ISR_REG_OFFSET: u32 = 0x9c;
pub const IRQ_ILR0_REG_OFFSET: u32 = 0x1c;
pub const IRQ_GMR_REG_OFFSET: u32 = 0xa0;
// #endif

pub const OMAP_TIMER32K_BASE: u32 = 0xFFFBC400;

pub const TIPB_PUBLIC_CNTL_BASE: u32 = 0xfffed300;
pub const MPU_PUBLIC_TIPB_CNTL: u32 = TIPB_PUBLIC_CNTL_BASE + 0x8;
pub const TIPB_PRIVATE_CNTL_BASE: u32 = 0xfffeca00;
pub const MPU_PRIVATE_TIPB_CNTL: u32 = TIPB_PRIVATE_CNTL_BASE + 0x8;

pub const MPUI_BASE: u32 = 0xfffec900;
pub const MPUI_CTRL: u32 = MPUI_BASE + 0x0;
pub const MPUI_DEBUG_ADDR: u32 = MPUI_BASE + 0x4;
pub const MPUI_DEBUG_DATA: u32 = MPUI_BASE + 0x8;
pub const MPUI_DEBUG_FLAG: u32 = MPUI_BASE + 0xc;
pub const MPUI_STATUS_REG: u32 = MPUI_BASE + 0x10;
pub const MPUI_DSP_STATUS: u32 = MPUI_BASE + 0x14;
pub const MPUI_DSP_BOOT_CONFIG: u32 = MPUI_BASE + 0x18;
pub const MPUI_DSP_API_CONFIG: u32 = MPUI_BASE + 0x1c;

pub const OMAP_LPG1_BASE: u32 = 0xfffbd000;
pub const OMAP_LPG2_BASE: u32 = 0xfffbd800;
pub const OMAP_LPG1_LCR: u32 = OMAP_LPG1_BASE + 0x00;
pub const OMAP_LPG1_PMR: u32 = OMAP_LPG1_BASE + 0x04;
pub const OMAP_LPG2_LCR: u32 = OMAP_LPG2_BASE + 0x00;
pub const OMAP_LPG2_PMR: u32 = OMAP_LPG2_BASE + 0x04;

pub const OMAP1_DSP_BASE: u32 = 0xE0000000;
pub const OMAP1_DSP_SIZE: u32 = 0x28000;
pub const OMAP1_DSP_START: u32 = 0xE0000000;
pub const OMAP1_DSPREG_BASE: u32 = 0xE1000000;
// SZ_128K from linux/sizes.h.
pub const OMAP1_DSPREG_SIZE: u32 = 128 * 1024;
pub const OMAP1_DSPREG_START: u32 = 0xE1000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
