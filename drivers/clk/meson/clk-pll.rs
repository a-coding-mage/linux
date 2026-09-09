// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2015 Endless Mobile, Inc.
 * Author: Carlo Caione <carlo@endlessm.com>
 *
 * Copyright (c) 2018 Baylibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

/*
 * In the most basic form, a Meson PLL is composed as follows:
 *
 *                     PLL
 *        +--------------------------------+
 *        |                                |
 *        |             +--+               |
 *  in >>-----[ /N ]--->|  |      +-----+  |
 *        |             |  |------| DCO |---->> out
 *        |  +--------->|  |      +--v--+  |
 *        |  |          +--+         |     |
 *        |  |                       |     |
 *        |  +--[ *(M + (F/Fmax) ]<--+     |
 *        |                                |
 *        +--------------------------------+
 *
 * out = in * (m + frac / frac_max) / n
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)] pub struct clk_regmap { pub data: *mut meson_clk_pll_data, pub map: *mut regmap }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong, pub best_parent_rate: c_ulong }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct meson_parm { pub width: c_uint }
#[repr(C)] pub struct meson_pll_table_entry { pub m: c_uint, pub n: c_uint }
#[repr(C)] pub struct meson_pll_range { pub min: c_uint, pub max: c_uint }
#[repr(C)] pub struct meson_clk_pll_data {
    pub flags: c_uint, pub frac_max: c_uint, pub frac: meson_parm, pub n: meson_parm,
    pub m: meson_parm, pub l: meson_parm, pub rst: meson_parm, pub en: meson_parm,
    pub current_en: meson_parm, pub l_detect: meson_parm,
    pub table: *const meson_pll_table_entry, pub range: *const meson_pll_range,
    pub init_regs: *const reg_sequence, pub init_count: c_uint,
}
#[repr(C)] pub struct reg_sequence { _private: [u8; 0] }
#[repr(C)] pub struct clk_ops {
    pub init: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
}
type c_int = i32; type c_uint = u32; type c_ulong = usize; type u64_ = u64;
const EINVAL: c_int = 22; const ENODATA: c_int = 61; const ETIMEDOUT: c_int = 110; const EIO: c_int = 5;
const CLK_MESON_PLL_ROUND_CLOSEST: c_uint = 1 << 0;
const CLK_MESON_PLL_NOINIT_ENABLED: c_uint = 1 << 1;

extern "C" {
    fn meson_parm_read(map: *mut regmap, parm: *const meson_parm) -> c_uint;
    fn meson_parm_write(map: *mut regmap, parm: *const meson_parm, value: c_uint);
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, count: c_uint) -> c_int;
    fn clk_regmap_init(hw: *mut clk_hw) -> c_int;
    fn to_clk_regmap(hw: *mut clk_hw) -> *mut clk_regmap;
    fn clk_hw_is_enabled(hw: *mut clk_hw) -> c_int;
    fn clk_hw_get_rate(hw: *mut clk_hw) -> c_ulong;
    fn clk_hw_get_name(hw: *mut clk_hw) -> *const u8;
    fn udelay(usecs: c_uint);
    fn pr_info(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
    fn meson_parm_applicable(parm: *const meson_parm) -> bool;
}

unsafe fn meson_clk_pll_data(clk: *mut clk_regmap) -> *mut meson_clk_pll_data { (*clk).data }
unsafe fn __pll_round_closest_mult(pll: *mut meson_clk_pll_data) -> c_int {
    if (*pll).flags & CLK_MESON_PLL_ROUND_CLOSEST != 0 && !meson_parm_applicable(&(*pll).frac) { 1 } else { 0 }
}
unsafe fn __pll_params_to_rate(parent_rate: c_ulong, m: c_uint, n: c_uint, frac: c_uint, pll: *mut meson_clk_pll_data) -> c_ulong {
    let frac_max = if (*pll).frac_max != 0 { (*pll).frac_max } else { 1u32 << (*pll).frac.width };
    let mut rate = (parent_rate as u64) * m as u64;
    if frac != 0 && meson_parm_applicable(&(*pll).frac) { rate += ((parent_rate as u64) * frac as u64 + frac_max as u64 - 1) / frac_max as u64; }
    ((rate + n as u64 - 1) / n as u64) as c_ulong
}
unsafe fn __pll_params_with_frac(rate: c_ulong, parent_rate: c_ulong, m: c_uint, n: c_uint, pll: *mut meson_clk_pll_data) -> c_uint {
    let frac_max = if (*pll).frac_max != 0 { (*pll).frac_max } else { 1u32 << (*pll).frac.width };
    if rate < parent_rate * m as usize / n as usize { return 0; }
    let val = (rate as u64 * n as u64 * frac_max as u64) / parent_rate as u64;
    (val.saturating_sub(m as u64 * frac_max as u64).min((frac_max - 1) as u64)) as c_uint
}
unsafe fn meson_clk_pll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let clk = to_clk_regmap(hw); let pll = meson_clk_pll_data(clk);
    let n = meson_parm_read((*clk).map, &(*pll).n); if n == 0 { return 0; }
    let m = meson_parm_read((*clk).map, &(*pll).m);
    let frac = if meson_parm_applicable(&(*pll).frac) { meson_parm_read((*clk).map, &(*pll).frac) } else { 0 };
    __pll_params_to_rate(parent_rate, m, n, frac, pll)
}
unsafe fn meson_clk_pll_is_better(rate: c_ulong, best: c_ulong, now: c_ulong, pll: *mut meson_clk_pll_data) -> bool {
    if __pll_round_closest_mult(pll) != 0 { now.abs_diff(rate) < best.abs_diff(rate) } else { now <= rate && best < now }
}
unsafe fn meson_clk_get_pll_table_index(index: c_uint, m: *mut c_uint, n: *mut c_uint, pll: *mut meson_clk_pll_data) -> c_int {
    let e = &*(*pll).table.add(index as usize); if e.n == 0 { return -EINVAL; } *m = e.m; *n = e.n; 0
}
unsafe fn meson_clk_get_pll_range_m(rate: c_ulong, parent_rate: c_ulong, n: c_uint, pll: *mut meson_clk_pll_data) -> c_uint { ((rate as u64 * n as u64) / parent_rate as u64) as c_uint }
unsafe fn meson_clk_get_pll_range_index(rate: c_ulong, parent_rate: c_ulong, index: c_uint, m: *mut c_uint, n: *mut c_uint, pll: *mut meson_clk_pll_data) -> c_int {
    *n = index + 1; if *n >= (1u32 << (*pll).n.width) { return -EINVAL; }
    if *n == 1 { let r = &*(*pll).range; if rate <= r.min as usize * parent_rate { *m = r.min; return -ENODATA; } if rate >= r.max as usize * parent_rate { *m = r.max; return -ENODATA; } }
    *m = meson_clk_get_pll_range_m(rate, parent_rate, *n, pll); if *m >= (1u32 << (*pll).m.width) { return -EINVAL; } 0
}
unsafe fn meson_clk_get_pll_get_index(rate: c_ulong, parent_rate: c_ulong, index: c_uint, m: *mut c_uint, n: *mut c_uint, pll: *mut meson_clk_pll_data) -> c_int {
    if !(*pll).range.is_null() { meson_clk_get_pll_range_index(rate, parent_rate, index, m, n, pll) } else if !(*pll).table.is_null() { meson_clk_get_pll_table_index(index, m, n, pll) } else { -EINVAL }
}
unsafe fn meson_clk_get_pll_settings(rate: c_ulong, parent_rate: c_ulong, best_m: *mut c_uint, best_n: *mut c_uint, pll: *mut meson_clk_pll_data) -> c_int {
    let mut best = 0; let mut i = 0; loop { let mut m = 0; let mut n = 0; let ret = meson_clk_get_pll_get_index(rate, parent_rate, i, &mut m, &mut n, pll); if ret == -EINVAL { break; } let now = __pll_params_to_rate(parent_rate, m, n, 0, pll); if meson_clk_pll_is_better(rate, best, now, pll) { best = now; *best_m = m; *best_n = n; if now == rate { break; } } i += 1; } if best != 0 { 0 } else { -EINVAL }
}

unsafe extern "C" fn meson_clk_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let clk = to_clk_regmap(hw); let pll = meson_clk_pll_data(clk); let mut m = 0; let mut n = 0;
    let ret = meson_clk_get_pll_settings((*req).rate, (*req).best_parent_rate, &mut m, &mut n, pll); if ret != 0 { return ret; }
    let round = __pll_params_to_rate((*req).best_parent_rate, m, n, 0, pll);
    if !meson_parm_applicable(&(*pll).frac) || (*req).rate == round { (*req).rate = round; return 0; }
    let frac = __pll_params_with_frac((*req).rate, (*req).best_parent_rate, m, n, pll);
    (*req).rate = __pll_params_to_rate((*req).best_parent_rate, m, n, frac, pll); 0
}
unsafe extern "C" fn meson_clk_pll_wait_lock(hw: *mut clk_hw) -> c_int {
    let clk = to_clk_regmap(hw); let pll = meson_clk_pll_data(clk); let mut delay = 5000;
    loop { if meson_parm_read((*clk).map, &(*pll).l) != 0 { return 0; } udelay(20); delay -= 1; if delay == 0 { return -ETIMEDOUT; } }
}
unsafe extern "C" fn meson_clk_pll_is_enabled(hw: *mut clk_hw) -> c_int { let clk=to_clk_regmap(hw); let p=meson_clk_pll_data(clk); if meson_parm_applicable(&(*p).rst)&&meson_parm_read((*clk).map,&(*p).rst)!=0{return 0;} if meson_parm_read((*clk).map,&(*p).en)==0||meson_parm_read((*clk).map,&(*p).l)==0{return 0;} 1 }
unsafe extern "C" fn meson_clk_pll_init(hw: *mut clk_hw) -> c_int { let clk=to_clk_regmap(hw); let p=meson_clk_pll_data(clk); let ret=clk_regmap_init(hw); if ret!=0{return ret;} if (*p).flags&CLK_MESON_PLL_NOINIT_ENABLED!=0&&meson_clk_pll_is_enabled(hw)!=0{return 0;} if (*p).init_count!=0 {if meson_parm_applicable(&(*p).rst){meson_parm_write((*clk).map,&(*p).rst,1);} regmap_multi_reg_write((*clk).map,(*p).init_regs,(*p).init_count); if meson_parm_applicable(&(*p).rst){meson_parm_write((*clk).map,&(*p).rst,0);}} 0 }
unsafe extern "C" fn meson_clk_pcie_pll_enable(hw:*mut clk_hw)->c_int { let mut r=10; loop {meson_clk_pll_init(hw);if meson_clk_pll_wait_lock(hw)==0{return 0;}r-=1;if r==0{return -EIO;}} }
unsafe extern "C" fn meson_clk_pll_enable(hw:*mut clk_hw)->c_int { let c=to_clk_regmap(hw);let p=meson_clk_pll_data(c);if clk_hw_is_enabled(hw)!=0{return 0;}if meson_parm_applicable(&(*p).rst){meson_parm_write((*c).map,&(*p).rst,1);}meson_parm_write((*c).map,&(*p).en,1);if meson_parm_applicable(&(*p).rst){meson_parm_write((*c).map,&(*p).rst,0);}if meson_parm_applicable(&(*p).current_en){udelay(10);meson_parm_write((*c).map,&(*p).current_en,1);udelay(40);}if meson_parm_applicable(&(*p).l_detect){meson_parm_write((*c).map,&(*p).l_detect,1);meson_parm_write((*c).map,&(*p).l_detect,0);}if meson_clk_pll_wait_lock(hw)!=0{-EIO}else{0} }
unsafe extern "C" fn meson_clk_pll_disable(hw:*mut clk_hw){let c=to_clk_regmap(hw);let p=meson_clk_pll_data(c);if meson_parm_applicable(&(*p).rst){meson_parm_write((*c).map,&(*p).rst,1);}meson_parm_write((*c).map,&(*p).en,0);if meson_parm_applicable(&(*p).current_en){meson_parm_write((*c).map,&(*p).current_en,0);}}
unsafe extern "C" fn meson_clk_pll_set_rate(hw:*mut clk_hw,rate:c_ulong,parent:c_ulong)->c_int{let c=to_clk_regmap(hw);let p=meson_clk_pll_data(c);if parent==0||rate==0{return -EINVAL;}let mut m=0;let mut n=0;let ret=meson_clk_get_pll_settings(rate,parent,&mut m,&mut n,p);if ret!=0{return ret;}let enabled=meson_parm_read((*c).map,&(*p).en);if enabled!=0{meson_clk_pll_disable(hw);}meson_parm_write((*c).map,&(*p).n,n);meson_parm_write((*c).map,&(*p).m,m);if meson_parm_applicable(&(*p).frac){meson_parm_write((*c).map,&(*p).frac,__pll_params_with_frac(rate,parent,m,n,p));}if enabled==0{return 0;}meson_clk_pll_enable(hw)}
pub static meson_clk_pcie_pll_ops: clk_ops = clk_ops { init: Some(clk_regmap_init), recalc_rate: Some(meson_clk_pll_recalc_rate), determine_rate: Some(meson_clk_pll_determine_rate), set_rate: None, is_enabled: Some(meson_clk_pll_is_enabled), enable: Some(meson_clk_pcie_pll_enable), disable: Some(meson_clk_pll_disable) };
pub static meson_clk_pll_ops: clk_ops = clk_ops { init: Some(meson_clk_pll_init), recalc_rate: Some(meson_clk_pll_recalc_rate), determine_rate: Some(meson_clk_pll_determine_rate), set_rate: Some(meson_clk_pll_set_rate), is_enabled: Some(meson_clk_pll_is_enabled), enable: Some(meson_clk_pll_enable), disable: Some(meson_clk_pll_disable) };
pub static meson_clk_pll_ro_ops: clk_ops = clk_ops { init: Some(clk_regmap_init), recalc_rate: Some(meson_clk_pll_recalc_rate), determine_rate: None, set_rate: None, is_enabled: Some(meson_clk_pll_is_enabled), enable: None, disable: None };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
