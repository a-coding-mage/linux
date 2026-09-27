/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/arm/mach-sa1100/include/mach/collie.h
 *
 * This file contains the hardware specific definitions for Collie
 * Only include this file from SA1100-specific files.
 *
 * ChangeLog:
 *   04-06-2001 Lineo Japan, Inc.
 *   04-16-2001 SHARP Corporation
 *   07-07-2002 Chris Larson <clarson@digi.com>
 *
 */

// `hardware.h` supplies GPIO_MAX and the other hardware constants referenced below.

pub const COLLIE_SCOOP_GPIO_BASE: _ = GPIO_MAX + 1;
pub const COLLIE_GPIO_CHARGE_ON: _ = COLLIE_SCOOP_GPIO_BASE + 0;
pub const COLLIE_SCP_DIAG_BOOT1: u16 = SCOOP_GPCR_PA12;
pub const COLLIE_SCP_DIAG_BOOT2: u16 = SCOOP_GPCR_PA13;
pub const COLLIE_SCP_MUTE_L: u16 = SCOOP_GPCR_PA14;
pub const COLLIE_SCP_MUTE_R: u16 = SCOOP_GPCR_PA15;
pub const COLLIE_SCP_5VON: u16 = SCOOP_GPCR_PA16;
pub const COLLIE_SCP_AMP_ON: u16 = SCOOP_GPCR_PA17;
pub const COLLIE_GPIO_VPEN: _ = COLLIE_SCOOP_GPIO_BASE + 7;
pub const COLLIE_SCP_LB_VOL_CHG: u16 = SCOOP_GPCR_PA19;

pub const COLLIE_SCOOP_IO_DIR: u16 = COLLIE_SCP_MUTE_L
    | COLLIE_SCP_MUTE_R
    | COLLIE_SCP_5VON
    | COLLIE_SCP_AMP_ON
    | COLLIE_SCP_LB_VOL_CHG;
pub const COLLIE_SCOOP_IO_OUT: u16 = COLLIE_SCP_MUTE_L | COLLIE_SCP_MUTE_R;

/* GPIOs for gpiolib */

pub const COLLIE_GPIO_ON_KEY: u32 = 0;
pub const COLLIE_GPIO_AC_IN: u32 = 1;
pub const COLLIE_GPIO_SDIO_INT: u32 = 11;
pub const COLLIE_GPIO_CF_IRQ: u32 = 14;
pub const COLLIE_GPIO_nREMOCON_INT: u32 = 15;
pub const COLLIE_GPIO_UCB1x00_RESET: u32 = 16;
pub const COLLIE_GPIO_nMIC_ON: u32 = 17;
pub const COLLIE_GPIO_nREMOCON_ON: u32 = 18;
pub const COLLIE_GPIO_CO: u32 = 20;
pub const COLLIE_GPIO_MCP_CLK: u32 = 21;
pub const COLLIE_GPIO_CF_CD: u32 = 22;
pub const COLLIE_GPIO_UCB1x00_IRQ: u32 = 23;
pub const COLLIE_GPIO_WAKEUP: u32 = 24;
pub const COLLIE_GPIO_GA_INT: u32 = 25;
pub const COLLIE_GPIO_MAIN_BAT_LOW: u32 = 26;

/* GPIO definitions for direct register access */

pub const _COLLIE_GPIO_ON_KEY: _ = GPIO_GPIO(0);
pub const _COLLIE_GPIO_AC_IN: _ = GPIO_GPIO(1);
pub const _COLLIE_GPIO_nREMOCON_INT: _ = GPIO_GPIO(15);
pub const _COLLIE_GPIO_UCB1x00_RESET: _ = GPIO_GPIO(16);
pub const _COLLIE_GPIO_nMIC_ON: _ = GPIO_GPIO(17);
pub const _COLLIE_GPIO_nREMOCON_ON: _ = GPIO_GPIO(18);
pub const _COLLIE_GPIO_CO: _ = GPIO_GPIO(20);
pub const _COLLIE_GPIO_WAKEUP: _ = GPIO_GPIO(24);

/* Interrupts */

pub const COLLIE_IRQ_GPIO_ON_KEY: i32 = IRQ_GPIO0;
pub const COLLIE_IRQ_GPIO_AC_IN: i32 = IRQ_GPIO1;
pub const COLLIE_IRQ_GPIO_SDIO_IRQ: i32 = IRQ_GPIO11;
pub const COLLIE_IRQ_GPIO_CF_IRQ: i32 = IRQ_GPIO14;
pub const COLLIE_IRQ_GPIO_nREMOCON_INT: i32 = IRQ_GPIO15;
pub const COLLIE_IRQ_GPIO_CO: i32 = IRQ_GPIO20;
pub const COLLIE_IRQ_GPIO_CF_CD: i32 = IRQ_GPIO22;
pub const COLLIE_IRQ_GPIO_UCB1x00_IRQ: i32 = IRQ_GPIO23;
pub const COLLIE_IRQ_GPIO_WAKEUP: i32 = IRQ_GPIO24;
pub const COLLIE_IRQ_GPIO_GA_INT: i32 = IRQ_GPIO25;
pub const COLLIE_IRQ_GPIO_MAIN_BAT_LOW: i32 = IRQ_GPIO26;

/* GPIO's on the TC35143AF (Toshiba Analog Frontend) */
pub const COLLIE_TC35143_GPIO_BASE: _ = GPIO_MAX + 13;
pub const COLLIE_TC35143_GPIO_VERSION0: u32 = UCB_IO_0;
pub const COLLIE_TC35143_GPIO_TBL_CHK: u32 = UCB_IO_1;
pub const COLLIE_TC35143_GPIO_VPEN_ON: u32 = UCB_IO_2;
pub const COLLIE_GPIO_IR_ON: _ = COLLIE_TC35143_GPIO_BASE + 3;
pub const COLLIE_TC35143_GPIO_AMP_ON: u32 = UCB_IO_4;
pub const COLLIE_TC35143_GPIO_VERSION1: u32 = UCB_IO_5;
pub const COLLIE_TC35143_GPIO_FS8KLPF: u32 = UCB_IO_5;
pub const COLLIE_TC35143_GPIO_BUZZER_BIAS: u32 = UCB_IO_6;
pub const COLLIE_GPIO_MBAT_ON: _ = COLLIE_TC35143_GPIO_BASE + 7;
pub const COLLIE_GPIO_BBAT_ON: _ = COLLIE_TC35143_GPIO_BASE + 8;
pub const COLLIE_GPIO_TMP_ON: _ = COLLIE_TC35143_GPIO_BASE + 9;
pub const COLLIE_TC35143_GPIO_IN: u32 = UCB_IO_0 | UCB_IO_2 | UCB_IO_5;
pub const COLLIE_TC35143_GPIO_OUT: u32 = UCB_IO_1 | UCB_IO_3 | UCB_IO_4 | UCB_IO_6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
