// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2012 MIPS Technologies, Inc.  All rights reserved.

// #define pr_fmt(fmt) "mips-gic-timer: " fmt
// C kernel includes are supplied by external dependencies.

static mut GIC_CLOCKEVENT_DEVICE: PerCpu<ClockEventDevice> = define_per_cpu();
static mut gic_timer_irq: i32 = 0;
static mut gic_frequency: u32 = 0;
static mut gic_count_width: u32 = 0;
static mut gic_clock_unstable: bool = false;

// Forward declaration: `gic_clocksource_unstable` is defined below.

unsafe fn gic_read_count_2x32() -> u64 {
    let (mut hi, mut hi2, mut lo): (u32, u32, u32);

    loop {
        hi = read_gic_counter_32h();
        lo = read_gic_counter_32l();
        hi2 = read_gic_counter_32h();
        if hi2 == hi {
            break;
        }
    }

    ((hi as u64) << 32).wrapping_add(lo as u64)
}

unsafe fn gic_read_count_64() -> u64 {
    read_gic_counter()
}

unsafe fn gic_read_count() -> u64 {
    if mips_cm_is64 {
        gic_read_count_64()
    } else {
        gic_read_count_2x32()
    }
}

unsafe fn gic_next_event(delta: usize, evt: *mut ClockEventDevice) -> i32 {
    let cpu = cpumask_first((*evt).cpumask);
    let mut cnt = gic_read_count();
    cnt = cnt.wrapping_add(delta as u64);
    if cpu == raw_smp_processor_id() {
        write_gic_vl_compare(cnt);
    } else {
        write_gic_vl_other(mips_cm_vp_id(cpu));
        write_gic_vo_compare(cnt);
    }
    if (gic_read_count().wrapping_sub(cnt) as i64) >= 0 { -ETIME } else { 0 }
}

unsafe fn gic_compare_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> IrqReturn {
    let cd = dev_id as *mut ClockEventDevice;
    write_gic_vl_compare(read_gic_vl_compare());
    ((*cd).event_handler)(cd);
    IRQ_HANDLED
}

unsafe fn gic_clockevent_cpu_init(cpu: u32, cd: *mut ClockEventDevice) {
    (*cd).name = b"MIPS GIC\0".as_ptr() as *const i8;
    (*cd).features = CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_C3STOP;
    (*cd).rating = 350;
    (*cd).irq = gic_timer_irq;
    (*cd).cpumask = cpumask_of(cpu);
    (*cd).set_next_event = Some(gic_next_event);
    clockevents_config_and_register(cd, gic_frequency, 0x300, 0x7fffffff);
    enable_percpu_irq(gic_timer_irq, IRQ_TYPE_NONE);
}

unsafe fn gic_clockevent_cpu_exit(_cd: *mut ClockEventDevice) {
    disable_percpu_irq(gic_timer_irq);
}

unsafe extern "C" fn gic_update_frequency(data: *mut core::ffi::c_void) {
    let rate = data as usize;
    clockevents_update_freq(this_cpu_ptr(&raw mut GIC_CLOCKEVENT_DEVICE), rate);
}

unsafe fn gic_starting_cpu(cpu: u32) -> i32 {
    clear_gic_config(GIC_CONFIG_COUNTSTOP);
    gic_clockevent_cpu_init(cpu, this_cpu_ptr(&raw mut GIC_CLOCKEVENT_DEVICE));
    0
}

unsafe fn gic_clk_notifier(_nb: *mut NotifierBlock, action: usize, data: *mut core::ffi::c_void) -> i32 {
    let cnd = data as *mut ClkNotifierData;
    if action == POST_RATE_CHANGE {
        gic_clocksource_unstable(b"ref clock rate change\0".as_ptr() as *mut i8);
        on_each_cpu(gic_update_frequency, (*cnd).new_rate as *mut core::ffi::c_void, 1);
    }
    NOTIFY_OK
}

unsafe fn gic_dying_cpu(_cpu: u32) -> i32 {
    gic_clockevent_cpu_exit(this_cpu_ptr(&raw mut GIC_CLOCKEVENT_DEVICE));
    0
}

static mut gic_clk_nb: NotifierBlock = NotifierBlock { notifier_call: Some(gic_clk_notifier) };

unsafe fn gic_clockevent_init() -> i32 {
    let mut ret: i32;
    if gic_frequency == 0 { return -ENXIO; }
    ret = request_percpu_irq(gic_timer_irq, Some(gic_compare_interrupt), b"timer\0".as_ptr() as *const i8, &raw mut GIC_CLOCKEVENT_DEVICE);
    if ret < 0 {
        pr_err!("IRQ %d setup failed (%d)\n", gic_timer_irq, ret);
        return ret;
    }
    cpuhp_setup_state(CPUHP_AP_MIPS_GIC_TIMER_STARTING, b"clockevents/mips/gic/timer:starting\0".as_ptr() as *const i8, Some(gic_starting_cpu), Some(gic_dying_cpu));
    0
}

