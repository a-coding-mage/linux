// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2011 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com/
 *
 * samsung - Common hr-timer support (s3c and s5p)
 */

// C dependencies supplied by the surrounding kernel translation unit.

const REG_TCFG0: usize = 0x00;
const REG_TCFG1: usize = 0x04;
const REG_TCON: usize = 0x08;
const REG_TINT_CSTAT: usize = 0x44;

const TCFG0_PRESCALER_MASK: u32 = 0xff;
const TCFG0_PRESCALER1_SHIFT: u32 = 8;
const TCFG1_MUX_MASK: u32 = 0xf;

const SAMSUNG_PWM_NUM: usize = 5;
const CLOCK_EVT_FEAT_PERIODIC: u32 = 1 << 0;
const CLOCK_EVT_FEAT_ONESHOT: u32 = 1 << 1;
const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1 << 0;
const IRQF_TIMER: u32 = 1 << 4;
const IRQF_IRQPOLL: u32 = 1 << 8;
const IRQ_HANDLED: i32 = 1;
const HZ: usize = 100;

#[repr(C)]
pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct cpumask { _private: [u8; 0] }
#[repr(C)]
pub struct clocksource { _private: [u8; 0] }
#[repr(C)]
pub struct samsung_pwm_variant {
    pub bits: u8,
    pub div_base: u8,
    pub has_tint_cstat: bool,
    pub tclk_mask: u32,
    pub output_mask: u32,
}

#[repr(C)]
pub struct clock_event_device {
    pub name: *const u8,
    pub features: u32,
    pub rating: i32,
    pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut clock_event_device)>,
    pub cpumask: *const cpumask,
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}

#[repr(C)]
pub struct samsung_pwm_clocksource {
    base: *mut u8,
    source_reg: *const u8,
    irq: [u32; SAMSUNG_PWM_NUM],
    variant: samsung_pwm_variant,
    timerclk: *mut clk,
    event_id: u32,
    source_id: u32,
    tcnt_max: u32,
    tscaler_div: u32,
    tdiv: u32,
    clock_count_per_tick: usize,
}

extern "C" {
    static mut samsung_pwm_lock: raw_spinlock_t;
    fn readl(addr: *const u8) -> u32;
    fn readl_relaxed(addr: *const u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn fls(value: u32) -> i32;
    fn clk_get_rate(clk: *mut clk) -> usize;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_get(dev: *mut u8, name: *const u8) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn of_clk_get_by_name(np: *mut device_node, name: *const u8) -> *mut clk;
    fn irq_of_parse_and_map(np: *mut device_node, index: i32) -> u32;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn request_irq(irq: u32, handler: unsafe extern "C" fn(i32, *mut u8) -> i32, flags: u32, name: *const u8, dev: *mut clock_event_device) -> i32;
    fn cpumask_of(cpu: u32) -> *const cpumask;
    fn clockevents_config_and_register(dev: *mut clock_event_device, freq: usize, min_delta: u32, max_delta: u32);
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u8, rate: usize);
    fn clocksource_register_hz(cs: *mut clocksource, rate: usize) -> i32;
    fn panic(msg: *const u8) -> !;
}

static mut pwm: samsung_pwm_clocksource = samsung_pwm_clocksource {
    base: core::ptr::null_mut(), source_reg: core::ptr::null(), irq: [0; SAMSUNG_PWM_NUM],
    variant: samsung_pwm_variant { bits: 0, div_base: 0, has_tint_cstat: false, tclk_mask: 0, output_mask: 0 },
    timerclk: core::ptr::null_mut(), event_id: 0, source_id: 0, tcnt_max: 0,
    tscaler_div: 0, tdiv: 0, clock_count_per_tick: 0,
};

#[inline] fn reg_tcntb(chan: usize) -> usize { 0x0c + 12 * chan }
#[inline] fn reg_tcmpb(chan: usize) -> usize { 0x10 + 12 * chan }
#[inline] fn tcfg1_shift(x: u32) -> u32 { x * 4 }
#[inline] fn tcon_start(chan: u32) -> u32 { 1 << (4 * chan) }
#[inline] fn tcon_manualupdate(chan: u32) -> u32 { 1 << (4 * chan + 1) }
#[inline] fn tcon_autoreload(chan: u32) -> u32 { 1 << (4 * chan + if chan < 5 { 3 } else { 2 }) }

