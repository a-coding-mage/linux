// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 *	timers.c -- generic ColdFire hardware timer support.
 *
 *	Copyright (C) 1999-2008, Greg Ungerer <gerg@snapgear.com>
 */

/***************************************************************************/

// C header dependencies are supplied by the surrounding kernel translation.

/* By default use timer1 as the system clock timer. */
const FREQ: u32 = MCF_BUSCLK / 16;

#[inline]
const fn ta(a: usize) -> usize { MCFTIMER_BASE1 + a }

// On CONFIG_M53xx or CONFIG_M5441x, these aliases select 32-bit access;
// otherwise they select 16-bit access.
#[cfg(any(CONFIG_M53xx, CONFIG_M5441x))]
#[inline]
unsafe fn mcf_writetrr(value: u32, address: usize) { mcf_write32(value, address); }

#[cfg(not(any(CONFIG_M53xx, CONFIG_M5441x)))]
#[inline]
unsafe fn mcf_writetrr(value: u16, address: usize) { mcf_write16(value, address); }

static mut mcftmr_cycles_per_jiffy: u32 = 0;
static mut mcftmr_cnt: u32 = 0;

unsafe fn init_timer_irq() {
    // MCFSIM_ICR_AUTOVEC conditional code is retained by the build configuration.
    #[cfg(MCFSIM_ICR_AUTOVEC)]
    {
        // Timer1 is always used as system timer
        mcf_write8(MCFSIM_ICR_AUTOVEC | MCFSIM_ICR_LEVEL6 | MCFSIM_ICR_PRI3,
                   MCFSIM_TIMER1ICR);
        mcf_mapirq2imr(MCF_IRQ_TIMER, MCFINTC_TIMER1);

        #[cfg(CONFIG_HIGHPROFILE)]
        {
            // Timer2 is to be used as a high speed profile timer
            mcf_write8(MCFSIM_ICR_AUTOVEC | MCFSIM_ICR_LEVEL7 | MCFSIM_ICR_PRI3,
                       MCFSIM_TIMER2ICR);
            mcf_mapirq2imr(MCF_IRQ_PROFILER, MCFINTC_TIMER2);
        }
    }
}

unsafe extern "C" fn mcftmr_tick(_irq: i32, _dummy: *mut core::ffi::c_void) -> irqreturn_t {
    // Reset the ColdFire timer
    mcf_write8(MCFTIMER_TER_CAP | MCFTIMER_TER_REF, ta(MCFTIMER_TER));

    mcftmr_cnt = mcftmr_cnt.wrapping_add(mcftmr_cycles_per_jiffy);
    legacy_timer_tick(1);
    IRQ_HANDLED
}

unsafe extern "C" fn mcftmr_read_clk(_cs: *mut clocksource) -> u64 {
    let mut flags: ulong = 0;
    let cycles: u32;
    let tcn: u16;

    local_irq_save(&mut flags);
    tcn = mcf_read16(ta(MCFTIMER_TCN));
    cycles = mcftmr_cnt;
    local_irq_restore(flags);

    cycles as u64 + tcn as u64
}

static mut mcftmr_clk: clocksource = clocksource {
    name: "tmr",
    rating: 250,
    read: Some(mcftmr_read_clk),
    mask: CLOCKSOURCE_MASK(32),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

pub unsafe extern "C" fn hw_timer_init() {
    let r: i32;

    mcf_write16(MCFTIMER_TMR_DISABLE, ta(MCFTIMER_TMR));
    mcftmr_cycles_per_jiffy = FREQ / HZ;
    /*
     *	The coldfire timer runs from 0 to TRR included, then 0
     *	again and so on.  It counts thus actually TRR + 1 steps
     *	for 1 tick, not TRR.  So if you want n cycles,
     *	initialize TRR with n - 1.
     */
    #[cfg(any(CONFIG_M53xx, CONFIG_M5441x))]
    mcf_writetrr(mcftmr_cycles_per_jiffy - 1, ta(MCFTIMER_TRR));
    #[cfg(not(any(CONFIG_M53xx, CONFIG_M5441x)))]
    mcf_writetrr((mcftmr_cycles_per_jiffy - 1) as u16, ta(MCFTIMER_TRR));
    mcf_write16(MCFTIMER_TMR_ENORI | MCFTIMER_TMR_CLK16 |
                MCFTIMER_TMR_RESTART | MCFTIMER_TMR_ENABLE,
                ta(MCFTIMER_TMR));

    clocksource_register_hz(&mut mcftmr_clk, FREQ);

    init_timer_irq();
    r = request_irq(MCF_IRQ_TIMER, Some(mcftmr_tick), IRQF_TIMER, "timer", core::ptr::null_mut());
    if r != 0 {
        pr_err!("Failed to request irq {} (timer): %pe\n", MCF_IRQ_TIMER, ERR_PTR(r));
    }

    #[cfg(CONFIG_HIGHPROFILE)]
    coldfire_profile_init();
}

#[cfg(CONFIG_HIGHPROFILE)]
const fn pa(a: usize) -> usize { MCFTIMER_BASE2 + a }

#[cfg(CONFIG_HIGHPROFILE)]
const PROFILEHZ: u32 = 1013;

#[cfg(CONFIG_HIGHPROFILE)]
pub unsafe extern "C" fn coldfire_profile_tick(_irq: i32, _dummy: *mut core::ffi::c_void) -> irqreturn_t {
    // Reset ColdFire timer2
    mcf_write8(MCFTIMER_TER_CAP | MCFTIMER_TER_REF, pa(MCFTIMER_TER));
    if (*current).pid != 0 {
        profile_tick(CPU_PROFILING);
    }
    IRQ_HANDLED
}

#[cfg(CONFIG_HIGHPROFILE)]
pub unsafe extern "C" fn coldfire_profile_init() {
    let ret: i32;

    printk!(KERN_INFO, "PROFILE: lodging TIMER2 @ {}Hz as profile timer\n", PROFILEHZ);

    // Set up TIMER 2 as high speed profile clock
    mcf_write16(MCFTIMER_TMR_DISABLE, pa(MCFTIMER_TMR));

    #[cfg(any(CONFIG_M53xx, CONFIG_M5441x))]
    mcf_writetrr((MCF_BUSCLK / 16) / PROFILEHZ, pa(MCFTIMER_TRR));
    #[cfg(not(any(CONFIG_M53xx, CONFIG_M5441x)))]
    mcf_writetrr(((MCF_BUSCLK / 16) / PROFILEHZ) as u16, pa(MCFTIMER_TRR));
    mcf_write16(MCFTIMER_TMR_ENORI | MCFTIMER_TMR_CLK16 |
                MCFTIMER_TMR_RESTART | MCFTIMER_TMR_ENABLE,
                pa(MCFTIMER_TMR));

    ret = request_irq(MCF_IRQ_PROFILER, Some(coldfire_profile_tick), IRQF_TIMER,
                      "profile timer", core::ptr::null_mut());
    if ret != 0 {
        pr_err!("Failed to request irq {} (profile timer): %pe\n",
                MCF_IRQ_PROFILER, ERR_PTR(ret));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
