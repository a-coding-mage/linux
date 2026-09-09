// SPDX-License-Identifier: GPL-2.0
/*
 * Pistachio clocksource based on general-purpose timers
 *
 * Copyright (C) 2015 Imagination Technologies
 */

const CR_TIMER_CTRL_CFG: u32 = 0x00;
const TIMER_ME_GLOBAL: u32 = 1 << 0;
const CR_TIMER_REV: u32 = 0x10;
const TIMER_CFG: u32 = 0x20;
const TIMER_ME_LOCAL: u32 = 1 << 0;
const TIMER_RELOAD_VALUE: u32 = 0x24;
const TIMER_CURRENT_VALUE: u32 = 0x28;
const TIMER_CURRENT_OVERFLOW_VALUE: u32 = 0x2c;
const TIMER_IRQ_STATUS: u32 = 0x30;
const TIMER_IRQ_CLEAR: u32 = 0x34;
const TIMER_IRQ_MASK: u32 = 0x38;
const PERIP_TIMER_CONTROL: u32 = 0x90;
const RELOAD_VALUE: u32 = 0xffff_ffff;

#[repr(C)]
pub struct RawSpinlock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Clocksource {
    pub name: *const u8,
    pub rating: i32,
    pub enable: Option<unsafe extern "C" fn(*mut Clocksource) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut Clocksource)>,
    pub read: Option<unsafe extern "C" fn(*mut Clocksource) -> u64>,
    pub mask: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct PistachioClocksource {
    pub base: *mut core::ffi::c_void,
    pub lock: RawSpinlock,
    pub cs: Clocksource,
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct Clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct Regmap {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn raw_spin_lock_irqsave(lock: *mut RawSpinlock, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut RawSpinlock, flags: usize);
    fn raw_spin_lock_init(lock: *mut RawSpinlock);
    fn of_iomap(node: *mut DeviceNode, index: i32) -> *mut core::ffi::c_void;
    fn syscon_regmap_lookup_by_phandle(node: *mut DeviceNode, name: *const u8) -> *mut Regmap;
    fn regmap_update_bits(map: *mut Regmap, offset: u32, mask: u32, value: u32) -> i32;
    fn of_clk_get_by_name(node: *mut DeviceNode, name: *const u8) -> *mut Clk;
    fn clk_prepare_enable(clk: *mut Clk) -> i32;
    fn clk_disable_unprepare(clk: *mut Clk);
    fn clk_get_rate(clk: *mut Clk) -> usize;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: usize);
    fn clocksource_register_hz(cs: *mut Clocksource, rate: usize) -> i32;
}

static mut PCS_GPT: PistachioClocksource = PistachioClocksource {
    base: core::ptr::null_mut(),
    lock: RawSpinlock { _private: [] },
    cs: Clocksource {
        name: b"gptimer\0".as_ptr(),
        rating: 300,
        enable: Some(pistachio_clocksource_enable),
        disable: Some(pistachio_clocksource_disable),
        read: Some(pistachio_clocksource_read_cycles),
        mask: u32::MAX,
        flags: 1 | 2,
    },
};

#[inline]
unsafe fn gpt_readl(base: *mut core::ffi::c_void, offset: u32, gpt_id: u32) -> u32 {
    readl(base.add((0x20 * gpt_id + offset) as usize))
}

#[inline]
unsafe fn gpt_writel(base: *mut core::ffi::c_void, value: u32, offset: u32, gpt_id: u32) {
    writel(value, base.add((0x20 * gpt_id + offset) as usize));
}

unsafe extern "C" fn pistachio_clocksource_read_cycles(cs: *mut Clocksource) -> u64 {
    let pcs = (cs as *mut u8).sub(core::mem::offset_of!(PistachioClocksource, cs))
        as *mut PistachioClocksource;
    let mut overflow: u32;
    let mut counter: u32;
    let mut flags: usize = 0;
    raw_spin_lock_irqsave(&mut (*pcs).lock, &mut flags);
    overflow = gpt_readl((*pcs).base, TIMER_CURRENT_OVERFLOW_VALUE, 0);
    counter = gpt_readl((*pcs).base, TIMER_CURRENT_VALUE, 0);
    raw_spin_unlock_irqrestore(&mut (*pcs).lock, flags);
    let _ = overflow;
    !(counter as u64)
}

