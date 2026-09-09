/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *      http://armlinux.simtec.co.uk/
 *      Ben Dooks <ben@simtec.co.uk>
 *
 * S3C64XX - syscon power and sleep control registers
 */

// Translated from the C header. `S3C_SYSREG!` is supplied by the dependent
// register definitions.

pub const S3C64XX_PWR_CFG: u32 = S3C_SYSREG!(0x804);

pub const S3C64XX_PWRCFG_OSC_OTG_DISABLE: u32 = 1 << 17;
pub const S3C64XX_PWRCFG_MMC2_DISABLE: u32 = 1 << 16;
pub const S3C64XX_PWRCFG_MMC1_DISABLE: u32 = 1 << 15;
pub const S3C64XX_PWRCFG_MMC0_DISABLE: u32 = 1 << 14;
pub const S3C64XX_PWRCFG_HSI_DISABLE: u32 = 1 << 13;
pub const S3C64XX_PWRCFG_TS_DISABLE: u32 = 1 << 12;
pub const S3C64XX_PWRCFG_RTC_TICK_DISABLE: u32 = 1 << 11;
pub const S3C64XX_PWRCFG_RTC_ALARM_DISABLE: u32 = 1 << 10;
pub const S3C64XX_PWRCFG_MSM_DISABLE: u32 = 1 << 9;
pub const S3C64XX_PWRCFG_KEY_DISABLE: u32 = 1 << 8;
pub const S3C64XX_PWRCFG_BATF_DISABLE: u32 = 1 << 7;

pub const S3C64XX_PWRCFG_CFG_WFI_MASK: u32 = 0x3 << 5;
pub const S3C64XX_PWRCFG_CFG_WFI_SHIFT: u32 = 5;
pub const S3C64XX_PWRCFG_CFG_WFI_IGNORE: u32 = 0x0 << 5;
pub const S3C64XX_PWRCFG_CFG_WFI_IDLE: u32 = 0x1 << 5;
pub const S3C64XX_PWRCFG_CFG_WFI_STOP: u32 = 0x2 << 5;
pub const S3C64XX_PWRCFG_CFG_WFI_SLEEP: u32 = 0x3 << 5;

pub const S3C64XX_PWRCFG_CFG_BATFLT_MASK: u32 = 0x3 << 3;
pub const S3C64XX_PWRCFG_CFG_BATFLT_SHIFT: u32 = 3;
pub const S3C64XX_PWRCFG_CFG_BATFLT_IGNORE: u32 = 0x0 << 3;
pub const S3C64XX_PWRCFG_CFG_BATFLT_IRQ: u32 = 0x1 << 3;
pub const S3C64XX_PWRCFG_CFG_BATFLT_SLEEP: u32 = 0x3 << 3;

pub const S3C64XX_PWRCFG_CFG_BAT_WAKE: u32 = 1 << 2;
pub const S3C64XX_PWRCFG_OSC27_EN: u32 = 1 << 0;

pub const S3C64XX_EINT_MASK: u32 = S3C_SYSREG!(0x808);
pub const S3C64XX_NORMAL_CFG: u32 = S3C_SYSREG!(0x810);

pub const S3C64XX_NORMALCFG_IROM_ON: u32 = 1 << 30;
pub const S3C64XX_NORMALCFG_DOMAIN_ETM_ON: u32 = 1 << 16;
pub const S3C64XX_NORMALCFG_DOMAIN_S_ON: u32 = 1 << 15;
pub const S3C64XX_NORMALCFG_DOMAIN_F_ON: u32 = 1 << 14;
pub const S3C64XX_NORMALCFG_DOMAIN_P_ON: u32 = 1 << 13;
pub const S3C64XX_NORMALCFG_DOMAIN_I_ON: u32 = 1 << 12;
pub const S3C64XX_NORMALCFG_DOMAIN_G_ON: u32 = 1 << 10;
pub const S3C64XX_NORMALCFG_DOMAIN_V_ON: u32 = 1 << 9;

