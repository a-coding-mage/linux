// SPDX-License-Identifier: GPL-2.0
/* linux/arch/sparc/kernel/time.c
 *
 * Copyright (C) 1995 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1996 Thomas K. Dyas (tdyas@eden.rutgers.edu)
 *
 * Chris Davis (cdavis@cois.on.ca) 03/27/1998
 * Added support for the intersil on the sun4/4200
 *
 * Gleb Raiko (rajko@mech.math.msu.su) 08/18/1998
 * Support for MicroSPARC-IIep, PCI CPU.
 *
 * This file handles the Sparc specific time handling details.
 *
 * 1997-09-10 Updated NTP code according to technical memorandum Jan '96
 *            "A Kernel Model for Precision Timekeeping" by Dave Mills
 */

// Linux and architecture headers are supplied by other translated units.

static mut TIMER_CS_LOCK: SeqLock = DEFINE_SEQLOCK!();
static mut TIMER_CS_INTERNAL_COUNTER: u64 = 0;
static mut TIMER_CS_ENABLED: i8 = 0;

static mut TIMER_CE: ClockEventDevice = ClockEventDevice::default();
static mut TIMER_CE_ENABLED: i8 = 0;

#[cfg(CONFIG_SMP)]
static mut SPARC32_CLOCKEVENT: PerCpu<ClockEventDevice> = PerCpu::new();

static mut RTC_LOCK: SpinLock = DEFINE_SPINLOCK!();

pub unsafe fn profile_pc(regs: *mut PtRegs) -> c_ulong {
    unsafe extern "C" {
        static __copy_user_begin: c_char;
        static __copy_user_end: c_char;
        static __bzero_begin: c_char;
        static __bzero_end: c_char;
    }

    let mut pc = (*regs).pc;
    if in_lock_functions(pc)
        || (pc >= (&__copy_user_begin as *const _ as c_ulong)
            && pc < (&__copy_user_end as *const _ as c_ulong))
        || (pc >= (&__bzero_begin as *const _ as c_ulong)
            && pc < (&__bzero_end as *const _ as c_ulong))
    {
        pc = (*regs).u_regs[UREG_RETPC];
    }
    pc
}

static mut MASTER_L10_COUNTER: *mut u32 = core::ptr::null_mut();

pub unsafe extern "C" fn timer_interrupt(_dummy: c_int, _dev_id: *mut c_void) -> IrqReturn {
    if TIMER_CS_ENABLED != 0 {
        write_seqlock(&raw mut TIMER_CS_LOCK);
        TIMER_CS_INTERNAL_COUNTER = TIMER_CS_INTERNAL_COUNTER.wrapping_add(1);
        (*SPARC_CONFIG).clear_clock_irq();
        write_sequnlock(&raw mut TIMER_CS_LOCK);
    } else {
        (*SPARC_CONFIG).clear_clock_irq();
    }

    if TIMER_CE_ENABLED != 0 {
        ((*TIMER_CE).event_handler)(&raw mut TIMER_CE);
    }
    IRQ_HANDLED
}

unsafe extern "C" fn timer_ce_shutdown(_evt: *mut ClockEventDevice) -> c_int {
    TIMER_CE_ENABLED = 0;
    smp_mb();
    0
}

unsafe extern "C" fn timer_ce_set_periodic(_evt: *mut ClockEventDevice) -> c_int {
    TIMER_CE_ENABLED = 1;
    smp_mb();
    0
}

unsafe fn setup_timer_ce() {
    let ce = &raw mut TIMER_CE;
    BUG_ON!(smp_processor_id() != BOOT_CPU_ID);
    (*ce).name = c"timer_ce".as_ptr();
    (*ce).rating = 100;
    (*ce).features = CLOCK_EVT_FEAT_PERIODIC;
    (*ce).set_state_shutdown = Some(timer_ce_shutdown);
    (*ce).set_state_periodic = Some(timer_ce_set_periodic);
    (*ce).tick_resume = Some(timer_ce_set_periodic);
    (*ce).cpumask = CPU_POSSIBLE_MASK;
    (*ce).shift = 32;
    (*ce).mult = div_sc((*SPARC_CONFIG).clock_rate, NSEC_PER_SEC, (*ce).shift);
    clockevents_register_device(ce);
}

