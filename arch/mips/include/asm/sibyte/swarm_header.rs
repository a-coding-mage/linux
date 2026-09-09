/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2000, 2001, 2002, 2003 Broadcom Corporation
 */

// C dependencies: <asm/sibyte/sb1250.h> and <asm/sibyte/sb1250_int.h>

// These configuration sections correspond to the C preprocessor symbols
// CONFIG_SIBYTE_SWARM, CONFIG_SIBYTE_LITTLESUR, and CONFIG_SIBYTE_CRHONE.
#[cfg(feature = "CONFIG_SIBYTE_SWARM")]
pub const SIBYTE_BOARD_NAME: &str = "BCM91250A (SWARM)";
#[cfg(feature = "CONFIG_SIBYTE_SWARM")]
pub const SIBYTE_HAVE_PCMCIA: i32 = 1;
#[cfg(feature = "CONFIG_SIBYTE_SWARM")]
pub const SIBYTE_HAVE_IDE: i32 = 1;

#[cfg(feature = "CONFIG_SIBYTE_LITTLESUR")]
pub const SIBYTE_BOARD_NAME: &str = "BCM91250C2 (LittleSur)";
#[cfg(feature = "CONFIG_SIBYTE_LITTLESUR")]
pub const SIBYTE_HAVE_PCMCIA: i32 = 0;
#[cfg(feature = "CONFIG_SIBYTE_LITTLESUR")]
pub const SIBYTE_HAVE_IDE: i32 = 1;
#[cfg(feature = "CONFIG_SIBYTE_LITTLESUR")]
pub const SIBYTE_DEFAULT_CONSOLE: &str = "cfe0";

#[cfg(feature = "CONFIG_SIBYTE_CRHONE")]
pub const SIBYTE_BOARD_NAME: &str = "BCM91125C (CRhone)";
#[cfg(feature = "CONFIG_SIBYTE_CRHONE")]
pub const SIBYTE_HAVE_PCMCIA: i32 = 0;
#[cfg(feature = "CONFIG_SIBYTE_CRHONE")]
pub const SIBYTE_HAVE_IDE: i32 = 0;

/* Generic bus chip selects */
pub const LEDS_CS: i32 = 3;
pub const LEDS_PHYS: u32 = 0x100a0000;

// In the C header these items are enabled when SIBYTE_HAVE_IDE is defined.
// The board configurations above provide the corresponding configuration.
#[cfg(any(
    feature = "CONFIG_SIBYTE_SWARM",
    feature = "CONFIG_SIBYTE_LITTLESUR"
))]
pub const IDE_CS: i32 = 4;
#[cfg(any(
    feature = "CONFIG_SIBYTE_SWARM",
    feature = "CONFIG_SIBYTE_LITTLESUR"
))]
pub const IDE_PHYS: u32 = 0x100b0000;
#[cfg(any(
    feature = "CONFIG_SIBYTE_SWARM",
    feature = "CONFIG_SIBYTE_LITTLESUR"
))]
pub const K_GPIO_GB_IDE: i32 = 4;
#[cfg(any(
    feature = "CONFIG_SIBYTE_SWARM",
    feature = "CONFIG_SIBYTE_LITTLESUR"
))]
pub const K_INT_GB_IDE: i32 = K_INT_GPIO_0 + K_GPIO_GB_IDE;

// In the C header these items are enabled when SIBYTE_HAVE_PCMCIA is defined.
#[cfg(feature = "CONFIG_SIBYTE_SWARM")]
pub const PCMCIA_CS: i32 = 6;
#[cfg(feature = "CONFIG_SIBYTE_SWARM")]
pub const PCMCIA_PHYS: u32 = 0x11000000;
#[cfg(feature = "CONFIG_SIBYTE_SWARM")]
pub const K_GPIO_PC_READY: i32 = 9;
#[cfg(feature = "CONFIG_SIBYTE_SWARM")]
pub const K_INT_PC_READY: i32 = K_INT_GPIO_0 + K_GPIO_PC_READY;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
