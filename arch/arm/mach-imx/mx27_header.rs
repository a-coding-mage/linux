/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2004-2007 Freescale Semiconductor, Inc. All Rights Reserved.
 * Copyright 2008 Juergen Beisert, kernel@pengutronix.de
 *
 * This contains i.MX27-specific hardware definitions. For those
 * hardware pieces that are common between i.MX21 and i.MX27, have a
 * look at mx2x.h.
 */

// `SZ_1M` and `IMX_IO_P2V` are supplied by the surrounding dependency set.

pub const MX27_AIPI_BASE_ADDR: u32 = 0x10000000;
pub const MX27_AIPI_SIZE: usize = SZ_1M;

pub const MX27_SAHB1_BASE_ADDR: u32 = 0x80000000;
pub const MX27_SAHB1_SIZE: usize = SZ_1M;

pub const MX27_X_MEMC_BASE_ADDR: u32 = 0xd8000000;
pub const MX27_X_MEMC_SIZE: usize = SZ_1M;

#[macro_export]
macro_rules! MX27_IO_P2V {
    ($x:expr) => {
        IMX_IO_P2V($x)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