unsafe fn sbus_cycles_offset() -> u32 {
    let val = sbus_readl(MASTER_L10_COUNTER);
    let mut offset = (val >> TIMER_VALUE_SHIFT) & TIMER_VALUE_MASK;
    if val & TIMER_LIMIT_BIT != 0 {
        offset = offset.wrapping_add((*SPARC_CONFIG).cs_period);
    }
    offset
}

unsafe extern "C" fn timer_cs_read(_cs: *mut ClockSource) -> u64 {
    let (cycles, offset);
    loop {
        let seq = read_seqbegin(&raw mut TIMER_CS_LOCK);
        cycles = TIMER_CS_INTERNAL_COUNTER;
        offset = ((*SPARC_CONFIG).get_cycles_offset)();
        if !read_seqretry(&raw mut TIMER_CS_LOCK, seq) {
            break;
        }
    }
    cycles.wrapping_mul((*SPARC_CONFIG).cs_period as u64)
        .wrapping_add(offset as u64)
}

static mut TIMER_CS: ClockSource = ClockSource {
    name: c"timer_cs".as_ptr(),
    rating: 100,
    read: Some(timer_cs_read),
    mask: CLOCKSOURCE_MASK!(64),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

unsafe fn setup_timer_cs() -> c_int {
    TIMER_CS_ENABLED = 1;
    clocksource_register_hz(&raw mut TIMER_CS, (*SPARC_CONFIG).clock_rate)
}

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn percpu_ce_shutdown(evt: *mut ClockEventDevice) -> c_int {
    let cpu = cpumask_first((*evt).cpumask);
    ((*SPARC_CONFIG).load_profile_irq)(cpu, 0);
    0
}

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn percpu_ce_set_periodic(evt: *mut ClockEventDevice) -> c_int {
    let cpu = cpumask_first((*evt).cpumask);
    ((*SPARC_CONFIG).load_profile_irq)(cpu, SBUS_CLOCK_RATE / HZ);
    0
}

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn percpu_ce_set_next_event(delta: c_ulong, evt: *mut ClockEventDevice) -> c_int {
    let cpu = cpumask_first((*evt).cpumask);
    ((*SPARC_CONFIG).load_profile_irq)(cpu, delta as c_uint);
    0
}

#[cfg(CONFIG_SMP)]
pub unsafe fn register_percpu_ce(cpu: c_int) {
    let ce = per_cpu_ptr(&raw mut SPARC32_CLOCKEVENT, cpu);
    let mut features = CLOCK_EVT_FEAT_PERIODIC;
    if (*SPARC_CONFIG).features & FEAT_L14_ONESHOT != 0 {
        features |= CLOCK_EVT_FEAT_ONESHOT;
    }
    (*ce).name = c"percpu_ce".as_ptr();
    (*ce).rating = 200;
    (*ce).features = features;
    (*ce).set_state_shutdown = Some(percpu_ce_shutdown);
    (*ce).set_state_periodic = Some(percpu_ce_set_periodic);
    (*ce).set_state_oneshot = Some(percpu_ce_shutdown);
    (*ce).set_next_event = Some(percpu_ce_set_next_event);
    (*ce).cpumask = cpumask_of(cpu);
    (*ce).shift = 32;
    (*ce).mult = div_sc((*SPARC_CONFIG).clock_rate, NSEC_PER_SEC, (*ce).shift);
    (*ce).max_delta_ns = clockevent_delta2ns((*SPARC_CONFIG).clock_rate, ce);
    (*ce).max_delta_ticks = (*SPARC_CONFIG).clock_rate as c_ulong;
    (*ce).min_delta_ns = clockevent_delta2ns(100, ce);
    (*ce).min_delta_ticks = 100;
    clockevents_register_device(ce);
}

unsafe fn mostek_read_byte(dev: *mut Device, ofs: u32) -> u8 {
    let pdev = to_platform_device(dev);
    let pdata = (*pdev).dev.platform_data as *mut M48t59PlatData;
    readb((*pdata).ioaddr.add(ofs as usize))
}

unsafe fn mostek_write_byte(dev: *mut Device, ofs: u32, val: u8) {
    let pdev = to_platform_device(dev);
    let pdata = (*pdev).dev.platform_data as *mut M48t59PlatData;
    writeb(val, (*pdata).ioaddr.add(ofs as usize));
}

static mut M48T59_DATA: M48t59PlatData = M48t59PlatData {
    read_byte: Some(mostek_read_byte),
    write_byte: Some(mostek_write_byte),
    yy_offset: 68,
    ..M48t59PlatData::zeroed()
};

static mut M48T59_RTC: PlatformDevice = PlatformDevice {
    name: c"rtc-m48t59".as_ptr(),
    id: 0,
    num_resources: 1,
    dev: Device { platform_data: &raw mut M48T59_DATA as *mut c_void, ..Device::zeroed() },
    ..PlatformDevice::zeroed()
};

unsafe fn clock_probe(op: *mut PlatformDevice) -> c_int {
    let dp = (*op).dev.of_node;
    let model = of_get_property(dp, c"model".as_ptr(), core::ptr::null_mut());
    if model.is_null() { return -ENODEV; }
    if !of_property_present(dp, c"address".as_ptr()) { return -ENODEV; }
    M48T59_RTC.resource = &mut (*op).resource[0];
    if strcmp(model, c"mk48t02".as_ptr()) == 0 {
        M48T59_DATA.ioaddr = of_ioremap(&(*op).resource[0], 0, 2048, c"rtc-m48t59".as_ptr());
        M48T59_DATA.r#type = M48T59RTC_TYPE_M48T02;
    } else if strcmp(model, c"mk48t08".as_ptr()) == 0 {
        M48T59_DATA.ioaddr = of_ioremap(&(*op).resource[0], 0, 8192, c"rtc-m48t59".as_ptr());
        M48T59_DATA.r#type = M48T59RTC_TYPE_M48T08;
    } else { return -ENODEV; }
    if platform_device_register(&raw mut M48T59_RTC) < 0 { printk!(KERN_ERR "Registering RTC device failed\n"); }
    0
}

static CLOCK_MATCH: [OfDeviceId; 2] = [OfDeviceId { name: c"eeprom".as_ptr(), ..OfDeviceId::zeroed() }, OfDeviceId::zeroed()];
static mut CLOCK_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(clock_probe),
    driver: Driver { name: c"rtc".as_ptr(), of_match_table: CLOCK_MATCH.as_ptr(), ..Driver::zeroed() },
    ..PlatformDriver::zeroed()
};

unsafe fn clock_init() -> c_int { platform_driver_register(&raw mut CLOCK_DRIVER) }
fs_initcall!(clock_init);

unsafe fn sparc32_late_time_init() {
    if (*SPARC_CONFIG).features & FEAT_L10_CLOCKEVENT != 0 { setup_timer_ce(); }
    if (*SPARC_CONFIG).features & FEAT_L10_CLOCKSOURCE != 0 { setup_timer_cs(); }
    #[cfg(CONFIG_SMP)]
    register_percpu_ce(smp_processor_id());
}

unsafe fn sbus_time_init() {
    (*SPARC_CONFIG).get_cycles_offset = sbus_cycles_offset;
    ((*SPARC_CONFIG).init_timers)();
}

pub unsafe fn time_init() {
    (*SPARC_CONFIG).features = 0;
    LATE_TIME_INIT = Some(sparc32_late_time_init);
    if pcic_present() { pci_time_init(); } else { sbus_time_init(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
