// SPDX-License-Identifier: GPL-2.0
/*
 * Common time service routines for LoongArch machines.
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Kernel and architecture dependencies are supplied by other translation units.

pub static mut cpu_clock_freq: u64 = 0;
pub static mut const_clock_freq: u64 = 0;

static mut state_lock: raw_spinlock_t = DEFINE_RAW_SPINLOCK();
static mut constant_clockevent_device: PerCpu<clock_event_device> = DEFINE_PER_CPU();

unsafe extern "C" fn constant_event_handler(_dev: *mut clock_event_device) {}

unsafe extern "C" fn constant_timer_interrupt(_irq: i32, _data: *mut core::ffi::c_void) -> irqreturn_t {
    let cpu: i32 = smp_processor_id();
    let cd: *mut clock_event_device;

    /* Clear Timer Interrupt */
    write_csr_tintclear(CSR_TINTCLR_TI);
    cd = &mut per_cpu(constant_clockevent_device, cpu);
    ((*cd).event_handler.unwrap())(cd);

    IRQ_HANDLED
}

unsafe extern "C" fn constant_set_state_oneshot(_evt: *mut clock_event_device) -> i32 {
    let mut timer_config: usize;

    raw_spin_lock(&mut state_lock);

    timer_config = csr_read(LOONGARCH_CSR_TCFG);
    timer_config |= CSR_TCFG_EN;
    timer_config &= !CSR_TCFG_PERIOD;
    csr_write(timer_config, LOONGARCH_CSR_TCFG);

    raw_spin_unlock(&mut state_lock);

    0
}

unsafe extern "C" fn constant_set_state_periodic(_evt: *mut clock_event_device) -> i32 {
    let mut timer_config: usize;
    let mut period: u64 = const_clock_freq;

    raw_spin_lock(&mut state_lock);

    period /= HZ as u64;
    timer_config = (period as usize) & CSR_TCFG_VAL;
    timer_config |= CSR_TCFG_PERIOD | CSR_TCFG_EN;
    csr_write(timer_config, LOONGARCH_CSR_TCFG);

    raw_spin_unlock(&mut state_lock);

    0
}

unsafe extern "C" fn constant_set_state_shutdown(_evt: *mut clock_event_device) -> i32 {
    let mut timer_config: usize;

    raw_spin_lock(&mut state_lock);

    timer_config = csr_read(LOONGARCH_CSR_TCFG);
    timer_config &= !CSR_TCFG_EN;
    csr_write(timer_config, LOONGARCH_CSR_TCFG);

    raw_spin_unlock(&mut state_lock);

    0
}

unsafe extern "C" fn constant_timer_next_event(mut delta: usize, _evt: *mut clock_event_device) -> i32 {
    let timer_config: usize;

    delta &= CSR_TCFG_VAL;
    timer_config = delta | CSR_TCFG_EN;
    csr_write(timer_config, LOONGARCH_CSR_TCFG);

    0
}

unsafe extern "C" fn arch_timer_starting(_cpu: u32) -> i32 {
    set_csr_ecfg(ECFGF_TIMER);
    0
}

unsafe extern "C" fn arch_timer_dying(_cpu: u32) -> i32 {
    /* Clear Timer Interrupt */
    write_csr_tintclear(CSR_TINTCLR_TI);
    0
}

unsafe fn get_loops_per_jiffy() -> usize {
    let mut lpj: u64 = const_clock_freq;
    lpj /= HZ as u64;
    lpj as usize
}

static mut init_offset: i64 = 0;

#[no_mangle]
pub unsafe extern "C" fn save_counter() {
    init_offset = get_cycles() as i64;
}

#[no_mangle]
pub unsafe extern "C" fn sync_counter() {
    /* Ensure counter begin at 0 */
    csr_write(init_offset as usize, LOONGARCH_CSR_CNTC);
}

#[no_mangle]
pub unsafe extern "C" fn constant_clockevent_init() -> i32 {
    let cpu: u32 = smp_processor_id() as u32;
    #[cfg(feature = "CONFIG_PREEMPT_RT")]
    let min_delta: usize = 100;
    #[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
    let min_delta: usize = 1000;
    let max_delta: usize = GENMASK_ULL(boot_cpu_data.timerbits, 0);
    let cd: *mut clock_event_device;
    static mut irq: i32 = 0;
    static mut timer_irq_installed: i32 = 0;

    if timer_irq_installed == 0 {
        irq = get_percpu_irq(INT_TI);
        if irq < 0 {
            pr_err!("Failed to map irq %d (timer)\n", irq);
        }
    }

    cd = &mut per_cpu(constant_clockevent_device, cpu as i32);
    (*cd).name = c"Constant".as_ptr();
    (*cd).features = CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_PERCPU;
    (*cd).irq = irq;
    (*cd).rating = 320;
    (*cd).cpumask = cpumask_of(cpu);
    (*cd).set_state_oneshot = Some(constant_set_state_oneshot);
    (*cd).set_state_oneshot_stopped = Some(constant_set_state_shutdown);
    (*cd).set_state_periodic = Some(constant_set_state_periodic);
    (*cd).set_state_shutdown = Some(constant_set_state_shutdown);
    (*cd).set_next_event = Some(constant_timer_next_event);
    (*cd).event_handler = Some(constant_event_handler);

    clockevents_config_and_register(cd, const_clock_freq as usize, min_delta, max_delta);

    if timer_irq_installed != 0 { return 0; }
    timer_irq_installed = 1;
    sync_counter();

    if request_irq(irq, Some(constant_timer_interrupt), IRQF_PERCPU | IRQF_TIMER, c"timer".as_ptr(), core::ptr::null_mut()) != 0 {
        pr_err!("Failed to request irq %d (timer)\n", irq);
    }
    lpj_fine = get_loops_per_jiffy();
    pr_info!("Constant clock event device register\n");
    cpuhp_setup_state(CPUHP_AP_LOONGARCH_ARCH_TIMER_STARTING, c"clockevents/loongarch/timer:starting".as_ptr(), Some(arch_timer_starting), Some(arch_timer_dying));
    0
}

unsafe extern "C" fn read_const_counter(_clk: *mut clocksource) -> u64 { get_cycles64() }
unsafe extern "C" fn sched_clock_read() -> u64 { get_cycles64() }

static mut clocksource_const: clocksource = clocksource {
    name: c"Constant".as_ptr(), rating: 400, read: Some(read_const_counter),
    mask: CLOCKSOURCE_MASK(64), flags: CLOCK_SOURCE_IS_CONTINUOUS,
    vdso_clock_mode: VDSO_CLOCKMODE_CPU,
};

#[no_mangle]
pub unsafe extern "C" fn constant_clocksource_init() -> i32 {
    let freq: usize = const_clock_freq as usize;
    let res = clocksource_register_hz(&mut clocksource_const, freq);
    sched_clock_register(Some(sched_clock_read), 64, freq);
    pr_info!("Constant clock source device register\n");
    res
}

#[no_mangle]
pub unsafe extern "C" fn time_init() {
    if !cpu_has_cpucfg { const_clock_freq = cpu_clock_freq; }
    else { const_clock_freq = calc_const_freq(); }
    init_offset = -(get_cycles() as i64 - csr_read(LOONGARCH_CSR_CNTC) as i64);
    constant_clockevent_init();
    constant_clocksource_init();
    pv_time_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
