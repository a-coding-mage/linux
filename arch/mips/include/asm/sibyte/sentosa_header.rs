/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2000, 2001 Broadcom Corporation
 */

// Dependencies supplied by the corresponding sb1250 Rust translations:
// <asm/sibyte/sb1250.h>
// <asm/sibyte/sb1250_int.h>

#[cfg(feature = "CONFIG_SIBYTE_SENTOSA")]
pub const SIBYTE_BOARD_NAME: &str = "BCM91250E (Sentosa)";

#[cfg(feature = "CONFIG_SIBYTE_RHONE")]
pub const SIBYTE_BOARD_NAME: &str = "BCM91125E (Rhone)";

/* Generic bus chip selects */
#[cfg(feature = "CONFIG_SIBYTE_RHONE")]
pub const LEDS_CS: i32 = 6;

#[cfg(feature = "CONFIG_SIBYTE_RHONE")]
pub const LEDS_PHYS: u32 = 0x1d0a0000;

/* GPIOs */
pub const K_GPIO_DBG_LED: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
