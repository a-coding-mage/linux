/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/arm/mach-sa1100/include/mach/assabet.h
 *
 * Created 2000/06/05 by Nicolas Pitre <nico@fluxnic.net>
 *
 * This file contains the hardware specific definitions for Assabet
 * Only include this file from SA1100-specific files.
 *
 * 2000/05/23 John Dorsey <john+@cs.cmu.edu>
 *      Definitions for Neponset added.
 */

/* System Configuration Register flags */
pub const ASSABET_SCR_SDRAM_LOW: i32 = 1 << 2; /* SDRAM size (low bit) */
pub const ASSABET_SCR_SDRAM_HIGH: i32 = 1 << 3; /* SDRAM size (high bit) */
pub const ASSABET_SCR_FLASH_LOW: i32 = 1 << 4; /* Flash size (low bit) */
pub const ASSABET_SCR_FLASH_HIGH: i32 = 1 << 5; /* Flash size (high bit) */
pub const ASSABET_SCR_GFX: i32 = 1 << 8; /* Graphics Accelerator (0 = present) */
pub const ASSABET_SCR_SA1111: i32 = 1 << 9; /* Neponset (0 = present) */

pub const ASSABET_SCR_INIT: i32 = -1;

unsafe extern "C" {
    pub static mut SCR_value: ::core::ffi::c_ulong;
}

#[cfg(CONFIG_ASSABET_NEPONSET)]
#[inline]
pub unsafe fn machine_has_neponset() -> bool {
    (SCR_value & ASSABET_SCR_SA1111 as ::core::ffi::c_ulong) == 0
}

#[cfg(not(CONFIG_ASSABET_NEPONSET))]
#[inline]
pub const fn machine_has_neponset() -> i32 {
    0
}

/* Board Control Register */
pub const ASSABET_BCR_BASE: usize = 0xf1000000;
pub const ASSABET_BCR: *mut u32 = ASSABET_BCR_BASE as *mut u32;

pub const ASSABET_BCR_CF_PWR: u32 = 1 << 0; /* Compact Flash Power (1 = 3.3v, 0 = off) */
pub const ASSABET_BCR_CF_RST: u32 = 1 << 1; /* Compact Flash Reset (1 = power up reset) */
pub const ASSABET_BCR_NGFX_RST: u32 = 1 << 1; /* Graphics Accelerator Reset (0 = hold reset) */
pub const ASSABET_BCR_NCODEC_RST: u32 = 1 << 2; /* 0 = Holds UCB1300, ADI7171, and UDA1341 in reset */
pub const ASSABET_BCR_IRDA_FSEL: u32 = 1 << 3; /* IRDA Frequency select (0 = SIR, 1 = MIR/ FIR) */
pub const ASSABET_BCR_IRDA_MD0: u32 = 1 << 4; /* Range/Power select */
pub const ASSABET_BCR_IRDA_MD1: u32 = 1 << 5; /* Range/Power select */
pub const ASSABET_BCR_STEREO_LB: u32 = 1 << 6; /* Stereo Loopback */
pub const ASSABET_BCR_CF_BUS_OFF: u32 = 1 << 7; /* Compact Flash bus (0 = on, 1 = off (float)) */
pub const ASSABET_BCR_AUDIO_ON: u32 = 1 << 8; /* Audio power on */
pub const ASSABET_BCR_LIGHT_ON: u32 = 1 << 9; /* Backlight */
pub const ASSABET_BCR_LCD_12RGB: u32 = 1 << 10; /* 0 = 16RGB, 1 = 12RGB */
pub const ASSABET_BCR_LCD_ON: u32 = 1 << 11; /* LCD power on */
pub const ASSABET_BCR_RS232EN: u32 = 1 << 12; /* RS232 transceiver enable */
pub const ASSABET_BCR_LED_RED: u32 = 1 << 13; /* D9 (0 = on, 1 = off) */
pub const ASSABET_BCR_LED_GREEN: u32 = 1 << 14; /* D8 (0 = on, 1 = off) */
pub const ASSABET_BCR_VIB_ON: u32 = 1 << 15; /* Vibration motor (quiet alert) */
pub const ASSABET_BCR_COM_DTR: u32 = 1 << 16; /* COMport Data Terminal Ready */
pub const ASSABET_BCR_COM_RTS: u32 = 1 << 17; /* COMport Request To Send */
pub const ASSABET_BCR_RAD_WU: u32 = 1 << 18; /* Radio wake up interrupt */
pub const ASSABET_BCR_SMB_EN: u32 = 1 << 19; /* System management bus enable */
pub const ASSABET_BCR_TV_IR_DEC: u32 = 1 << 20; /* TV IR Decode Enable (not implemented) */
pub const ASSABET_BCR_QMUTE: u32 = 1 << 21; /* Quick Mute */
pub const ASSABET_BCR_RAD_ON: u32 = 1 << 22; /* Radio Power On */
pub const ASSABET_BCR_SPK_OFF: u32 = 1 << 23; /* 1 = Speaker amplifier power off */

#[cfg(CONFIG_SA1100_ASSABET)]
unsafe extern "C" {
    pub fn ASSABET_BCR_frob(mask: u32, set: u32);
}

#[cfg(not(CONFIG_SA1100_ASSABET))]
#[inline]
pub fn ASSABET_BCR_frob(_x: u32, _y: u32) {}

unsafe extern "C" {
    pub fn assabet_uda1341_reset(set: ::core::ffi::c_int);
}

#[inline]
pub unsafe fn ASSABET_BCR_set(x: u32) { ASSABET_BCR_frob(x, x); }

#[inline]
pub unsafe fn ASSABET_BCR_clear(x: u32) { ASSABET_BCR_frob(x, 0); }

pub const ASSABET_BSR_BASE: usize = 0xf1000000;
pub const ASSABET_BSR: *mut u32 = ASSABET_BSR_BASE as *mut u32;

pub const ASSABET_BSR_RS232_VALID: u32 = 1 << 24;
pub const ASSABET_BSR_COM_DCD: u32 = 1 << 25;
pub const ASSABET_BSR_COM_CTS: u32 = 1 << 26;
pub const ASSABET_BSR_COM_DSR: u32 = 1 << 27;
pub const ASSABET_BSR_RAD_CTS: u32 = 1 << 28;
pub const ASSABET_BSR_RAD_DSR: u32 = 1 << 29;
pub const ASSABET_BSR_RAD_DCD: u32 = 1 << 30;
pub const ASSABET_BSR_RAD_RI: u32 = 1 << 31;

/* GPIOs (bitmasks) for which the generic definition doesn't say much */
pub const ASSABET_GPIO_RADIO_IRQ: u32 = GPIO_GPIO(14); /* Radio interrupt request  */
pub const ASSABET_GPIO_PS_MODE_SYNC: u32 = GPIO_GPIO(16); /* Power supply mode/sync   */
pub const ASSABET_GPIO_STEREO_64FS_CLK: u32 = GPIO_GPIO(19); /* SSP UDA1341 clock input  */
pub const ASSABET_GPIO_GFX_IRQ: u32 = GPIO_GPIO(24); /* Graphics IRQ */
pub const ASSABET_GPIO_BATT_LOW: u32 = GPIO_GPIO(26); /* Low battery */
pub const ASSABET_GPIO_RCLK: u32 = GPIO_GPIO(26); /* CCLK/2  */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
