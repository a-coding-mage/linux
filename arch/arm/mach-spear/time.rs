// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/plat-spear/time.c
 *
 * Copyright (C) 2010 ST Microelectronics
 * Shiraz Hashim<shiraz.linux.kernel@gmail.com>
 */

/* Kernel includes and the generic timer dependency are supplied externally. */

/*
 * We would use TIMER0 and TIMER1 as clockevent and clocksource.
 * Timer0 and Timer1 both belong to same gpt block in cpu subbsystem. Further
 * they share same functional clock. Any change in one's functional clock will
 * also affect other timer.
 */

const CLKEVT: usize = 0; /* gpt0, channel0 as clockevent */
const CLKSRC: usize = 1; /* gpt0, channel1 as clocksource */

/* Register offsets, x is channel number */
const fn CR(x: usize) -> usize { x * 0x80 + 0x80 }
const fn IR(x: usize) -> usize { x * 0x80 + 0x84 }
const fn LOAD(x: usize) -> usize { x * 0x80 + 0x88 }
const fn COUNT(x: usize) -> usize { x * 0x80 + 0x8C }

/* Reg bit definitions */
const CTRL_INT_ENABLE: u16 = 0x0100;
const CTRL_ENABLE: u16 = 0x0020;
const CTRL_ONE_SHOT: u16 = 0x0010;

const CTRL_PRESCALER1: u32 = 0x0;
const CTRL_PRESCALER2: u32 = 0x1;
const CTRL_PRESCALER4: u32 = 0x2;
const CTRL_PRESCALER8: u32 = 0x3;
const CTRL_PRESCALER16: u32 = 0x4;
const CTRL_PRESCALER32: u32 = 0x5;
const CTRL_PRESCALER64: u32 = 0x6;
const CTRL_PRESCALER128: u32 = 0x7;
const CTRL_PRESCALER256: u32 = 0x8;

const INT_STATUS: u16 = 0x1;

/* Minimum clocksource/clockevent timer range in seconds */
const SPEAR_MIN_RANGE: u32 = 4;

static mut gpt_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut gpt_clk: *mut clk = core::ptr::null_mut();

extern "C" {
    type clk;
    type clock_event_device;
    type device_node;
    type cpumask;
    type clocksource;
    static mut clkevt: clock_event_device;
    fn clk_get_rate(clk: *mut clk) -> u32;
    fn writew(value: u16, address: *mut core::ffi::c_void);
    fn readw(address: *mut core::ffi::c_void) -> u16;
    fn clocksource_mmio_init(address: *mut core::ffi::c_void, name: *const u8,
        rating: u32, mask: u32, bits: u32, read: *const core::ffi::c_void) -> i32;
    fn clockevents_config_and_register(dev: *mut clock_event_device, freq: u32, min: u32, max: u32);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
        flags: u32, name: *const u8, dev: *mut core::ffi::c_void) -> i32;
    fn irq_of_parse_and_map(np: *mut device_node, index: i32) -> i32;
    fn of_find_matching_node(from: *mut device_node, match_table: *const of_device_id) -> *mut device_node;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn clk_get_sys(name: *const u8, id: *const u8) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_put(clk: *mut clk);
    fn iounmap(address: *mut core::ffi::c_void);
    fn of_node_put(np: *mut device_node);
    fn cpumask_of(cpu: u32) -> *const cpumask;
    fn pr_err(format: *const u8, ...);
    fn IS_ERR(ptr: *mut clk) -> bool;
    fn clocksource_mmio_readw_up() -> u64;
}

type irqreturn_t = i32;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_TIMER: u32 = 0x00000200;
const HZ: u32 = 100;

#[repr(C)]
struct of_device_id { compatible: *const u8 }

static mut timer_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"st,spear-timer\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

unsafe fn spear_clocksource_init() {
    let mut tick_rate: u32;
    let mut val: u16;
    writew(CTRL_PRESCALER256 as u16, gpt_base.add(CR(CLKSRC)));
    tick_rate = clk_get_rate(gpt_clk);
    tick_rate >>= CTRL_PRESCALER256;
    writew(0xFFFF, gpt_base.add(LOAD(CLKSRC)));
    val = readw(gpt_base.add(CR(CLKSRC)));
    val &= !CTRL_ONE_SHOT;
    val |= CTRL_ENABLE;
    writew(val, gpt_base.add(CR(CLKSRC)));
    clocksource_mmio_init(gpt_base.add(COUNT(CLKSRC)), b"tmr1\0".as_ptr(), 200, tick_rate, 16, clocksource_mmio_readw_up as *const _);
}

