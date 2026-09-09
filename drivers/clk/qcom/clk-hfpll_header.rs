/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding clock-provider implementation:
// linux/clk-provider.h, linux/spinlock.h, and clk-regmap.h.

#[repr(C)]
pub struct hfpll_data {
    pub mode_reg: u32,
    pub l_reg: u32,
    pub m_reg: u32,
    pub n_reg: u32,
    pub user_reg: u32,
    pub droop_reg: u32,
    pub config_reg: u32,
    pub status_reg: u32,
    pub lock_bit: u8,

    pub l_val: u32,
    pub droop_val: u32,
    pub config_val: u32,
    pub user_val: u32,
    pub user_vco_mask: u32,
    pub low_vco_max_rate: ::core::ffi::c_ulong,

    pub min_rate: ::core::ffi::c_ulong,
    pub max_rate: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct clk_hfpll {
    pub d: *const hfpll_data,
    pub init_done: i32,

    pub clkr: clk_regmap,
    pub lock: spinlock_t,
}

// Equivalent of:
// container_of(to_clk_regmap(_hw), struct clk_hfpll, clkr)
#[macro_export]
macro_rules! to_clk_hfpll {
    ($hw:expr) => {
        container_of!(to_clk_regmap!($hw), clk_hfpll, clkr)
    };
}

extern "C" {
    pub static clk_ops_hfpll: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
