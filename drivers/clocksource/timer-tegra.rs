// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 Google, Inc.
 *
 * Author:
 *	Colin Cross <ccross@google.com>
 */

// Dependency declarations supplied by the Linux kernel and timer-of headers

const RTC_SECONDS: usize = 0x08;
const RTC_SHADOW_SECONDS: usize = 0x0c;
const RTC_MILLISECONDS: usize = 0x10;

const TIMERUS_CNTR_1US: usize = 0x10;
const TIMERUS_USEC_CFG: usize = 0x14;
const TIMERUS_CNTR_FREEZE: usize = 0x4c;

const TIMER_PTV: usize = 0x0;
const TIMER_PTV_EN: u32 = 1u32 << 31;
const TIMER_PTV_PER: u32 = 1u32 << 30;
const TIMER_PCR: usize = 0x4;
const TIMER_PCR_INTR_CLR: u32 = 1u32 << 30;

const TIMER1_BASE: usize = 0x00;
const TIMER2_BASE: usize = 0x08;
const TIMER3_BASE: usize = 0x50;
const TIMER4_BASE: usize = 0x58;
const TIMER10_BASE: usize = 0x90;

const TIMER1_IRQ_IDX: i32 = 0;
const TIMER10_IRQ_IDX: i32 = 10;
const TIMER_1MHZ: u32 = 1_000_000;

static mut usec_config: u32 = 0;
static mut timer_reg_base: *mut core::ffi::c_void = core::ptr::null_mut();

extern "C" {
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
}

unsafe fn tegra_timer_set_next_event(cycles: usize, evt: *mut clock_event_device) -> i32 {
    let reg_base = timer_of_base(to_timer_of(evt));
    writel_relaxed(TIMER_PTV_EN | cycles.wrapping_sub(1) as u32, reg_base.add(TIMER_PTV));
    0
}

unsafe fn tegra_timer_shutdown(evt: *mut clock_event_device) -> i32 {
    let reg_base = timer_of_base(to_timer_of(evt));
    writel_relaxed(0, reg_base.add(TIMER_PTV));
    0
}

unsafe fn tegra_timer_set_periodic(evt: *mut clock_event_device) -> i32 {
    let reg_base = timer_of_base(to_timer_of(evt));
    let period = timer_of_period(to_timer_of(evt));
    writel_relaxed(TIMER_PTV_EN | TIMER_PTV_PER | period.wrapping_sub(1) as u32,
                   reg_base.add(TIMER_PTV));
    0
}

unsafe extern "C" fn tegra_timer_isr(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let evt = dev_id as *mut clock_event_device;
    let reg_base = timer_of_base(to_timer_of(evt));
    writel_relaxed(TIMER_PCR_INTR_CLR, reg_base.add(TIMER_PCR));
    ((*evt).event_handler)(evt);
    IRQ_HANDLED
}

unsafe fn tegra_timer_suspend(evt: *mut clock_event_device) {
    let reg_base = timer_of_base(to_timer_of(evt));
    writel_relaxed(TIMER_PCR_INTR_CLR, reg_base.add(TIMER_PCR));
}

unsafe fn tegra_timer_resume(_evt: *mut clock_event_device) {
    writel_relaxed(usec_config, timer_reg_base.add(TIMERUS_USEC_CFG));
}

