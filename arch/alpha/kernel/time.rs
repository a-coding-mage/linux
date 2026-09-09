// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/alpha/kernel/time.c
 *
 *  Copyright (C) 1991, 1992, 1995, 1999, 2000  Linus Torvalds
 *
 * This file contains the clocksource time handling.
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

#[no_mangle]
pub static mut rtc_lock: u8 = 0;

#[no_mangle]
pub static mut est_cycle_freq: ::core::ffi::c_ulong = 0;

#[cfg(feature = "CONFIG_IRQ_WORK")]
#[no_mangle]
pub static mut irq_work_pending: u8 = 0;

#[cfg(feature = "CONFIG_IRQ_WORK")]
#[no_mangle]
pub unsafe extern "C" fn arch_irq_work_raise() {
    irq_work_pending = 1;
}

#[inline]
unsafe fn rpcc() -> u32 {
    __builtin_alpha_rpcc()
}

/* The RTC as a clock_event_device primitive. */

static mut cpu_ce: ::core::mem::MaybeUninit<clock_event_device> =
    ::core::mem::MaybeUninit::uninit();

#[no_mangle]
pub unsafe extern "C" fn rtc_timer_interrupt(
    irq: ::core::ffi::c_int,
    dev: *mut ::core::ffi::c_void,
) -> irqreturn_t {
    let cpu = smp_processor_id();
    let ce = &mut *per_cpu_ptr(&mut cpu_ce as *mut _, cpu);

    /* Don't run the hook for UNUSED or SHUTDOWN. */
    if likely(clockevent_state_periodic(ce)) {
        ((*ce).event_handler)(ce);
    }

    #[cfg(feature = "CONFIG_IRQ_WORK")]
    if irq_work_pending != 0 {
        irq_work_pending = 0;
        irq_work_run();
    }

    IRQ_HANDLED
}

unsafe extern "C" fn rtc_ce_set_next_event(
    evt: ::core::ffi::c_ulong,
    ce: *mut clock_event_device,
) -> ::core::ffi::c_int {
    /* This hook is for oneshot mode, which we don't support. */
    -EINVAL
}

unsafe fn init_rtc_clockevent() {
    let cpu = smp_processor_id();
    let ce = &mut *per_cpu_ptr(&mut cpu_ce as *mut _, cpu);
    *ce = clock_event_device {
        name: b"rtc\0".as_ptr() as *const _,
        features: CLOCK_EVT_FEAT_PERIODIC,
        rating: 100,
        cpumask: cpumask_of(cpu),
        set_next_event: Some(rtc_ce_set_next_event),
        ..::core::mem::zeroed()
    };
    clockevents_config_and_register(ce, CONFIG_HZ, 0, 0);
}

/* The QEMU clock as a clocksource primitive. */

unsafe extern "C" fn qemu_cs_read(cs: *mut clocksource) -> u64 {
    qemu_get_vmtime()
}

static mut qemu_cs: clocksource = clocksource {
    name: b"qemu\0".as_ptr() as *const _,
    rating: 400,
    read: Some(qemu_cs_read),
    mask: CLOCKSOURCE_MASK(64),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    max_idle_ns: LONG_MAX,
    ..::core::mem::zeroed()
};

/* The QEMU alarm as a clock_event_device primitive. */

unsafe extern "C" fn qemu_ce_shutdown(ce: *mut clock_event_device) -> ::core::ffi::c_int {
    /* The mode member of CE is updated for us in generic code.
       Just make sure that the event is disabled. */
    qemu_set_alarm_abs(0);
    0
}

unsafe extern "C" fn qemu_ce_set_next_event(
    evt: ::core::ffi::c_ulong,
    ce: *mut clock_event_device,
) -> ::core::ffi::c_int {
    qemu_set_alarm_rel(evt);
    0
}

unsafe extern "C" fn qemu_timer_interrupt(
    irq: ::core::ffi::c_int,
    dev: *mut ::core::ffi::c_void,
) -> irqreturn_t {
    let cpu = smp_processor_id();
    let ce = &mut *per_cpu_ptr(&mut cpu_ce as *mut _, cpu);
    ((*ce).event_handler)(ce);
    IRQ_HANDLED
}

