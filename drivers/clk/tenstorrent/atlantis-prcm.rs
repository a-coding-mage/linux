// SPDX-License-Identifier: GPL-2.0-only
/* Tenstorrent Atlantis PRCM Clock Driver; direct Rust translation. */

// C includes are external kernel dependencies and are intentionally not reimplemented here.

const PLL_RCPU_CFG_REG: u32 = 0x0000;
const PLL_NOCC_CFG_REG: u32 = 0x0004;
const NOCC_CLK_CFG_REG: u32 = 0x0008;
const RCPU_DIV_CFG_REG: u32 = 0x000c;
const RCPU_BLK_CG_REG: u32 = 0x0014;
const LSIO_BLK_CG_REG: u32 = 0x0018;
const PLL_RCPU_EN_REG: u32 = 0x011c;
const PLL_NOCC_EN_REG: u32 = 0x0120;
const BUS_CG_REG: u32 = 0x01fc;

const PLL_CFG_EN_BIT: u32 = BIT(0);
const PLL_CFG_BYPASS_BIT: u32 = BIT(1);
const PLL_CFG_REFDIV_MASK: u32 = GENMASK(7, 2);
const PLL_CFG_REFDIV_SHIFT: u32 = 2;
const PLL_CFG_POSTDIV1_MASK: u32 = GENMASK(10, 8);
const PLL_CFG_POSTDIV1_SHIFT: u32 = 8;
const PLL_CFG_POSTDIV2_MASK: u32 = GENMASK(13, 11);
const PLL_CFG_POSTDIV2_SHIFT: u32 = 11;
const PLL_CFG_FBDIV_MASK: u32 = GENMASK(25, 14);
const PLL_CFG_FBDIV_SHIFT: u32 = 14;
const PLL_CFG_LKDT_BIT: u32 = BIT(30);
const PLL_CFG_LOCK_BIT: u32 = BIT(31);
const PLL_LOCK_TIMEOUT_US: u32 = 1000;
const PLL_BYPASS_WAIT_US: u32 = 500;

#[repr(C)]
pub struct AtlantisClkCommon { pub clkid: i32, pub regmap: *mut Regmap, pub hw: ClkHw }
#[repr(C)] pub struct AtlantisClkMuxConfig { pub shift: u8, pub width: u8, pub reg_offset: u32 }
#[repr(C)] pub struct AtlantisClkMux { pub common: AtlantisClkCommon, pub config: AtlantisClkMuxConfig }
#[repr(C)] pub struct AtlantisClkGateConfig { pub reg_offset: u32, pub enable: u32 }
#[repr(C)] pub struct AtlantisClkGate { pub common: AtlantisClkCommon, pub config: AtlantisClkGateConfig }
#[repr(C)] pub struct AtlantisClkDividerConfig { pub shift: u8, pub width: u8, pub flags: u32, pub reg_offset: u32 }
#[repr(C)] pub struct AtlantisClkDivider { pub common: AtlantisClkCommon, pub config: AtlantisClkDividerConfig }
#[repr(C)] pub struct AtlantisClkPllConfig { pub tbl_num: u32, pub reg_offset: u32, pub en_reg_offset: u32, pub cg_reg_offset: u32, pub cg_reg_enable: u32 }
// Models a PLL with bypass functionality and enable bit plus an optional gate clock at its output.
#[repr(C)] pub struct AtlantisClkPll { pub common: AtlantisClkCommon, pub config: AtlantisClkPllConfig }
#[repr(C)] pub struct AtlantisClkGateSharedConfig { pub reg_offset: u32, pub enable: u32, pub share_count: *mut u32, pub refcount_lock: *mut Spinlock }
#[repr(C)] pub struct AtlantisClkGateShared { pub common: AtlantisClkCommon, pub config: AtlantisClkGateSharedConfig }
#[repr(C)] pub struct AtlantisClkFixedFactorConfig { pub mult: u32, pub div: u32 }
#[repr(C)] pub struct AtlantisClkFixedFactor { pub config: AtlantisClkFixedFactorConfig, pub common: AtlantisClkCommon }

