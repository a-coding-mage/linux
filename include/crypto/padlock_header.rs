/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Driver for VIA PadLock
 *
 * Copyright (c) 2004 Michal Ludvig <michal@logix.cz>
 */

pub const PADLOCK_ALIGNMENT: usize = 16;

pub const PFX: &str = concat!(KBUILD_MODNAME, ": ");

pub const PADLOCK_CRA_PRIORITY: i32 = 300;
pub const PADLOCK_COMPOSITE_PRIORITY: i32 = 400;

// The original condition is the build-time C CONFIG_64BIT setting.
#[cfg(feature = "CONFIG_64BIT")]
pub const STACK_ALIGN: usize = 16;

#[cfg(not(feature = "CONFIG_64BIT"))]
pub const STACK_ALIGN: usize = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
