/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/include/net/ethoc.h
 *
 * Copyright (C) 2008-2009 Avionic Design GmbH
 *
 * Written by Thierry Reding <thierry.reding@avionic-design.de>
 */

// C dependencies: `IFHWADDRLEN` is supplied by the translated Linux network
// interface declarations.

#[repr(C)]
pub struct ethoc_platform_data {
    pub hwaddr: [u8; IFHWADDRLEN],
    pub phy_id: i8,
    pub eth_clkfreq: u32,
    pub big_endian: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