unsafe extern "C" fn pistachio_read_sched_clock() -> u64 {
    pistachio_clocksource_read_cycles(&raw mut PCS_GPT.cs)
}

unsafe fn pistachio_clksrc_set_mode(cs: *mut Clocksource, timeridx: i32, enable: bool) {
    let pcs = (cs as *mut u8).sub(core::mem::offset_of!(PistachioClocksource, cs))
        as *mut PistachioClocksource;
    let mut val = gpt_readl((*pcs).base, TIMER_CFG, timeridx as u32);
    if enable { val |= TIMER_ME_LOCAL; } else { val &= !TIMER_ME_LOCAL; }
    gpt_writel((*pcs).base, val, TIMER_CFG, timeridx as u32);
}

unsafe fn pistachio_clksrc_enable(cs: *mut Clocksource, timeridx: i32) {
    let pcs = (cs as *mut u8).sub(core::mem::offset_of!(PistachioClocksource, cs))
        as *mut PistachioClocksource;
    pistachio_clksrc_set_mode(cs, timeridx, false);
    gpt_writel((*pcs).base, RELOAD_VALUE, TIMER_RELOAD_VALUE, timeridx as u32);
    pistachio_clksrc_set_mode(cs, timeridx, true);
}

unsafe fn pistachio_clksrc_disable(cs: *mut Clocksource, timeridx: i32) {
    pistachio_clksrc_set_mode(cs, timeridx, false);
}

unsafe extern "C" fn pistachio_clocksource_enable(cs: *mut Clocksource) -> i32 {
    pistachio_clksrc_enable(cs, 0);
    0
}

unsafe extern "C" fn pistachio_clocksource_disable(cs: *mut Clocksource) {
    pistachio_clksrc_disable(cs, 0);
}

unsafe extern "C" fn pistachio_clksrc_of_init(node: *mut DeviceNode) -> i32 {
    let pcs = &raw mut PCS_GPT;
    (*pcs).base = of_iomap(node, 0);
    if (*pcs).base.is_null() { return -6; }

    let periph_regs = syscon_regmap_lookup_by_phandle(node, b"img,cr-periph\0".as_ptr());
    if periph_regs.is_null() { return -1; }
    let mut ret = regmap_update_bits(periph_regs, PERIP_TIMER_CONTROL, 0xf, 0x0);
    if ret != 0 { return ret; }

    let sys_clk = of_clk_get_by_name(node, b"sys\0".as_ptr());
    if sys_clk.is_null() { return -1; }
    let fast_clk = of_clk_get_by_name(node, b"fast\0".as_ptr());
    if fast_clk.is_null() { return -1; }

    ret = clk_prepare_enable(sys_clk);
    if ret < 0 { return ret; }
    ret = clk_prepare_enable(fast_clk);
    if ret < 0 {
        clk_disable_unprepare(sys_clk);
        return ret;
    }

    let rate = clk_get_rate(fast_clk);
    gpt_writel((*pcs).base, 0, TIMER_IRQ_MASK, 0);
    gpt_writel((*pcs).base, 0, TIMER_IRQ_MASK, 1);
    gpt_writel((*pcs).base, 0, TIMER_IRQ_MASK, 2);
    gpt_writel((*pcs).base, 0, TIMER_IRQ_MASK, 3);
    writel(TIMER_ME_GLOBAL, (*pcs).base);
    raw_spin_lock_init(&mut (*pcs).lock);
    sched_clock_register(pistachio_read_sched_clock, 32, rate);
    clocksource_register_hz(&mut (*pcs).cs, rate)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
