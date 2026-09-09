/* SPDX-License-Identifier: GPL-2.0-only */

// C dependencies: linux/clk-provider.h, linux/platform_device.h, linux/types.h

#[repr(C)]
pub struct ultrarisc_pll_layout {
    pub cfg1_offset: u32,
    pub cfg2_offset: u32,
    pub frac_mask: u32,
    pub m_mask: u32,
    pub n_mask: u32,
    pub oddiv1_mask: u32,
    pub oddiv2_mask: u32,
}

#[repr(C)]
pub struct ultrarisc_pll_desc {
    pub id: u32,
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct ultrarisc_fixed_factor_desc {
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub parent_id: u32,
    pub mult: u32,
    pub div: u32,
}

#[repr(C)]
pub struct ultrarisc_divider_desc {
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub offset: u32,
    pub parent_id: u32,
    pub max_rate: usize,
    pub load_mask: u32,
    pub div_shift: u8,
    pub div_width: u8,
    pub gate_bit: u8,
    pub divider_flags: u16,
    pub gate_flags: u8,
}

#[repr(C)]
pub struct ultrarisc_gate_desc {
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub offset: u32,
    pub parent_id: u32,
    pub gate_bit: u8,
    pub gate_flags: u8,
}

#[repr(C)]
pub struct ultrarisc_clk_soc_data {
    pub pll_layout: *const ultrarisc_pll_layout,
    pub plls: *const ultrarisc_pll_desc,
    pub num_plls: u32,
    pub fixed_factors: *const ultrarisc_fixed_factor_desc,
    pub num_fixed_factors: u32,
    pub dividers: *const ultrarisc_divider_desc,
    pub num_dividers: u32,
    pub gates: *const ultrarisc_gate_desc,
    pub num_gates: u32,
    pub num_clks: u32,
}

// External dependency supplied by the Linux platform-device bindings.
#[allow(improper_ctypes)]
extern "C" {
    pub fn ultrarisc_clk_probe(
        pdev: *mut platform_device,
        soc_data: *const ultrarisc_clk_soc_data,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
