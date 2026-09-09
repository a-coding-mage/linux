// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Mediatek SoCs General-Purpose Timer handling.
 *
 * Copyright (C) 2014 Matthias Brugger
 *
 * Matthias Brugger <matthias.bgg@gmail.com>
 */

// Kernel includes and build-time configuration are supplied by the surrounding tree.

const TIMER_CLK_EVT: u8 = 1;
const TIMER_CLK_SRC: u8 = 2;
const TIMER_SYNC_TICKS: u32 = 3;

const GPT_IRQ_EN_REG: usize = 0x00;
const GPT_IRQ_ACK_REG: usize = 0x08;
const GPT_CTRL_OP_ONESHOT: u32 = 0;
const GPT_CTRL_OP_REPEAT: u32 = 1;
const GPT_CTRL_OP_FREERUN: u32 = 3;
const GPT_CTRL_CLEAR: u32 = 2;
const GPT_CTRL_ENABLE: u32 = 1;
const GPT_CTRL_DISABLE: u32 = 0;
const GPT_CLK_SRC_SYS13M: u32 = 0;
const GPT_CLK_DIV1: u32 = 0x0;
const SYST_BASE: usize = 0x40;
const SYST_CON: usize = SYST_BASE + 0x0;
const SYST_VAL: usize = SYST_BASE + 0x4;
const SYST_CON_EN: u32 = 1 << 0;
const SYST_CON_IRQ_EN: u32 = 1 << 1;
const SYST_CON_IRQ_CLR: u32 = 1 << 4;

const fn gpt_irq_enable(val: u8) -> u32 { 1u32 << (val - 1) }
const fn gpt_irq_ack(val: u8) -> u32 { 1u32 << (val - 1) }
const fn gpt_ctrl_reg(val: u8) -> usize { 0x10 * val as usize }
const fn gpt_ctrl_op(val: u32) -> u32 { (val & 0x3) << 4 }
const fn gpt_clk_reg(val: u8) -> usize { 0x04 + 0x10 * val as usize }
const fn gpt_clk_src(val: u32) -> u32 { (val & 0x1) << 4 }
const fn gpt_cnt_reg(val: u8) -> usize { 0x08 + 0x10 * val as usize }
const fn gpt_cmp_reg(val: u8) -> usize { 0x0c + 0x10 * val as usize }

extern "C" {
    static mut gpt_sched_reg: *mut core::ffi::c_void;
}

extern "C" {
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn timer_of_base(to: *mut timer_of) -> *mut core::ffi::c_void;
    fn to_timer_of(clkevt: *mut clock_event_device) -> *mut timer_of;
    fn timer_of_init(node: *mut device_node, to: *mut timer_of) -> i32;
    fn timer_of_rate(to: *mut timer_of) -> u32;
    fn clockevents_config_and_register(clkevt: *mut clock_event_device, rate: u32, min_delta: u32, max_delta: u32);
    fn clocksource_mmio_init(reg: *mut core::ffi::c_void, name: *const i8, rating: u32, hz: u32, bits: u32, read: *const core::ffi::c_void) -> i32;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: u32);
}

#[repr(C)] pub struct device_node { pub name: *const i8 }
#[repr(C)] pub struct timer_of { pub flags: u32, pub clkevt: clock_event_device, pub of_irq: timer_of_irq, pub of_clk: timer_of_clock }
#[repr(C)] pub struct timer_of_clock { pub period: u32 }
#[repr(C)] pub struct timer_of_irq { pub flags: u32, pub handler: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t> }
#[repr(C)] pub struct clock_event_device {
    pub name: *const i8, pub rating: i32, pub cpumask: *const core::ffi::c_void, pub features: u32,
    pub event_handler: unsafe extern "C" fn(*mut clock_event_device),
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(*mut clock_event_device)>, pub resume: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}
pub type irqreturn_t = i32;
const IRQ_HANDLED: irqreturn_t = 1;