unsafe fn spear_timer_shutdown(_evt: *mut clock_event_device) {
    let mut val = readw(gpt_base.add(CR(CLKEVT)));
    val &= !CTRL_ENABLE;
    writew(val, gpt_base.add(CR(CLKEVT)));
}

unsafe fn spear_shutdown(evt: *mut clock_event_device) -> i32 { spear_timer_shutdown(evt); 0 }
unsafe fn spear_set_oneshot(evt: *mut clock_event_device) -> i32 {
    spear_timer_shutdown(evt);
    let mut val = readw(gpt_base.add(CR(CLKEVT)));
    val |= CTRL_ONE_SHOT;
    writew(val, gpt_base.add(CR(CLKEVT)));
    0
}
unsafe fn spear_set_periodic(evt: *mut clock_event_device) -> i32 {
    spear_timer_shutdown(evt);
    let mut period = clk_get_rate(gpt_clk) / HZ;
    period >>= CTRL_PRESCALER16;
    writew(period as u16, gpt_base.add(LOAD(CLKEVT)));
    let mut val = readw(gpt_base.add(CR(CLKEVT)));
    val &= !CTRL_ONE_SHOT;
    val |= CTRL_ENABLE | CTRL_INT_ENABLE;
    writew(val, gpt_base.add(CR(CLKEVT)));
    0
}

unsafe fn clockevent_next_event(cycles: u32, _dev: *mut clock_event_device) -> i32 {
    let mut val = readw(gpt_base.add(CR(CLKEVT)));
    if val & CTRL_ENABLE != 0 { writew(val & !CTRL_ENABLE, gpt_base.add(CR(CLKEVT))); }
    writew(cycles as u16, gpt_base.add(LOAD(CLKEVT)));
    val |= CTRL_ENABLE | CTRL_INT_ENABLE;
    writew(val, gpt_base.add(CR(CLKEVT)));
    0
}

unsafe extern "C" fn spear_timer_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    writew(INT_STATUS, gpt_base.add(IR(CLKEVT)));
    1
}

unsafe fn spear_clockevent_init(irq: i32) {
    writew(CTRL_PRESCALER16 as u16, gpt_base.add(CR(CLKEVT)));
    let tick_rate = clk_get_rate(gpt_clk) >> CTRL_PRESCALER16;
    clockevents_config_and_register(&mut clkevt, tick_rate, 3, 0xfff0);
    if request_irq(irq, spear_timer_interrupt, IRQF_TIMER, b"timer\0".as_ptr(), core::ptr::null_mut()) != 0 {
        pr_err(b"Failed to request irq %d (timer)\n\0".as_ptr(), irq);
    }
}

pub unsafe fn spear_setup_of_timer() {
    let np = of_find_matching_node(core::ptr::null_mut(), timer_of_match.as_ptr());
    if np.is_null() { pr_err(b"%s: No timer passed via DT\n\0".as_ptr(), b"spear_setup_of_timer\0".as_ptr()); return; }
    let irq = irq_of_parse_and_map(np, 0);
    if irq == 0 { pr_err(b"%s: No irq passed for timer via DT\n\0".as_ptr(), b"spear_setup_of_timer\0".as_ptr()); of_node_put(np); return; }
    gpt_base = of_iomap(np, 0);
    if gpt_base.is_null() { pr_err(b"%s: of iomap failed\n\0".as_ptr(), b"spear_setup_of_timer\0".as_ptr()); of_node_put(np); return; }
    gpt_clk = clk_get_sys(b"gpt0\0".as_ptr(), core::ptr::null());
    if IS_ERR(gpt_clk) { pr_err(b"%s:couldn't get clk for gpt\n\0".as_ptr(), b"spear_setup_of_timer\0".as_ptr()); iounmap(gpt_base); of_node_put(np); return; }
    if clk_prepare_enable(gpt_clk) < 0 { pr_err(b"%s:couldn't prepare-enable gpt clock\n\0".as_ptr(), b"spear_setup_of_timer\0".as_ptr()); clk_put(gpt_clk); iounmap(gpt_base); of_node_put(np); return; }
    of_node_put(np);
    spear_clockevent_init(irq);
    spear_clocksource_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
