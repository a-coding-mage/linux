/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SSM2518 amplifier audio driver
 *
 * Copyright 2013 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

pub const SSM2518_SYSCLK: i32 = 0;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ssm2518_sysclk_src {
    SSM2518_SYSCLK_SRC_MCLK = 0,
    SSM2518_SYSCLK_SRC_BCLK = 1,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
