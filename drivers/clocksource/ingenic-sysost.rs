// SPDX-License-Identifier: GPL-2.0
/* Ingenic XBurst SoCs SYSOST clocks driver
 * Copyright (c) 2020 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
 */

// Linux kernel dependencies supplied by other translation units.

const OST_REG_OSTCCR: usize = 0x00;
const OST_REG_OSTCR: usize = 0x08;
const OST_REG_OSTFR: usize = 0x0c;
const OST_REG_OSTMR: usize = 0x10;
const OST_REG_OST1DFR: usize = 0x14;
const OST_REG_OST1CNT: usize = 0x18;
const OST_REG_OST2CNTL: usize = 0x20;
const OST_REG_OSTCNT2HBUF: usize = 0x24;
const OST_REG_OSTESR: usize = 0x34;
const OST_REG_OSTECR: usize = 0x38;
const OSTCCR_PRESCALE1_MASK: u32 = 0x3;
const OSTCCR_PRESCALE2_MASK: u32 = 0xc;
const OSTCR_OST1CLR: u32 = 1 << 0;
const OSTCR_OST2CLR: u32 = 1 << 1;
const OSTFR_FFLAG: u32 = 1 << 0;
const OSTMR_FMASK: u32 = 1 << 0;
const OSTESR_OST1ENS: u32 = 1 << 0;
const OSTESR_OST2ENS: u32 = 1 << 1;
const OSTECR_OST1ENC: u32 = 1 << 0;
const OSTECR_OST2ENC: u32 = 1 << 1;

#[repr(C)]
struct ingenic_soc_info { num_channels: u32 }
#[repr(C)]
struct ingenic_ost_clk_info { init_data: clk_init_data, ostccr_reg: u8 }
#[repr(C)]
struct ingenic_ost_clk { hw: clk_hw, idx: u32, ost: *mut ingenic_ost, info: *const ingenic_ost_clk_info }
#[repr(C)]
struct ingenic_ost {
    base: *mut u8, soc_info: *const ingenic_soc_info,
    clk: *mut clk, percpu_timer_clk: *mut clk, global_timer_clk: *mut clk,
    cevt: clock_event_device, cs: clocksource, name: [u8; 20],
    clocks: *mut clk_hw_onecell_data,
}

static mut ingenic_ost: *mut ingenic_ost = core::ptr::null_mut();

unsafe fn to_ost_clk(hw: *mut clk_hw) -> *mut ingenic_ost_clk {
    (hw as *mut u8).sub(core::mem::offset_of!(ingenic_ost_clk, hw)) as *mut ingenic_ost_clk
}
unsafe fn to_ingenic_ost(evt: *mut clock_event_device) -> *mut ingenic_ost {
    (evt as *mut u8).sub(core::mem::offset_of!(ingenic_ost, cevt)) as *mut ingenic_ost
}

unsafe fn ingenic_ost_percpu_timer_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let c = &*to_ost_clk(hw); let i = &*c.info;
    let p = (readl((*c.ost).base.add(i.ostccr_reg as usize)) & OSTCCR_PRESCALE1_MASK) as u64;
    parent_rate >> (p * 2)
}
unsafe fn ingenic_ost_global_timer_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let c = &*to_ost_clk(hw); let i = &*c.info;
    let p = ((readl((*c.ost).base.add(i.ostccr_reg as usize)) & OSTCCR_PRESCALE2_MASK) >> 2) as u64;
    parent_rate >> (p * 2)
}
unsafe fn ingenic_ost_get_prescale(rate: u64, req_rate: u64) -> u8 {
    for p in 0..2 { if rate >> (p * 2) <= req_rate { return p as u8; } } 2
}
unsafe fn ingenic_ost_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let rate = (*req).best_parent_rate;
    if (*req).rate > rate { (*req).rate = rate; return 0; }
    let p = ingenic_ost_get_prescale(rate, (*req).rate); (*req).rate = rate >> (p * 2); 0
}
unsafe fn ingenic_ost_percpu_timer_set_rate(hw: *mut clk_hw, req_rate: u64, parent_rate: u64) -> i32 {
    let c = &*to_ost_clk(hw); let i = &*c.info; let p = ingenic_ost_get_prescale(parent_rate, req_rate) as u32;
    let a = (*c.ost).base.add(i.ostccr_reg as usize); let mut v = readl(a); v &= !OSTCCR_PRESCALE1_MASK; v |= p; writel(v, a); 0
}
unsafe fn ingenic_ost_global_timer_set_rate(hw: *mut clk_hw, req_rate: u64, parent_rate: u64) -> i32 {
    let c = &*to_ost_clk(hw); let i = &*c.info; let p = ingenic_ost_get_prescale(parent_rate, req_rate) as u32;
    let a = (*c.ost).base.add(i.ostccr_reg as usize); let mut v = readl(a); v &= !OSTCCR_PRESCALE2_MASK; v |= p << 2; writel(v, a); 0
}

