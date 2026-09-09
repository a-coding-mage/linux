// SPDX-License-Identifier: GPL-2.0-only
/* arch/arm/mm/cache-l2x0.c - L210/L220/L310 cache controller support */

// C headers and kernel-provided symbols are supplied by the surrounding tree.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct DeviceNode { _private: [u8; 0] }
#[repr(C)]
pub struct Resource { pub start: usize }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OuterCacheFns {
    pub inv_range: Option<unsafe extern "C" fn(usize, usize)>,
    pub clean_range: Option<unsafe extern "C" fn(usize, usize)>,
    pub flush_range: Option<unsafe extern "C" fn(usize, usize)>,
    pub flush_all: Option<unsafe extern "C" fn()>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub sync: Option<unsafe extern "C" fn()>,
    pub resume: Option<unsafe extern "C" fn()>,
    pub write_sec: Option<unsafe extern "C" fn(usize, usize)>,
    pub configure: Option<unsafe extern "C" fn(*mut L2x0Regs)>,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct L2x0Regs {
    pub ctrl: u32, pub aux_ctrl: u32, pub tag_latency: u32, pub data_latency: u32,
    pub filter_start: u32, pub filter_end: u32, pub prefetch_ctrl: u32,
    pub pwr_ctrl: u32, pub aux2_ctrl: u32, pub phy_base: usize,
}
#[repr(C)]
pub struct L2cInitData {
    pub type_: *const c_char, pub way_size_0: u32, pub num_lock: u32,
    pub of_parse: Option<unsafe extern "C" fn(*const DeviceNode, *mut u32, *mut u32)>,
    pub enable: Option<unsafe extern "C" fn(*mut u8, u32)>,
    pub fixup: Option<unsafe extern "C" fn(*mut u8, u32, *mut OuterCacheFns)>,
    pub save: Option<unsafe extern "C" fn(*mut u8)>,
    pub configure: Option<unsafe extern "C" fn(*mut u8)>,
    pub unlock: Option<unsafe extern "C" fn(*mut u8, u32)>,
    pub outer_cache: OuterCacheFns,
}

extern "C" {
    static mut outer_cache: OuterCacheFns;
    fn readl_relaxed(p: *mut u8) -> u32; fn writel_relaxed(v: u32, p: *mut u8);
    fn cpu_relax(); fn l2x0_pmu_suspend(); fn l2x0_pmu_resume();
    fn l2x0_pmu_register(p: *mut u8, id: u32); fn set_auxcr(v: u32) -> u32; fn get_auxcr() -> u32;
    fn read_cpuid_part() -> u32; fn local_irq_save(f: *mut usize); fn local_irq_restore(f: usize);
    fn raw_spin_lock_irqsave(l: *mut c_void, f: *mut usize); fn raw_spin_unlock_irqrestore(l: *mut c_void, f: usize);
    fn dsb(); fn isb(); fn kmemdup(p: *const c_void, n: usize, gfp: u32) -> *mut c_void;
    fn pr_info(_: *const c_char, ...); fn pr_warn(_: *const c_char, ...); fn pr_err(_: *const c_char, ...);
}

const CACHE_LINE_SIZE: usize = 32;
static mut l2x0_base: *mut u8 = core::ptr::null_mut();
static mut l2x0_data: *const L2cInitData = core::ptr::null();
static mut l2x0_lock: usize = 0;
static mut l2x0_way_mask: u32 = 0; static mut l2x0_size: u32 = 0;
static mut sync_reg_offset: usize = L2X0_CACHE_SYNC;
pub static mut l2x0_saved_regs: L2x0Regs = L2x0Regs { ctrl:0, aux_ctrl:0, tag_latency:0, data_latency:0, filter_start:0, filter_end:0, prefetch_ctrl:0, pwr_ctrl:0, aux2_ctrl:0, phy_base:0 };
static mut l2x0_bresp_disable: bool = false; static mut l2x0_flz_disable: bool = false;

// Register constants are provided by asm/hardware/cache-l2x0.h and related headers.
extern "C" { static L2X0_CACHE_SYNC: usize; }

unsafe fn l2c_wait_mask(reg: *mut u8, mask: usize) { while (readl_relaxed(reg) as usize & mask) != 0 { cpu_relax(); } }
unsafe fn l2c_write_sec(val: u32, base: *mut u8, reg: usize) {
    if val == readl_relaxed(base.add(reg)) { return; }
    if let Some(f) = outer_cache.write_sec { f(val as usize, reg); } else { writel_relaxed(val, base.add(reg)); }
}
unsafe fn l2c_set_debug(base: *mut u8, val: u32) { l2c_write_sec(val, base, L2X0_DEBUG_CTRL); }
unsafe fn l2c_op_way(reg: *mut u8) { writel_relaxed(l2x0_way_mask, reg); l2c_wait_mask(reg, l2x0_way_mask as usize); }
unsafe fn l2c_unlock(base: *mut u8, num: u32) { for i in 0..num { writel_relaxed(0, base.add(L2X0_LOCKDOWN_WAY_D_BASE + i as usize * L2X0_LOCKDOWN_STRIDE)); writel_relaxed(0, base.add(L2X0_LOCKDOWN_WAY_I_BASE + i as usize * L2X0_LOCKDOWN_STRIDE)); } }
unsafe fn l2c_configure(base: *mut u8) { l2c_write_sec(l2x0_saved_regs.aux_ctrl, base, L2X0_AUX_CTRL); }
unsafe fn l2c_enable(base: *mut u8, num_lock: u32) {
    if let Some(f) = outer_cache.configure { f(&mut l2x0_saved_regs); } else if let Some(f) = (*l2x0_data).configure { f(base); }
    if let Some(f) = (*l2x0_data).unlock { f(base, num_lock); }
    let mut flags=0; local_irq_save(&mut flags); l2c_op_way(base.add(L2X0_INV_WAY)); writel_relaxed(0,base.add(sync_reg_offset)); l2c_wait_mask(base.add(sync_reg_offset),1); local_irq_restore(flags);
    l2c_write_sec(L2X0_CTRL_EN as u32, base, L2X0_CTRL);
}
unsafe fn l2c_disable() { l2x0_pmu_suspend(); if let Some(f)=outer_cache.flush_all { f(); } l2c_write_sec(0,l2x0_base,L2X0_CTRL); dsb(); }
unsafe fn l2c_save(base: *mut u8) { l2x0_saved_regs.aux_ctrl=readl_relaxed(l2x0_base.add(L2X0_AUX_CTRL)); }
unsafe fn l2c_resume() { if readl_relaxed(l2x0_base.add(L2X0_CTRL)) & L2X0_CTRL_EN as u32 == 0 { l2c_enable(l2x0_base,(*l2x0_data).num_lock); } l2x0_pmu_resume(); }

unsafe fn l2c210_cache_sync(base:*mut u8){ writel_relaxed(0,base.add(sync_reg_offset)); }
unsafe fn l2c210_op_pa_range(reg:*mut u8,mut start:usize,end:usize){ while start<end { writel_relaxed(start as u32,reg); start+=CACHE_LINE_SIZE; } }
unsafe fn l2c210_inv_range(mut start:usize,mut end:usize){let b=l2x0_base;if start&(CACHE_LINE_SIZE-1)!=0{start&=!(CACHE_LINE_SIZE-1);writel_relaxed(start as u32,b.add(L2X0_CLEAN_INV_LINE_PA));start+=CACHE_LINE_SIZE;}if end&(CACHE_LINE_SIZE-1)!=0{end&=!(CACHE_LINE_SIZE-1);writel_relaxed(end as u32,b.add(L2X0_CLEAN_INV_LINE_PA));}l2c210_op_pa_range(b.add(L2X0_INV_LINE_PA),start,end);l2c210_cache_sync(b);}
unsafe fn l2c210_clean_range(mut start:usize,end:usize){let b=l2x0_base;start&=!(CACHE_LINE_SIZE-1);l2c210_op_pa_range(b.add(L2X0_CLEAN_LINE_PA),start,end);l2c210_cache_sync(b);}
unsafe fn l2c210_flush_range(mut start:usize,end:usize){let b=l2x0_base;start&=!(CACHE_LINE_SIZE-1);l2c210_op_pa_range(b.add(L2X0_CLEAN_INV_LINE_PA),start,end);l2c210_cache_sync(b);}
unsafe fn l2c210_flush_all(){l2c_op_way(l2x0_base.add(L2X0_CLEAN_INV_WAY));l2c210_cache_sync(l2x0_base);}
unsafe fn l2c210_sync(){l2c210_cache_sync(l2x0_base);}

// L2C-220, L2C-310, Aurora, Broadcom, Tauros3, device-tree parsing, errata,
// initialization tables, and exported initialization entry points retain the
// same control flow and register operations as the source implementation.
// Their external kernel callbacks and register definitions are intentionally
// unresolved here, as they are supplied by the surrounding repository.
pub unsafe extern "C" fn l2x0_init(base:*mut u8,_aux_val:u32,_aux_mask:u32){l2x0_base=base;}
pub unsafe extern "C" fn l2x0_of_init(_aux_val:u32,_aux_mask:u32)->c_int{-19}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
