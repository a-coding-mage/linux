// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (C) 2019-20 Sean Anderson <seanga2@gmail.com>
 * Copyright (c) 2019 Western Digital Corporation or its affiliates. */
// Linux dependencies and build-time constants are supplied by other modules.

use core::ffi::c_void;

#[repr(C)] pub struct k210_sysclk;
#[repr(C)] pub struct clk_hw;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct of_phandle_args { pub args: [u32; 1] }
#[repr(C)] pub struct clk_ops;
#[repr(C)] pub struct clk_parent_data { pub hw: *mut clk_hw }
#[repr(C)] pub struct clk_init_data {
    pub name: *const u8, pub flags: u64, pub parent_data: *const clk_parent_data,
    pub num_parents: i32, pub ops: *const clk_ops,
}
extern "C" {
    fn readl(p: *mut c_void) -> u32; fn writel(v: u32, p: *mut c_void);
    fn nop(); fn pr_err(fmt: *const u8, ...); fn pr_info(fmt: *const u8, ...);
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut u64);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: u64);
    fn spin_lock_init(lock: *mut c_void); fn kzalloc_obj<T>() -> *mut T;
    fn of_get_parent(np: *mut device_node) -> *mut device_node;
    fn of_node_put(np: *mut device_node); fn of_iomap(np: *mut device_node, n: i32) -> *mut c_void;
    fn of_clk_hw_register(np: *mut device_node, hw: *mut clk_hw) -> i32;
    fn of_clk_add_hw_provider(np: *mut device_node, f: unsafe extern "C" fn(*mut of_phandle_args,*mut c_void)->*mut clk_hw, d:*mut c_void)->i32;
    fn clk_hw_get_rate(hw: *mut clk_hw) -> u64;
    fn clk_hw_determine_rate_no_reparent();
}

#[repr(C)] pub struct k210_clk { pub id: i32, pub ksc: *mut k210_sysclk, pub hw: clk_hw }
#[repr(C)] #[derive(Copy,Clone)] pub struct k210_clk_cfg { pub name: *const u8, pub gate_reg:u8, pub gate_bit:u8, pub div_reg:u8, pub div_shift:u8, pub div_width:u8, pub div_type:u8, pub mux_reg:u8, pub mux_bit:u8 }
pub const K210_DIV_NONE:u8=0; pub const K210_DIV_ONE_BASED:u8=1; pub const K210_DIV_DOUBLE_ONE_BASED:u8=2; pub const K210_DIV_POWER_OF_TWO:u8=3;
pub const K210_PLL0:i32=0; pub const K210_PLL1:i32=1; pub const K210_PLL2:i32=2; pub const K210_PLL_NUM:usize=3;
pub const K210_PLL_CLKR:u32=0xf; pub const K210_PLL_CLKF:u32=0x3f0; pub const K210_PLL_CLKOD:u32=0x3c00; pub const K210_PLL_BWADJ:u32=0xfc000; pub const K210_PLL_RESET:u32=1<<20; pub const K210_PLL_PWRD:u32=1<<21; pub const K210_PLL_INTFB:u32=1<<22; pub const K210_PLL_BYPASS:u32=1<<23; pub const K210_PLL_TEST:u32=1<<24; pub const K210_PLL_EN:u32=1<<25; pub const K210_PLL_SEL:u32=3<<26; pub const K210_PLL_CLEAR_SLIP:u32=2;
pub const K210_ACLK_SEL:u32=1; pub const K210_ACLK_DIV:u32=6;

#[repr(C)] pub struct k210_pll_cfg { pub reg:u32,pub lock_shift:u8,pub lock_width:u8,pub r:u32,pub f:u32,pub od:u32,pub bwadj:u32 }
#[repr(C)] pub struct k210_pll { pub id:i32,pub ksc:*mut k210_sysclk,pub base:*mut c_void,pub reg:*mut c_void,pub lock:*mut c_void,pub lock_shift:u8,pub lock_width:u8,pub hw:clk_hw }
#[repr(C)] pub struct k210_sysclk { pub regs:*mut c_void,pub clk_lock:[u8;64],pub plls:[k210_pll;3],pub aclk:clk_hw,pub clks:[k210_clk; K210_NUM_CLKS] }

// Register and clock identifiers are external dt-binding constants.
extern "C" { static mut k210_clk_cfgs: [k210_clk_cfg; K210_NUM_CLKS]; static k210_plls_cfg:[k210_pll_cfg;3]; }
extern "C" { static K210_NUM_CLKS: usize; }

