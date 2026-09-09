// SPDX-License-Identifier: GPL-2.0
/*
 * R-Car Gen2 Clock Pulse Generator
 *
 * Copyright (C) 2016 Cogent Embedded Inc.
 */

// Linux dependencies supplied by the surrounding kernel translation.

const CPG_FRQCRB: usize = 0x0004;
const CPG_FRQCRB_KICK: u32 = 1 << 31;
const CPG_SDCKCR: usize = 0x0074;
const CPG_PLL0CR: usize = 0x00d8;
const CPG_PLL0CR_STC_SHIFT: u32 = 24;
const CPG_PLL0CR_STC_MASK: u32 = 0x7f << CPG_PLL0CR_STC_SHIFT;
const CPG_FRQCRC: usize = 0x00e0;
const CPG_FRQCRC_ZFC_SHIFT: u32 = 8;
const CPG_FRQCRC_ZFC_MASK: u32 = 0x1f << CPG_FRQCRC_ZFC_SHIFT;
const CPG_ADSPCKCR: usize = 0x025c;
const CPG_RCANCKCR: usize = 0x0270;

static mut cpg_lock: Spinlock = Spinlock {};

#[repr(C)] pub struct Spinlock {}
#[repr(C)] pub struct ClkHw { pub init: *const ClkInitData }
#[repr(C)] pub struct Clk { _private: [u8; 0] }
#[repr(C)] pub struct ClkOps {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut ClkHw, *mut ClkRateRequest) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut ClkHw, c_ulong, c_ulong) -> c_int>,
}
#[repr(C)] pub struct ClkRateRequest { pub rate: c_ulong, pub min_rate: c_ulong, pub max_rate: c_ulong, pub best_parent_rate: c_ulong }
#[repr(C)] pub struct ClkInitData { pub name: *const c_char, pub ops: *const ClkOps, pub parent_names: *const *const c_char, pub num_parents: u8 }
#[repr(C)] pub struct ClkFixedFactor { pub hw: ClkHw, pub mult: u32, pub div: u32 }
#[repr(C)] pub struct ClkGate { pub hw: ClkHw, pub reg: *mut u8, pub bit_idx: u8, pub flags: u32, pub lock: *mut Spinlock }
#[repr(C)] pub struct ClkDivider { pub hw: ClkHw, pub reg: *mut u8, pub shift: u8, pub width: u8, pub table: *const ClkDivTable, pub lock: *mut Spinlock }
#[repr(C)] pub struct ClkDivTable { pub val: u32, pub div: u32 }
#[repr(C)] pub struct CpgZClk { pub hw: ClkHw, pub reg: *mut u8, pub kick_reg: *mut u8 }
#[repr(C)] pub struct Device { _private: [u8; 0] }
#[repr(C)] pub struct CpgCoreClk { pub name: *const c_char, pub parent: usize, pub r#type: u32 }
#[repr(C)] pub struct CpgMssrInfo { _private: [u8; 0] }
#[repr(C)] pub struct CpgMssrPub { pub base0: *mut u8, pub clks: *mut *mut Clk }
#[repr(C)] pub struct RcarGen2CpgPllConfig { pub extal_div: u32, pub pll0_mult: u32, pub pll1_mult: u32, pub pll3_mult: u32 }
#[repr(C)] pub struct SocDeviceAttribute { pub soc_id: *const c_char, pub data: *const core::ffi::c_void }

type c_ulong = usize; type c_int = i32; type c_char = i8; type u32_alias = u32;
const EINVAL: c_int = 22; const ENOMEM: c_int = 12; const EBUSY: c_int = 16; const ETIMEDOUT: c_int = 110;
const CLK_GATE_SET_TO_DISABLE: u32 = 1;

extern "C" {
    fn readl(addr: *mut u8) -> u32; fn writel(val: u32, addr: *mut u8);
    fn cpu_relax(); fn clk_register(_: *mut core::ffi::c_void, hw: *mut ClkHw) -> *mut Clk;
    fn clk_register_composite(_: *mut core::ffi::c_void, name: *const c_char, parents: *const *const c_char, n: u8, _: *mut ClkHw, _: *const ClkOps, _: *mut ClkHw, _: *const ClkOps, _: *mut ClkHw, _: *const ClkOps, flags: u32) -> *mut Clk;
    fn clk_register_fixed_factor(_: *mut core::ffi::c_void, name: *const c_char, parent: *const c_char, flags: u32, mult: u32, div: u32) -> *mut Clk;
    fn clk_register_divider_table(_: *mut core::ffi::c_void, name: *const c_char, parent: *const c_char, flags: u32, reg: *mut u8, shift: u8, width: u8, flags2: u32, table: *const ClkDivTable, lock: *mut Spinlock) -> *mut Clk;
    fn __clk_get_name(clk: *const Clk) -> *const c_char; fn soc_device_match(a: *const SocDeviceAttribute) -> *const SocDeviceAttribute;
}

