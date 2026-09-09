/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Based on linux/arch/mips/kernel/cevt-r4k.c,
 *          linux/arch/mips/jmr3927/rbhma3100/setup.c
 *
 * Copyright 2001 MontaVista Software Inc.
 * Copyright (C) 2000-2001 Toshiba Corporation
 * Copyright (C) 2007 MIPS Technologies, Inc.
 * Copyright (C) 2007 Ralf Baechle <ralf@linux-mips.org>
 */

/* Dependencies supplied by the surrounding kernel translation. */

const TCR_BASE: u32 = TXx9_TMTCR_CCDE | TXx9_TMTCR_CRE | TXx9_TMTCR_TMODE_ITVL;
const TIMER_CCD: u32 = 0; /* 1/2 */

#[inline]
const fn timer_clk(imclk: u32) -> u32 {
    imclk / (2 << TIMER_CCD)
}

#[repr(C)]
struct txx9_clocksource {
    cs: clocksource,
    tmrptr: *mut txx9_tmr_reg,
}

unsafe fn txx9_cs_read(cs: *mut clocksource) -> u64 {
    let txx9_cs = container_of!(cs, txx9_clocksource, cs);
    core::ptr::read_volatile(core::ptr::addr_of!((*(*txx9_cs).tmrptr).trr)) as u64
}

/* Use 1 bit smaller width to use full bits in that width */
const TXX9_CLOCKSOURCE_BITS: u32 = TXX9_TIMER_BITS - 1;

static mut txx9_clocksource: txx9_clocksource = txx9_clocksource {
    cs: clocksource {
        name: "TXx9",
        rating: 200,
        read: Some(txx9_cs_read),
        mask: CLOCKSOURCE_MASK!(TXX9_CLOCKSOURCE_BITS),
        flags: CLOCK_SOURCE_IS_CONTINUOUS,
    },
    tmrptr: core::ptr::null_mut(),
};

unsafe fn txx9_read_sched_clock() -> u64 {
    core::ptr::read_volatile(core::ptr::addr_of!((*(*txx9_clocksource).tmrptr).trr)) as u64
}

unsafe fn txx9_clocksource_init(baseaddr: usize, imbusclk: u32) {
    clocksource_register_hz(&mut txx9_clocksource.cs, timer_clk(imbusclk));

    let tmrptr = ioremap(baseaddr, core::mem::size_of::<txx9_tmr_reg>()) as *mut txx9_tmr_reg;
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).tcr), TCR_BASE);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).tisr), 0);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).ccdr), TIMER_CCD);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).itmr), TXx9_TMITMR_TZCE);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).cpra), 1u32 << TXX9_CLOCKSOURCE_BITS);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).tcr), TCR_BASE | TXx9_TMTCR_TCE);
    txx9_clocksource.tmrptr = tmrptr;

    sched_clock_register(txx9_read_sched_clock, TXX9_CLOCKSOURCE_BITS, timer_clk(imbusclk));
}

#[repr(C)]
struct txx9_clock_event_device {
    cd: clock_event_device,
    tmrptr: *mut txx9_tmr_reg,
}

unsafe fn txx9tmr_stop_and_clear(tmrptr: *mut txx9_tmr_reg) {
    /* stop and reset counter */
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).tcr), TCR_BASE);
    /* clear pending interrupt */
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).tisr), 0);
}

unsafe fn txx9tmr_set_state_periodic(evt: *mut clock_event_device) -> i32 {
    let txx9_cd = container_of!(evt, txx9_clock_event_device, cd);
    let tmrptr = (*txx9_cd).tmrptr;
    txx9tmr_stop_and_clear(tmrptr);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).itmr), TXx9_TMITMR_TIIE | TXx9_TMITMR_TZCE);
    /* start timer */
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).cpra), (((NSEC_PER_SEC / HZ) as u64 * (*evt).mult as u64) >> (*evt).shift) as u32);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).tcr), TCR_BASE | TXx9_TMTCR_TCE);
    0
}

unsafe fn txx9tmr_set_state_oneshot(evt: *mut clock_event_device) -> i32 {
    let txx9_cd = container_of!(evt, txx9_clock_event_device, cd);
    txx9tmr_stop_and_clear((*txx9_cd).tmrptr);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*(*txx9_cd).tmrptr).itmr), TXx9_TMITMR_TIIE);
    0
}

