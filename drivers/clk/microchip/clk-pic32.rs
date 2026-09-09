// SPDX-License-Identifier: GPL-2.0-only
/* Purna Chandra Mandal, Copyright (C) 2015 Microchip Technology Inc. */

/* Linux dependencies and clk-pic32.h are supplied by the surrounding translation. */

const OSC_CUR_MASK: u32 = 0x07;
const OSC_CUR_SHIFT: u32 = 12;
const OSC_NEW_MASK: u32 = 0x07;
const OSC_NEW_SHIFT: u32 = 8;
const OSC_SWEN: u32 = 1 << 0;
const PLL_RANGE_MASK: u32 = 0x07;
const PLL_RANGE_SHIFT: u32 = 0;
const PLL_ICLK_MASK: u32 = 0x01;
const PLL_ICLK_SHIFT: u32 = 7;
const PLL_IDIV_MASK: u32 = 0x07;
const PLL_IDIV_SHIFT: u32 = 8;
const PLL_ODIV_MASK: u32 = 0x07;
const PLL_ODIV_SHIFT: u32 = 24;
const PLL_MULT_MASK: u32 = 0x7f;
const PLL_MULT_SHIFT: u32 = 16;
const PLL_MULT_MAX: u32 = 128;
const PLL_ODIV_MIN: u32 = 1;
const PLL_ODIV_MAX: u32 = 5;
const PB_DIV_MASK: u32 = 0x7f;
const PB_DIV_SHIFT: u32 = 0;
const PB_DIV_READY: u32 = 1 << 11;
const PB_DIV_ENABLE: u32 = 1 << 15;
const PB_DIV_MAX: u32 = 128;
const PB_DIV_MIN: u32 = 0;
const REFO_SEL_MASK: u32 = 0x0f;
const REFO_SEL_SHIFT: u32 = 0;
const REFO_ACTIVE: u32 = 1 << 8;
const REFO_DIVSW_EN: u32 = 1 << 9;
const REFO_OE: u32 = 1 << 12;
const REFO_ON: u32 = 1 << 15;
const REFO_DIV_SHIFT: u32 = 16;
const REFO_DIV_MASK: u32 = 0x7fff;
const REFO_TRIM_REG: usize = 0x10;
const REFO_TRIM_MASK: u32 = 0x1ff;
const REFO_TRIM_SHIFT: u32 = 23;
const REFO_TRIM_MAX: u32 = 511;
const SLEW_BUSY: u32 = 1 << 0;
const SLEW_DOWNEN: u32 = 1 << 1;
const SLEW_UPEN: u32 = 1 << 2;
const SLEW_DIV: u32 = 0x07;
const SLEW_DIV_SHIFT: u32 = 8;
const SLEW_SYSDIV: u32 = 0x0f;
const SLEW_SYSDIV_SHIFT: u32 = 20;
const LOCK_TIMEOUT_US: u32 = USEC_PER_MSEC;

static mut pic32_sclk_hw: *mut clk_hw = core::ptr::null_mut();

#[repr(C)]
struct pic32_periph_clk { hw: clk_hw, ctrl_reg: *mut core::ffi::c_void, core: *mut pic32_clk_common }
#[repr(C)]
struct pic32_ref_osc { hw: clk_hw, ctrl_reg: *mut core::ffi::c_void, parent_map: *const u32, core: *mut pic32_clk_common }
#[repr(C)]
struct pic32_sys_pll { hw: clk_hw, ctrl_reg: *mut core::ffi::c_void, status_reg: *mut core::ffi::c_void, lock_mask: u32, idiv: u32, core: *mut pic32_clk_common }
#[repr(C)]
struct pic32_sys_clk { hw: clk_hw, mux_reg: *mut core::ffi::c_void, slew_reg: *mut core::ffi::c_void, slew_div: u32, parent_map: *const u32, core: *mut pic32_clk_common }
#[repr(C)]
struct pic32_sec_osc { hw: clk_hw, enable_reg: *mut core::ffi::c_void, status_reg: *mut core::ffi::c_void, enable_mask: u32, status_mask: u32, fixed_rate: usize, core: *mut pic32_clk_common }

unsafe fn pbclk_is_enabled(hw: *mut clk_hw) -> i32 { (readl((*container_of!(hw, pic32_periph_clk, hw)).ctrl_reg) & PB_DIV_ENABLE) as i32 }
unsafe fn pbclk_enable(hw: *mut clk_hw) -> i32 { let pb = container_of!(hw, pic32_periph_clk, hw); writel(PB_DIV_ENABLE, PIC32_SET((*pb).ctrl_reg)); 0 }
unsafe fn pbclk_disable(hw: *mut clk_hw) { let pb = container_of!(hw, pic32_periph_clk, hw); writel(PB_DIV_ENABLE, PIC32_CLR((*pb).ctrl_reg)); }