static mut cpg_z_clk_ops: ClkOps = ClkOps { recalc_rate: Some(cpg_z_clk_recalc_rate), determine_rate: Some(cpg_z_clk_determine_rate), set_rate: Some(cpg_z_clk_set_rate) };

unsafe extern "C" fn cpg_z_clk_recalc_rate(hw: *mut ClkHw, parent_rate: c_ulong) -> c_ulong {
    let zclk = hw as *mut CpgZClk; let val = (readl((*zclk).reg) & CPG_FRQCRC_ZFC_MASK) >> CPG_FRQCRC_ZFC_SHIFT; let mult = 32 - val;
    ((parent_rate as u64 * mult as u64) / 32) as c_ulong
}
unsafe extern "C" fn cpg_z_clk_determine_rate(_: *mut ClkHw, req: *mut ClkRateRequest) -> c_int {
    let r = &mut *req; let min_mult = core::cmp::max(((r.min_rate as u64 * 32) / r.best_parent_rate as u64) as u32, 1); let max_mult = core::cmp::min(((r.max_rate as u64 * 32) / r.best_parent_rate as u64) as u32, 32); if max_mult < min_mult { return -EINVAL; }
    let mult = core::cmp::min(core::cmp::max(((r.rate as u64 * 32) / r.best_parent_rate as u64) as u32, min_mult), max_mult); r.rate = ((r.best_parent_rate as u64 * mult as u64) / 32) as c_ulong; 0
}
unsafe extern "C" fn cpg_z_clk_set_rate(hw: *mut ClkHw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let z = hw as *mut CpgZClk; let mult = core::cmp::min(core::cmp::max(((rate as u64 * 32) / parent_rate as u64) as u32, 1), 32); if readl((*z).kick_reg) & CPG_FRQCRB_KICK != 0 { return -EBUSY; }
    let mut val = readl((*z).reg) & !CPG_FRQCRC_ZFC_MASK; val |= (32 - mult) << CPG_FRQCRC_ZFC_SHIFT; writel(val, (*z).reg); let mut kick = readl((*z).kick_reg) | CPG_FRQCRB_KICK; writel(kick, (*z).kick_reg);
    for _ in 0..1000 { if readl((*z).kick_reg) & CPG_FRQCRB_KICK == 0 { return 0; } cpu_relax(); } -ETIMEDOUT
}

unsafe fn cpg_z_clk_register(name: *const c_char, parent_name: *const c_char, base: *mut u8) -> *mut Clk {
    let mut z = Box::new(CpgZClk { hw: ClkHw { init: core::ptr::null() }, reg: base.add(CPG_FRQCRC), kick_reg: base.add(CPG_FRQCRB) }); let parents = parent_name; let init = Box::new(ClkInitData { name, ops: &cpg_z_clk_ops, parent_names: &parents, num_parents: 1 }); z.hw.init = Box::into_raw(init); let p = Box::into_raw(z); clk_register(core::ptr::null_mut(), &mut (*p).hw)
}

static cpg_adsp_div_table: [ClkDivTable; 11] = [ClkDivTable{val:1,div:3},ClkDivTable{val:2,div:4},ClkDivTable{val:3,div:6},ClkDivTable{val:4,div:8},ClkDivTable{val:5,div:12},ClkDivTable{val:6,div:16},ClkDivTable{val:7,div:18},ClkDivTable{val:8,div:24},ClkDivTable{val:10,div:36},ClkDivTable{val:11,div:48},ClkDivTable{val:0,div:0}];
static cpg_sdh_div_table: [ClkDivTable; 12] = [ClkDivTable{val:0,div:2},ClkDivTable{val:1,div:3},ClkDivTable{val:2,div:4},ClkDivTable{val:3,div:6},ClkDivTable{val:4,div:8},ClkDivTable{val:5,div:12},ClkDivTable{val:6,div:16},ClkDivTable{val:7,div:18},ClkDivTable{val:8,div:24},ClkDivTable{val:10,div:36},ClkDivTable{val:11,div:48},ClkDivTable{val:0,div:0}];
static cpg_sd01_div_table: [ClkDivTable; 9] = [ClkDivTable{val:4,div:8},ClkDivTable{val:5,div:12},ClkDivTable{val:6,div:16},ClkDivTable{val:7,div:18},ClkDivTable{val:8,div:24},ClkDivTable{val:10,div:36},ClkDivTable{val:11,div:48},ClkDivTable{val:12,div:10},ClkDivTable{val:0,div:0}];

