/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Header providing constants for bcm2835 pinctrl bindings.
 *
 * Copyright (C) 2015 Stefan Wahren <stefan.wahren@i2se.com>
 */

/* brcm,function property */
pub const BCM2835_FSEL_GPIO_IN: i32 = 0;
pub const BCM2835_FSEL_GPIO_OUT: i32 = 1;
pub const BCM2835_FSEL_ALT5: i32 = 2;
pub const BCM2835_FSEL_ALT4: i32 = 3;
pub const BCM2835_FSEL_ALT0: i32 = 4;
pub const BCM2835_FSEL_ALT1: i32 = 5;
pub const BCM2835_FSEL_ALT2: i32 = 6;
pub const BCM2835_FSEL_ALT3: i32 = 7;

/* brcm,pull property */
pub const BCM2835_PUD_OFF: i32 = 0;
pub const BCM2835_PUD_DOWN: i32 = 1;
pub const BCM2835_PUD_UP: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
