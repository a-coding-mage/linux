/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from clk-alpha-pll.h. */

// Dependencies supplied by other headers.
pub type U8 = u8;
pub type U32 = u32;
pub type SizeT = usize;
#[repr(C)] pub struct ClkRegmap;
#[repr(C)] pub struct Regmap;
#[repr(C)] pub struct ClkOps;
#[repr(C)] pub struct ClkDivTable;

#[repr(u32)]
pub enum ClkAlphaPllType {
    Default = 0,
    Huayra,
    HuayraApss,
    Huayra2290,
    Brammo,
    Fabia,
    Trion,
    Lucid = ClkAlphaPllType::Trion as u32,
    Agera,
    Zonda,
    Regera = ClkAlphaPllType::Zonda as u32,
    ZondaOle,
    LucidEvo,
    LucidOle,
    PongoElu,
    PongoEkoT = ClkAlphaPllType::PongoElu as u32,
    TaycanElu,
    TaycanEkoT = ClkAlphaPllType::TaycanElu as u32,
    TaycanEhaT = ClkAlphaPllType::TaycanElu as u32,
    RivianEvo,
    RivianElu,
    RivianEkoT = ClkAlphaPllType::RivianElu as u32,
    DefaultEvo,
    BrammoEvo,
    Stromer,
    StromerPlus,
    NssHuayra,
    Max,
}

pub const CLK_ALPHA_PLL_TYPE_MAX: usize = ClkAlphaPllType::Max as usize;

#[repr(usize)]
pub enum PllOffset {
    LVal, CalLVal, AlphaVal, AlphaValU, UserCtl, UserCtlU, UserCtlU1,
    ConfigCtl, ConfigCtlU, ConfigCtlU1, ConfigCtlU2, TestCtl, TestCtlU,
    TestCtlU1, TestCtlU2, TestCtlU3, State, Status, Opmode, Frac, CalVal,
    MaxRegs,
}
pub const PLL_OFF_MAX_REGS: usize = PllOffset::MaxRegs as usize;

pub const SUPPORTS_OFFLINE_REQ: u8 = 1 << 0;
pub const SUPPORTS_FSM_MODE: u8 = 1 << 2;
pub const SUPPORTS_DYNAMIC_UPDATE: u8 = 1 << 3;
pub const SUPPORTS_FSM_LEGACY_MODE: u8 = 1 << 4;

#[repr(C)]
pub struct PllVco {
    pub min_freq: c_ulong,
    pub max_freq: c_ulong,
    pub val: U32,
}
pub type CULong = usize;
pub type c_ulong = CULong;

#[repr(C)]
pub struct ClkAlphaPll {
    pub offset: U32,
    pub regs: *const U8,
    pub config: *const AlphaPllConfig,
    pub vco_table: *const PllVco,
    pub num_vco: SizeT,
    pub flags: U8,
    pub clkr: ClkRegmap,
}

#[repr(C)]
pub struct ClkAlphaPllPostdiv {
    pub offset: U32,
    pub width: U8,
    pub regs: *const U8,
    pub clkr: ClkRegmap,
    pub post_div_shift: i32,
    pub post_div_table: *const ClkDivTable,
    pub num_post_div: SizeT,
}

#[repr(C)]
pub struct AlphaPllConfig {
    pub l: U32, pub cal_l: U32, pub alpha: U32, pub alpha_hi: U32,
    pub config_ctl_val: U32, pub config_ctl_hi_val: U32,
    pub config_ctl_hi1_val: U32, pub config_ctl_hi2_val: U32,
    pub user_ctl_val: U32, pub user_ctl_hi_val: U32, pub user_ctl_hi1_val: U32,
    pub test_ctl_val: U32, pub test_ctl_mask: U32, pub test_ctl_hi_val: U32,
    pub test_ctl_hi_mask: U32, pub test_ctl_hi1_val: U32,
    pub test_ctl_hi2_val: U32, pub test_ctl_hi3_val: U32,
    pub main_output_mask: U32, pub aux_output_mask: U32, pub aux2_output_mask: U32,
    pub early_output_mask: U32, pub alpha_en_mask: U32, pub alpha_mode_mask: U32,
    pub pre_div_val: U32, pub pre_div_mask: U32, pub post_div_val: U32,
    pub post_div_mask: U32, pub vco_val: U32, pub vco_mask: U32,
    pub status_val: U32, pub status_mask: U32, pub lock_det: U32,
}