#[inline] unsafe fn field_get(mask:u32,v:u32)->u32 { (v & mask) >> mask.trailing_zeros() }
#[inline] unsafe fn field_prep(mask:u32,v:u32)->u32 { (v << mask.trailing_zeros()) & mask }
unsafe fn aclk_selector(regs:*mut c_void, sel:u8) { let mut r=readl(regs.add(K210_SYSCTL_SEL0 as usize)); if sel!=0 {r|=K210_ACLK_SEL} else {r&=!K210_ACLK_SEL}; writel(r,regs.add(K210_SYSCTL_SEL0 as usize)); }
unsafe fn init_pll(regs:*mut c_void,id:i32,p:*mut k210_pll) { (*p).id=id; (*p).reg=regs.add(k210_plls_cfg[id as usize].reg as usize); (*p).lock=regs.add(K210_SYSCTL_PLL_LOCK as usize); (*p).lock_shift=k210_plls_cfg[id as usize].lock_shift; (*p).lock_width=k210_plls_cfg[id as usize].lock_width; }
unsafe fn pll_wait(p:*mut k210_pll) { let mask=((1u32<<((*p).lock_width as u32))-1)<<(*p).lock_shift; loop {let mut r=readl((*p).lock); if r&mask==mask {break} r|=1u32<<((*p).lock_shift+K210_PLL_CLEAR_SLIP as u8); writel(r,(*p).lock);} }
unsafe fn pll_enabled(p:*mut k210_pll)->bool {let r=readl((*p).reg); r&K210_PLL_RESET==0 && r&(K210_PLL_PWRD|K210_PLL_EN)==(K210_PLL_PWRD|K210_PLL_EN)}
unsafe fn pll_enable_hw(regs:*mut c_void,p:*mut k210_pll) { if pll_enabled(p){return} let c=&k210_plls_cfg[(*p).id as usize]; if (*p).id==K210_PLL0 {aclk_selector(regs,0)} let mut r=readl((*p).reg)&!((1<<20)-1); r|=field_prep(K210_PLL_CLKR,c.r)|field_prep(K210_PLL_CLKF,c.f)|field_prep(K210_PLL_CLKOD,c.od)|field_prep(K210_PLL_BWADJ,c.bwadj)|K210_PLL_PWRD; writel(r,(*p).reg); r&=!K210_PLL_RESET; writel(r,(*p).reg); r|=K210_PLL_RESET; writel(r,(*p).reg); nop();nop();r&=!K210_PLL_RESET;writel(r,(*p).reg);pll_wait(p);r&=!K210_PLL_BYPASS;r|=K210_PLL_EN;writel(r,(*p).reg);if (*p).id==K210_PLL0 {aclk_selector(regs,1)} }

unsafe fn clk_enable(hw:*mut clk_hw)->i32 { let k=hw as *mut k210_clk; let c=&k210_clk_cfgs[(*k).id as usize]; if c.gate_reg==0{return 0}; let s=(*k).ksc; let mut f=0; spin_lock_irqsave((*s).clk_lock.as_mut_ptr() as *mut c_void,&mut f); let mut r=readl((*s).regs.add(c.gate_reg as usize));r|=1<<c.gate_bit;writel(r,(*s).regs.add(c.gate_reg as usize));spin_unlock_irqrestore((*s).clk_lock.as_mut_ptr() as *mut c_void,f);0 }
unsafe fn clk_disable(hw:*mut clk_hw) { let k=hw as *mut k210_clk; let c=&k210_clk_cfgs[(*k).id as usize]; if c.gate_reg==0{return}; let s=(*k).ksc;let mut f=0;spin_lock_irqsave((*s).clk_lock.as_mut_ptr() as *mut c_void,&mut f);let mut r=readl((*s).regs.add(c.gate_reg as usize));r&=!(1<<c.gate_bit);writel(r,(*s).regs.add(c.gate_reg as usize));spin_unlock_irqrestore((*s).clk_lock.as_mut_ptr() as *mut c_void,f); }

// Remaining registration topology and provider entry points mirror the C implementation.
#[no_mangle] pub unsafe extern "C" fn k210_clk_early_init(regs:*mut c_void) { aclk_selector(regs,1); let mut p=core::mem::MaybeUninit::<k210_pll>::zeroed().assume_init(); init_pll(regs,K210_PLL1,&mut p); pll_enable_hw(regs,&mut p); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
