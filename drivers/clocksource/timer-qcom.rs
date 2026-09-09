// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2007 Google, Inc.
 * Copyright (c) 2009-2012,2014, The Linux Foundation. All rights reserved.
 */

// Linux kernel dependencies supplied by other translation units.

const TIMER_MATCH_VAL: usize = 0x0000;
const TIMER_COUNT_VAL: usize = 0x0004;
const TIMER_ENABLE: usize = 0x0008;
const TIMER_ENABLE_CLR_ON_MATCH_EN: u32 = 1 << 1;
const TIMER_ENABLE_EN: u32 = 1 << 0;
const TIMER_CLEAR: usize = 0x000C;
const DGT_CLK_CTL: usize = 0x10;
const DGT_CLK_CTL_DIV_4: u32 = 0x3;
const TIMER_STS_GPT0_CLR_PEND: u32 = 1 << 10;

const GPT_HZ: u32 = 32768;

static mut event_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut sts_base: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe extern "C" {
    fn clockevent_state_oneshot(evt: *mut clock_event_device) -> bool;
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn cpu_relax();
    fn pr_err(fmt: *const u8, ...);
    fn per_cpu_ptr(ptr: *mut clock_event_device, cpu: u32) -> *mut clock_event_device;
    fn enable_percpu_irq(irq: i32, irq_type: u32);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
                   flags: u32, name: *const u8, dev: *mut core::ffi::c_void) -> i32;
    fn disable_percpu_irq(irq: i32);
    fn clockevents_config_and_register(evt: *mut clock_event_device, freq: u32, min_delta: u32, max_delta: u32);
    fn alloc_percpu_clock_event_device() -> *mut clock_event_device;
    fn request_percpu_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
                          name: *const u8, dev: *mut core::ffi::c_void) -> i32;
    fn cpuhp_setup_state(state: i32, name: *const u8,
                         starting: unsafe extern "C" fn(u32) -> i32,
                         dying: unsafe extern "C" fn(u32) -> i32) -> i32;
    fn free_percpu_irq(irq: i32, dev: *mut clock_event_device);
    fn clocksource_register_hz(cs: *mut clocksource, hz: u32) -> i32;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: i32, hz: u32);
    fn register_current_timer_delay(timer: *mut delay_timer);
    fn of_iomap(np: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn irq_of_parse_and_map(np: *mut device_node, index: i32) -> i32;
    fn of_property_read_u32(np: *mut device_node, name: *const u8, value: *mut u32) -> i32;
    fn of_address_to_resource(np: *mut device_node, index: i32, res: *mut resource) -> i32;
    fn ioremap(start: u64, size: u64) -> *mut core::ffi::c_void;
    fn resource_size(res: *const resource) -> u64;
    fn iounmap(addr: *mut core::ffi::c_void);
}

#[repr(C)] pub struct clock_event_device {
    pub irq: i32, pub name: *const u8, pub features: u32, pub rating: i32,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    pub cpumask: *const core::ffi::c_void,
    pub event_handler: unsafe extern "C" fn(*mut clock_event_device),
}
#[repr(C)] pub struct clocksource { pub name: *const u8, pub rating: i32, pub read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>, pub mask: u64, pub flags: u32 }
#[repr(C)] pub struct delay_timer { pub read_current_timer: Option<unsafe extern "C" fn() -> u32>, pub freq: u32 }
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct resource { pub start: u64, pub end: u64 }
#[repr(C)] pub struct irqreturn_t(pub i32);

const IRQ_HANDLED: irqreturn_t = irqreturn_t(1);
const IRQ_TYPE_EDGE_RISING: u32 = 1;
const IRQF_TIMER: u32 = 1 << 4;
const IRQF_NOBALANCING: u32 = 1 << 8;
const IRQF_TRIGGER_RISING: u32 = 1 << 21;
const CLOCK_EVT_FEAT_ONESHOT: u32 = 1;
const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1 << 1;

static mut msm_evt: *mut clock_event_device = core::ptr::null_mut();
static mut source_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut msm_timer_irq: i32 = 0;
static mut msm_timer_has_ppi: i32 = 0;

unsafe extern "C" fn msm_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let evt = dev_id as *mut clock_event_device;
    if clockevent_state_oneshot(evt) {
        let mut ctrl = readl_relaxed(event_base.add(TIMER_ENABLE));
        ctrl &= !TIMER_ENABLE_EN;
        writel_relaxed(ctrl, event_base.add(TIMER_ENABLE));
    }
    ((*evt).event_handler)(evt);
    IRQ_HANDLED
}

unsafe extern "C" fn msm_timer_set_next_event(cycles: usize, _evt: *mut clock_event_device) -> i32 {
    let mut ctrl = readl_relaxed(event_base.add(TIMER_ENABLE));
    ctrl &= !TIMER_ENABLE_EN;
    writel_relaxed(ctrl, event_base.add(TIMER_ENABLE));
    writel_relaxed(ctrl, event_base.add(TIMER_CLEAR));
    writel_relaxed(cycles as u32, event_base.add(TIMER_MATCH_VAL));
    if !sts_base.is_null() { while readl_relaxed(sts_base) & TIMER_STS_GPT0_CLR_PEND != 0 { cpu_relax(); } }
    writel_relaxed(ctrl | TIMER_ENABLE_EN, event_base.add(TIMER_ENABLE));
    0
}

unsafe extern "C" fn msm_timer_shutdown(_evt: *mut clock_event_device) -> i32 {
    let mut ctrl = readl_relaxed(event_base.add(TIMER_ENABLE));
    ctrl &= !(TIMER_ENABLE_EN | TIMER_ENABLE_CLR_ON_MATCH_EN);
    writel_relaxed(ctrl, event_base.add(TIMER_ENABLE));
    0
}