unsafe fn calc_best_divided_rate(rate: usize, parent_rate: usize, max: u32, min: u32) -> usize {
    let div = (parent_rate / rate).clamp(min as usize, max as usize);
    let div_up = (div + 1).clamp(min as usize, max as usize);
    let a = parent_rate / div; let b = parent_rate / div_up;
    if (rate as isize - b as isize).unsigned_abs() < (rate as isize - a as isize).unsigned_abs() { b } else { a }
}
unsafe fn pbclk_read_pbdiv(pb: *mut pic32_periph_clk) -> u32 { ((readl((*pb).ctrl_reg) >> PB_DIV_SHIFT) & PB_DIV_MASK) + 1 }
unsafe fn pbclk_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize { let pb = container_of!(hw, pic32_periph_clk, hw); parent_rate / pbclk_read_pbdiv(pb) as usize }
unsafe fn pbclk_determine_rate(_: *mut clk_hw, req: *mut clk_rate_request) -> i32 { (*req).rate = calc_best_divided_rate((*req).rate, (*req).best_parent_rate, PB_DIV_MAX, PB_DIV_MIN); 0 }

unsafe fn roclk_is_enabled(hw: *mut clk_hw) -> i32 { let r=container_of!(hw,pic32_ref_osc,hw); (readl((*r).ctrl_reg)&REFO_ON) as i32 }
unsafe fn roclk_enable(hw:*mut clk_hw)->i32 {let r=container_of!(hw,pic32_ref_osc,hw);writel(REFO_ON|REFO_OE,PIC32_SET((*r).ctrl_reg));0}
unsafe fn roclk_disable(hw:*mut clk_hw){let r=container_of!(hw,pic32_ref_osc,hw);writel(REFO_ON|REFO_OE,PIC32_CLR((*r).ctrl_reg));}
unsafe fn roclk_init(hw:*mut clk_hw)->i32{roclk_disable(hw);0}
unsafe fn roclk_get_parent(hw:*mut clk_hw)->u8{let r=container_of!(hw,pic32_ref_osc,hw);let v=(readl((*r).ctrl_reg)>>REFO_SEL_SHIFT)&REFO_SEL_MASK;if !(*r).parent_map.is_null(){for i in 0..clk_hw_get_num_parents(hw){if *(*r).parent_map.add(i as usize)==v{return i as u8;}}}v as u8}
unsafe fn roclk_calc_rate(parent:usize, div:u32, trim:u32)->usize{if trim!=0{((parent as u128)<<8)/(((div as u128)<<9)+trim as u128) as usize}else if div!=0{parent/(div as usize*2)}else{parent}}
unsafe fn roclk_calc_div_trim(rate:usize,parent:usize,dp:*mut u32,tp:*mut u32){let (d,t)=if parent<=rate{(0,0)}else{let d=parent/(rate*2);let f=((parent as u128)<<8)/(rate as u128)-(d as u128<<9);(d.min(REFO_DIV_MASK as usize) as u32,(f as u32).min(REFO_TRIM_MAX))};if !dp.is_null(){*dp=d}if !tp.is_null(){*tp=t}}
unsafe fn roclk_recalc_rate(hw:*mut clk_hw,parent:usize)->usize{let r=container_of!(hw,pic32_ref_osc,hw);let d=(readl((*r).ctrl_reg)>>REFO_DIV_SHIFT)&REFO_DIV_MASK;let t=(readl((*r).ctrl_reg.add(REFO_TRIM_REG))>>REFO_TRIM_SHIFT)&REFO_TRIM_MASK;roclk_calc_rate(parent,d,t)}

unsafe fn spll_odiv_to_divider(mut o:u32)->u32{o=o.clamp(PLL_ODIV_MIN,PLL_ODIV_MAX);1<<o}
unsafe fn spll_calc_mult_div(p:*mut pic32_sys_pll,rate:usize,parent:usize,mp:*mut u32,op:*mut u32)->usize{let parent=parent/(*p).idiv;let(mut bm,mut bd,mut br,mut found)=(1,1,rate,false);let mut best_delta=usize::MAX;for m in 1..=PLL_MULT_MAX{for d in PLL_ODIV_MIN..=PLL_ODIV_MAX{let n=parent*m as usize/(1usize<<d);let delta=(rate as isize-n as isize).unsigned_abs();if n>=rate&&delta<best_delta{best_delta=delta;br=n;bm=m;bd=d;found=true}}}if !found{return 0}if !mp.is_null(){*mp=bm-1}if !op.is_null(){*op=bd}br}

// The remaining clk_ops tables and registration functions retain the C ABI and
// are represented using the surrounding kernel translation's declarations.
// TODO: direct body translation of the low-level setter/registration callbacks.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