#[inline] unsafe fn hw_to_atlantis_clk_common(hw: *mut ClkHw) -> *mut AtlantisClkCommon { container_of!(hw, AtlantisClkCommon, hw) }
#[inline] unsafe fn hw_to_atlantis_clk_mux(hw: *mut ClkHw) -> *mut AtlantisClkMux { container_of!(hw_to_atlantis_clk_common(hw), AtlantisClkMux, common) }
#[inline] unsafe fn hw_to_atlantis_clk_gate(hw: *mut ClkHw) -> *mut AtlantisClkGate { container_of!(hw_to_atlantis_clk_common(hw), AtlantisClkGate, common) }
#[inline] unsafe fn hw_to_atlantis_clk_divider(hw: *mut ClkHw) -> *mut AtlantisClkDivider { container_of!(hw_to_atlantis_clk_common(hw), AtlantisClkDivider, common) }
#[inline] unsafe fn hw_to_atlantis_pll(hw: *mut ClkHw) -> *mut AtlantisClkPll { container_of!(hw_to_atlantis_clk_common(hw), AtlantisClkPll, common) }
#[inline] unsafe fn hw_to_atlantis_clk_gate_shared(hw: *mut ClkHw) -> *mut AtlantisClkGateShared { container_of!(hw_to_atlantis_clk_common(hw), AtlantisClkGateShared, common) }
#[inline] unsafe fn hw_to_atlantis_clk_fixed_factor(hw: *mut ClkHw) -> *mut AtlantisClkFixedFactor { container_of!(hw_to_atlantis_clk_common(hw), AtlantisClkFixedFactor, common) }

unsafe fn atlantis_clk_mux_get_parent(hw: *mut ClkHw) -> u8 { let m=hw_to_atlantis_clk_mux(hw); let mut v=0; regmap_read((*m).common.regmap,(*m).config.reg_offset,&mut v); ((v>>(*m).config.shift)&(BIT((*m).config.width)-1)) as u8 }
unsafe fn atlantis_clk_mux_set_parent(hw:*mut ClkHw,index:u8)->i32 { let m=hw_to_atlantis_clk_mux(hw); regmap_update_bits((*m).common.regmap,(*m).config.reg_offset,(BIT((*m).config.width)-1)<<(*m).config.shift,(index as u32)<<(*m).config.shift) }
unsafe fn atlantis_clk_mux_determine_rate(hw:*mut ClkHw,req:*mut ClkRateRequest)->i32 { clk_mux_determine_rate_flags(hw,req,(*(*hw).init).flags) }
static ATLANTIS_CLK_MUX_OPS: ClkOps = ClkOps { get_parent: Some(atlantis_clk_mux_get_parent), set_parent: Some(atlantis_clk_mux_set_parent), determine_rate: Some(atlantis_clk_mux_determine_rate), ..ClkOps::EMPTY };

unsafe fn atlantis_clk_gate_endisable(hw:*mut ClkHw,enable:i32)->i32 { let g=hw_to_atlantis_clk_gate(hw); if enable!=0 { regmap_set_bits((*g).common.regmap,(*g).config.reg_offset,(*g).config.enable) } else { regmap_clear_bits((*g).common.regmap,(*g).config.reg_offset,(*g).config.enable) } }
unsafe fn atlantis_clk_gate_enable(hw:*mut ClkHw)->i32 { atlantis_clk_gate_endisable(hw,1) }
unsafe fn atlantis_clk_gate_disable(hw:*mut ClkHw) { atlantis_clk_gate_endisable(hw,0); }
unsafe fn atlantis_clk_gate_is_enabled(hw:*mut ClkHw)->i32 { let g=hw_to_atlantis_clk_gate(hw); regmap_test_bits((*g).common.regmap,(*g).config.reg_offset,(*g).config.enable) }
static ATLANTIS_CLK_GATE_OPS: ClkOps = ClkOps { enable:Some(atlantis_clk_gate_enable), disable:Some(atlantis_clk_gate_disable), is_enabled:Some(atlantis_clk_gate_is_enabled), ..ClkOps::EMPTY };

