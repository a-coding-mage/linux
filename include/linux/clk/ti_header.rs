/* SPDX-License-Identifier: GPL-2.0-only */
/* TI clock drivers support (Rust translation of ti.h). */

use core::ffi::c_void;

#[repr(C)]
pub struct clk_omap_reg {
    pub ptr: *mut c_void,
    pub offset: u16,
    pub bit: u8,
    pub index: u8,
    pub flags: u8,
}

#[repr(C)]
pub struct dpll_data {
    pub mult_div1_reg: clk_omap_reg,
    pub mult_mask: u32,
    pub div1_mask: u32,
    pub clk_bypass: *mut clk_hw,
    pub clk_ref: *mut clk_hw,
    pub control_reg: clk_omap_reg,
    pub enable_mask: u32,
    pub last_rounded_rate: c_ulong,
    pub last_rounded_m: u16,
    pub last_rounded_m4xen: u8,
    pub last_rounded_lpmode: u8,
    pub max_multiplier: u16,
    pub last_rounded_n: u8,
    pub min_divider: u8,
    pub max_divider: u16,
    pub max_rate: c_ulong,
    pub modes: u8,
    pub autoidle_reg: clk_omap_reg,
    pub idlest_reg: clk_omap_reg,
    pub autoidle_mask: u32,
    pub freqsel_mask: u32,
    pub idlest_mask: u32,
    pub dco_mask: u32,
    pub sddiv_mask: u32,
    pub dcc_mask: u32,
    pub dcc_rate: c_ulong,
    pub lpmode_mask: u32,
    pub m4xen_mask: u32,
    pub auto_recal_bit: u8,
    pub recal_en_bit: u8,
    pub recal_st_bit: u8,
    pub ssc_deltam_reg: clk_omap_reg,
    pub ssc_modfreq_reg: clk_omap_reg,
    pub ssc_deltam_int_mask: u32,
    pub ssc_deltam_frac_mask: u32,
    pub ssc_modfreq_mant_mask: u32,
    pub ssc_modfreq_exp_mask: u32,
    pub ssc_enable_mask: u32,
    pub ssc_downspread_mask: u32,
    pub ssc_modfreq: u32,
    pub ssc_deltam: u32,
    pub ssc_downspread: bool,
    pub flags: u8,
}

#[repr(C)]
pub struct clk_hw_omap_ops {
    pub find_idlest: Option<unsafe extern "C" fn(*mut clk_hw_omap, *mut clk_omap_reg, *mut u8, *mut u8)>,
    pub find_companion: Option<unsafe extern "C" fn(*mut clk_hw_omap, *mut clk_omap_reg, *mut u8)>,
    pub allow_idle: Option<unsafe extern "C" fn(*mut clk_hw_omap)>,
    pub deny_idle: Option<unsafe extern "C" fn(*mut clk_hw_omap)>,
}

#[repr(C)]
pub struct clk_hw_omap {
    pub hw: clk_hw,
    pub node: list_head,
    pub fixed_rate: c_ulong,
    pub fixed_div: u8,
    pub enable_reg: clk_omap_reg,
    pub enable_bit: u8,
    pub flags: c_ulong,
    pub clksel_reg: clk_omap_reg,
    pub dpll_data: *mut dpll_data,
    pub clkdm_name: *const i8,
    pub clkdm: *mut clockdomain,
    pub ops: *const clk_hw_omap_ops,
    pub context: u32,
    pub autoidle_count: i32,
}

#[repr(C)]
pub struct ti_clk_ll_ops {
    pub clk_readl: Option<unsafe extern "C" fn(*const clk_omap_reg) -> u32>,
    pub clk_writel: Option<unsafe extern "C" fn(u32, *const clk_omap_reg)>,
    pub clk_rmw: Option<unsafe extern "C" fn(u32, u32, *const clk_omap_reg)>,
    pub clkdm_clk_enable: Option<unsafe extern "C" fn(*mut clockdomain, *mut clk) -> i32>,
    pub clkdm_clk_disable: Option<unsafe extern "C" fn(*mut clockdomain, *mut clk) -> i32>,
    pub clkdm_lookup: Option<unsafe extern "C" fn(*const i8) -> *mut clockdomain>,
    pub cm_wait_module_ready: Option<unsafe extern "C" fn(u8, i16, u16, u8) -> i32>,
    pub cm_split_idlest_reg: Option<unsafe extern "C" fn(*mut clk_omap_reg, *mut i16, *mut u8) -> i32>,
}

#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct clockdomain { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
pub type c_ulong = usize;

pub const ENABLE_REG_32BIT: u32 = 1 << 0;
pub const CLOCK_IDLE_CONTROL: u32 = 1 << 1;
pub const CLOCK_NO_IDLE_PARENT: u32 = 1 << 2;
pub const ENABLE_ON_INIT: u32 = 1 << 3;
pub const INVERT_ENABLE: u32 = 1 << 4;
pub const CLOCK_CLKOUTX2: u32 = 1 << 5;
pub const DPLL_LOW_POWER_STOP: u32 = 0x1;
pub const DPLL_LOW_POWER_BYPASS: u32 = 0x5;
pub const DPLL_LOCKED: u32 = 0x7;
pub const DPLL_J_TYPE: u32 = 0x1;