unsafe fn cpg_rcan_clk_register(name: *const c_char, parent: *const c_char, base: *mut u8) -> *mut Clk {
    let fixed = Box::into_raw(Box::new(ClkFixedFactor { hw: ClkHw { init: core::ptr::null() }, mult: 1, div: 6 }));
    let gate = Box::into_raw(Box::new(ClkGate { hw: ClkHw { init: core::ptr::null() }, reg: base.add(CPG_RCANCKCR), bit_idx: 8, flags: CLK_GATE_SET_TO_DISABLE, lock: &mut cpg_lock }));
    clk_register_composite(core::ptr::null_mut(), name, &parent, 1, core::ptr::null_mut(), core::ptr::null(), &mut (*fixed).hw, core::ptr::null(), &mut (*gate).hw, core::ptr::null(), 0)
}

unsafe fn cpg_adsp_clk_register(name: *const c_char, parent: *const c_char, base: *mut u8) -> *mut Clk {
    let div = Box::into_raw(Box::new(ClkDivider { hw: ClkHw { init: core::ptr::null() }, reg: base.add(CPG_ADSPCKCR), shift: 0, width: 4, table: cpg_adsp_div_table.as_ptr(), lock: &mut cpg_lock }));
    let gate = Box::into_raw(Box::new(ClkGate { hw: ClkHw { init: core::ptr::null() }, reg: base.add(CPG_ADSPCKCR), bit_idx: 8, flags: CLK_GATE_SET_TO_DISABLE, lock: &mut cpg_lock }));
    clk_register_composite(core::ptr::null_mut(), name, &parent, 1, core::ptr::null_mut(), core::ptr::null(), &mut (*div).hw, core::ptr::null(), &mut (*gate).hw, core::ptr::null(), 0)
}

static mut cpg_pll_config: *const RcarGen2CpgPllConfig = core::ptr::null(); static mut cpg_pll0_div: u32 = 0; static mut cpg_mode: u32 = 0; static mut cpg_quirks: u32 = 0;
const SD_SKIP_FIRST: u32 = 1;

pub unsafe fn rcar_gen2_cpg_clk_register(_: *mut Device, core: *const CpgCoreClk, _: *const CpgMssrInfo, pub_: *mut CpgMssrPub) -> *mut Clk {
    let p = &*pub_; let parent = *p.clks.add((*core).parent); if parent.is_null() { return core::ptr::null_mut(); } let parent_name = __clk_get_name(parent); let mut mult=1; let mut div=1;
    match (*core).r#type { 0 => div=(*cpg_pll_config).extal_div, 1 => {mult=(*cpg_pll_config).pll0_mult; div=cpg_pll0_div; if mult==0 {mult=(((readl(p.base0.add(CPG_PLL0CR))&CPG_PLL0CR_STC_MASK)>>CPG_PLL0CR_STC_SHIFT)+1)*2;}}, 2=>mult=(*cpg_pll_config).pll1_mult/2, 3=>mult=(*cpg_pll_config).pll3_mult, 4=>return cpg_z_clk_register((*core).name,parent_name,p.base0), 5=>div=if cpg_mode&(1<<18)!=0{36}else{24}, 6=>return cpg_adsp_clk_register((*core).name,parent_name,p.base0), 7=>return clk_register_fixed_factor(core::ptr::null_mut(),(*core).name,parent_name,0,mult,div), 8=>div=if cpg_mode&((1<<3)|(1<<2)|(1<<1))==(1<<2){8}else{10}, 9=>return cpg_rcan_clk_register((*core).name,parent_name,p.base0), _=>return core::ptr::null_mut() }
    clk_register_fixed_factor(core::ptr::null_mut(),(*core).name,parent_name,0,mult,div)
}

pub unsafe fn rcar_gen2_cpg_init(config: *const RcarGen2CpgPllConfig, pll0_div: u32, mode: u32) -> c_int { cpg_pll_config=config; cpg_pll0_div=pll0_div; cpg_mode=mode; cpg_quirks=0; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