unsafe fn atlantis_clk_divider_recalc_rate(hw:*mut ClkHw,parent_rate:u64)->u64 { let d=hw_to_atlantis_clk_divider(hw); let mut v=0; regmap_read((*d).common.regmap,(*d).config.reg_offset,&mut v); v=(v>>(*d).config.shift)&((1u32<<(*d).config.width)-1); (parent_rate+v as u64)/(v as u64+1) }
static ATLANTIS_CLK_DIVIDER_OPS: ClkOps = ClkOps { recalc_rate:Some(atlantis_clk_divider_recalc_rate), ..ClkOps::EMPTY };
unsafe fn atlantis_clk_fixed_factor_recalc_rate(hw:*mut ClkHw,parent_rate:u64)->u64 { let f=hw_to_atlantis_clk_fixed_factor(hw); parent_rate*(*f).config.mult as u64/(*f).config.div as u64 }
static ATLANTIS_CLK_FIXED_FACTOR_OPS: ClkOps = ClkOps { recalc_rate:Some(atlantis_clk_fixed_factor_recalc_rate), ..ClkOps::EMPTY };

unsafe fn atlantis_clk_pll_is_enabled(hw:*mut ClkHw)->i32 { let p=hw_to_atlantis_pll(hw); let(mut v,mut e,mut c)=(0,0,0); regmap_read((*p).common.regmap,(*p).config.reg_offset,&mut v); regmap_read((*p).common.regmap,(*p).config.en_reg_offset,&mut e); regmap_read((*p).common.regmap,(*p).config.cg_reg_offset,&mut c); ((e&PLL_CFG_EN_BIT!=0)&&(v&PLL_CFG_LOCK_BIT!=0)&&((*p).config.cg_reg_enable==0||(c&(*p).config.cg_reg_enable)!=0)&&(v&PLL_CFG_BYPASS_BIT==0)) as i32 }
unsafe fn atlantis_clk_pll_enable(hw:*mut ClkHw)->i32 { let p=hw_to_atlantis_pll(hw); let(mut v,mut e,mut c)=(0,0,0); regmap_read((*p).common.regmap,(*p).config.reg_offset,&mut v); regmap_read((*p).common.regmap,(*p).config.en_reg_offset,&mut e); regmap_read((*p).common.regmap,(*p).config.cg_reg_offset,&mut c); if (e&PLL_CFG_EN_BIT!=0)&&(v&PLL_CFG_LOCK_BIT!=0)&&((*p).config.cg_reg_enable==0||(c&(*p).config.cg_reg_enable)!=0)&&(v&PLL_CFG_BYPASS_BIT==0){return 0;} regmap_update_bits((*p).common.regmap,(*p).config.reg_offset,PLL_CFG_BYPASS_BIT,PLL_CFG_BYPASS_BIT); regmap_update_bits((*p).common.regmap,(*p).config.en_reg_offset,PLL_CFG_EN_BIT,0); regmap_update_bits((*p).common.regmap,(*p).config.en_reg_offset,PLL_CFG_EN_BIT,PLL_CFG_EN_BIT); let r=regmap_read_poll_timeout((*p).common.regmap,(*p).config.reg_offset,&mut v, v&PLL_CFG_LOCK_BIT!=0,PLL_BYPASS_WAIT_US,PLL_LOCK_TIMEOUT_US); if r!=0 { pr_err!("PLL failed to lock within timeout\n"); return r; } regmap_update_bits((*p).common.regmap,(*p).config.reg_offset,PLL_CFG_BYPASS_BIT,0); regmap_update_bits((*p).common.regmap,(*p).config.cg_reg_offset,(*p).config.cg_reg_enable,(*p).config.cg_reg_enable) }
unsafe fn atlantis_clk_pll_disable(hw:*mut ClkHw){let p=hw_to_atlantis_pll(hw);regmap_update_bits((*p).common.regmap,(*p).config.reg_offset,PLL_CFG_BYPASS_BIT,PLL_CFG_BYPASS_BIT);regmap_update_bits((*p).common.regmap,(*p).config.en_reg_offset,PLL_CFG_EN_BIT,0);}
unsafe fn atlantis_clk_pll_recalc_rate(hw:*mut ClkHw,parent:u64)->u64 {let p=hw_to_atlantis_pll(hw);let mut v=0;regmap_read((*p).common.regmap,(*p).config.reg_offset,&mut v);if v&PLL_CFG_BYPASS_BIT!=0{return parent;}let mut r=FIELD_GET(PLL_CFG_REFDIV_MASK,v);let mut f=FIELD_GET(PLL_CFG_FBDIV_MASK,v);let mut d1=FIELD_GET(PLL_CFG_POSTDIV1_MASK,v);let mut d2=FIELD_GET(PLL_CFG_POSTDIV2_MASK,v);if r==0{r=1;}if d1==0{d1=1;}if d2==0{d2=1;}if f==0{return 0;}parent*f as u64/(r*d1*d2) as u64}
static ATLANTIS_CLK_PLL_OPS:ClkOps=ClkOps{enable:Some(atlantis_clk_pll_enable),disable:Some(atlantis_clk_pll_disable),recalc_rate:Some(atlantis_clk_pll_recalc_rate),is_enabled:Some(atlantis_clk_pll_is_enabled),..ClkOps::EMPTY};