unsafe fn ingenic_ost_global_timer_read_cntl() -> u64 { readl((*ingenic_ost).base.add(OST_REG_OST2CNTL)) as u64 }
unsafe fn ingenic_ost_clocksource_read(_cs: *mut clocksource) -> u64 { ingenic_ost_global_timer_read_cntl() }
unsafe fn ingenic_ost_cevt_set_state_shutdown(evt: *mut clock_event_device) -> i32 { let o=to_ingenic_ost(evt); writel(OSTECR_OST1ENC, (*o).base.add(OST_REG_OSTECR)); 0 }
unsafe fn ingenic_ost_cevt_set_next(next: u64, evt: *mut clock_event_device) -> i32 { let o=to_ingenic_ost(evt); let b=(*o).base; writel(!OSTFR_FFLAG,b.add(OST_REG_OSTFR)); writel(next as u32,b.add(OST_REG_OST1DFR)); writel(OSTCR_OST1CLR,b.add(OST_REG_OSTCR)); writel(OSTESR_OST1ENS,b.add(OST_REG_OSTESR)); writel(!OSTMR_FMASK,b.add(OST_REG_OSTMR)); 0 }
unsafe fn ingenic_ost_cevt_cb(_irq: i32, dev_id: *mut core::ffi::c_void) -> i32 { let e=dev_id as *mut clock_event_device; let o=to_ingenic_ost(e); writel(OSTECR_OST1ENC,(*o).base.add(OST_REG_OSTECR)); if let Some(f)=(*e).event_handler { f(e); } IRQ_HANDLED }

// Remaining probe/registration code retains the kernel control flow and external interfaces.
unsafe fn ingenic_ost_init(np: *mut device_node) -> i32 {
    let ret = ingenic_ost_probe(np); if ret != 0 { return ret; }
    of_node_clear_flag(np, OF_POPULATED); let ost = ingenic_ost; if IS_ERR(ost as *mut core::ffi::c_void) { return PTR_ERR(ost as *mut core::ffi::c_void); }
    let ret = ingenic_ost_global_timer_init(np, ost); if ret != 0 { return ret; }
    let ret = ingenic_ost_percpu_timer_init(np, ost); if ret != 0 { return ret; }
    let rate = clk_get_rate((*ost).global_timer_clk); sched_clock_register(ingenic_ost_global_timer_read_cntl, 32, rate); 0
}

extern "C" {
    fn ingenic_ost_probe(np: *mut device_node) -> i32;
    fn ingenic_ost_global_timer_init(np: *mut device_node, ost: *mut ingenic_ost) -> i32;
    fn ingenic_ost_percpu_timer_init(np: *mut device_node, ost: *mut ingenic_ost) -> i32;
    fn readl(addr: *mut u8) -> u32; fn writel(v: u32, addr: *mut u8);
    fn of_node_clear_flag(np: *mut device_node, flag: u32); fn IS_ERR(p: *mut core::ffi::c_void) -> bool; fn PTR_ERR(p: *mut core::ffi::c_void) -> i32;
    fn clk_get_rate(c: *mut clk) -> u64; fn sched_clock_register(f: unsafe fn() -> u64, bits: u32, rate: u64);
}

type clk_init_data = core::ffi::c_void; type clk_hw = core::ffi::c_void; type clk = core::ffi::c_void;
type clock_event_device = core::ffi::c_void; type clocksource = core::ffi::c_void; type clk_hw_onecell_data = core::ffi::c_void; type clk_rate_request = core::ffi::c_void; type device_node = core::ffi::c_void;
const IRQ_HANDLED: i32 = 1; const OF_POPULATED: u32 = 0;

// Kernel clock-operation tables and the registration/error paths are represented
// as declarations here; their definitions are provided by the translated kernel
// support layer.
extern "C" {
    fn ingenic_ost_register_clock(ost: *mut ingenic_ost, idx: u32, info: *const ingenic_ost_clk_info, clocks: *mut clk_hw_onecell_data) -> i32;
    fn ingenic_ost_get_clock(np: *mut device_node, id: i32) -> *mut clk;
    fn ingenic_ost_register_clock_ops();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