unsafe extern "C" fn msm_read_timer_count(_cs: *mut clocksource) -> u64 { readl_relaxed(source_base.add(TIMER_COUNT_VAL)) as u64 }
static mut msm_clocksource: clocksource = clocksource { name: b"dg_timer\0".as_ptr(), rating: 300, read: Some(msm_read_timer_count), mask: 0xffff_ffff, flags: CLOCK_SOURCE_IS_CONTINUOUS };

unsafe extern "C" fn msm_local_timer_starting_cpu(cpu: u32) -> i32 { let evt = per_cpu_ptr(msm_evt, cpu); (*evt).irq = msm_timer_irq; (*evt).name = b"msm_timer\0".as_ptr(); (*evt).features = CLOCK_EVT_FEAT_ONESHOT; (*evt).rating = 200; (*evt).set_state_shutdown = Some(msm_timer_shutdown); (*evt).set_state_oneshot = Some(msm_timer_shutdown); (*evt).tick_resume = Some(msm_timer_shutdown); (*evt).set_next_event = Some(msm_timer_set_next_event); clockevents_config_and_register(evt, GPT_HZ, 4, 0xffff_ffff); if msm_timer_has_ppi != 0 { enable_percpu_irq((*evt).irq, IRQ_TYPE_EDGE_RISING); } else { let err = request_irq((*evt).irq, msm_timer_interrupt, IRQF_TIMER | IRQF_NOBALANCING | IRQF_TRIGGER_RISING, b"gp_timer\0".as_ptr(), evt as *mut _); if err != 0 { pr_err(b"request_irq failed\n\0".as_ptr()); } } 0 }
unsafe extern "C" fn msm_local_timer_dying_cpu(cpu: u32) -> i32 { let evt = per_cpu_ptr(msm_evt, cpu); disable_percpu_irq((*evt).irq); 0 }
unsafe extern "C" fn msm_sched_clock_read() -> u64 { (msm_clocksource.read.unwrap())(&mut msm_clocksource) }
unsafe extern "C" fn msm_read_current_timer() -> u32 { msm_sched_clock_read() as u32 }
static mut msm_delay_timer: delay_timer = delay_timer { read_current_timer: Some(msm_read_current_timer), freq: 0 };

// Device-tree registration is provided by the kernel's TIMER_OF_DECLARE mechanism.

unsafe extern "C" fn msm_timer_init(dgt_hz: u32, sched_bits: i32, irq: i32, percpu: bool) -> i32 {
    msm_timer_irq = irq;
    msm_timer_has_ppi = percpu as i32;
    msm_evt = alloc_percpu_clock_event_device();
    if msm_evt.is_null() { pr_err(b"memory allocation failed for clockevents\n\0".as_ptr()); }
    let mut res = 0;
    if !msm_evt.is_null() && percpu { res = request_percpu_irq(irq, msm_timer_interrupt, b"gp_timer\0".as_ptr(), msm_evt); }
    if res != 0 { pr_err(b"request_percpu_irq failed\n\0".as_ptr()); }
    else if !msm_evt.is_null() {
        res = cpuhp_setup_state(0, b"clockevents/qcom/timer:starting\0".as_ptr(), msm_local_timer_starting_cpu, msm_local_timer_dying_cpu);
        if res != 0 { free_percpu_irq(irq, msm_evt); }
    }
    writel_relaxed(TIMER_ENABLE_EN, source_base.add(TIMER_ENABLE));
    res = clocksource_register_hz(&mut msm_clocksource, dgt_hz);
    if res != 0 { pr_err(b"clocksource_register failed\n\0".as_ptr()); }
    sched_clock_register(msm_sched_clock_read, sched_bits, dgt_hz);
    msm_delay_timer.freq = dgt_hz;
    register_current_timer_delay(&mut msm_delay_timer);
    res
}

unsafe extern "C" fn msm_dt_timer_init(np: *mut device_node) -> i32 {
    let mut freq = 0u32;
    let mut percpu_offset = 0u32;
    let base = of_iomap(np, 0);
    if base.is_null() { pr_err(b"Failed to map event base\n\0".as_ptr()); return -6; }
    let irq = irq_of_parse_and_map(np, 1);
    if irq <= 0 { pr_err(b"Can't get irq\n\0".as_ptr()); return -22; }
    if of_property_read_u32(np, b"cpu-offset\0".as_ptr(), &mut percpu_offset) != 0 { percpu_offset = 0; }
    let mut res = resource { start: 0, end: 0 };
    let ret = of_address_to_resource(np, 0, &mut res);
    if ret != 0 { pr_err(b"Failed to parse DGT resource\n\0".as_ptr()); return ret; }
    let cpu0_base = ioremap(res.start + percpu_offset as u64, resource_size(&res));
    if cpu0_base.is_null() { pr_err(b"Failed to map source base\n\0".as_ptr()); return -22; }
    if of_property_read_u32(np, b"clock-frequency\0".as_ptr(), &mut freq) != 0 { iounmap(cpu0_base); pr_err(b"Unknown frequency\n\0".as_ptr()); return -22; }
    event_base = base.add(0x4);
    sts_base = base.add(0x88);
    source_base = cpu0_base.add(0x24);
    freq /= 4;
    writel_relaxed(DGT_CLK_CTL_DIV_4, source_base.add(DGT_CLK_CTL));
    let ret = msm_timer_init(freq, 32, irq, percpu_offset != 0);
    if ret != 0 { iounmap(cpu0_base); }
    ret
}

// TIMER_OF_DECLARE(kpss_timer, "qcom,kpss-timer", msm_dt_timer_init);
// TIMER_OF_DECLARE(scss_timer, "qcom,scss-timer", msm_dt_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
