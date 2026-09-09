/* SPDX-License-Identifier: GPL-2.0+
 *
 * Copyright 2013 Ideas On Board SPRL
 * Copyright 2013, 2014 Horms Solutions Ltd.
 *
 * Contact: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
 * Contact: Simon Horman <horms@verge.net.au>
 */

// Dependencies: linux/clk-provider.h, linux/types.h, linux/units.h

use core::ffi::c_void;

pub enum device {}
pub enum device_node {}
pub enum generic_pm_domain {}

pub unsafe extern "C" fn cpg_mstp_add_clk_domain(np: *mut device_node);

#[cfg(CONFIG_CLK_RENESAS_CPG_MSTP)]
pub unsafe extern "C" fn cpg_mstp_attach_dev(
    unused: *mut generic_pm_domain,
    dev: *mut device,
) -> i32;
#[cfg(CONFIG_CLK_RENESAS_CPG_MSTP)]
pub unsafe extern "C" fn cpg_mstp_detach_dev(
    unused: *mut generic_pm_domain,
    dev: *mut device,
);
#[cfg(not(CONFIG_CLK_RENESAS_CPG_MSTP))]
pub const cpg_mstp_attach_dev: Option<unsafe extern "C" fn(*mut generic_pm_domain, *mut device) -> i32> = None;
#[cfg(not(CONFIG_CLK_RENESAS_CPG_MSTP))]
pub const cpg_mstp_detach_dev: Option<unsafe extern "C" fn(*mut generic_pm_domain, *mut device)> = None;

#[cfg(CONFIG_CLK_RENESAS_CPG_MSSR)]
pub unsafe extern "C" fn cpg_mssr_attach_dev(
    unused: *mut generic_pm_domain,
    dev: *mut device,
) -> i32;
#[cfg(CONFIG_CLK_RENESAS_CPG_MSSR)]
pub unsafe extern "C" fn cpg_mssr_detach_dev(
    unused: *mut generic_pm_domain,
    dev: *mut device,
);
#[cfg(not(CONFIG_CLK_RENESAS_CPG_MSSR))]
pub const cpg_mssr_attach_dev: Option<unsafe extern "C" fn(*mut generic_pm_domain, *mut device) -> i32> = None;
#[cfg(not(CONFIG_CLK_RENESAS_CPG_MSSR))]
pub const cpg_mssr_detach_dev: Option<unsafe extern "C" fn(*mut generic_pm_domain, *mut device)> = None;

pub const PLL5_TARGET_DPI: i32 = 0;
pub const PLL5_TARGET_DSI: i32 = 1;

#[cfg(CONFIG_CLK_RZG2L)]
pub unsafe extern "C" fn rzg2l_cpg_dsi_div_set_divider(divider: u8, target: i32);
#[cfg(not(CONFIG_CLK_RZG2L))]
pub unsafe extern "C" fn rzg2l_cpg_dsi_div_set_divider(_divider: u8, _target: i32) {}

/**
 * struct rzv2h_pll_limits - PLL parameter constraints
 *
 * This structure defines the minimum and maximum allowed values for
 * various parameters used to configure a PLL. These limits ensure the
 * PLL operates within valid and stable ranges.
 *
 * @input_fref: Reference input frequency to the PLL (in Hz). If set
 * to 0, a default value of 24MHz is used.
 *
 * @fout: Output frequency range (in MHz)
 * @fout.min: Minimum allowed output frequency
 * @fout.max: Maximum allowed output frequency
 *
 * @fvco: PLL oscillation frequency range (in MHz)
 * @fvco.min: Minimum allowed VCO frequency
 * @fvco.max: Maximum allowed VCO frequency
 *
 * @m: Main-divider range
 * @m.min: Minimum main-divider value
 * @m.max: Maximum main-divider value
 *
 * @p: Pre-divider range
 * @p.min: Minimum pre-divider value
 * @p.max: Maximum pre-divider value
 *
 * @s: Divider range
 * @s.min: Minimum divider value
 * @s.max: Maximum divider value
 *
 * @k: Delta-sigma modulator range (signed)
 * @k.min: Minimum delta-sigma value
 * @k.max: Maximum delta-sigma value
 */
#[repr(C)]
pub struct rzv2h_pll_limits {
    pub input_fref: u32,
    pub fout: rzv2h_pll_limits_u32_pair,
    pub fvco: rzv2h_pll_limits_u32_pair,
    pub m: rzv2h_pll_limits_u16_pair,
    pub p: rzv2h_pll_limits_u8_pair,
    pub s: rzv2h_pll_limits_u8_pair,
    pub k: rzv2h_pll_limits_i16_pair,
}
#[repr(C)] pub struct rzv2h_pll_limits_u32_pair { pub min: u32, pub max: u32 }
#[repr(C)] pub struct rzv2h_pll_limits_u16_pair { pub min: u16, pub max: u16 }
#[repr(C)] pub struct rzv2h_pll_limits_u8_pair { pub min: u8, pub max: u8 }
#[repr(C)] pub struct rzv2h_pll_limits_i16_pair { pub min: i16, pub max: i16 }

