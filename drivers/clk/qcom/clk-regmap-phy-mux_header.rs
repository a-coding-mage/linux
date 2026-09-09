/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2022, Linaro Ltd.
 */

// Dependency intent: this declaration uses types supplied by "clk-regmap.h".

/*
 * A clock implementation for PHY pipe and symbols clock muxes.
 *
 * If the clock is running off the from-PHY source, report it as enabled.
 * Report it as disabled otherwise (if it uses reference source).
 *
 * This way the PHY will disable the pipe clock before turning off the GDSC,
 * which in turn would lead to disabling corresponding pipe_clk_src (and thus
 * it being parked to a safe, reference clock source). And vice versa, after
 * enabling the GDSC the PHY will enable the pipe clock, which would cause
 * pipe_clk_src to be switched from a safe source to the working one.
 *
 * For some platforms this should be used for the UFS symbol_clk_src clocks
 * too.
 */
#[repr(C)]
pub struct clk_regmap_phy_mux {
    pub reg: u32,
    pub clkr: clk_regmap,
}

unsafe extern "C" {
    pub static clk_regmap_phy_mux_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