unsafe fn init_qemu_clockevent() {
    let cpu = smp_processor_id();
    let ce = &mut *per_cpu_ptr(&mut cpu_ce as *mut _, cpu);
    *ce = clock_event_device {
        name: b"qemu\0".as_ptr() as *const _,
        features: CLOCK_EVT_FEAT_ONESHOT,
        rating: 400,
        cpumask: cpumask_of(cpu),
        set_state_shutdown: Some(qemu_ce_shutdown),
        set_state_oneshot: Some(qemu_ce_shutdown),
        tick_resume: Some(qemu_ce_shutdown),
        set_next_event: Some(qemu_ce_set_next_event),
        ..::core::mem::zeroed()
    };
    clockevents_config_and_register(ce, NSEC_PER_SEC, 1000, LONG_MAX);
}

pub unsafe fn common_init_rtc() {
    let mut x: u8;
    let mut sel: u8 = 0;

    /* Reset periodic interrupt frequency. */
    #[cfg(any(CONFIG_HZ = "1024", CONFIG_HZ = "1200"))]
    {
        x = CMOS_READ(RTC_FREQ_SELECT) & 0x3f;
        if x != 0x26 && x != 0x25 && x != 0x19 && x != 0x06 {
            sel = RTC_REF_CLCK_32KHZ + 6;
        }
    }
    #[cfg(any(CONFIG_HZ = "256", CONFIG_HZ = "128", CONFIG_HZ = "64", CONFIG_HZ = "32"))]
    {
        sel = RTC_REF_CLCK_32KHZ + __builtin_ffs(32768 / CONFIG_HZ) as u8;
    }
    if sel != 0 {
        printk(KERN_INFO, CONFIG_HZ, sel);
        CMOS_WRITE(sel, RTC_FREQ_SELECT);
    }

    x = CMOS_READ(RTC_CONTROL);
    if x & RTC_PIE == 0 {
        printk_str("Turning on RTC interrupts.\n");
        x |= RTC_PIE;
        x &= !(RTC_AIE | RTC_UIE);
        CMOS_WRITE(x, RTC_CONTROL);
    }
    let _ = CMOS_READ(RTC_INTR_FLAGS);
    outb(0x36, 0x43);
    outb(0x00, 0x40);
    outb(0x00, 0x40);
    outb(0xb6, 0x43);
    outb(0x31, 0x42);
    outb(0x13, 0x42);
    init_rtc_irq(::core::ptr::null_mut());
}

#[cfg(not(feature = "CONFIG_ALPHA_WTINT"))]
unsafe extern "C" fn read_rpcc(cs: *mut clocksource) -> u64 { rpcc() as u64 }

#[cfg(not(feature = "CONFIG_ALPHA_WTINT"))]
static mut clocksource_rpcc: clocksource = clocksource {
    name: b"rpcc\0".as_ptr() as *const _,
    rating: 300,
    read: Some(read_rpcc),
    mask: CLOCKSOURCE_MASK(32),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    ..::core::mem::zeroed()
};

unsafe fn validate_cc_value(cc: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    #[repr(C)]
    struct Bounds { min: u32, max: u32 }
    let cpu_hz: [Bounds; 17] = [
        Bounds { min: 50_000_000, max: 200_000_000 }, Bounds { min: 100_000_000, max: 300_000_000 },
        Bounds { min: 100_000_000, max: 300_000_000 }, Bounds { min: 200_000_000, max: 300_000_000 },
        Bounds { min: 250_000_000, max: 433_000_000 }, Bounds { min: 333_000_000, max: 667_000_000 },
        Bounds { min: 400_000_000, max: 600_000_000 }, Bounds { min: 500_000_000, max: 600_000_000 },
        Bounds { min: 466_000_000, max: 600_000_000 }, Bounds { min: 600_000_000, max: 750_000_000 },
        Bounds { min: 750_000_000, max: 940_000_000 }, Bounds { min: 1_000_000_000, max: 1_333_333_333 },
        Bounds { min: 1_000_000_000, max: 1_700_000_000 }, Bounds { min: 1_000_000_000, max: 1_700_000_000 },
        Bounds { min: 800_000_000, max: 1_400_000_000 }, Bounds { min: 1_000_000_000, max: 2_000_000_000 },
        Bounds { min: 0, max: 0 },
    ];
    const DEVIATION: u32 = 10_000_000;
    let cpu = (hwrpb as *mut u8).add((*hwrpb).processor_offset as usize) as *mut percpu_struct;
    let index = ((*cpu).type_ & 0xffff_ffff) as usize;
    if index >= cpu_hz.len() || cpu_hz[index].max == 0 { return cc; }
    if cc < (cpu_hz[index].min - DEVIATION) as _ || cc > (cpu_hz[index].max + DEVIATION) as _ { 0 } else { cc }
}

