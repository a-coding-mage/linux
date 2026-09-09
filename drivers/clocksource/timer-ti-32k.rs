// SPDX-License-Identifier: GPL-2.0-only
/*
 * timer-ti-32k.c - OMAP2 32k Timer Support
 *
 * Copyright (C) 2009 Nokia Corporation
 *
 * Update to use new clocksource/clockevent layers
 * Author: Kevin Hilman, MontaVista Software, Inc. <source@mvista.com>
 * Copyright (C) 2007 MontaVista Software, Inc.
 *
 * Original driver:
 * Copyright (C) 2005 Nokia Corporation
 * Author: Paul Mundt <paul.mundt@nokia.com>
 *         Juha Yrjölä <juha.yrjola@nokia.com>
 * OMAP Dual-mode timer framework support by Timo Teras
 *
 * Some parts based off of TI's 24xx code:
 *
 * Copyright (C) 2004-2009 Texas Instruments, Inc.
 *
 * Roughly modelled after the OMAP1 MPU timer code.
 * Added OMAP4 support - Santosh Shilimkar <santosh.shilimkar@ti.com>
 *
 * Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com
 */

// C dependencies supplied by the surrounding kernel translation.

const OMAP2_32KSYNCNT_REV_OFF: usize = 0x0;
const OMAP2_32KSYNCNT_REV_SCHEME: u32 = 0x3 << 30;
const OMAP2_32KSYNCNT_CR_OFF_LOW: usize = 0x10;
const OMAP2_32KSYNCNT_CR_OFF_HIGH: usize = 0x30;

#[repr(C)]
struct Ti32k {
    base: *mut core::ffi::c_void,
    counter: *mut core::ffi::c_void,
    cs: Clocksource,
}

#[inline]
unsafe fn to_ti_32k(cs: *mut Clocksource) -> *mut Ti32k {
    // Equivalent to container_of(cs, struct ti_32k, cs).
    (cs as *mut u8).sub(core::mem::offset_of!(Ti32k, cs)) as *mut Ti32k
}

unsafe fn ti_32k_read_cycles(cs: *mut Clocksource) -> u64 {
    let ti = &*to_ti_32k(cs);
    readl_relaxed(ti.counter) as u64
}

static mut TI_32K_TIMER: Ti32k = Ti32k {
    base: core::ptr::null_mut(),
    counter: core::ptr::null_mut(),
    cs: Clocksource {
        name: "32k_counter",
        rating: 250,
        read: Some(ti_32k_read_cycles),
        mask: clocksource_mask(32),
        flags: CLOCK_SOURCE_IS_CONTINUOUS,
    },
};

unsafe fn omap_32k_read_sched_clock() -> u64 {
    ti_32k_read_cycles(&raw mut TI_32K_TIMER.cs)
}

unsafe fn ti_32k_timer_enable_clock(np: *mut DeviceNode, name: *const i8) {
    let clock: *mut Clk = of_clk_get_by_name((*np).parent, name);
    if is_err(clock) {
        // Only some SoCs have a separate interface clock
        if ptr_err(clock) == -22 && !strncmp(b"ick\0".as_ptr() as *const i8, name, 3) {
            return;
        }

        pr_warn("%s: could not get clock %s %li\n", "ti_32k_timer_enable_clock", name, ptr_err(clock));
        return;
    }

    let error = clk_prepare_enable(clock);
    if error != 0 {
        pr_warn!("%s: could not enable %s: %i\n", "ti_32k_timer_enable_clock", name, error);
        return;
    }
}

unsafe fn ti_32k_timer_module_init(np: *mut DeviceNode, base: *mut core::ffi::c_void) {
    let sysc = (base as *mut u8).add(4) as *mut core::ffi::c_void;

    if !of_device_is_compatible((*np).parent, b"ti,sysc\0".as_ptr() as *const i8) {
        return;
    }

    ti_32k_timer_enable_clock(np, b"fck\0".as_ptr() as *const i8);
    ti_32k_timer_enable_clock(np, b"ick\0".as_ptr() as *const i8);

    /*
     * Force idle module as wkup domain is active with MPU.
     * No need to tag the module disabled for ti-sysc probe.
     */
    writel_relaxed(0, sysc);
}

unsafe fn ti_32k_timer_init(np: *mut DeviceNode) -> i32 {
    let mut ret: i32;

    TI_32K_TIMER.base = of_iomap(np, 0);
    if TI_32K_TIMER.base.is_null() {
        pr_err!("Can't ioremap 32k timer base\n");
        return -6;
    }

    if !of_machine_is_compatible(b"ti,am43\0".as_ptr() as *const i8) {
        TI_32K_TIMER.cs.flags |= CLOCK_SOURCE_SUSPEND_NONSTOP;
    }

    TI_32K_TIMER.counter = TI_32K_TIMER.base;
    ti_32k_timer_module_init(np, TI_32K_TIMER.base);

    /*
     * 32k sync Counter IP register offsets vary between the highlander
     * version and the legacy ones.
     *
     * The 'SCHEME' bits(30-31) of the revision register is used to identify
     * the version.
     */
    if readl_relaxed((TI_32K_TIMER.base as *mut u8).add(OMAP2_32KSYNCNT_REV_OFF) as *mut core::ffi::c_void)
        & OMAP2_32KSYNCNT_REV_SCHEME != 0
    {
        TI_32K_TIMER.counter = (TI_32K_TIMER.counter as *mut u8).add(OMAP2_32KSYNCNT_CR_OFF_HIGH) as *mut core::ffi::c_void;
    } else {
        TI_32K_TIMER.counter = (TI_32K_TIMER.counter as *mut u8).add(OMAP2_32KSYNCNT_CR_OFF_LOW) as *mut core::ffi::c_void;
    }

    pr_info!("OMAP clocksource: 32k_counter at 32768 Hz\n");

    ret = clocksource_register_hz(&raw mut TI_32K_TIMER.cs, 32768);
    if ret != 0 {
        pr_err!("32k_counter: can't register clocksource\n");
        return ret;
    }

    sched_clock_register(Some(omap_32k_read_sched_clock), 32, 32768);
    0
}

// TIMER_OF_DECLARE(ti_32k_timer, "ti,omap-counter32k", ti_32k_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
