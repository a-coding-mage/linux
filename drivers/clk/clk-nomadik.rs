// SPDX-License-Identifier: GPL-2.0-only
/*
 * Nomadik clock implementation
 * Copyright (C) 2013 ST-Ericsson AB
 * Author: Linus Walleij <linus.walleij@linaro.org>
 */

// Linux dependencies supplied by the surrounding translation unit.

const SRC_CR: u32 = 0x00;
const SRC_CR_T0_ENSEL: u32 = 1 << 15;
const SRC_CR_T1_ENSEL: u32 = 1 << 17;
const SRC_CR_T2_ENSEL: u32 = 1 << 19;
const SRC_CR_T3_ENSEL: u32 = 1 << 21;
const SRC_CR_T4_ENSEL: u32 = 1 << 23;
const SRC_CR_T5_ENSEL: u32 = 1 << 25;
const SRC_CR_T6_ENSEL: u32 = 1 << 27;
const SRC_CR_T7_ENSEL: u32 = 1 << 29;
const SRC_XTALCR: u32 = 0x0c;
const SRC_XTALCR_XTALTIMEN: u32 = 1 << 20;
const SRC_XTALCR_SXTALDIS: u32 = 1 << 19;
const SRC_XTALCR_MXTALSTAT: u32 = 1 << 2;
const SRC_XTALCR_MXTALEN: u32 = 1 << 1;
const SRC_XTALCR_MXTALOVER: u32 = 1 << 0;
const SRC_PLLCR: u32 = 0x10;
const SRC_PLLCR_PLLTIMEN: u32 = 1 << 29;
const SRC_PLLCR_PLL2EN: u32 = 1 << 28;
const SRC_PLLCR_PLL1STAT: u32 = 1 << 2;
const SRC_PLLCR_PLL1EN: u32 = 1 << 1;
const SRC_PLLCR_PLL1OVER: u32 = 1 << 0;
const SRC_PLLFR: u32 = 0x14;
const SRC_PCKEN0: u32 = 0x24;
const SRC_PCKDIS0: u32 = 0x28;
const SRC_PCKENSR0: u32 = 0x2c;
const SRC_PCKSR0: u32 = 0x30;
const SRC_PCKEN1: u32 = 0x34;
const SRC_PCKDIS1: u32 = 0x38;
const SRC_PCKENSR1: u32 = 0x3c;
const SRC_PCKSR1: u32 = 0x40;