const CALIBRATE_LATCH: u32 = 0xffff;
const TIMEOUT_COUNT: i32 = 0x100000;

unsafe fn calibrate_cc_with_pit() -> ::core::ffi::c_ulong {
    let mut count = 0;
    outb((inb(0x61) & !0x02) | 0x01, 0x61);
    outb(0xb0, 0x43);
    outb(CALIBRATE_LATCH & 0xff, 0x42);
    outb(CALIBRATE_LATCH >> 8, 0x42);
    let start = rpcc();
    loop { count += 1; if (inb(0x61) & 0x20) != 0 || count >= TIMEOUT_COUNT { break; } }
    let cc = rpcc().wrapping_sub(start);
    if count <= 1 || count == TIMEOUT_COUNT { return 0; }
    ((cc as i64 * PIT_TICK_RATE as i64) / (CALIBRATE_LATCH as i64 + 1)) as _
}

unsafe fn rpcc_after_update_in_progress() -> ::core::ffi::c_ulong {
    while CMOS_READ(RTC_FREQ_SELECT) & RTC_UIP == 0 {}
    while CMOS_READ(RTC_FREQ_SELECT) & RTC_UIP != 0 {}
    rpcc() as _
}

pub unsafe fn time_init() {
    let mut cc1: u32;
    let mut cc2: u32;
    let mut cycle_freq: ::core::ffi::c_ulong;
    let mut tolerance: ::core::ffi::c_ulong;
    let mut diff: i64;
    if alpha_using_qemu {
        clocksource_register_hz(&mut qemu_cs, NSEC_PER_SEC);
        init_qemu_clockevent();
        init_rtc_irq(Some(qemu_timer_interrupt));
        return;
    }
    if est_cycle_freq == 0 { est_cycle_freq = validate_cc_value(calibrate_cc_with_pit()); }
    cc1 = rpcc();
    if est_cycle_freq == 0 {
        cc1 = rpcc_after_update_in_progress() as _;
        cc2 = rpcc_after_update_in_progress() as _;
        est_cycle_freq = validate_cc_value(cc2.wrapping_sub(cc1) as _);
        cc1 = cc2;
    }
    cycle_freq = (*hwrpb).cycle_freq;
    if est_cycle_freq != 0 {
        tolerance = cycle_freq / 4000;
        diff = cycle_freq as i64 - est_cycle_freq as i64;
        if diff < 0 { diff = -diff; }
        if diff as _ > tolerance { cycle_freq = est_cycle_freq; printk_estimated(cycle_freq); } else { est_cycle_freq = 0; }
    } else if validate_cc_value(cycle_freq) == 0 { printk_str("HWRPB cycle frequency bogus, and unable to estimate a proper value!\n"); }
    #[cfg(not(feature = "CONFIG_ALPHA_WTINT"))]
    if (*hwrpb).nr_processors == 1 { clocksource_register_hz(&mut clocksource_rpcc, cycle_freq); }
    (alpha_mv.init_rtc)();
    init_rtc_clockevent();
}

#[cfg(feature = "CONFIG_SMP")]
pub unsafe fn init_clockevent() {
    if alpha_using_qemu { init_qemu_clockevent(); } else { init_rtc_clockevent(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
