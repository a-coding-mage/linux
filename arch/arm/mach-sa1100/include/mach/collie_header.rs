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
pub const COLLIE_SCP_DIAG_BOOT1: _ = SCOOP_GPCR_PA12;
pub const COLLIE_SCP_DIAG_BOOT2: _ = SCOOP_GPCR_PA13;
pub const COLLIE_SCP_MUTE_L: _ = SCOOP_GPCR_PA14;
pub const COLLIE_SCP_MUTE_R: _ = SCOOP_GPCR_PA15;
pub const COLLIE_SCP_5VON: _ = SCOOP_GPCR_PA16;
pub const COLLIE_SCP_AMP_ON: _ = SCOOP_GPCR_PA17;
pub const COLLIE_GPIO_VPEN: _ = COLLIE_SCOOP_GPIO_BASE + 7;
pub const COLLIE_SCP_LB_VOL_CHG: _ = SCOOP_GPCR_PA19;

pub const COLLIE_SCOOP_IO_DIR: _ = COLLIE_SCP_MUTE_L
    | COLLIE_SCP_MUTE_R
    | COLLIE_SCP_5VON
    | COLLIE_SCP_AMP_ON
    | COLLIE_SCP_LB_VOL_CHG;
pub const COLLIE_SCOOP_IO_OUT: _ = COLLIE_SCP_MUTE_L | COLLIE_SCP_MUTE_R;

/* GPIOs for gpiolib */

pub const COLLIE_GPIO_ON_KEY: _ = 0;
pub const COLLIE_GPIO_AC_IN: _ = 1;
pub const COLLIE_GPIO_SDIO_INT: _ = 11;
pub const COLLIE_GPIO_CF_IRQ: _ = 14;
pub const COLLIE_GPIO_nREMOCON_INT: _ = 15;
pub const COLLIE_GPIO_UCB1x00_RESET: _ = 16;
pub const COLLIE_GPIO_nMIC_ON: _ = 17;
pub const COLLIE_GPIO_nREMOCON_ON: _ = 18;
pub const COLLIE_GPIO_CO: _ = 20;
pub const COLLIE_GPIO_MCP_CLK: _ = 21;
pub const COLLIE_GPIO_CF_CD: _ = 22;
pub const COLLIE_GPIO_UCB1x00_IRQ: _ = 23;
pub const COLLIE_GPIO_WAKEUP: _ = 24;
pub const COLLIE_GPIO_GA_INT: _ = 25;
pub const COLLIE_GPIO_MAIN_BAT_LOW: _ = 26;

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

pub const COLLIE_IRQ_GPIO_ON_KEY: _ = IRQ_GPIO0;
pub const COLLIE_IRQ_GPIO_AC_IN: _ = IRQ_GPIO1;
pub const COLLIE_IRQ_GPIO_SDIO_IRQ: _ = IRQ_GPIO11;
pub const COLLIE_IRQ_GPIO_CF_IRQ: _ = IRQ_GPIO14;
pub const COLLIE_IRQ_GPIO_nREMOCON_INT: _ = IRQ_GPIO15;
pub const COLLIE_IRQ_GPIO_CO: _ = IRQ_GPIO20;
pub const COLLIE_IRQ_GPIO_CF_CD: _ = IRQ_GPIO22;
pub const COLLIE_IRQ_GPIO_UCB1x00_IRQ: _ = IRQ_GPIO23;
pub const COLLIE_IRQ_GPIO_WAKEUP: _ = IRQ_GPIO24;
pub const COLLIE_IRQ_GPIO_GA_INT: _ = IRQ_GPIO25;
pub const COLLIE_IRQ_GPIO_MAIN_BAT_LOW: _ = IRQ_GPIO26;

/* GPIO's on the TC35143AF (Toshiba Analog Frontend) */
pub const COLLIE_TC35143_GPIO_BASE: _ = GPIO_MAX + 13;
pub const COLLIE_TC35143_GPIO_VERSION0: _ = UCB_IO_0;
pub const COLLIE_TC35143_GPIO_TBL_CHK: _ = UCB_IO_1;
pub const COLLIE_TC35143_GPIO_VPEN_ON: _ = UCB_IO_2;
pub const COLLIE_GPIO_IR_ON: _ = COLLIE_TC35143_GPIO_BASE + 3;
pub const COLLIE_TC35143_GPIO_AMP_ON: _ = UCB_IO_4;
pub const COLLIE_TC35143_GPIO_VERSION1: _ = UCB_IO_5;
pub const COLLIE_TC35143_GPIO_FS8KLPF: _ = UCB_IO_5;
pub const COLLIE_TC35143_GPIO_BUZZER_BIAS: _ = UCB_IO_6;
pub const COLLIE_GPIO_MBAT_ON: _ = COLLIE_TC35143_GPIO_BASE + 7;
pub const COLLIE_GPIO_BBAT_ON: _ = COLLIE_TC35143_GPIO_BASE + 8;
pub const COLLIE_GPIO_TMP_ON: _ = COLLIE_TC35143_GPIO_BASE + 9;
pub const COLLIE_TC35143_GPIO_IN: _ = UCB_IO_0 | UCB_IO_2 | UCB_IO_5;
pub const COLLIE_TC35143_GPIO_OUT: _ = UCB_IO_1 | UCB_IO_3 | UCB_IO_4 | UCB_IO_6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
