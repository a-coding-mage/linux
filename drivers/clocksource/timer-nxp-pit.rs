// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012-2013 Freescale Semiconductor, Inc.
 * Copyright 2018,2021-2025 NXP
 */

// Translated from the Linux kernel implementation source.

const PIT0_OFFSET: usize = 0x100;
const PIT_CH: usize = |n: usize| PIT0_OFFSET + 0x10 * n;
const PITMCR_FRZ: u32 = 1 << 0;
const PITMCR_MDIS: u32 = 1 << 1;
const PITCVAL_OFFSET: usize = 0x04;
const PITTCTRL_TEN: u32 = 1 << 0;
const PITTCTRL_TIE: u32 = 1 << 1;
const PITTFLG_TIF: u32 = 1 << 0;

#[repr(C)]
struct pit_timer {
    clksrc_base: *mut core::ffi::c_void,
    clkevt_base: *mut core::ffi::c_void,
    ced: clock_event_device,
    cs: clocksource,
    rate: i32,
}

#[repr(C)]
struct pit_timer_data {
    max_pit_instances: i32,
}

static mut pit_timers: PerCpu<*mut pit_timer> = PerCpu::new();
static mut pit_instances: i32 = 0;
static mut max_pit_instances: i32 = 1;
static mut sched_clock_base: *mut core::ffi::c_void = core::ptr::null_mut();

#[inline]
unsafe fn ced_to_pit(ced: *mut clock_event_device) -> *mut pit_timer {
    container_of!(ced, pit_timer, ced)
}

#[inline]
unsafe fn cs_to_pit(cs: *mut clocksource) -> *mut pit_timer {
    container_of!(cs, pit_timer, cs)
}

#[inline]
unsafe fn pit_module_enable(base: *mut core::ffi::c_void) { writel(0, base); }

#[inline]
unsafe fn pit_module_disable(base: *mut core::ffi::c_void) { writel(PITMCR_MDIS, base); }

#[inline]
unsafe fn pit_timer_enable(base: *mut core::ffi::c_void, tie: bool) {
    let val = PITTCTRL_TEN | if tie { PITTCTRL_TIE } else { 0 };
    writel(val, base.add(0x08));
}

#[inline]
unsafe fn pit_timer_disable(base: *mut core::ffi::c_void) { writel(0, base.add(0x08)); }

#[inline]
unsafe fn pit_timer_set_counter(base: *mut core::ffi::c_void, cnt: u32) { writel(cnt, base); }

#[inline]
unsafe fn pit_timer_irqack(pit: *mut pit_timer) { writel(PITTFLG_TIF, (*pit).clkevt_base.add(0x0c)); }

unsafe fn pit_read_sched_clock() -> u64 { (!readl(sched_clock_base)) as u64 }

unsafe fn pit_timer_clocksource_read(cs: *mut clocksource) -> u64 {
    let pit = cs_to_pit(cs);
    (!readl((*pit).clksrc_base.add(PITCVAL_OFFSET))) as u64
}

unsafe fn pit_clocksource_init(pit: *mut pit_timer, name: *const core::ffi::c_char,
                               base: *mut core::ffi::c_void, rate: usize) -> i32 {
    (*pit).clksrc_base = base.add(PIT_CH(2));
    (*pit).cs.name = name;
    (*pit).cs.rating = 300;
    (*pit).cs.read = Some(pit_timer_clocksource_read);
    (*pit).cs.mask = clocksource_mask(32);
    (*pit).cs.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    pit_timer_disable((*pit).clksrc_base);
    pit_timer_set_counter((*pit).clksrc_base, u32::MAX);
    pit_timer_enable((*pit).clksrc_base, false);
    sched_clock_base = (*pit).clksrc_base.add(PITCVAL_OFFSET);
    sched_clock_register(pit_read_sched_clock, 32, rate);
    clocksource_register_hz(&mut (*pit).cs, rate)
}