pub const S3C64XX_STOP_CFG: u32 = S3C_SYSREG!(0x814);
pub const S3C64XX_STOPCFG_MEMORY_ARM_ON: u32 = 1 << 29;
pub const S3C64XX_STOPCFG_TOP_MEMORY_ON: u32 = 1 << 20;
pub const S3C64XX_STOPCFG_ARM_LOGIC_ON: u32 = 1 << 17;
pub const S3C64XX_STOPCFG_TOP_LOGIC_ON: u32 = 1 << 8;
pub const S3C64XX_STOPCFG_OSC_EN: u32 = 1 << 0;

pub const S3C64XX_SLEEP_CFG: u32 = S3C_SYSREG!(0x818);
pub const S3C64XX_SLEEPCFG_OSC_EN: u32 = 1 << 0;
pub const S3C64XX_STOP_MEM_CFG: u32 = S3C_SYSREG!(0x81c);
pub const S3C64XX_STOPMEMCFG_MODEMIF_RETAIN: u32 = 1 << 6;
pub const S3C64XX_STOPMEMCFG_HOSTIF_RETAIN: u32 = 1 << 5;
pub const S3C64XX_STOPMEMCFG_OTG_RETAIN: u32 = 1 << 4;
pub const S3C64XX_STOPMEMCFG_HSMCC_RETAIN: u32 = 1 << 3;
pub const S3C64XX_STOPMEMCFG_IROM_RETAIN: u32 = 1 << 2;
pub const S3C64XX_STOPMEMCFG_IRDA_RETAIN: u32 = 1 << 1;
pub const S3C64XX_STOPMEMCFG_NFCON_RETAIN: u32 = 1 << 0;

pub const S3C64XX_OSC_STABLE: u32 = S3C_SYSREG!(0x824);
pub const S3C64XX_PWR_STABLE: u32 = S3C_SYSREG!(0x828);
pub const S3C64XX_WAKEUP_STAT: u32 = S3C_SYSREG!(0x908);
pub const S3C64XX_WAKEUPSTAT_MMC2: u32 = 1 << 11;
pub const S3C64XX_WAKEUPSTAT_MMC1: u32 = 1 << 10;
pub const S3C64XX_WAKEUPSTAT_MMC0: u32 = 1 << 9;
pub const S3C64XX_WAKEUPSTAT_HSI: u32 = 1 << 8;
pub const S3C64XX_WAKEUPSTAT_BATFLT: u32 = 1 << 6;
pub const S3C64XX_WAKEUPSTAT_MSM: u32 = 1 << 5;
pub const S3C64XX_WAKEUPSTAT_KEY: u32 = 1 << 4;
pub const S3C64XX_WAKEUPSTAT_TS: u32 = 1 << 3;
pub const S3C64XX_WAKEUPSTAT_RTC_TICK: u32 = 1 << 2;
pub const S3C64XX_WAKEUPSTAT_RTC_ALARM: u32 = 1 << 1;
pub const S3C64XX_WAKEUPSTAT_EINT: u32 = 1 << 0;

pub const S3C64XX_BLK_PWR_STAT: u32 = S3C_SYSREG!(0x90c);
pub const S3C64XX_BLKPWRSTAT_G: u32 = 1 << 7;
pub const S3C64XX_BLKPWRSTAT_ETM: u32 = 1 << 6;
pub const S3C64XX_BLKPWRSTAT_S: u32 = 1 << 5;
pub const S3C64XX_BLKPWRSTAT_F: u32 = 1 << 4;
pub const S3C64XX_BLKPWRSTAT_P: u32 = 1 << 3;
pub const S3C64XX_BLKPWRSTAT_I: u32 = 1 << 2;
pub const S3C64XX_BLKPWRSTAT_V: u32 = 1 << 1;
pub const S3C64XX_BLKPWRSTAT_TOP: u32 = 1 << 0;

pub const S3C64XX_INFORM0: u32 = S3C_SYSREG!(0xA00);
pub const S3C64XX_INFORM1: u32 = S3C_SYSREG!(0xA04);
pub const S3C64XX_INFORM2: u32 = S3C_SYSREG!(0xA08);
pub const S3C64XX_INFORM3: u32 = S3C_SYSREG!(0xA0C);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