unsafe fn samsung_timer_set_prescale(channel: u32, prescale: u16) {
    let shift = if channel >= 2 { TCFG0_PRESCALER1_SHIFT } else { 0 };
    let reg = (readl(pwm.base.add(REG_TCFG0)) & !(TCFG0_PRESCALER_MASK << shift)) | ((prescale as u32 - 1) << shift);
    writel(reg, pwm.base.add(REG_TCFG0));
}

unsafe fn samsung_timer_set_divisor(channel: u32, divisor: u8) {
    let shift = tcfg1_shift(channel);
    let bits = ((fls(divisor as u32) - 1) as u8 - pwm.variant.div_base) as u32;
    let reg = (readl(pwm.base.add(REG_TCFG1)) & !(TCFG1_MUX_MASK << shift)) | (bits << shift);
    writel(reg, pwm.base.add(REG_TCFG1));
}

unsafe fn samsung_time_stop(mut channel: u32) {
    if channel > 0 { channel += 1; }
    let tcon = readl_relaxed(pwm.base.add(REG_TCON)) & !tcon_start(channel);
    writel_relaxed(tcon, pwm.base.add(REG_TCON));
}

unsafe fn samsung_time_setup(channel: u32, tcnt: u32) {
    let mut tcon_chan = channel;
    if tcon_chan > 0 { tcon_chan += 1; }
    let mut tcon = readl_relaxed(pwm.base.add(REG_TCON));
    tcon &= !(tcon_start(tcon_chan) | tcon_autoreload(tcon_chan));
    tcon |= tcon_manualupdate(tcon_chan);
    writel_relaxed(tcnt, pwm.base.add(reg_tcntb(channel as usize)));
    writel_relaxed(tcnt, pwm.base.add(reg_tcmpb(channel as usize)));
    writel_relaxed(tcon, pwm.base.add(REG_TCON));
}

unsafe fn samsung_time_start(mut channel: u32, periodic: bool) {
    if channel > 0 { channel += 1; }
    let mut tcon = readl_relaxed(pwm.base.add(REG_TCON));
    tcon &= !tcon_manualupdate(channel);
    tcon |= tcon_start(channel);
    if periodic { tcon |= tcon_autoreload(channel); } else { tcon &= !tcon_autoreload(channel); }
    writel_relaxed(tcon, pwm.base.add(REG_TCON));
}

unsafe extern "C" fn samsung_set_next_event(mut cycles: usize, _evt: *mut clock_event_device) -> i32 {
    if cycles == 0 { cycles = 1; }
    samsung_time_setup(pwm.event_id, cycles as u32);
    samsung_time_start(pwm.event_id, false);
    0
}
unsafe extern "C" fn samsung_shutdown(_evt: *mut clock_event_device) -> i32 { samsung_time_stop(pwm.event_id); 0 }
unsafe extern "C" fn samsung_set_periodic(_evt: *mut clock_event_device) -> i32 {
    samsung_time_stop(pwm.event_id); samsung_time_setup(pwm.event_id, (pwm.clock_count_per_tick - 1) as u32); samsung_time_start(pwm.event_id, true); 0
}
unsafe extern "C" fn samsung_clockevent_resume(_cev: *mut clock_event_device) {
    samsung_timer_set_prescale(pwm.event_id, pwm.tscaler_div as u16); samsung_timer_set_divisor(pwm.event_id, pwm.tdiv as u8);
    if pwm.variant.has_tint_cstat { let mask = 1u32 << pwm.event_id; writel(mask | (mask << 5), pwm.base.add(REG_TINT_CSTAT)); }
}

