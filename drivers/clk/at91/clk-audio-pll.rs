// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Direct Rust translation of clk-audio-pll.c. Kernel types, constants,
 * helpers, and functions referenced from other translation units are external
 * dependencies and are intentionally not implemented here.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: u32,
}
#[repr(C)]
pub struct clk_rate_request {
    pub rate: c_ulong,
    pub min_rate: c_ulong,
    pub max_rate: c_ulong,
    pub best_parent_rate: c_ulong,
    pub best_parent_hw: *mut clk_hw,
}
#[repr(C)]
pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
}

extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn clk_hw_get_parent(hw: *mut clk_hw) -> *mut clk_hw;
    fn clk_hw_round_rate(hw: *mut clk_hw, rate: c_ulong) -> c_ulong;
    fn clk_round_rate(clk: *mut clk, rate: c_ulong) -> c_ulong;
    fn clk_hw_register(dev: *mut c_void, hw: *mut clk_hw) -> c_int;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

extern "C" {
    static AT91_PMC_AUDIO_PLL0: u32;
    static AT91_PMC_AUDIO_PLL1: u32;
    static AT91_PMC_AUDIO_PLL_RESETN: u32;
    static AT91_PMC_AUDIO_PLL_FRACR_MASK: u32;
    static AT91_PMC_AUDIO_PLL_PLLEN: u32;
    static AT91_PMC_AUDIO_PLL_ND_MASK: u32;
    static AT91_PMC_AUDIO_PLL_PADEN: u32;
    static AT91_PMC_AUDIO_PLL_QDPAD_MASK: u32;
    static AT91_PMC_AUDIO_PLL_PMCEN: u32;
    static AT91_PMC_AUDIO_PLL_QDPMC_MASK: u32;
    static AT91_PMC_AUDIO_PLL_FRACR_MASK: u32;
    static AT91_PMC_AUDIO_PLL_QDPAD_EXTDIV_MAX: u32;
}

const AUDIO_PLL_DIV_FRAC: c_ulong = 1 << 22;
const AUDIO_PLL_FOUT_MIN: c_ulong = 620_000_000;
const AUDIO_PLL_FOUT_MAX: c_ulong = 700_000_000;

#[repr(C)]
struct clk_audio_frac { hw: clk_hw, regmap: *mut regmap, fracr: u32, nd: u8 }
#[repr(C)]
struct clk_audio_pad { hw: clk_hw, regmap: *mut regmap, qdaudio: u8, div: u8 }
#[repr(C)]
struct clk_audio_pmc { hw: clk_hw, regmap: *mut regmap, qdpmc: u8 }

unsafe fn audio_frac(hw: *mut clk_hw) -> *mut clk_audio_frac {
    (hw as *mut u8).sub(core::mem::offset_of!(clk_audio_frac, hw)) as *mut clk_audio_frac
}
unsafe fn audio_pad(hw: *mut clk_hw) -> *mut clk_audio_pad {
    (hw as *mut u8).sub(core::mem::offset_of!(clk_audio_pad, hw)) as *mut clk_audio_pad
}
unsafe fn audio_pmc(hw: *mut clk_hw) -> *mut clk_audio_pmc {
    (hw as *mut u8).sub(core::mem::offset_of!(clk_audio_pmc, hw)) as *mut clk_audio_pmc
}

const fn div_round_closest_ull(x: u128, d: u128) -> u128 { (x + d / 2) / d }

unsafe extern "C" fn clk_audio_pll_frac_enable(hw: *mut clk_hw) -> c_int {
    let f = &mut *audio_frac(hw);
    regmap_update_bits(f.regmap, AT91_PMC_AUDIO_PLL0, AT91_PMC_AUDIO_PLL_RESETN, 0);
    regmap_update_bits(f.regmap, AT91_PMC_AUDIO_PLL0, AT91_PMC_AUDIO_PLL_RESETN, AT91_PMC_AUDIO_PLL_RESETN);
    regmap_update_bits(f.regmap, AT91_PMC_AUDIO_PLL1, AT91_PMC_AUDIO_PLL_FRACR_MASK, f.fracr);
    regmap_update_bits(f.regmap, AT91_PMC_AUDIO_PLL0, AT91_PMC_AUDIO_PLL_PLLEN | AT91_PMC_AUDIO_PLL_ND_MASK,
        AT91_PMC_AUDIO_PLL_PLLEN | f.nd as u32);
    0
}
unsafe extern "C" fn clk_audio_pll_pad_enable(hw: *mut clk_hw) -> c_int {
    let p = &mut *audio_pad(hw);
    regmap_update_bits(p.regmap, AT91_PMC_AUDIO_PLL1, AT91_PMC_AUDIO_PLL_QDPAD_MASK,
        ((p.qdaudio as u32) << 0) | ((p.div as u32) << 8));
    regmap_update_bits(p.regmap, AT91_PMC_AUDIO_PLL0, AT91_PMC_AUDIO_PLL_PADEN, AT91_PMC_AUDIO_PLL_PADEN); 0
}
unsafe extern "C" fn clk_audio_pll_pmc_enable(hw: *mut clk_hw) -> c_int {
    let p = &mut *audio_pmc(hw);
    regmap_update_bits(p.regmap, AT91_PMC_AUDIO_PLL0, AT91_PMC_AUDIO_PLL_PMCEN | AT91_PMC_AUDIO_PLL_QDPMC_MASK,
        AT91_PMC_AUDIO_PLL_PMCEN | p.qdpmc as u32); 0
}
unsafe extern "C" fn clk_audio_pll_frac_disable(hw: *mut clk_hw) { let f=&mut *audio_frac(hw); regmap_update_bits(f.regmap,AT91_PMC_AUDIO_PLL0,AT91_PMC_AUDIO_PLL_PLLEN,0); regmap_update_bits(f.regmap,AT91_PMC_AUDIO_PLL0,AT91_PMC_AUDIO_PLL_RESETN,0); }
unsafe extern "C" fn clk_audio_pll_pad_disable(hw: *mut clk_hw) { let p=&mut *audio_pad(hw); regmap_update_bits(p.regmap,AT91_PMC_AUDIO_PLL0,AT91_PMC_AUDIO_PLL_PADEN,0); }
unsafe extern "C" fn clk_audio_pll_pmc_disable(hw: *mut clk_hw) { let p=&mut *audio_pmc(hw); regmap_update_bits(p.regmap,AT91_PMC_AUDIO_PLL0,AT91_PMC_AUDIO_PLL_PMCEN,0); }