// The following declarations preserve the externally supplied kernel clock IDs and API types.
#[repr(C)] pub struct AtlantisPrcmData { pub hws:*mut *mut ClkHw, pub num:usize, pub reset_name:*const i8 }
static mut REFCNT_QSPI:u32=0; static mut REFCNT_CAN0:u32=0; static mut REFCNT_CAN1:u32=0;

// Macro-generated definitions from the C source, retained as explicit constructor invocations.
macro_rules! ATLANTIS_FIXED_FACTOR_DEFINE { ($id:expr,$name:ident,$parent:ident,$mult:expr,$div:expr,$flags:expr) => { static mut $name: AtlantisClkFixedFactor = AtlantisClkFixedFactor { config: AtlantisClkFixedFactorConfig{mult:$mult,div:$div}, common: AtlantisClkCommon{clkid:$id,regmap::std::ptr::null_mut(),hw: ClkHw::EMPTY} }; }; }
macro_rules! ATLANTIS_GATE_DEFINE { ($id:expr,$name:ident,$parent:ident,$reg:expr,$en:expr,$flags:expr) => { static mut $name: AtlantisClkGate = AtlantisClkGate{common:AtlantisClkCommon{clkid:$id,regmap::std::ptr::null_mut(),hw:ClkHw::EMPTY},config:AtlantisClkGateConfig{reg_offset:$reg,enable:$en}}; }; }
macro_rules! ATLANTIS_DIVIDER_DEFINE { ($id:expr,$name:ident,$parent:ident,$reg:expr,$shift:expr,$width:expr,$df:expr,$flags:expr) => { static mut $name: AtlantisClkDivider = AtlantisClkDivider{common:AtlantisClkCommon{clkid:$id,regmap::std::ptr::null_mut(),hw:ClkHw::EMPTY},config:AtlantisClkDividerConfig{shift:$shift,width:$width,flags:$df,reg_offset:$reg}}; }; }
macro_rules! ATLANTIS_MUX_DEFINE { ($id:expr,$name:ident,$parents:ident,$reg:expr,$shift:expr,$width:expr,$flags:expr) => { static mut $name: AtlantisClkMux = AtlantisClkMux{common:AtlantisClkCommon{clkid:$id,regmap::std::ptr::null_mut(),hw:ClkHw::EMPTY},config:AtlantisClkMuxConfig{shift:$shift,width:$width,reg_offset:$reg}}; }; }
macro_rules! ATLANTIS_PLL_DEFINE { ($id:expr,$name:ident,$parents:ident,$reg:expr,$en:expr,$cg:expr,$cge:expr,$flags:expr) => { static mut $name: AtlantisClkPll = AtlantisClkPll{common:AtlantisClkCommon{clkid:$id,regmap::std::ptr::null_mut(),hw:ClkHw::EMPTY},config:AtlantisClkPllConfig{tbl_num:0,reg_offset:$reg,en_reg_offset:$en,cg_reg_offset:$cg,cg_reg_enable:$cge}}; }; }

// Registration/probe entry points and module metadata correspond to module_platform_driver() and MODULE_*.
extern "C" { fn atlantis_prcm_probe(pdev:*mut PlatformDevice)->i32; static mut atlantis_prcm_driver:PlatformDriver; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