unsafe fn mtk_syst_ack_irq(to: *mut timer_of) { writel(SYST_CON_EN, timer_of_base(to).add(SYST_CON)); writel(SYST_CON_IRQ_CLR | SYST_CON_EN, timer_of_base(to).add(SYST_CON)); }
unsafe extern "C" fn mtk_syst_handler(_: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t { let clkevt = dev_id as *mut clock_event_device; mtk_syst_ack_irq(to_timer_of(clkevt)); ((*clkevt).event_handler)(clkevt); IRQ_HANDLED }
unsafe extern "C" fn mtk_syst_clkevt_next_event(ticks: usize, clkevt: *mut clock_event_device) -> i32 { let to = to_timer_of(clkevt); writel(SYST_CON_EN, timer_of_base(to).add(SYST_CON)); writel(ticks as u32, timer_of_base(to).add(SYST_VAL)); writel(SYST_CON_EN | SYST_CON_IRQ_EN, timer_of_base(to).add(SYST_CON)); 0 }
unsafe extern "C" fn mtk_syst_clkevt_shutdown(clkevt: *mut clock_event_device) -> i32 { let to = to_timer_of(clkevt); mtk_syst_ack_irq(to); writel(0, timer_of_base(to).add(SYST_CON)); 0 }
unsafe extern "C" fn mtk_syst_clkevt_resume(clkevt: *mut clock_event_device) -> i32 { mtk_syst_clkevt_shutdown(clkevt) }
unsafe extern "C" fn mtk_syst_clkevt_oneshot(_: *mut clock_event_device) -> i32 { 0 }
unsafe extern "C" fn mtk_gpt_read_sched_clock() -> u64 { readl_relaxed(gpt_sched_reg as *mut _) as u64 }

unsafe fn mtk_gpt_clkevt_time_stop(to: *mut timer_of, timer: u8) { let reg = timer_of_base(to).add(gpt_ctrl_reg(timer)); let val = readl(reg); writel(val & !GPT_CTRL_ENABLE, reg); }
unsafe fn mtk_gpt_clkevt_time_setup(to: *mut timer_of, delay: usize, timer: u8) { writel(delay as u32, timer_of_base(to).add(gpt_cmp_reg(timer))); }
unsafe fn mtk_gpt_clkevt_time_start(to: *mut timer_of, periodic: bool, timer: u8) { let base = timer_of_base(to); writel(gpt_irq_ack(timer), base.add(GPT_IRQ_ACK_REG)); let mut val = readl(base.add(gpt_ctrl_reg(timer))); val &= !gpt_ctrl_op(0x3); val |= gpt_ctrl_op(if periodic { GPT_CTRL_OP_REPEAT } else { GPT_CTRL_OP_ONESHOT }); writel(val | GPT_CTRL_ENABLE | GPT_CTRL_CLEAR, base.add(gpt_ctrl_reg(timer))); }
unsafe extern "C" fn mtk_gpt_clkevt_shutdown(clk: *mut clock_event_device) -> i32 { mtk_gpt_clkevt_time_stop(to_timer_of(clk), TIMER_CLK_EVT); 0 }
unsafe extern "C" fn mtk_gpt_clkevt_set_periodic(clk: *mut clock_event_device) -> i32 { let to = to_timer_of(clk); mtk_gpt_clkevt_time_stop(to, TIMER_CLK_EVT); mtk_gpt_clkevt_time_setup(to, (*to).of_clk.period as usize, TIMER_CLK_EVT); mtk_gpt_clkevt_time_start(to, true, TIMER_CLK_EVT); 0 }
unsafe extern "C" fn mtk_gpt_clkevt_next_event(event: usize, clk: *mut clock_event_device) -> i32 { let to = to_timer_of(clk); mtk_gpt_clkevt_time_stop(to, TIMER_CLK_EVT); mtk_gpt_clkevt_time_setup(to, event, TIMER_CLK_EVT); mtk_gpt_clkevt_time_start(to, false, TIMER_CLK_EVT); 0 }
unsafe extern "C" fn mtk_gpt_interrupt(_: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t { let clkevt = dev_id as *mut clock_event_device; let to = to_timer_of(clkevt); writel(gpt_irq_ack(TIMER_CLK_EVT), timer_of_base(to).add(GPT_IRQ_ACK_REG)); ((*clkevt).event_handler)(clkevt); IRQ_HANDLED }

unsafe fn mtk_gpt_setup(to: *mut timer_of, timer: u8, option: u8) { let base = timer_of_base(to); writel(GPT_CTRL_CLEAR | GPT_CTRL_DISABLE, base.add(gpt_ctrl_reg(timer))); writel(gpt_clk_src(GPT_CLK_SRC_SYS13M) | GPT_CLK_DIV1, base.add(gpt_clk_reg(timer))); writel(0, base.add(gpt_cmp_reg(timer))); writel(gpt_ctrl_op(option as u32) | GPT_CTRL_ENABLE, base.add(gpt_ctrl_reg(timer))); }
unsafe fn mtk_gpt_enable_irq(to: *mut timer_of, timer: u8) { let base = timer_of_base(to); writel(0, base.add(GPT_IRQ_EN_REG)); writel(0x3f, base.add(GPT_IRQ_ACK_REG)); let val = readl(base.add(GPT_IRQ_EN_REG)); writel(val | gpt_irq_enable(timer), base.add(GPT_IRQ_EN_REG)); }
unsafe extern "C" fn mtk_gpt_resume(clk: *mut clock_event_device) { mtk_gpt_enable_irq(to_timer_of(clk), TIMER_CLK_EVT); }
unsafe extern "C" fn mtk_gpt_suspend(clk: *mut clock_event_device) { let base = timer_of_base(to_timer_of(clk)); writel(0, base.add(GPT_IRQ_EN_REG)); writel(0x3f, base.add(GPT_IRQ_ACK_REG)); }

static mut TO: timer_of = timer_of { flags: 0, clkevt: clock_event_device { name: core::ptr::null(), rating: 0, cpumask: core::ptr::null(), features: 0, event_handler: mtk_syst_handler_event, set_state_shutdown: None, set_state_oneshot: None, set_state_periodic: None, tick_resume: None, set_next_event: None, suspend: None, resume: None }, of_irq: timer_of_irq { flags: 0, handler: None }, of_clk: timer_of_clock { period: 0 } };
unsafe extern "C" fn mtk_syst_handler_event(_: *mut clock_event_device) {}

unsafe extern "C" fn mtk_syst_init(node: *mut device_node) -> i32 {
    TO.clkevt.features = 0; TO.clkevt.set_state_shutdown = Some(mtk_syst_clkevt_shutdown); TO.clkevt.set_state_oneshot = Some(mtk_syst_clkevt_oneshot); TO.clkevt.tick_resume = Some(mtk_syst_clkevt_resume); TO.clkevt.set_next_event = Some(mtk_syst_clkevt_next_event); TO.of_irq.handler = Some(mtk_syst_handler);
    let ret = timer_of_init(node, &mut TO); if ret != 0 { return ret; }
    clockevents_config_and_register(&mut TO.clkevt, timer_of_rate(&mut TO), TIMER_SYNC_TICKS, 0xffff_ffff); 0
}

unsafe extern "C" fn mtk_gpt_init(node: *mut device_node) -> i32 {
    TO.clkevt.features = 0; TO.clkevt.set_state_shutdown = Some(mtk_gpt_clkevt_shutdown); TO.clkevt.set_state_periodic = Some(mtk_gpt_clkevt_set_periodic); TO.clkevt.set_state_oneshot = Some(mtk_gpt_clkevt_shutdown); TO.clkevt.tick_resume = Some(mtk_gpt_clkevt_shutdown); TO.clkevt.set_next_event = Some(mtk_gpt_clkevt_next_event); TO.clkevt.suspend = Some(mtk_gpt_suspend); TO.clkevt.resume = Some(mtk_gpt_resume); TO.of_irq.handler = Some(mtk_gpt_interrupt);
    let ret = timer_of_init(node, &mut TO); if ret != 0 { return ret; }
    mtk_gpt_setup(&mut TO, TIMER_CLK_SRC, GPT_CTRL_OP_FREERUN as u8);
    let base = timer_of_base(&mut TO).add(gpt_cnt_reg(TIMER_CLK_SRC)); gpt_sched_reg = base;
    sched_clock_register(mtk_gpt_read_sched_clock, 32, timer_of_rate(&mut TO));
    mtk_gpt_setup(&mut TO, TIMER_CLK_EVT, GPT_CTRL_OP_REPEAT as u8); clockevents_config_and_register(&mut TO.clkevt, timer_of_rate(&mut TO), TIMER_SYNC_TICKS, 0xffff_ffff); mtk_gpt_enable_irq(&mut TO, TIMER_CLK_EVT); 0
}

// TIMER_OF_DECLARE(mtk_mt6577, "mediatek,mt6577-timer", mtk_gpt_init);
// TIMER_OF_DECLARE(mtk_mt6765, "mediatek,mt6765-timer", mtk_syst_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