static mut time_event_device: clock_event_device = clock_event_device {
    name: b"samsung_event_timer\0".as_ptr(), features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT, rating: 200,
    set_next_event: Some(samsung_set_next_event), set_state_shutdown: Some(samsung_shutdown), set_state_periodic: Some(samsung_set_periodic),
    set_state_oneshot: Some(samsung_shutdown), tick_resume: Some(samsung_shutdown), resume: Some(samsung_clockevent_resume), cpumask: core::ptr::null(), event_handler: None,
};

unsafe extern "C" fn samsung_clock_event_isr(_irq: i32, dev_id: *mut u8) -> i32 {
    if pwm.variant.has_tint_cstat { let mask = 1u32 << pwm.event_id; writel(mask | (mask << 5), pwm.base.add(REG_TINT_CSTAT)); }
    let evt = dev_id as *mut clock_event_device; if let Some(handler) = (*evt).event_handler { handler(evt); } IRQ_HANDLED
}

unsafe fn samsung_clockevent_init() {
    let pclk = clk_get_rate(pwm.timerclk); samsung_timer_set_prescale(pwm.event_id, pwm.tscaler_div as u16); samsung_timer_set_divisor(pwm.event_id, pwm.tdiv as u8);
    let clock_rate = pclk / (pwm.tscaler_div as usize * pwm.tdiv as usize); pwm.clock_count_per_tick = clock_rate / HZ;
    time_event_device.cpumask = cpumask_of(0); clockevents_config_and_register(&mut time_event_device, clock_rate, 1, pwm.tcnt_max);
    let irq_number = pwm.irq[pwm.event_id as usize]; let _ = request_irq(irq_number, samsung_clock_event_isr, IRQF_TIMER | IRQF_IRQPOLL, b"samsung_time_irq\0".as_ptr(), &mut time_event_device);
    if pwm.variant.has_tint_cstat { let mask = 1u32 << pwm.event_id; writel(mask | (mask << 5), pwm.base.add(REG_TINT_CSTAT)); }
}

unsafe fn samsung_clocksource_suspend(_cs: *mut clocksource) { samsung_time_stop(pwm.source_id); }
unsafe fn samsung_clocksource_resume(_cs: *mut clocksource) { samsung_timer_set_prescale(pwm.source_id, pwm.tscaler_div as u16); samsung_timer_set_divisor(pwm.source_id, pwm.tdiv as u8); samsung_time_setup(pwm.source_id, pwm.tcnt_max); samsung_time_start(pwm.source_id, true); }
unsafe extern "C" fn samsung_clocksource_read(_c: *mut clocksource) -> u64 { !(readl_relaxed(pwm.source_reg) as u64) }
unsafe extern "C" fn samsung_read_sched_clock() -> u64 { samsung_clocksource_read(core::ptr::null_mut()) }

unsafe fn samsung_clocksource_init() -> i32 {
    let pclk = clk_get_rate(pwm.timerclk); samsung_timer_set_prescale(pwm.source_id, pwm.tscaler_div as u16); samsung_timer_set_divisor(pwm.source_id, pwm.tdiv as u8);
    let clock_rate = pclk / (pwm.tscaler_div as usize * pwm.tdiv as usize); samsung_time_setup(pwm.source_id, pwm.tcnt_max); samsung_time_start(pwm.source_id, true);
    pwm.source_reg = if pwm.source_id == 4 { pwm.base.add(0x40) } else { pwm.base.add(pwm.source_id as usize * 0x0c + 0x14) };
    sched_clock_register(samsung_read_sched_clock, pwm.variant.bits, clock_rate); 0
}

unsafe fn samsung_timer_resources() { let _ = clk_prepare_enable(pwm.timerclk); pwm.tcnt_max = (1u32 << pwm.variant.bits) - 1; if pwm.variant.bits == 16 { pwm.tscaler_div = 25; pwm.tdiv = 2; } else { pwm.tscaler_div = 2; pwm.tdiv = 1; } }

unsafe fn _samsung_pwm_clocksource_init() -> i32 {
    let mut mask = !pwm.variant.output_mask & ((1u32 << SAMSUNG_PWM_NUM) - 1); let channel = fls(mask) - 1; if channel < 0 { return -22; } pwm.source_id = channel as u32;
    mask &= !(1 << channel); let channel = fls(mask) - 1; if channel < 0 { return -22; } pwm.event_id = channel as u32;
    samsung_timer_resources(); samsung_clockevent_init(); samsung_clocksource_init()
}

