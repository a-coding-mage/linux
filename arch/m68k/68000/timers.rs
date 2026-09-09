/*
 *  timers.c - Generic hardware timer support.
 *
 *  Copyright (C) 1993 Hamish Macdonald
 *  Copyright (C) 1999 D. Jeff Dionne
 *  Copyright (C) 2001 Georges Menie, Ken Desmet
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

/* The C preprocessor selects these values from the target configuration. */
#[cfg(CONFIG_DRAGEN2)]
const CLOCK_SOURCE: u32 = TCTL_CLKSOURCE_SYSCLK;
#[cfg(CONFIG_DRAGEN2)]
const CLOCK_PRE: u32 = 7;
#[cfg(CONFIG_DRAGEN2)]
const TICKS_PER_JIFFY: u32 = 41450;

#[cfg(all(not(CONFIG_DRAGEN2), CONFIG_XCOPILOT_BUGS))]
const CLOCK_SOURCE: u32 = TCTL_CLKSOURCE_SYSCLK;
#[cfg(all(not(CONFIG_DRAGEN2), CONFIG_XCOPILOT_BUGS))]
const CLOCK_PRE: u32 = 2;
#[cfg(all(not(CONFIG_DRAGEN2), CONFIG_XCOPILOT_BUGS))]
const TICKS_PER_JIFFY: u32 = 0xd7e4;

#[cfg(all(not(CONFIG_DRAGEN2), not(CONFIG_XCOPILOT_BUGS)))]
const CLOCK_SOURCE: u32 = TCTL_CLKSOURCE_32KHZ;
#[cfg(all(not(CONFIG_DRAGEN2), not(CONFIG_XCOPILOT_BUGS)))]
const CLOCK_PRE: u32 = 31;
#[cfg(all(not(CONFIG_DRAGEN2), not(CONFIG_XCOPILOT_BUGS)))]
const TICKS_PER_JIFFY: u32 = 10;

static mut m68328_tick_cnt: u32 = 0;

unsafe fn hw_tick(_irq: i32, _dummy: *mut core::ffi::c_void) -> irqreturn_t {
    /* Reset Timer1 */
    TSTAT &= 0;

    m68328_tick_cnt = m68328_tick_cnt.wrapping_add(TICKS_PER_JIFFY);
    legacy_timer_tick(1);
    IRQ_HANDLED
}

unsafe fn m68328_read_clk(_cs: *mut clocksource) -> u64 {
    let mut flags: c_ulong = 0;
    let cycles: u32;

    local_irq_save(&mut flags);
    cycles = m68328_tick_cnt.wrapping_add(TCN);
    local_irq_restore(flags);

    cycles as u64
}

static mut m68328_clk: clocksource = clocksource {
    name: "timer",
    rating: 250,
    read: Some(m68328_read_clk),
    mask: CLOCKSOURCE_MASK(32),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

pub unsafe fn hw_timer_init() {
    let mut ret: i32;

    /* disable timer 1 */
    TCTL = 0;

    /* set ISR */
    ret = request_irq(TMR_IRQ_NUM, Some(hw_tick), IRQF_TIMER, "timer", core::ptr::null_mut());
    if ret != 0 {
        pr_err!(
            "Failed to request irq {} (timer): %pe\\n",
            TMR_IRQ_NUM,
            ERR_PTR(ret)
        );
    }

    /* Restart mode, Enable int, Set clock source */
    TCTL = TCTL_OM | TCTL_IRQEN | CLOCK_SOURCE;
    TPRER = CLOCK_PRE;
    TCMP = TICKS_PER_JIFFY;

    /* Enable timer 1 */
    TCTL |= TCTL_TEN;
    clocksource_register_hz(&mut m68328_clk, TICKS_PER_JIFFY * HZ);
}

pub unsafe fn m68328_hwclk(set: i32, t: *mut rtc_time) -> i32 {
    if set == 0 {
        let now: c_long = RTCTIME;
        (*t).tm_year = 1;
        (*t).tm_mon = 0;
        (*t).tm_mday = 1;
        (*t).tm_hour = (now >> 24) % 24;
        (*t).tm_min = (now >> 16) % 60;
        (*t).tm_sec = now % 60;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
