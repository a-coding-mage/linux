// SPDX-License-Identifier: GPL-2.0-only
/*
 * Carsten Langgaard, carstenl@mips.com
 * Copyright (C) 1999,2000 MIPS Technologies, Inc.  All rights reserved.
 *
 * Setting up the clock on the MIPS boards.
 */

// Linux and MIPS declarations supplied by the surrounding kernel translation.

static mut MIPS_CPU_TIMER_IRQ: i32 = 0;
static mut MIPS_CPU_PERF_IRQ: i32 = 0;
extern "C" {
    static mut cp0_perfcount_irq: i32;
}

static mut GIC_FREQUENCY: u32 = 0;

unsafe fn mips_timer_dispatch() {
    do_IRQ(MIPS_CPU_TIMER_IRQ);
}

unsafe fn mips_perf_dispatch() {
    do_IRQ(MIPS_CPU_PERF_IRQ);
}

unsafe fn freqround(mut freq: u32, amount: u32) -> u32 {
    freq = freq.wrapping_add(amount);
    freq = freq.wrapping_sub(freq % amount.wrapping_mul(2));
    freq
}

/*
 * Estimate CPU and GIC frequencies.
 */
unsafe fn estimate_frequencies() {
    let flags: u64;
    let mut count: u32;
    let mut start: u32;
    let mut secs1: u8;
    let mut secs2: u8;
    let mut ctrl: u8;
    let mut secs: i32;
    let mut giccount: u64 = 0;
    let mut gicstart: u64 = 0;

    local_irq_save(&mut flags);

    if mips_gic_present() {
        clear_gic_config(GIC_CONFIG_COUNTSTOP);
    }

    /*
     * Read counters exactly on rising edge of update flag.
     * This helps get an accurate reading under virtualisation.
     */
    while CMOS_READ(RTC_REG_A) & RTC_UIP != 0 {}
    while CMOS_READ(RTC_REG_A) & RTC_UIP == 0 {}
    start = read_c0_count();
    if mips_gic_present() {
        gicstart = read_gic_counter();
    }

    /* Wait for falling edge before reading RTC. */
    while CMOS_READ(RTC_REG_A) & RTC_UIP != 0 {}
    secs1 = CMOS_READ(RTC_SECONDS);

    /* Read counters again exactly on rising edge of update flag. */
    while CMOS_READ(RTC_REG_A) & RTC_UIP == 0 {}
    count = read_c0_count();
    if mips_gic_present() {
        giccount = read_gic_counter();
    }

    /* Wait for falling edge before reading RTC again. */
    while CMOS_READ(RTC_REG_A) & RTC_UIP != 0 {}
    secs2 = CMOS_READ(RTC_SECONDS);

    ctrl = CMOS_READ(RTC_CONTROL);

    local_irq_restore(flags);

    if (ctrl & RTC_DM_BINARY == 0) || RTC_ALWAYS_BCD {
        secs1 = bcd2bin(secs1);
        secs2 = bcd2bin(secs2);
    }
    secs = secs2 as i32 - secs1 as i32;
    if secs < 1 {
        secs += 60;
    }

    count = count.wrapping_sub(start);
    count /= secs as u32;
    mips_hpt_frequency = count;

    if mips_gic_present() {
        giccount = (giccount.wrapping_sub(gicstart)) / secs as u64;
        GIC_FREQUENCY = giccount as u32;
    }
}

pub unsafe fn read_persistent_clock64(ts: *mut timespec64) {
    (*ts).tv_sec = mc146818_get_cmos_time();
    (*ts).tv_nsec = 0;
}

pub unsafe fn get_c0_fdc_int() -> i32 {
    /*
     * Some cores claim the FDC is routable through the GIC, but it doesn't
     * actually seem to be connected for those Malta bitstreams.
     */
    match current_cpu_type() {
        CPU_INTERAPTIV | CPU_PROAPTIV => return -1,
        _ => {}
    }

    if cpu_has_veic {
        -1
    } else if mips_gic_present() {
        gic_get_c0_fdc_int()
    } else if cp0_fdc_irq >= 0 {
        MIPS_CPU_IRQ_BASE + cp0_fdc_irq
    } else {
        -1
    }
}

