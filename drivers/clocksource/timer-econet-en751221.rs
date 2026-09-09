// SPDX-License-Identifier: GPL-2.0
/*
 * Timer present on EcoNet EN75xx MIPS based SoCs.
 *
 * Copyright (C) 2025 by Caleb James DeLisle <cjd@cjdns.fr>
 */

// Linux dependencies supplied by the surrounding kernel translation.

const ECONET_BITS: u32 = 32;
const ECONET_MIN_DELTA: u32 = 0x0000_1000;
const ECONET_MAX_DELTA: u32 = (1u32 << (ECONET_BITS - 2)) - 1;
/* 34Kc hardware has 1 block and 1004Kc has 2. */
const ECONET_NUM_BLOCKS: usize = (NR_CPUS + 1) / 2;

#[repr(C)]
struct EconetTimer {
    membase: [*mut core::ffi::c_void; ECONET_NUM_BLOCKS],
    freq_hz: u32,
}

static mut ECONET_TIMER: EconetTimer = EconetTimer {
    membase: [core::ptr::null_mut(); ECONET_NUM_BLOCKS],
    freq_hz: 0,
};

static mut ECONET_TIMER_PCPU: [ClockEventDevice; NR_CPUS] = [ClockEventDevice::zeroed(); NR_CPUS];

/* Each memory block has 2 timers, the order of registers is:
 * CTL, CMR0, CNT0, CMR1, CNT1
 */
#[inline]
unsafe fn reg_ctl(timer_n: u32) -> *mut core::ffi::c_void {
    ECONET_TIMER.membase[(timer_n >> 1) as usize]
}

#[inline]
unsafe fn reg_compare(timer_n: u32) -> *mut core::ffi::c_void {
    (ECONET_TIMER.membase[(timer_n >> 1) as usize] as *mut u8)
        .add(((timer_n & 1) * 0x08 + 0x04) as usize) as *mut core::ffi::c_void
}

#[inline]
unsafe fn reg_count(timer_n: u32) -> *mut core::ffi::c_void {
    (ECONET_TIMER.membase[(timer_n >> 1) as usize] as *mut u8)
        .add(((timer_n & 1) * 0x08 + 0x08) as usize) as *mut core::ffi::c_void
}

#[inline]
fn ctl_bit_enabled(timer_n: u32) -> u32 { 1u32 << (timer_n & 1) }

#[inline]
fn ctl_bit_pending(timer_n: u32) -> u32 { 1u32 << ((timer_n & 1) + 16) }

unsafe fn cevt_is_pending(cpu_id: i32) -> bool {
    ioread32(reg_ctl(cpu_id as u32)) & ctl_bit_pending(cpu_id as u32) != 0
}

unsafe extern "C" fn cevt_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> IrqReturn {
    let dev = this_cpu_ptr(&raw mut ECONET_TIMER_PCPU);
    let cpu = cpumask_first((*dev).cpumask);

    /* Each VPE has its own events,
     * so this will only happen on spurious interrupt.
     */
    if !cevt_is_pending(cpu) { return IRQ_NONE; }

    iowrite32(ioread32(reg_count(cpu as u32)), reg_compare(cpu as u32));
    ((*dev).event_handler)(dev);
    IRQ_HANDLED
}

unsafe extern "C" fn cevt_set_next_event(delta: ulong, dev: *mut ClockEventDevice) -> i32 {
    let cpu = cpumask_first((*dev).cpumask) as u32;
    let next = ioread32(reg_count(cpu)).wrapping_add(delta as u32);
    iowrite32(next, reg_compare(cpu));

    if (next.wrapping_sub(ioread32(reg_count(cpu))) as i32) < (ECONET_MIN_DELTA / 2) as i32 { return -ETIME; }
    0
}

unsafe extern "C" fn cevt_init_cpu(cpu: u32) -> i32 {
    let cd = &mut ECONET_TIMER_PCPU[cpu as usize];
    pr_debug!("{}: Setting up clockevent for CPU {}\n", cd.name, cpu);

    let reg = ioread32(reg_ctl(cpu)) | ctl_bit_enabled(cpu);
    iowrite32(reg, reg_ctl(cpu));
    enable_percpu_irq(cd.irq, IRQ_TYPE_NONE);

    /* Do this last because it synchronously configures the timer */
    clockevents_config_and_register(cd, ECONET_TIMER.freq_hz, ECONET_MIN_DELTA, ECONET_MAX_DELTA);
    0
}

unsafe extern "C" fn sched_clock_read() -> u64 {
    /* Always read from clock zero no matter the CPU */
    ioread32(reg_count(0)) as u64
}

/* Init */

unsafe fn cevt_dev_init(cpu: u32) {
    iowrite32(0, reg_count(cpu));
    iowrite32(u32::MAX, reg_compare(cpu));
}

unsafe extern "C" fn cevt_init(np: *mut DeviceNode) -> i32 {
    let irq = irq_of_parse_and_map(np, 0);
    if irq <= 0 { pr_err!("%pOFn: irq_of_parse_and_map failed", np); return -EINVAL; }

    let ret = request_percpu_irq(irq, cevt_interrupt, (*np).name, &raw mut ECONET_TIMER_PCPU);
    if ret < 0 { pr_err!("%pOFn: IRQ {} setup failed ({})\n", np, irq, ret); irq_dispose_mapping(irq); return ret; }

    for i in for_each_possible_cpu() {
        let cd = &mut ECONET_TIMER_PCPU[i as usize];
        cd.rating = 310;
        cd.features = CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_C3STOP | CLOCK_EVT_FEAT_PERCPU;
        cd.set_next_event = Some(cevt_set_next_event);
        cd.irq = irq;
        cd.cpumask = cpumask_of(i);
        cd.name = (*np).name;
        cevt_dev_init(i);
    }
    cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "clockevents/econet/timer:starting", cevt_init_cpu, None);
    0
}

unsafe extern "C" fn timer_init(np: *mut DeviceNode) -> i32 {
    let num_blocks = (num_possible_cpus() + 1) / 2;
    let clk = of_clk_get(np, 0);
    if IS_ERR(clk) { pr_err!("%pOFn: Failed to get CPU clock from DT %ld\n", np, PTR_ERR(clk)); return PTR_ERR(clk); }
    ECONET_TIMER.freq_hz = clk_get_rate(clk);

    for i in 0..num_blocks {
        ECONET_TIMER.membase[i] = of_iomap(np, i);
        if ECONET_TIMER.membase[i].is_null() { pr_err!("%pOFn: failed to map register [{}]\n", np, i); return -ENXIO; }
    }

    /* For clocksource purposes always read clock zero, whatever the CPU */
    let ret = clocksource_mmio_init(reg_count(0), (*np).name, ECONET_TIMER.freq_hz, 301, ECONET_BITS, clocksource_mmio_readl_up);
    if ret != 0 { pr_err!("%pOFn: clocksource_mmio_init failed: {}", np, ret); return ret; }
    let ret = cevt_init(np);
    if ret < 0 { return ret; }
    sched_clock_register(sched_clock_read, ECONET_BITS, ECONET_TIMER.freq_hz);
    pr_info!("%pOFn: using {}.{:03} MHz high precision timer\n", np, ECONET_TIMER.freq_hz / 1_000_000, (ECONET_TIMER.freq_hz / 1_000) % 1_000);
    0
}

TIMER_OF_DECLARE!(econet_timer_hpt, "econet,en751221-timer", timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
