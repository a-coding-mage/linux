/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/plat-spear/include/plat/pl080.h
 *
 * DMAC pl080 definitions for SPEAr platform
 *
 * Copyright (C) 2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

#[repr(C)]
pub struct pl08x_channel_data {
    _opaque: [u8; 0],
}

extern "C" {
    pub fn pl080_get_signal(cd: *const pl08x_channel_data) -> i32;
    pub fn pl080_put_signal(cd: *const pl08x_channel_data, signal: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