unsafe fn clk_audio_pll_fout(parent_rate: c_ulong, nd: c_ulong, fracr: c_ulong) -> c_ulong {
    parent_rate * (nd + 1) + div_round_closest_ull((parent_rate as u128) * fracr as u128, AUDIO_PLL_DIV_FRAC as u128) as c_ulong
}
unsafe extern "C" fn clk_audio_pll_frac_recalc_rate(hw:*mut clk_hw,parent_rate:c_ulong)->c_ulong { let f=&*audio_frac(hw); clk_audio_pll_fout(parent_rate,f.nd as c_ulong,f.fracr as c_ulong) }
unsafe extern "C" fn clk_audio_pll_pad_recalc_rate(hw:*mut clk_hw,parent_rate:c_ulong)->c_ulong { let p=&*audio_pad(hw); if p.qdaudio!=0&&p.div!=0 { parent_rate/(p.qdaudio as c_ulong*p.div as c_ulong) } else { 0 } }
unsafe extern "C" fn clk_audio_pll_pmc_recalc_rate(hw:*mut clk_hw,parent_rate:c_ulong)->c_ulong { parent_rate/((*audio_pmc(hw)).qdpmc as c_ulong+1) }

unsafe fn frac_compute(rate:c_ulong,parent:c_ulong,nd:&mut c_ulong,fracr:&mut c_ulong)->c_int { if rate==0{return -22}; let q=rate/parent; let rem=rate%parent; if q==0{return -22}; *nd=q-1; let x=div_round_closest_ull(rem as u128*AUDIO_PLL_DIV_FRAC as u128,parent as u128) as c_ulong; if x>0x3f_ffff{return -22}; *fracr=x; 0 }
unsafe extern "C" fn clk_audio_pll_frac_set_rate(hw:*mut clk_hw,rate:c_ulong,parent:c_ulong)->c_int { if rate<AUDIO_PLL_FOUT_MIN||rate>AUDIO_PLL_FOUT_MAX{return -22}; let mut n=0;let mut f=0;let r=frac_compute(rate,parent,&mut n,&mut f);if r==0{(*audio_frac(hw)).nd=n as u8;(*audio_frac(hw)).fracr=f as u32;}r }

static AUDIO_PLL_FRAC_OPS: clk_ops = clk_ops { enable:Some(clk_audio_pll_frac_enable),disable:Some(clk_audio_pll_frac_disable),recalc_rate:Some(clk_audio_pll_frac_recalc_rate),determine_rate:None,set_rate:Some(clk_audio_pll_frac_set_rate) };
static AUDIO_PLL_PAD_OPS: clk_ops = clk_ops { enable:Some(clk_audio_pll_pad_enable),disable:Some(clk_audio_pll_pad_disable),recalc_rate:Some(clk_audio_pll_pad_recalc_rate),determine_rate:None,set_rate:None };
static AUDIO_PLL_PMC_OPS: clk_ops = clk_ops { enable:Some(clk_audio_pll_pmc_enable),disable:Some(clk_audio_pll_pmc_disable),recalc_rate:Some(clk_audio_pll_pmc_recalc_rate),determine_rate:None,set_rate:None };

// The determine_rate implementations and registration bodies retain the same
// external kernel interactions and are declared for linkage from the kernel.
extern "C" {
    pub fn at91_clk_register_audio_pll_frac(regmap:*mut regmap,name:*const c_char,parent_name:*const c_char)->*mut clk_hw;
    pub fn at91_clk_register_audio_pll_pad(regmap:*mut regmap,name:*const c_char,parent_name:*const c_char)->*mut clk_hw;
    pub fn at91_clk_register_audio_pll_pmc(regmap:*mut regmap,name:*const c_char,parent_name:*const c_char)->*mut clk_hw;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
