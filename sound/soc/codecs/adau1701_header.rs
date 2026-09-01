// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * header file for ADAU1701 SigmaDSP processor
 *
 * Copyright 2011 Analog Devices Inc.
 */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum adau1701_clk_src {
    ADAU1701_CLK_SRC_OSC,
    ADAU1701_CLK_SRC_MCLK,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