extern "C" {
    pub static clk_alpha_pll_regs: [[U8; PLL_OFF_MAX_REGS]; ClkAlphaPllType::Max as usize];
    pub static clk_alpha_pll_ops: ClkOps;
    pub static clk_alpha_pll_fixed_ops: ClkOps;
    pub static clk_alpha_pll_hwfsm_ops: ClkOps;
    pub static clk_alpha_pll_postdiv_ops: ClkOps;
    pub static clk_alpha_pll_huayra_ops: ClkOps;
    pub static clk_alpha_pll_postdiv_ro_ops: ClkOps;
    pub static clk_alpha_pll_stromer_ops: ClkOps;
    pub static clk_alpha_pll_stromer_plus_ops: ClkOps;
    pub static clk_alpha_pll_fabia_ops: ClkOps;
    pub static clk_alpha_pll_fixed_fabia_ops: ClkOps;
    pub static clk_alpha_pll_postdiv_fabia_ops: ClkOps;
    pub static clk_alpha_pll_trion_ops: ClkOps;
    pub static clk_alpha_pll_fixed_trion_ops: ClkOps;
    pub static clk_alpha_pll_postdiv_trion_ops: ClkOps;
    pub static clk_alpha_pll_lucid_ops: ClkOps;
    pub static clk_alpha_pll_postdiv_lucid_ops: ClkOps;
    pub static clk_alpha_pll_agera_ops: ClkOps;
    pub static clk_alpha_pll_lucid_5lpe_ops: ClkOps;
    pub static clk_alpha_pll_fixed_lucid_5lpe_ops: ClkOps;
    pub static clk_alpha_pll_postdiv_lucid_5lpe_ops: ClkOps;
    pub static clk_alpha_pll_zonda_ops: ClkOps;
    pub static clk_alpha_pll_lucid_evo_ops: ClkOps;
    pub static clk_alpha_pll_reset_lucid_evo_ops: ClkOps;
    pub static clk_alpha_pll_fixed_lucid_evo_ops: ClkOps;
    pub static clk_alpha_pll_postdiv_lucid_evo_ops: ClkOps;
    pub static clk_alpha_pll_pongo_elu_ops: ClkOps;
    pub static clk_alpha_pll_rivian_evo_ops: ClkOps;
    pub static clk_alpha_pll_regera_ops: ClkOps;
    pub static clk_alpha_pll_slew_ops: ClkOps;
}

pub const clk_alpha_pll_fixed_lucid_ops: *const ClkOps = &clk_alpha_pll_fixed_trion_ops;
pub const clk_alpha_pll_postdiv_zonda_ops: *const ClkOps = &clk_alpha_pll_postdiv_fabia_ops;
pub const clk_alpha_pll_zonda_ole_ops: *const ClkOps = &clk_alpha_pll_zonda_ops;

extern "C" {
    pub fn clk_alpha_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    pub fn clk_huayra_2290_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    pub fn clk_fabia_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    pub fn clk_trion_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    pub fn clk_agera_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    pub fn clk_zonda_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    pub fn clk_lucid_5lpe_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    pub fn clk_lucid_evo_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    pub fn clk_lucid_ole_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    pub fn clk_pongo_elu_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    pub fn clk_rivian_evo_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    pub fn clk_stromer_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    pub fn clk_regera_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap, config: *const AlphaPllConfig);
    pub fn qcom_clk_alpha_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap);
}

pub use clk_trion_pll_configure as clk_lucid_pll_configure;
pub use clk_lucid_evo_pll_configure as clk_taycan_elu_pll_configure;
pub use clk_lucid_evo_pll_configure as clk_taycan_eko_t_pll_configure;
pub use clk_lucid_evo_pll_configure as clk_taycan_eha_t_pll_configure;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
