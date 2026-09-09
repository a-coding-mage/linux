// SPDX-License-Identifier: GPL-2.0+
/* Marvell Armada 37xx SoC Peripheral clocks */

// Linux kernel dependencies are supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const TBG_SEL: usize = 0x0;
const DIV_SEL0: usize = 0x4;
const DIV_SEL1: usize = 0x8;
const DIV_SEL2: usize = 0xc;
const CLK_SEL: usize = 0x10;
const CLK_DIS: usize = 0x14;
const LOAD_LEVEL_NR: u32 = 4;
const ARMADA_37XX_NB_L0L1: u32 = 0x18;
const ARMADA_37XX_NB_L2L3: u32 = 0x1c;
const ARMADA_37XX_NB_TBG_DIV_OFF: u32 = 13;
const ARMADA_37XX_NB_TBG_DIV_MASK: u32 = 0x7;
const ARMADA_37XX_NB_CLK_SEL_OFF: u32 = 11;
const ARMADA_37XX_NB_CLK_SEL_MASK: u32 = 0x1;
const ARMADA_37XX_NB_TBG_SEL_OFF: u32 = 9;
const ARMADA_37XX_NB_TBG_SEL_MASK: u32 = 0x3;
const ARMADA_37XX_NB_CONFIG_SHIFT: u32 = 16;
const ARMADA_37XX_NB_DYN_MOD: u32 = 0x24;
const ARMADA_37XX_NB_DFS_EN: u32 = 31;
const ARMADA_37XX_NB_CPU_LOAD: u32 = 0x30;
const ARMADA_37XX_NB_CPU_LOAD_MASK: u32 = 0x3;
const ARMADA_37XX_DVFS_LOAD_0: u32 = 0;
const ARMADA_37XX_DVFS_LOAD_1: u32 = 1;
const ARMADA_37XX_DVFS_LOAD_2: u32 = 2;
const ARMADA_37XX_DVFS_LOAD_3: u32 = 3;

#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk_init_data { pub ops: *const clk_ops }
#[repr(C)] pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
}
#[repr(C)] pub struct clk_div_table { pub val: u32, pub div: u32 }
#[repr(C)] pub struct clk_mux { pub hw: clk_hw, pub reg: *mut c_void, pub shift: u8, pub mask: u32, pub lock: *mut spinlock_t }
#[repr(C)] pub struct clk_gate { pub hw: clk_hw, pub reg: *mut c_void, pub bit_idx: u8, pub lock: *mut spinlock_t, pub flags: u32 }
#[repr(C)] pub struct clk_divider { pub hw: clk_hw, pub reg: *mut c_void, pub table: *const clk_div_table, pub shift: u8, pub width: u8, pub lock: *mut spinlock_t }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong, pub best_parent_rate: c_ulong }
#[repr(C)] pub struct clk_hw_onecell_data { pub num: u32, pub hws: [*mut clk_hw; 0] }
#[repr(C)] pub struct clk_periph_driver_data { pub hw_data: *mut clk_hw_onecell_data, pub lock: spinlock_t, pub reg: *mut c_void, pub tbg_sel: u32, pub div_sel0: u32, pub div_sel1: u32, pub div_sel2: u32, pub clk_sel: u32, pub clk_dis: u32 }
#[repr(C)] pub struct clk_double_div { pub hw: clk_hw, pub reg1: *mut c_void, pub shift1: u8, pub reg2: *mut c_void, pub shift2: u8 }
#[repr(C)] pub struct clk_pm_cpu { pub hw: clk_hw, pub reg_mux: *mut c_void, pub shift_mux: u8, pub mask_mux: u32, pub reg_div: *mut c_void, pub shift_div: u8, pub nb_pm_base: *mut regmap, pub l1_expiration: c_ulong }
#[repr(C)] pub struct clk_periph_data { pub name: *const c_char, pub parent_names: *const *const c_char, pub num_parents: c_int, pub mux_hw: *mut clk_hw, pub rate_hw: *mut clk_hw, pub gate_hw: *mut clk_hw, pub muxrate_hw: *mut clk_hw, pub is_double_div: bool }