unsafe fn txx9tmr_set_state_shutdown(evt: *mut clock_event_device) -> i32 {
    let txx9_cd = container_of!(evt, txx9_clock_event_device, cd);
    txx9tmr_stop_and_clear((*txx9_cd).tmrptr);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*(*txx9_cd).tmrptr).itmr), 0);
    0
}

unsafe fn txx9tmr_tick_resume(evt: *mut clock_event_device) -> i32 {
    let txx9_cd = container_of!(evt, txx9_clock_event_device, cd);
    let tmrptr = (*txx9_cd).tmrptr;
    txx9tmr_stop_and_clear(tmrptr);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).ccdr), TIMER_CCD);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).itmr), 0);
    0
}

unsafe fn txx9tmr_set_next_event(delta: u32, evt: *mut clock_event_device) -> i32 {
    let txx9_cd = container_of!(evt, txx9_clock_event_device, cd);
    let tmrptr = (*txx9_cd).tmrptr;
    txx9tmr_stop_and_clear(tmrptr);
    /* start timer */
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).cpra), delta);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).tcr), TCR_BASE | TXx9_TMTCR_TCE);
    0
}

static mut txx9_clock_event_device: txx9_clock_event_device = txx9_clock_event_device {
    cd: clock_event_device {
        name: "TXx9",
        features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
        rating: 200,
        set_state_shutdown: Some(txx9tmr_set_state_shutdown),
        set_state_periodic: Some(txx9tmr_set_state_periodic),
        set_state_oneshot: Some(txx9tmr_set_state_oneshot),
        tick_resume: Some(txx9tmr_tick_resume),
        set_next_event: Some(txx9tmr_set_next_event),
    },
    tmrptr: core::ptr::null_mut(),
};

unsafe fn txx9tmr_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let txx9_cd = dev_id as *mut txx9_clock_event_device;
    let cd = core::ptr::addr_of_mut!((*txx9_cd).cd);
    let tmrptr = (*txx9_cd).tmrptr;
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).tisr), 0); /* ack interrupt */
    ((*cd).event_handler.unwrap())(cd);
    IRQ_HANDLED
}

unsafe fn txx9_clockevent_init(baseaddr: usize, irq: i32, imbusclk: u32) {
    let cd = core::ptr::addr_of_mut!(txx9_clock_event_device.cd);
    let tmrptr = ioremap(baseaddr, core::mem::size_of::<txx9_tmr_reg>()) as *mut txx9_tmr_reg;
    txx9tmr_stop_and_clear(tmrptr);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).ccdr), TIMER_CCD);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).itmr), 0);
    txx9_clock_event_device.tmrptr = tmrptr;
    clockevent_set_clock(cd, timer_clk(imbusclk));
    (*cd).max_delta_ns = clockevent_delta2ns(0xffff_ffff >> (32 - TXX9_TIMER_BITS), cd);
    (*cd).max_delta_ticks = 0xffff_ffff >> (32 - TXX9_TIMER_BITS);
    (*cd).min_delta_ns = clockevent_delta2ns(0xf, cd);
    (*cd).min_delta_ticks = 0xf;
    (*cd).irq = irq;
    (*cd).cpumask = cpumask_of(0);
    clockevents_register_device(cd);
    if request_irq(irq, txx9tmr_interrupt, IRQF_PERCPU | IRQF_TIMER, "txx9tmr", &mut txx9_clock_event_device as *mut _ as *mut core::ffi::c_void) != 0 {
        pr_err!("Failed to request irq {} (txx9tmr)\n", irq);
    }
    printk!(KERN_INFO, "TXx9: clockevent device at 0x{:lx}, irq {}\n", baseaddr, irq);
}

unsafe fn txx9_tmr_init(baseaddr: usize) {
    let tmrptr = ioremap(baseaddr, core::mem::size_of::<txx9_tmr_reg>()) as *mut txx9_tmr_reg;
    /* Start once to make CounterResetEnable effective */
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).tcr), TXx9_TMTCR_CRE | TXx9_TMTCR_TCE);
    /* Stop and reset the counter */
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).tcr), TXx9_TMTCR_CRE);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).tisr), 0);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).cpra), 0xffff_ffff);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).itmr), 0);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).ccdr), 0);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*tmrptr).pgmr), 0);
    iounmap(tmrptr as *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
