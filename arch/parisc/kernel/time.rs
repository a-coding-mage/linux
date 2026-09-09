// SPDX-License-Identifier: GPL-2.0
/*
 * Common time service routines for parisc machines.
 * based on arch/loongarch/kernel/time.c
 *
 * Copyright (C) 2024 Helge Deller <deller@gmx.de>
 */

// C includes translated as dependencies supplied by the surrounding kernel.

static mut cr16_clock_freq: u64 = 0;
static mut clocktick: usize = 0;

static mut time_keeper_id: i32 = 0; /* CPU used for timekeeping */

// DEFINE_PER_CPU(struct clock_event_device, parisc_clockevent_device);
static mut parisc_clockevent_device: clock_event_device = unsafe { core::mem::zeroed() };

unsafe extern "C" {
    fn mfctl(reg: i32) -> usize;
    fn mtctl(value: usize, reg: i32);
    fn smp_processor_id() -> u32;
    fn per_cpu_clockevent_device(cpu: u32) -> *mut clock_event_device;
    fn clockevent_state_periodic(dev: *mut clock_event_device) -> bool;
    fn clockevent_state_oneshot(dev: *mut clock_event_device) -> bool;
    fn instruction_pointer(regs: *mut pt_regs) -> usize;
    fn in_lock_functions(pc: usize) -> bool;
    fn pdc_pat_pd_get_platform_counter(
        pclock: *mut *mut u64,
        freq: *mut usize,
        unique: *mut usize,
    ) -> i32;
    fn pdc_tod_read(data: *mut pdc_tod) -> i32;
    fn pdc_tod_set(secs: i64, usec: u32) -> i32;
    fn rtc_time64_to_tm(secs: u64, tm: *mut rtc_time);
    fn rtc_tm_to_time64(tm: *mut rtc_time) -> i64;
    fn platform_device_register_data(
        parent: *mut device,
        name: *const u8,
        id: i32,
        data: *const rtc_class_ops,
        size: usize,
    ) -> *mut platform_device;
    fn ptr_err_or_zero(ptr: *mut platform_device) -> i32;
    fn printk(level: usize, msg: *const u8);
    fn get_cycles() -> u64;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, freq: u64);
    fn clockevents_config_and_register(
        dev: *mut clock_event_device,
        freq: u64,
        min_delta: usize,
        max_delta: usize,
    );
    fn clocksource_register_hz(cs: *mut clocksource, freq: u64) -> i32;
}

unsafe extern "C" fn parisc_event_handler(_dev: *mut clock_event_device) {}

unsafe extern "C" fn parisc_timer_next_event(
    delta: usize,
    _evt: *mut clock_event_device,
) -> i32 {
    let new_cr16 = mfctl(16).wrapping_add(delta);
    mtctl(new_cr16, 16);
    0
}

#[repr(C)]
pub struct irqreturn_t(pub i32);

pub unsafe extern "C" fn timer_interrupt(_irq: i32, _data: *mut core::ffi::c_void) -> irqreturn_t {
    let cpu = smp_processor_id();
    let cd = per_cpu_clockevent_device(cpu);

    if clockevent_state_periodic(cd) {
        parisc_timer_next_event(clocktick, cd);
    }

    if clockevent_state_periodic(cd) || clockevent_state_oneshot(cd) {
        ((*cd).event_handler)(cd);
    }

    irqreturn_t(1) // IRQ_HANDLED
}

unsafe extern "C" fn parisc_set_state_oneshot(evt: *mut clock_event_device) -> i32 {
    parisc_timer_next_event(clocktick, evt);
    0
}

unsafe extern "C" fn parisc_set_state_periodic(evt: *mut clock_event_device) -> i32 {
    parisc_timer_next_event(clocktick, evt);
    0
}

unsafe extern "C" fn parisc_set_state_shutdown(_evt: *mut clock_event_device) -> i32 { 0 }

pub unsafe extern "C" fn parisc_clockevent_init() {
    let cpu = smp_processor_id();
    let min_delta: usize = 0x600; // XXX
    let max_delta: usize = 1usize << (usize::BITS - 1);
    let cd = per_cpu_clockevent_device(cpu);

    (*cd).name = b"cr16_clockevent\0".as_ptr();
    (*cd).features = CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_PERCPU;
    (*cd).irq = TIMER_IRQ;
    (*cd).rating = 320;
    (*cd).cpumask = cpumask_of(cpu);
    (*cd).set_state_oneshot = Some(parisc_set_state_oneshot);
    (*cd).set_state_oneshot_stopped = Some(parisc_set_state_shutdown);
    (*cd).set_state_periodic = Some(parisc_set_state_periodic);
    (*cd).set_state_shutdown = Some(parisc_set_state_shutdown);
    (*cd).set_next_event = Some(parisc_timer_next_event);
    (*cd).event_handler = parisc_event_handler;

    clockevents_config_and_register(cd, cr16_clock_freq, min_delta, max_delta);
}