extern "C" {
    static mut src_lock: SpinLock;
    static mut src_base: *mut u8;
    fn readl(addr: *mut u8) -> u32;
    fn writel(val: u32, addr: *mut u8);
    fn cpu_relax();
    fn pr_crit(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn pr_debug(fmt: *const u8, ...);
    fn spin_lock(lock: *mut SpinLock);
    fn spin_unlock(lock: *mut SpinLock);
}

#[repr(C)] pub struct SpinLock { _private: [u8; 0] }
#[repr(C)] pub struct Device { _private: [u8; 0] }
#[repr(C)] pub struct DeviceNode { pub name: *const u8 }
#[repr(C)] pub struct NotifierBlock { pub notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, usize, *mut core::ffi::c_void) -> i32> }
#[repr(C)] pub struct ClkHw { pub init: *mut ClkInitData }
#[repr(C)] pub struct ClkInitData { pub name: *const u8, pub ops: *const ClkOps, pub parent_names: *const *const u8, pub num_parents: u32, pub flags: u32 }
#[repr(C)] pub struct ClkOps { pub enable: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>, pub disable: Option<unsafe extern "C" fn(*mut ClkHw)>, pub is_enabled: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>, pub recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw, usize) -> usize> }

#[repr(C)] pub struct ClkPll { pub hw: ClkHw, pub id: i32 }
#[repr(C)] pub struct ClkSrc { pub hw: ClkHw, pub id: i32, pub group1: bool, pub clkbit: u32 }

#[inline] unsafe fn pll_from_hw(hw: *mut ClkHw) -> *mut ClkPll { hw as *mut ClkPll }
#[inline] unsafe fn src_from_hw(hw: *mut ClkHw) -> *mut ClkSrc { hw as *mut ClkSrc }
#[inline] unsafe fn reg(off: u32) -> *mut u8 { src_base.add(off as usize) }

unsafe extern "C" fn pll_clk_enable(hw: *mut ClkHw) -> i32 {
    let pll = &mut *pll_from_hw(hw); spin_lock(&mut src_lock); let mut val = readl(reg(SRC_PLLCR));
    if pll.id == 1 { if val & SRC_PLLCR_PLL1OVER != 0 { val |= SRC_PLLCR_PLL1EN; writel(val, reg(SRC_PLLCR)); } }
    else if pll.id == 2 { val |= SRC_PLLCR_PLL2EN; writel(val, reg(SRC_PLLCR)); }
    spin_unlock(&mut src_lock); 0
}
unsafe extern "C" fn pll_clk_disable(hw: *mut ClkHw) { let pll=&mut *pll_from_hw(hw); spin_lock(&mut src_lock); let mut val=readl(reg(SRC_PLLCR)); if pll.id==1 { if val&SRC_PLLCR_PLL1OVER!=0 { val &= !SRC_PLLCR_PLL1EN; writel(val,reg(SRC_PLLCR)); } } else if pll.id==2 { val &= !SRC_PLLCR_PLL2EN; writel(val,reg(SRC_PLLCR)); } spin_unlock(&mut src_lock); }
unsafe extern "C" fn pll_clk_is_enabled(hw: *mut ClkHw) -> i32 { let pll=&mut *pll_from_hw(hw); let val=readl(reg(SRC_PLLCR)); if pll.id==1 && val&SRC_PLLCR_PLL1OVER!=0 { return ((val&SRC_PLLCR_PLL1EN)!=0) as i32; } if pll.id==2 { return ((val&SRC_PLLCR_PLL2EN)!=0) as i32; } 1 }
unsafe extern "C" fn pll_clk_recalc_rate(hw:*mut ClkHw,parent_rate:usize)->usize { let pll=&mut *pll_from_hw(hw); let val=readl(reg(SRC_PLLFR)); if pll.id==1 { let mul=((val>>8)&0x3f)+2; return parent_rate.wrapping_mul(mul as usize) >> (val&7); } if pll.id==2 { let mul=((val>>24)&0x3f)+2; return parent_rate.wrapping_mul(mul as usize); } 0 }
static PLL_CLK_OPS: ClkOps = ClkOps { enable:Some(pll_clk_enable), disable:Some(pll_clk_disable), is_enabled:Some(pll_clk_is_enabled), recalc_rate:Some(pll_clk_recalc_rate) };

unsafe extern "C" fn src_clk_enable(hw:*mut ClkHw)->i32 { let s=&mut *src_from_hw(hw); let en=if s.group1 {SRC_PCKEN1}else{SRC_PCKEN0}; let sr=if s.group1 {SRC_PCKSR1}else{SRC_PCKSR0}; writel(s.clkbit,reg(en)); while readl(reg(sr))&s.clkbit==0 {cpu_relax();} 0 }
unsafe extern "C" fn src_clk_disable(hw:*mut ClkHw) { let s=&mut *src_from_hw(hw); let dis=if s.group1 {SRC_PCKDIS1}else{SRC_PCKDIS0}; let sr=if s.group1 {SRC_PCKSR1}else{SRC_PCKSR0}; writel(s.clkbit,reg(dis)); while readl(reg(sr))&s.clkbit!=0 {cpu_relax();} }
unsafe extern "C" fn src_clk_is_enabled(hw:*mut ClkHw)->i32 { let s=&mut *src_from_hw(hw); let sr=if s.group1 {SRC_PCKSR1}else{SRC_PCKSR0}; ((readl(reg(sr))&s.clkbit)!=0) as i32 }
unsafe extern "C" fn src_clk_recalc_rate(_hw:*mut ClkHw,parent_rate:usize)->usize {parent_rate}
static SRC_CLK_OPS: ClkOps = ClkOps { enable:Some(src_clk_enable), disable:Some(src_clk_disable), is_enabled:Some(src_clk_is_enabled), recalc_rate:Some(src_clk_recalc_rate) };

extern "C" {
    fn of_find_matching_node(from:*mut DeviceNode, match_table:*const core::ffi::c_void)->*mut DeviceNode;
    fn of_iomap(np:*mut DeviceNode,index:i32)->*mut u8;
    fn of_node_put(np:*mut DeviceNode);
    fn of_property_read_bool(np:*mut DeviceNode,name:*const u8)->bool;
    fn register_reboot_notifier(nb:*mut NotifierBlock)->i32;
    fn of_property_read_u32(np:*mut DeviceNode,name:*const u8,val:*mut u32)->i32;
    fn of_clk_get_parent_name(np:*mut DeviceNode,index:i32)->*const u8;
    fn clk_hw_register(dev:*mut Device,hw:*mut ClkHw)->i32;
    fn of_clk_add_hw_provider(np:*mut DeviceNode,getter:*const core::ffi::c_void,data:*mut ClkHw)->i32;
    fn clk_hw_register_divider(dev:*mut Device,name:*const u8,parent:*const u8,flags:u32,reg:*mut u8,shift:u8,width:u8,div_flags:u32,lock:*mut SpinLock)->*mut ClkHw;
}

unsafe extern "C" fn nomadik_clk_reboot_handler(_this:*mut NotifierBlock,_code:usize,_unused:*mut core::ffi::c_void)->i32 { let mut val=readl(reg(SRC_XTALCR)); val &= !SRC_XTALCR_MXTALOVER; val |= SRC_XTALCR_MXTALEN; writel(val,reg(SRC_XTALCR)); 0x0001 }
static mut NOMADIK_CLK_REBOOT_NOTIFIER: NotifierBlock = NotifierBlock { notifier_call:Some(nomadik_clk_reboot_handler) };

#[no_mangle] pub unsafe extern "C" fn nomadik_src_init() { if src_base.is_null() { return; } let mut val=readl(reg(SRC_CR)); val |= SRC_CR_T0_ENSEL|SRC_CR_T1_ENSEL|SRC_CR_T2_ENSEL|SRC_CR_T3_ENSEL|SRC_CR_T4_ENSEL|SRC_CR_T5_ENSEL|SRC_CR_T6_ENSEL|SRC_CR_T7_ENSEL; writel(val,reg(SRC_CR)); val=readl(reg(SRC_XTALCR)); if of_property_read_bool(core::ptr::null_mut(),b"disable-sxtalo\0".as_ptr()) {val|=SRC_XTALCR_SXTALDIS;} if of_property_read_bool(core::ptr::null_mut(),b"disable-mxtalo\0".as_ptr()) {val|=SRC_XTALCR_MXTALOVER;val&=!SRC_XTALCR_MXTALEN;} writel(val,reg(SRC_XTALCR)); register_reboot_notifier(&mut NOMADIK_CLK_REBOOT_NOTIFIER); }

#[no_mangle] pub unsafe extern "C" fn of_nomadik_pll_setup(np:*mut DeviceNode) { if src_base.is_null(){nomadik_src_init();} let mut id=0; if of_property_read_u32(np,b"pll-id\0".as_ptr(),&mut id)!=0{return;} let hw=pll_clk_register(core::ptr::null_mut(),(*np).name,of_clk_get_parent_name(np,0),id); if !hw.is_null(){of_clk_add_hw_provider(np,core::ptr::null(),hw);} }
#[no_mangle] pub unsafe extern "C" fn of_nomadik_hclk_setup(np:*mut DeviceNode) { if src_base.is_null(){nomadik_src_init();} let hw=clk_hw_register_divider(core::ptr::null_mut(),(*np).name,of_clk_get_parent_name(np,0),0,reg(SRC_CR),13,2,0,&mut src_lock); if !hw.is_null(){of_clk_add_hw_provider(np,core::ptr::null(),hw);} }
#[no_mangle] pub unsafe extern "C" fn of_nomadik_src_clk_setup(np:*mut DeviceNode) { if src_base.is_null(){nomadik_src_init();} let mut id=0; if of_property_read_u32(np,b"clock-id\0".as_ptr(),&mut id)!=0{return;} let hw=src_clk_register(core::ptr::null_mut(),(*np).name,of_clk_get_parent_name(np,0),id as u8); if !hw.is_null(){of_clk_add_hw_provider(np,core::ptr::null(),hw);} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
