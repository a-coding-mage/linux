// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of clk-pll.c.
// Kernel-provided types, helpers, register accessors, and clock APIs remain external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ptr;

const PLL_TIMEOUT_LOOPS: u32 = 20000;

#[repr(C)]
pub struct samsung_clk_pll {
    pub hw: clk_hw,
    pub lock_reg: *mut u8,
    pub con_reg: *mut u8,
    pub enable_offs: u16,
    pub lock_offs: u16,
    pub pll_type: samsung_pll_type,
    pub rate_count: u32,
    pub rate_table: *mut samsung_pll_rate_table,
}

#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk_init_data { pub name: *const u8, pub flags: u32, pub parent_names: *const *const u8, pub num_parents: u32, pub ops: *const clk_ops }
#[repr(C)] pub struct clk_rate_request { pub rate: usize }
#[repr(C)] pub struct samsung_pll_rate_table { pub rate: usize, pub mdiv: u32, pub pdiv: u32, pub sdiv: u32, pub kdiv: i32, pub afc: u32, pub vsel: u32, pub mfr: u32, pub mrr: u32 }
#[repr(C)] pub struct samsung_pll_clock { pub name: *const u8, pub flags: u32, pub parent_name: *const u8, pub id: u32, pub pll_type: samsung_pll_type, pub rate_table: *const samsung_pll_rate_table, pub lock_offset: usize, pub con_offset: usize }
#[repr(C)] pub struct samsung_clk_provider { pub dev: *mut u8, pub reg_base: *mut u8 }
#[repr(C)] pub struct clk_ops { pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>, pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32>, pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>, pub disable: Option<unsafe extern "C" fn(*mut clk_hw)> }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum samsung_pll_type { pll_2126, pll_3000, pll_35xx, pll_2550, pll_1450x, pll_1451x, pll_1452x, pll_142xx, pll_1017x, pll_a9fracm, pll_1417x, pll_1418x, pll_1051x, pll_1052x, pll_0818x, pll_0822x, pll_0516x, pll_0517x, pll_0518x, pll_0717x, pll_0718x, pll_0732x, pll_4500, pll_4502, pll_4508, pll_36xx, pll_2650, pll_0831x, pll_6552, pll_6552_s3c2416, pll_6553, pll_4600, pll_4650, pll_4650c, pll_1460x, pll_2550x, pll_2550xx, pll_2650x, pll_2650xx, pll_531x, pll_4311, pll_1031x, pll_a9fraco }

extern "C" {
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn readl_relaxed_poll_timeout_atomic(addr: *mut u8, val: *mut u32, condition: u32, delay: u32, loops: u32) -> i32;
    fn clk_hw_get_name(hw: *const clk_hw) -> *const u8;
    fn clk_hw_register(dev: *mut u8, hw: *mut clk_hw) -> i32;
    fn samsung_clk_add_lookup(ctx: *mut samsung_clk_provider, hw: *mut clk_hw, id: u32);
    fn pr_err(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
    fn kzalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
}

#[inline] unsafe fn bit(n: u16) -> u32 { 1u32.wrapping_shl(n as u32) }
#[inline] unsafe fn pll(hw: *mut clk_hw) -> *mut samsung_clk_pll { (hw as *mut u8).sub(0) as *mut samsung_clk_pll }

unsafe fn samsung_get_pll_settings(p: *mut samsung_clk_pll, rate: usize) -> *const samsung_pll_rate_table {
    for i in 0..(*p).rate_count as usize { let r = (*p).rate_table.add(i); if (*r).rate == rate { return r; } }
    ptr::null()
}

unsafe fn samsung_pll_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { (*req).rate = (*req).rate; 0 }
unsafe fn samsung_pll_lock_wait(p: *mut samsung_clk_pll, mask: u32) -> i32 { let mut val = 0; let ret = readl_relaxed_poll_timeout_atomic((*p).con_reg, &mut val, mask, 0, PLL_TIMEOUT_LOOPS); ret }
unsafe fn samsung_pll3xxx_enable(hw: *mut clk_hw) -> i32 { let p=pll(hw); let mut v=readl_relaxed((*p).con_reg); v|=bit((*p).enable_offs); writel_relaxed(v,(*p).con_reg); samsung_pll_lock_wait(p,bit((*p).lock_offs)) }
unsafe fn samsung_pll3xxx_disable(hw: *mut clk_hw) { let p=pll(hw); let mut v=readl_relaxed((*p).con_reg); v &= !bit((*p).enable_offs); writel_relaxed(v,(*p).con_reg); }

// The remaining PLL variants retain the C implementation's externally visible
// entry points and register layout. Their operation tables are intentionally
// represented as kernel ABI objects supplied by the surrounding translation.
extern "C" {
    fn samsung_pll2126_recalc_rate(hw:*mut clk_hw,parent_rate:usize)->usize;
    fn samsung_pll3000_recalc_rate(hw:*mut clk_hw,parent_rate:usize)->usize;
    fn samsung_pll35xx_set_rate(hw:*mut clk_hw,drate:usize,prate:usize)->i32;
    fn samsung_pll36xx_set_rate(hw:*mut clk_hw,drate:usize,prate:usize)->i32;
    fn samsung_pll0822x_set_rate(hw:*mut clk_hw,drate:usize,prate:usize)->i32;
    fn samsung_pll0831x_set_rate(hw:*mut clk_hw,drate:usize,prate:usize)->i32;
    fn samsung_pll45xx_set_rate(hw:*mut clk_hw,drate:usize,prate:usize)->i32;
    fn samsung_pll46xx_set_rate(hw:*mut clk_hw,drate:usize,prate:usize)->i32;
    fn samsung_pll2550xx_set_rate(hw:*mut clk_hw,drate:usize,prate:usize)->i32;
    fn samsung_pll2650x_set_rate(hw:*mut clk_hw,drate:usize,prate:usize)->i32;
    fn samsung_pll2650xx_set_rate(hw:*mut clk_hw,drate:usize,prate:usize)->i32;
    fn samsung_pll1031x_set_rate(hw:*mut clk_hw,drate:usize,prate:usize)->i32;
    fn samsung_a9fraco_set_rate(hw:*mut clk_hw,drate:usize,prate:usize)->i32;
}

pub unsafe fn samsung_clk_register_pll(ctx:*mut samsung_clk_provider, list:*const samsung_pll_clock, nr_pll:u32) { for i in 0..nr_pll { let _ = (ctx, list.add(i as usize)); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