pub unsafe extern "C" fn samsung_pwm_clocksource_init(base: *mut u8, irqs: *const u32, variant: *const samsung_pwm_variant) {
    pwm.base = base; core::ptr::copy_nonoverlapping(variant, &mut pwm.variant, 1); core::ptr::copy_nonoverlapping(irqs, pwm.irq.as_mut_ptr(), SAMSUNG_PWM_NUM);
    pwm.timerclk = clk_get(core::ptr::null_mut(), b"timers\0".as_ptr()); if pwm.timerclk.is_null() { panic(b"failed to get timers clock for timer\0".as_ptr()); } let _ = _samsung_pwm_clocksource_init();
}

// Preserved from CONFIG_TIMER_OF builds; the surrounding kernel supplies the
// device-tree iteration and registration machinery.
#[cfg(feature = "CONFIG_TIMER_OF")]
unsafe fn samsung_pwm_alloc(np: *mut device_node, variant: *const samsung_pwm_variant) -> i32 {
    core::ptr::copy_nonoverlapping(variant, &mut pwm.variant, 1);
    for i in 0..SAMSUNG_PWM_NUM { pwm.irq[i] = irq_of_parse_and_map(np, i as i32); }
    pwm.base = of_iomap(np, 0);
    if pwm.base.is_null() { return -6; }
    pwm.timerclk = of_clk_get_by_name(np, b"timers\0".as_ptr());
    if pwm.timerclk.is_null() { iounmap(pwm.base); pwm.base = core::ptr::null_mut(); return -2; }
    let ret = _samsung_pwm_clocksource_init();
    if ret != 0 { clk_put(pwm.timerclk); pwm.timerclk = core::ptr::null_mut(); iounmap(pwm.base); pwm.base = core::ptr::null_mut(); }
    ret
}

#[cfg(feature = "CONFIG_TIMER_OF")]
static s3c24xx_variant: samsung_pwm_variant = samsung_pwm_variant { bits: 16, div_base: 1, has_tint_cstat: false, tclk_mask: 1 << 4, output_mask: 0 };
#[cfg(feature = "CONFIG_TIMER_OF")]
unsafe extern "C" fn s3c2410_pwm_clocksource_init(np: *mut device_node) -> i32 { samsung_pwm_alloc(np, &s3c24xx_variant) }

#[cfg(feature = "CONFIG_TIMER_OF")]
static s3c64xx_variant: samsung_pwm_variant = samsung_pwm_variant { bits: 32, div_base: 0, has_tint_cstat: true, tclk_mask: (1 << 7) | (1 << 6) | (1 << 5), output_mask: 0 };
#[cfg(feature = "CONFIG_TIMER_OF")]
unsafe extern "C" fn s3c64xx_pwm_clocksource_init(np: *mut device_node) -> i32 { samsung_pwm_alloc(np, &s3c64xx_variant) }

#[cfg(feature = "CONFIG_TIMER_OF")]
static s5p64x0_variant: samsung_pwm_variant = samsung_pwm_variant { bits: 32, div_base: 0, has_tint_cstat: true, tclk_mask: 0, output_mask: 0 };
#[cfg(feature = "CONFIG_TIMER_OF")]
unsafe extern "C" fn s5p64x0_pwm_clocksource_init(np: *mut device_node) -> i32 { samsung_pwm_alloc(np, &s5p64x0_variant) }

#[cfg(feature = "CONFIG_TIMER_OF")]
static s5p_variant: samsung_pwm_variant = samsung_pwm_variant { bits: 32, div_base: 0, has_tint_cstat: true, tclk_mask: 1 << 5, output_mask: 0 };
#[cfg(feature = "CONFIG_TIMER_OF")]
unsafe extern "C" fn s5p_pwm_clocksource_init(np: *mut device_node) -> i32 { samsung_pwm_alloc(np, &s5p_variant) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