unsafe fn pit_set_next_event(delta: usize, ced: *mut clock_event_device) -> i32 {
    let pit = ced_to_pit(ced);
    pit_timer_disable((*pit).clkevt_base);
    pit_timer_set_counter((*pit).clkevt_base, delta.wrapping_sub(1) as u32);
    pit_timer_enable((*pit).clkevt_base, true);
    0
}

unsafe fn pit_shutdown(ced: *mut clock_event_device) -> i32 {
    pit_timer_disable((*ced_to_pit(ced)).clkevt_base); 0
}

unsafe fn pit_set_periodic(ced: *mut clock_event_device) -> i32 {
    let pit = ced_to_pit(ced);
    pit_set_next_event(((*pit).rate as usize) / HZ, ced); 0
}

unsafe fn pit_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let ced = dev_id as *mut clock_event_device;
    let pit = ced_to_pit(ced);
    pit_timer_irqack(pit);
    if likely(clockevent_state_oneshot(ced)) { pit_timer_disable((*pit).clkevt_base); }
    ((*ced).event_handler)(ced);
    IRQ_HANDLED
}

unsafe fn pit_clockevent_per_cpu_init(pit: *mut pit_timer, name: *const core::ffi::c_char,
    base: *mut core::ffi::c_void, rate: usize, irq: i32, cpu: u32) -> i32 {
    (*pit).clkevt_base = base.add(PIT_CH(3));
    (*pit).rate = rate as i32;
    pit_timer_disable((*pit).clkevt_base);
    pit_timer_irqack(pit);
    let ret = request_irq(irq, Some(pit_timer_interrupt), IRQF_TIMER | IRQF_NOBALANCING,
                          name, &mut (*pit).ced as *mut _ as *mut core::ffi::c_void);
    if ret != 0 { return ret; }
    (*pit).ced.cpumask = cpumask_of(cpu);
    (*pit).ced.irq = irq;
    (*pit).ced.name = name;
    (*pit).ced.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    (*pit).ced.set_state_shutdown = Some(pit_shutdown);
    (*pit).ced.set_state_periodic = Some(pit_set_periodic);
    (*pit).ced.set_next_event = Some(pit_set_next_event);
    (*pit).ced.rating = 300;
    per_cpu!(pit_timers, cpu, pit);
    0
}

unsafe fn pit_clockevent_per_cpu_exit(pit: *mut pit_timer, cpu: u32) {
    pit_timer_disable((*pit).clkevt_base);
    free_irq((*pit).ced.irq, &mut (*pit).ced as *mut _ as *mut core::ffi::c_void);
    per_cpu!(pit_timers, cpu, core::ptr::null_mut());
}

unsafe fn pit_clockevent_starting_cpu(cpu: u32) -> i32 {
    let pit = per_cpu!(pit_timers, cpu);
    if pit.is_null() { return 0; }
    let ret = irq_force_affinity((*pit).ced.irq, cpumask_of(cpu));
    if ret != 0 { pit_clockevent_per_cpu_exit(pit, cpu); return ret; }
    clockevents_config_and_register(&mut (*pit).ced, (*pit).rate as usize, 2, 0xffff_ffff);
    0
}

// The remaining kernel-facing declarations and registration macros are preserved as extern interfaces.
unsafe fn pit_timer_init(np: *mut device_node) -> i32 { todo!("direct translation requires kernel dependencies") }
unsafe fn pit_timer_probe(pdev: *mut platform_device) -> i32 { todo!("direct translation requires kernel dependencies") }

static mut s32g2_data: pit_timer_data = pit_timer_data { max_pit_instances: 2 };
static mut pit_timer_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "nxp,s32g2-pit", data: &mut s32g2_data as *mut _ as *const core::ffi::c_void },
    of_device_id::empty(),
];

static mut nxp_pit_driver: platform_driver = platform_driver::new("nxp-pit", pit_timer_probe);

// MODULE_DEVICE_TABLE(of, pit_timer_of_match);
// builtin_platform_driver(nxp_pit_driver);
// TIMER_OF_DECLARE(vf610, "fsl,vf610-pit", pit_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