pub unsafe fn get_c0_perfcount_int() -> i32 {
    if cpu_has_veic {
        set_vi_handler(MSC01E_INT_PERFCTR, mips_perf_dispatch);
        MIPS_CPU_PERF_IRQ = MSC01E_INT_BASE + MSC01E_INT_PERFCTR;
    } else if mips_gic_present() {
        MIPS_CPU_PERF_IRQ = gic_get_c0_perfcount_int();
    } else if cp0_perfcount_irq >= 0 {
        MIPS_CPU_PERF_IRQ = MIPS_CPU_IRQ_BASE + cp0_perfcount_irq;
    } else {
        MIPS_CPU_PERF_IRQ = -1;
    }

    MIPS_CPU_PERF_IRQ
}

// EXPORT_SYMBOL_GPL(get_c0_perfcount_int);

pub unsafe fn get_c0_compare_int() -> u32 {
    if cpu_has_veic {
        set_vi_handler(MSC01E_INT_CPUCTR, mips_timer_dispatch);
        MIPS_CPU_TIMER_IRQ = MSC01E_INT_BASE + MSC01E_INT_CPUCTR;
    } else if mips_gic_present() {
        MIPS_CPU_TIMER_IRQ = gic_get_c0_compare_int();
    } else {
        MIPS_CPU_TIMER_IRQ = MIPS_CPU_IRQ_BASE + cp0_compare_irq;
    }

    MIPS_CPU_TIMER_IRQ as u32
}

unsafe fn init_rtc() {
    let mut freq: u8;
    let mut ctrl: u8;

    /* Set 32KHz time base if not already set */
    freq = CMOS_READ(RTC_FREQ_SELECT);
    if freq & RTC_DIV_CTL != RTC_REF_CLCK_32KHZ {
        CMOS_WRITE(RTC_REF_CLCK_32KHZ, RTC_FREQ_SELECT);
    }

    /* Ensure SET bit is clear so RTC can run */
    ctrl = CMOS_READ(RTC_CONTROL);
    if ctrl & RTC_SET != 0 {
        CMOS_WRITE(ctrl & !RTC_SET, RTC_CONTROL);
    }
}

// CONFIG_CLKSRC_MIPS_GIC conditional declarations and code are preserved below.
#[cfg(CONFIG_CLKSRC_MIPS_GIC)]
static mut gic_frequency_dt: u32 = 0;

#[cfg(CONFIG_CLKSRC_MIPS_GIC)]
static mut gic_frequency_prop: property = property {
    name: "clock-frequency",
    length: core::mem::size_of::<u32>() as u32,
    value: &raw mut gic_frequency_dt as *mut u8,
};

#[cfg(CONFIG_CLKSRC_MIPS_GIC)]
unsafe fn update_gic_frequency_dt() {
    let mut node: *mut device_node;

    gic_frequency_dt = cpu_to_be32(GIC_FREQUENCY);
    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "mti,gic-timer");
    if node.is_null() {
        pr_err("mti,gic-timer device node not found\n");
        return;
    }

    if of_update_property(node, &raw mut gic_frequency_prop) < 0 {
        pr_err("error updating gic frequency property\n");
    }
    of_node_put(node);
}

pub unsafe fn plat_time_init() {
    let prid: u32 = read_c0_prid() & (PRID_COMP_MASK | PRID_IMP_MASK);
    let mut freq: u32;

    init_rtc();
    estimate_frequencies();

    freq = mips_hpt_frequency;
    if prid != (PRID_COMP_MIPS | PRID_IMP_20KC) &&
       prid != (PRID_COMP_MIPS | PRID_IMP_25KF) {
        freq = freq.wrapping_mul(2);
    }
    freq = freqround(freq, 5000);
    printk("CPU frequency %d.%02d MHz\n", freq / 1000000,
           (freq % 1000000) * 100 / 1000000);

    // CONFIG_I8253: Only Malta has a PIT.
    #[cfg(CONFIG_I8253)]
    setup_pit_timer();

    if mips_gic_present() {
        freq = freqround(GIC_FREQUENCY, 5000);
        printk("GIC frequency %d.%02d MHz\n", freq / 1000000,
               (freq % 1000000) * 100 / 1000000);
        #[cfg(CONFIG_CLKSRC_MIPS_GIC)]
        {
            update_gic_frequency_dt();
            timer_probe();
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