unsafe fn gic_hpt_read(_cs: *mut ClockSource) -> u64 { gic_read_count() }

unsafe fn gic_hpt_read_multicluster(_cs: *mut ClockSource) -> u64 {
    let (mut hi, mut hi2, mut lo): (u32, u32, u32);
    let count: u64;
    mips_cm_lock_other(0, 0, 0, CM_GCR_Cx_OTHER_BLOCK_GLOBAL);
    if mips_cm_is64 {
        count = read_gic_redir_counter();
    } else {
        hi = read_gic_redir_counter_32h();
        loop {
            lo = read_gic_redir_counter_32l();
            hi2 = read_gic_redir_counter_32h();
            if hi2 == hi { break; }
            hi = hi2;
        }
        count = ((hi as u64) << 32).wrapping_add(lo as u64);
    }
    mips_cm_unlock_other();
    count
}

static mut gic_clocksource: ClockSource = ClockSource {
    name: b"GIC\0".as_ptr() as *const i8, read: Some(gic_hpt_read), flags: CLOCK_SOURCE_IS_CONTINUOUS,
    vdso_clock_mode: VDSO_CLOCKMODE_GIC, mask: 0, rating: 0,
};

unsafe fn gic_clocksource_unstable(reason: *mut i8) {
    if gic_clock_unstable { return; }
    gic_clock_unstable = true;
    pr_info!("GIC timer is unstable due to %s\n", reason);
    clocksource_mark_unstable(&raw mut gic_clocksource);
}

unsafe fn __gic_clocksource_init() -> i32 {
    let mut ret: i32;
    gic_count_width = read_gic_config() & GIC_CONFIG_COUNTBITS;
    gic_count_width >>= __ffs(GIC_CONFIG_COUNTBITS);
    gic_count_width *= 4;
    gic_count_width += 32;
    gic_clocksource.mask = CLOCKSOURCE_MASK(gic_count_width);
    if mips_cm_revision() >= CM_REV_CM3 || !IS_ENABLED_CONFIG_CPU_FREQ {
        gic_clocksource.rating = 300;
    } else { gic_clocksource.rating = 200; }
    gic_clocksource.rating += clamp(gic_frequency / 10000000, 0, 99);
    if mips_cps_multicluster_cpus() {
        gic_clocksource.read = Some(gic_hpt_read_multicluster);
        gic_clocksource.vdso_clock_mode = VDSO_CLOCKMODE_NONE;
    }
    ret = clocksource_register_hz(&raw mut gic_clocksource, gic_frequency);
    if ret < 0 { pr_warn!("Unable to register clocksource\n"); }
    ret
}

unsafe fn gic_clocksource_of_init(node: *mut DeviceNode) -> i32 {
    let mut clk: *mut Clk;
    let mut ret: i32;
    if !mips_gic_present() || (*node).parent.is_null() || !of_device_is_compatible((*node).parent, b"mti,gic\0".as_ptr() as *const i8) {
        pr_warn!("No DT definition\n"); return -ENXIO;
    }
    clk = of_clk_get(node, 0);
    if !IS_ERR(clk) {
        ret = clk_prepare_enable(clk);
        if ret < 0 { pr_err!("Failed to enable clock\n"); clk_put(clk); return ret; }
        gic_frequency = clk_get_rate(clk);
    } else if of_property_read_u32(node, b"clock-frequency\0".as_ptr() as *const i8, &raw mut gic_frequency) != 0 {
        pr_err!("Frequency not specified\n"); return -EINVAL;
    }
    gic_timer_irq = irq_of_parse_and_map(node, 0);
    if gic_timer_irq == 0 { pr_err!("IRQ not specified\n"); return -EINVAL; }
    ret = __gic_clocksource_init(); if ret != 0 { return ret; }
    ret = gic_clockevent_init();
    if ret == 0 && !IS_ERR(clk) { if clk_notifier_register(clk, &raw mut gic_clk_nb) < 0 { pr_warn!("Unable to register clock notifier\n"); } }
    if (mips_cm_revision() >= CM_REV_CM3 || !IS_ENABLED_CONFIG_CPU_FREQ) && !mips_cps_multicluster_cpus() {
        sched_clock_register(if mips_cm_is64 { Some(gic_read_count_64) } else { Some(gic_read_count_2x32) }, gic_count_width, gic_frequency);
    }
    0
}

// TIMER_OF_DECLARE(mips_gic_timer, "mti,gic-timer", gic_clocksource_of_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