// DEFINE_PER_CPU(struct timer_of, tegra_to)
static mut tegra_to: timer_of = timer_of {
    flags: TIMER_OF_CLOCK | TIMER_OF_BASE,
    clkevt: clock_event_device {
        name: "tegra_timer\0".as_ptr() as *const i8,
        features: CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_PERIODIC,
        set_next_event: Some(tegra_timer_set_next_event),
        set_state_shutdown: Some(tegra_timer_shutdown),
        set_state_periodic: Some(tegra_timer_set_periodic),
        set_state_oneshot: Some(tegra_timer_shutdown),
        tick_resume: Some(tegra_timer_shutdown),
        suspend: Some(tegra_timer_suspend),
        resume: Some(tegra_timer_resume),
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

unsafe fn tegra_timer_setup(cpu: u32) -> i32 {
    let to = per_cpu_ptr(&mut tegra_to, cpu);
    writel_relaxed(0, timer_of_base(to).add(TIMER_PTV));
    writel_relaxed(TIMER_PCR_INTR_CLR, timer_of_base(to).add(TIMER_PCR));
    irq_force_affinity((*to).clkevt.irq, cpumask_of(cpu));
    enable_irq((*to).clkevt.irq);
    clockevents_config_and_register(&mut (*to).clkevt, timer_of_rate(to), 1, 0x1fffffff + 1);
    0
}

unsafe fn tegra_timer_stop(cpu: u32) -> i32 {
    let to = per_cpu_ptr(&mut tegra_to, cpu);
    disable_irq_nosync((*to).clkevt.irq);
    0
}

unsafe fn tegra_read_sched_clock() -> u64 {
    readl_relaxed(timer_reg_base.add(TIMERUS_CNTR_1US)) as u64
}

#[cfg(CONFIG_ARM)]
unsafe fn tegra_delay_timer_read_counter_long() -> usize {
    readl_relaxed(timer_reg_base.add(TIMERUS_CNTR_1US)) as usize
}

#[cfg(CONFIG_ARM)]
static mut tegra_delay_timer: delay_timer = delay_timer {
    read_current_timer: Some(tegra_delay_timer_read_counter_long),
    freq: TIMER_1MHZ,
};

static mut suspend_rtc_to: timer_of = timer_of {
    flags: TIMER_OF_BASE | TIMER_OF_CLOCK,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn tegra_rtc_read_ms(_cs: *mut clocksource) -> u64 {
    let reg_base = timer_of_base(&mut suspend_rtc_to);
    let ms = readl_relaxed(reg_base.add(RTC_MILLISECONDS));
    let s = readl_relaxed(reg_base.add(RTC_SHADOW_SECONDS));
    s as u64 * MSEC_PER_SEC as u64 + ms as u64
}

static mut suspend_rtc_clocksource: clocksource = clocksource {
    name: "tegra_suspend_timer\0".as_ptr() as *const i8,
    rating: 200,
    read: Some(tegra_rtc_read_ms),
    mask: CLOCKSOURCE_MASK(32),
    flags: CLOCK_SOURCE_IS_CONTINUOUS | CLOCK_SOURCE_SUSPEND_NONSTOP,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn tegra_base_for_cpu(cpu: i32, tegra20: bool) -> u32 {
    if tegra20 {
        match cpu { 0 => TIMER1_BASE as u32, 1 => TIMER2_BASE as u32,
                    2 => TIMER3_BASE as u32, _ => TIMER4_BASE as u32 }
    } else { TIMER10_BASE as u32 + cpu as u32 * 8 }
}

unsafe fn tegra_irq_idx_for_cpu(cpu: i32, tegra20: bool) -> i32 {
    if tegra20 { TIMER1_IRQ_IDX + cpu } else { TIMER10_IRQ_IDX + cpu }
}

unsafe fn tegra_rate_for_timer(to: *mut timer_of, tegra20: bool) -> usize {
    if tegra20 { TIMER_1MHZ as usize } else { timer_of_rate(to) }
}

// The remaining init routines retain the kernel's external APIs and cleanup labels.
// Build-time kernel declarations, registration macros, and dependency types are supplied externally.
unsafe fn tegra_init_timer(np: *mut device_node, tegra20: bool, rating: i32) -> i32 {
    let to = this_cpu_ptr(&mut tegra_to);
    let mut ret = timer_of_init(np, to);
    if ret != 0 { timer_of_cleanup(to); return ret; }
    timer_reg_base = timer_of_base(to);
    usec_config = match timer_of_rate(to) {
        12000000 => 0x000b, 12800000 => 0x043f, 13000000 => 0x000c,
        16800000 => 0x0453, 19200000 => 0x045f, 26000000 => 0x0019,
        38400000 => 0x04bf, 48000000 => 0x002f, _ => { timer_of_cleanup(to); return -EINVAL; }
    };
    writel_relaxed(usec_config, timer_reg_base.add(TIMERUS_USEC_CFG));
    for_each_possible_cpu(|cpu: i32| {
        let cpu_to = per_cpu_ptr(&mut tegra_to, cpu as u32);
        let flags = IRQF_TIMER | IRQF_NOBALANCING;
        let rate = tegra_rate_for_timer(to, tegra20);
        let base = tegra_base_for_cpu(cpu, tegra20);
        let idx = tegra_irq_idx_for_cpu(cpu, tegra20);
        let irq = irq_of_parse_and_map(np, idx);
        if irq == 0 {
            pr_err("failed to map irq for cpu%d\n", cpu);
            ret = -EINVAL;
            return;
        }
        (*cpu_to).clkevt.irq = irq;
        (*cpu_to).clkevt.rating = rating;
        (*cpu_to).clkevt.cpumask = cpumask_of(cpu as u32);
        (*cpu_to).of_base.base = timer_reg_base.add(base as usize);
        (*cpu_to).of_clk.period = rate / HZ;
        (*cpu_to).of_clk.rate = rate;
        irq_set_status_flags((*cpu_to).clkevt.irq, IRQ_NOAUTOEN);
        ret = request_irq((*cpu_to).clkevt.irq, Some(tegra_timer_isr), flags,
                          (*cpu_to).clkevt.name, &mut (*cpu_to).clkevt as *mut _ as *mut core::ffi::c_void);
        if ret != 0 {
            pr_err("failed to set up irq for cpu%d: %d\n", cpu, ret);
            irq_dispose_mapping((*cpu_to).clkevt.irq);
            (*cpu_to).clkevt.irq = 0;
        }
    });
    sched_clock_register(tegra_read_sched_clock, 32, TIMER_1MHZ);
    ret = clocksource_mmio_init(timer_reg_base.add(TIMERUS_CNTR_1US), "timer_us\0".as_ptr() as *const i8,
                                TIMER_1MHZ, 300, 32, clocksource_mmio_readl_up);
    #[cfg(CONFIG_ARM)] register_current_timer_delay(&mut tegra_delay_timer);
    ret = cpuhp_setup_state(CPUHP_AP_TEGRA_TIMER_STARTING, "AP_TEGRA_TIMER_STARTING\0".as_ptr() as *const i8,
                            tegra_timer_setup, tegra_timer_stop);
    ret
}

unsafe fn tegra210_init_timer(np: *mut device_node) -> i32 { tegra_init_timer(np, false, 460) }
unsafe fn tegra20_init_timer(np: *mut device_node) -> i32 {
    let rating = if of_machine_is_compatible("nvidia,tegra20\0".as_ptr() as *const i8) ||
                    of_machine_is_compatible("nvidia,tegra30\0".as_ptr() as *const i8) { 460 } else { 330 };
    tegra_init_timer(np, true, rating)
}
unsafe fn tegra20_init_rtc(np: *mut device_node) -> i32 {
    let ret = timer_of_init(np, &mut suspend_rtc_to);
    if ret != 0 { ret } else { clocksource_register_hz(&mut suspend_rtc_clocksource, 1000) }
}

// TIMER_OF_DECLARE(tegra210_timer, "nvidia,tegra210-timer", tegra210_init_timer);
// TIMER_OF_DECLARE(tegra20_timer, "nvidia,tegra20-timer", tegra20_init_timer);
// TIMER_OF_DECLARE(tegra20_rtc, "nvidia,tegra20-rtc", tegra20_init_rtc);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
