/* SPDX-License-Identifier: GPL-2.0 */

// Translated from clk-exynos5-subcmu.h.

#[repr(C)]
pub struct exynos5_subcmu_reg_dump {
    pub offset: u32,
    pub value: u32,
    pub mask: u32,
    pub save: u32,
}

#[repr(C)]
pub struct exynos5_subcmu_info {
    pub div_clks: *const samsung_div_clock,
    pub nr_div_clks: core::ffi::c_uint,
    pub gate_clks: *const samsung_gate_clock,
    pub nr_gate_clks: core::ffi::c_uint,
    pub suspend_regs: *mut exynos5_subcmu_reg_dump,
    pub nr_suspend_regs: core::ffi::c_uint,
    pub pd_name: *const core::ffi::c_char,
}

unsafe extern "C" {
    pub fn exynos5_subcmus_init(
        ctx: *mut samsung_clk_provider,
        nr_cmus: core::ffi::c_int,
        cmu: *const *const exynos5_subcmu_info,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