pub const TI_CLKM_CM: u32 = 0;
pub const TI_CLKM_CM2: u32 = 1;
pub const TI_CLKM_PRM: u32 = 2;
pub const TI_CLKM_SCRM: u32 = 3;
pub const TI_CLKM_CTRL: u32 = 4;
pub const TI_CLKM_CTRL_AUX: u32 = 5;
pub const TI_CLKM_PLLSS: u32 = 6;
pub const CLK_MAX_MEMMAPS: u32 = 7;

#[repr(C)]
pub struct ti_clk_features {
    pub flags: u32,
    pub fint_min: isize,
    pub fint_max: isize,
    pub fint_band1_max: isize,
    pub fint_band2_min: isize,
    pub dpll_bypass_vals: u8,
    pub cm_idlest_val: u8,
}

pub const TI_CLK_DPLL_HAS_FREQSEL: u32 = 1 << 0;
pub const TI_CLK_DPLL4_DENY_REPROGRAM: u32 = 1 << 1;
pub const TI_CLK_DISABLE_CLKDM_CONTROL: u32 = 1 << 2;
pub const TI_CLK_ERRATA_I810: u32 = 1 << 3;
pub const TI_CLK_CLKCTRL_COMPAT: u32 = 1 << 4;
pub const TI_CLK_DEVICE_TYPE_GP: u32 = 1 << 5;

/* C preprocessor dependency: container_of(_hw, struct clk_hw_omap, hw). */
extern "C" {
    pub fn omap2_clk_is_hw_omap(hw: *mut clk_hw) -> bool;
    pub fn omap2_clk_disable_autoidle_all() -> i32;
    pub fn omap2_clk_enable_autoidle_all() -> i32;
    pub fn omap2_clk_allow_idle(clk: *mut clk) -> i32;
    pub fn omap2_clk_deny_idle(clk: *mut clk) -> i32;
    pub fn omap2_dpllcore_recalc(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong;
    pub fn omap2_reprogram_dpllcore(clk: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> i32;
    pub fn omap2xxx_clkt_dpllcore_init(hw: *mut clk_hw);
    pub fn omap2xxx_clkt_vps_init();
    pub fn omap2_get_dpll_rate(clk: *mut clk_hw_omap) -> c_ulong;
    pub fn ti_dt_clk_init_retry_clks();
    pub fn ti_dt_clockdomains_setup();
    pub fn ti_clk_setup_ll_ops(ops: *mut ti_clk_ll_ops) -> i32;
    pub fn omap2_clk_provider_init(parent: *mut device_node, index: i32, syscon: *mut regmap, mem: *mut c_void) -> i32;
    pub fn omap2_clk_legacy_provider_init(index: i32, mem: *mut c_void);
    pub fn omap3430_dt_clk_init() -> i32;
    pub fn omap3630_dt_clk_init() -> i32;
    pub fn am35xx_dt_clk_init() -> i32;
    pub fn dm814x_dt_clk_init() -> i32;
    pub fn dm816x_dt_clk_init() -> i32;
    pub fn omap4xxx_dt_clk_init() -> i32;
    pub fn omap5xxx_dt_clk_init() -> i32;
    pub fn dra7xx_dt_clk_init() -> i32;
    pub fn am33xx_dt_clk_init() -> i32;
    pub fn am43xx_dt_clk_init() -> i32;
    pub fn omap2420_dt_clk_init() -> i32;
    pub fn omap2430_dt_clk_init() -> i32;
    pub fn ti_clk_setup_features(features: *mut ti_clk_features);
    pub fn ti_clk_get_features() -> *const ti_clk_features;
    pub fn ti_clk_is_in_standby(clk: *mut clk) -> bool;
    pub fn omap3_noncore_dpll_save_context(hw: *mut clk_hw) -> i32;
    pub fn omap3_noncore_dpll_restore_context(hw: *mut clk_hw);
    pub fn omap3_core_dpll_save_context(hw: *mut clk_hw) -> i32;
    pub fn omap3_core_dpll_restore_context(hw: *mut clk_hw);
    pub static clkhwops_omap2xxx_dpll: clk_hw_omap_ops;
}

/* CONFIG_ATAGS conditional: declarations are present when enabled; otherwise
 * the C inline functions return -ENXIO. */
#[cfg(feature = "CONFIG_ATAGS")]
extern "C" {
    pub fn omap3430_clk_legacy_init() -> i32;
    pub fn omap3430es1_clk_legacy_init() -> i32;
    pub fn omap36xx_clk_legacy_init() -> i32;
    pub fn am35xx_clk_legacy_init() -> i32;
}

#[cfg(not(feature = "CONFIG_ATAGS"))]
pub unsafe fn omap3430_clk_legacy_init() -> i32 { -6 }
#[cfg(not(feature = "CONFIG_ATAGS"))]
pub unsafe fn omap3430es1_clk_legacy_init() -> i32 { -6 }
#[cfg(not(feature = "CONFIG_ATAGS"))]
pub unsafe fn omap36xx_clk_legacy_init() -> i32 { -6 }
#[cfg(not(feature = "CONFIG_ATAGS"))]
pub unsafe fn am35xx_clk_legacy_init() -> i32 { -6 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
