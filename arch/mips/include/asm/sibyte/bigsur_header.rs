/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2000,2001,2002,2003,2004 Broadcom Corporation
 */

// Dependencies supplied by asm/sibyte/sb1250.h and
// asm/sibyte/bcm1480_int.h remain external to this translation.

// #ifdef CONFIG_SIBYTE_BIGSUR
#[cfg(feature = "CONFIG_SIBYTE_BIGSUR")]
pub const SIBYTE_BOARD_NAME: &str = "BCM91x80A/B (BigSur)";
#[cfg(feature = "CONFIG_SIBYTE_BIGSUR")]
pub const SIBYTE_HAVE_PCMCIA: i32 = 1;
#[cfg(feature = "CONFIG_SIBYTE_BIGSUR")]
pub const SIBYTE_HAVE_IDE: i32 = 1;

/* Generic bus chip selects */
pub const LEDS_CS: i32 = 3;
pub const LEDS_PHYS: u32 = 0x100a0000;

// #ifdef SIBYTE_HAVE_IDE
#[cfg(feature = "CONFIG_SIBYTE_BIGSUR")]
pub const IDE_CS: i32 = 4;
#[cfg(feature = "CONFIG_SIBYTE_BIGSUR")]
pub const IDE_PHYS: u32 = 0x100b0000;
#[cfg(feature = "CONFIG_SIBYTE_BIGSUR")]
pub const K_GPIO_GB_IDE: i32 = 4;
#[cfg(feature = "CONFIG_SIBYTE_BIGSUR")]
pub const K_INT_GB_IDE: i32 = K_INT_GPIO_0 + K_GPIO_GB_IDE;

// #ifdef SIBYTE_HAVE_PCMCIA
#[cfg(feature = "CONFIG_SIBYTE_BIGSUR")]
pub const PCMCIA_CS: i32 = 6;
#[cfg(feature = "CONFIG_SIBYTE_BIGSUR")]
pub const PCMCIA_PHYS: u32 = 0x11000000;
#[cfg(feature = "CONFIG_SIBYTE_BIGSUR")]
pub const K_GPIO_PC_READY: i32 = 9;
#[cfg(feature = "CONFIG_SIBYTE_BIGSUR")]
pub const K_INT_PC_READY: i32 = K_INT_GPIO_0 + K_GPIO_PC_READY;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