unsafe extern "C" fn parisc_find_64bit_counter() {
    // CONFIG_64BIT conditional retained from the C source.
    #[cfg(target_pointer_width = "64")]
    {
        let mut pclock: *mut u64 = core::ptr::null_mut();
        let mut freq: usize = 0;
        let mut unique: usize = 0;
        let ret = pdc_pat_pd_get_platform_counter(&mut pclock, &mut freq, &mut unique);
        if ret == PDC_OK {
            pr_info_64bit_counter_found(pclock, freq, unique);
        } else {
            pr_info_64bit_counter_not_found();
        }
    }
}

pub unsafe extern "C" fn profile_pc(regs: *mut pt_regs) -> usize {
    let mut pc = instruction_pointer(regs);
    if (*regs).gr[0] & PSW_N != 0 { pc = pc.wrapping_sub(4); }
    // CONFIG_SMP conditional retained from the C source.
    #[cfg(feature = "CONFIG_SMP")]
    if in_lock_functions(pc) { pc = (*regs).gr[2]; }
    pc
}

// EXPORT_SYMBOL(profile_pc);

#[cfg(feature = "CONFIG_RTC_DRV_GENERIC")]
unsafe extern "C" fn rtc_generic_get_time(_dev: *mut device, tm: *mut rtc_time) -> i32 {
    let mut tod_data: pdc_tod = core::mem::zeroed();
    core::ptr::write_bytes(tm, 0, 1);
    if pdc_tod_read(&mut tod_data) < 0 { return -95; } // -EOPNOTSUPP
    rtc_time64_to_tm(tod_data.tod_sec, tm);
    0
}

#[cfg(feature = "CONFIG_RTC_DRV_GENERIC")]
unsafe extern "C" fn rtc_generic_set_time(_dev: *mut device, tm: *mut rtc_time) -> i32 {
    let secs = rtc_tm_to_time64(tm);
    let ret = pdc_tod_set(secs, 0);
    if ret != 0 {
        pr_warn_pdc_tod_set(secs, ret);
        if ret == PDC_INVALID_ARG { return -22; } // -EINVAL
        return -95; // -EOPNOTSUPP
    }
    0
}

#[cfg(feature = "CONFIG_RTC_DRV_GENERIC")]
static rtc_generic_ops: rtc_class_ops = rtc_class_ops {
    read_time: Some(rtc_generic_get_time),
    set_time: Some(rtc_generic_set_time),
};

#[cfg(feature = "CONFIG_RTC_DRV_GENERIC")]
unsafe extern "C" fn rtc_init() -> i32 {
    let pdev = platform_device_register_data(
        core::ptr::null_mut(), b"rtc-generic\0".as_ptr(), -1,
        &rtc_generic_ops, core::mem::size_of::<rtc_class_ops>(),
    );
    ptr_err_or_zero(pdev)
}

// device_initcall(rtc_init);

pub unsafe extern "C" fn read_persistent_clock64(ts: *mut timespec64) {
    static mut tod_data: pdc_tod = unsafe { core::mem::zeroed() };
    if pdc_tod_read(&raw mut tod_data) == 0 {
        (*ts).tv_sec = tod_data.tod_sec;
        (*ts).tv_nsec = tod_data.tod_usec * 1000;
    } else {
        printk(KERN_ERR, b"Error reading tod clock\n\0".as_ptr());
        (*ts).tv_sec = 0;
        (*ts).tv_nsec = 0;
    }
}

unsafe extern "C" fn read_cr16_sched_clock() -> u64 { get_cycles() }
unsafe extern "C" fn read_cr16(_cs: *mut clocksource) -> u64 { get_cycles() }

static mut clocksource_cr16: clocksource = clocksource {
    name: b"cr16\0".as_ptr(),
    rating: 300,
    read: Some(read_cr16),
    mask: CLOCKSOURCE_MASK(usize::BITS),
    flags: CLOCK_SOURCE_IS_CONTINUOUS | CLOCK_SOURCE_VALID_FOR_HRES,
};

/* timer interrupt and sched_clock() initialization */
pub unsafe extern "C" fn time_init() {
    cr16_clock_freq = 100 * PAGE0.mem_10msec; // Hz
    clocktick = cr16_clock_freq as usize / HZ;
    sched_clock_register(read_cr16_sched_clock, usize::BITS, cr16_clock_freq);
    parisc_clockevent_init();
    parisc_find_64bit_counter();
    clocksource_register_hz(&raw mut clocksource_cr16, cr16_clock_freq);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