#[repr(C)]
pub struct rzv2h_pll_pars {
    pub m: u16,
    pub p: u8,
    pub s: u8,
    pub k: i16,
    pub freq_millihz: u64,
    pub error_millihz: i64,
}

#[repr(C)]
pub struct rzv2h_pll_div_pars {
    pub pll: rzv2h_pll_pars,
    pub div: rzv2h_pll_div_pars_div,
}
#[repr(C)]
pub struct rzv2h_pll_div_pars_div {
    pub divider_value: u8,
    pub freq_millihz: u64,
    pub error_millihz: i64,
}

// MEGA is supplied by linux/units.h.
#[macro_export]
macro_rules! RZV2H_CPG_PLL_DSI_LIMITS {
    ($name:ident) => { pub const $name: $crate::rzv2h_pll_limits = $crate::rzv2h_pll_limits {
        input_fref: 0, fout: $crate::rzv2h_pll_limits_u32_pair { min: 25 * MEGA, max: 375 * MEGA },
        fvco: $crate::rzv2h_pll_limits_u32_pair { min: 1600 * MEGA, max: 3200 * MEGA },
        m: $crate::rzv2h_pll_limits_u16_pair { min: 64, max: 533 }, p: $crate::rzv2h_pll_limits_u8_pair { min: 1, max: 4 },
        s: $crate::rzv2h_pll_limits_u8_pair { min: 0, max: 6 }, k: $crate::rzv2h_pll_limits_i16_pair { min: -32768, max: 32767 },
    }; };
}
#[macro_export]
macro_rules! RZG3E_CPG_PLL_DSI0_LIMITS { ($name:ident) => { pub const $name: $crate::rzv2h_pll_limits = $crate::rzv2h_pll_limits { input_fref: 0, fout: $crate::rzv2h_pll_limits_u32_pair { min: 25 * MEGA, max: 1218 * MEGA }, fvco: $crate::rzv2h_pll_limits_u32_pair { min: 1600 * MEGA, max: 3200 * MEGA }, m: $crate::rzv2h_pll_limits_u16_pair { min: 64, max: 533 }, p: $crate::rzv2h_pll_limits_u8_pair { min: 1, max: 4 }, s: $crate::rzv2h_pll_limits_u8_pair { min: 0, max: 6 }, k: $crate::rzv2h_pll_limits_i16_pair { min: -32768, max: 32767 } }; }; }
#[macro_export]
macro_rules! RZG3E_CPG_PLL_DSI1_LIMITS { ($name:ident) => { pub const $name: $crate::rzv2h_pll_limits = $crate::rzv2h_pll_limits { input_fref: 0, fout: $crate::rzv2h_pll_limits_u32_pair { min: 25 * MEGA, max: 609 * MEGA }, fvco: $crate::rzv2h_pll_limits_u32_pair { min: 1600 * MEGA, max: 3200 * MEGA }, m: $crate::rzv2h_pll_limits_u16_pair { min: 64, max: 533 }, p: $crate::rzv2h_pll_limits_u8_pair { min: 1, max: 4 }, s: $crate::rzv2h_pll_limits_u8_pair { min: 0, max: 6 }, k: $crate::rzv2h_pll_limits_i16_pair { min: -32768, max: 32767 } }; }; }

#[cfg(CONFIG_CLK_RZV2H_CPG_LIB)]
pub unsafe extern "C" fn rzv2h_cpg_get_pll_pars(limits: *const rzv2h_pll_limits, pars: *mut rzv2h_pll_pars, freq_millihz: u64) -> bool;
#[cfg(CONFIG_CLK_RZV2H_CPG_LIB)]
pub unsafe extern "C" fn rzv2h_cpg_get_pll_divs_pars(limits: *const rzv2h_pll_limits, pars: *mut rzv2h_pll_div_pars, table: *const u8, table_size: u8, freq_millihz: u64) -> bool;
#[cfg(not(CONFIG_CLK_RZV2H_CPG_LIB))]
pub unsafe extern "C" fn rzv2h_cpg_get_pll_pars(_limits: *const rzv2h_pll_limits, _pars: *mut rzv2h_pll_pars, _freq_millihz: u64) -> bool { false }
#[cfg(not(CONFIG_CLK_RZV2H_CPG_LIB))]
pub unsafe extern "C" fn rzv2h_cpg_get_pll_divs_pars(_limits: *const rzv2h_pll_limits, _pars: *mut rzv2h_pll_div_pars, _table: *const u8, _table_size: u8, _freq_millihz: u64) -> bool { false }

pub use rzv2h_cpg_get_pll_pars as rzv2h_get_pll_pars;
pub use rzv2h_cpg_get_pll_divs_pars as rzv2h_get_pll_divs_pars;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