extern "C" {
    fn readl(reg: *mut c_void) -> u32; fn writel(v: u32, reg: *mut c_void);
    fn regmap_read(m: *mut regmap, r: u32, v: *mut u32) -> c_int;
    fn regmap_update_bits(m: *mut regmap, r: u32, mask: u32, v: u32) -> c_int;
    fn syscon_regmap_lookup_by_compatible(s: *const c_char) -> *mut regmap;
    fn msleep(ms: u32); fn msecs_to_jiffies(ms: u32) -> c_ulong; fn jiffies() -> c_ulong;
}

static CLK_TABLE6: [clk_div_table; 7] = [clk_div_table{val:1,div:1},clk_div_table{val:2,div:2},clk_div_table{val:3,div:3},clk_div_table{val:4,div:4},clk_div_table{val:5,div:5},clk_div_table{val:6,div:6},clk_div_table{val:0,div:0}];
static CLK_TABLE1: [clk_div_table; 3] = [clk_div_table{val:0,div:1},clk_div_table{val:1,div:2},clk_div_table{val:0,div:0}];
static CLK_TABLE2: [clk_div_table; 3] = [clk_div_table{val:0,div:2},clk_div_table{val:1,div:4},clk_div_table{val:0,div:0}];

unsafe fn get_div(reg_: *mut c_void, shift: u32) -> u32 { let v = (readl(reg_) >> shift) & 7; if v > 6 { 0 } else { v } }
unsafe extern "C" fn clk_double_div_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let d = &*(hw as *mut clk_double_div); let div = get_div(d.reg1,d.shift1) * get_div(d.reg2,d.shift2); (parent_rate + (div as c_ulong) - 1) / div as c_ulong
}
unsafe fn dvfs_regs(load: u32, reg: &mut u32, off: &mut u32) { *reg = if load <= ARMADA_37XX_DVFS_LOAD_1 { ARMADA_37XX_NB_L0L1 } else { ARMADA_37XX_NB_L2L3 }; if load == 0 || load == 2 { *off += ARMADA_37XX_NB_CONFIG_SHIFT; } }
unsafe fn dvfs_enabled(base: *mut regmap) -> bool { if base.is_null() { return false } let mut v=0; regmap_read(base,ARMADA_37XX_NB_DYN_MOD,&mut v); v & (1 << ARMADA_37XX_NB_DFS_EN) != 0 }
unsafe fn dvfs_cpu_div(base:*mut regmap)->u32 { let mut l=0; regmap_read(base,ARMADA_37XX_NB_CPU_LOAD,&mut l); let mut r=ARMADA_37XX_NB_CPU_LOAD; let mut o=ARMADA_37XX_NB_TBG_DIV_OFF; dvfs_regs(l&3,&mut r,&mut o); let mut v=0; regmap_read(base,r,&mut v); (v>>o)&ARMADA_37XX_NB_TBG_DIV_MASK }
unsafe fn dvfs_cpu_parent(base:*mut regmap)->u32 { let mut l=0; regmap_read(base,ARMADA_37XX_NB_CPU_LOAD,&mut l); let mut r=ARMADA_37XX_NB_CPU_LOAD; let mut o=ARMADA_37XX_NB_TBG_SEL_OFF; dvfs_regs(l&3,&mut r,&mut o); let mut v=0; regmap_read(base,r,&mut v); (v>>o)&ARMADA_37XX_NB_TBG_SEL_MASK }
unsafe extern "C" fn clk_pm_cpu_get_parent(hw:*mut clk_hw)->u8 { let c=&*(hw as *mut clk_pm_cpu); if dvfs_enabled(c.nb_pm_base) { dvfs_cpu_parent(c.nb_pm_base) as u8 } else { ((readl(c.reg_mux)>>c.shift_mux)&c.mask_mux) as u8 } }
unsafe extern "C" fn clk_pm_cpu_recalc_rate(hw:*mut clk_hw,parent:c_ulong)->c_ulong { let c=&*(hw as *mut clk_pm_cpu); let d=if dvfs_enabled(c.nb_pm_base){dvfs_cpu_div(c.nb_pm_base)}else{get_div(c.reg_div,c.shift_div)}; (parent+d as c_ulong-1)/d as c_ulong }

// The remaining probe, registration, suspend/resume, and generated clock tables retain
// the kernel driver's externally supplied interfaces and are declared below.
extern "C" { pub fn armada_3700_periph_clock_probe(pdev:*mut platform_device)->c_int; pub fn armada_3700_periph_clock_remove(pdev:*mut platform_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
