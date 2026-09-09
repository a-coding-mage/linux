// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/common/time-acorn.c
 *
 *  Copyright (c) 1996-2000 Russell King.
 *
 *  Changelog:
 *   24-Sep-1996 RMK Created
 *   10-Oct-1996 RMK Brought up to date with arch-sa110eval
 *   04-Dec-1997 RMK Updated for new arch/arm/time.c
 *   13=Jun-2004 DS Moved to arch/arm/common b/c shared w/CLPS7500
 */

// Dependencies supplied by the surrounding kernel translation.

const RPC_CLOCK_FREQ: u32 = 2_000_000;
const RPC_LATCH: u32 = (RPC_CLOCK_FREQ + HZ / 2) / HZ;

static mut ioc_time: u32 = 0;

unsafe fn ioc_timer_read(_cs: *mut clocksource) -> u64 {
    let mut count1: u32;
    let mut count2: u32;
    let mut status: u32;
    let mut flags: c_ulong = 0;
    let ticks: u32;

    local_irq_save(&mut flags);
    ioc_writeb(0, IOC_T0LATCH);
    barrier();
    count1 = ioc_readb(IOC_T0CNTL) | (ioc_readb(IOC_T0CNTH) << 8);
    barrier();
    status = ioc_readb(IOC_IRQREQA);
    barrier();
    ioc_writeb(0, IOC_T0LATCH);
    barrier();
    count2 = ioc_readb(IOC_T0CNTL) | (ioc_readb(IOC_T0CNTH) << 8);
    ticks = ioc_time.wrapping_add(RPC_LATCH).wrapping_sub(count2);
    local_irq_restore(flags);

    let mut ticks = ticks;
    if count2 < count1 {
        /*
         * The timer has not reloaded between reading count1 and
         * count2, check whether an interrupt was actually pending.
         */
        if status & (1 << 5) != 0 {
            ticks = ticks.wrapping_add(RPC_LATCH);
        }
    } else if count2 > count1 {
        /*
         * The timer has reloaded, so count2 indicates the new
         * count since the wrap.  The interrupt would not have
         * been processed, so add the missed ticks.
         */
        ticks = ticks.wrapping_add(RPC_LATCH);
    }

    ticks as u64
}

static mut ioctime_clocksource: clocksource = clocksource {
    read: Some(ioc_timer_read),
    mask: CLOCKSOURCE_MASK(32),
    rating: 100,
};

unsafe fn ioctime_init() {
    ioc_writeb(RPC_LATCH & 255, IOC_T0LTCHL);
    ioc_writeb(RPC_LATCH >> 8, IOC_T0LTCHH);
    ioc_writeb(0, IOC_T0GO);
}

unsafe extern "C" fn ioc_timer_interrupt(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    ioc_time = ioc_time.wrapping_add(RPC_LATCH);
    legacy_timer_tick(1);
    IRQ_HANDLED
}

/*
 * Set up timer interrupt.
 */
unsafe fn ioc_timer_init() {
    WARN_ON(clocksource_register_hz(&raw mut ioctime_clocksource, RPC_CLOCK_FREQ));
    ioctime_init();
    if request_irq(IRQ_TIMER0, Some(ioc_timer_interrupt), 0, b"timer\0".as_ptr(), core::ptr::null_mut()) != 0 {
        pr_err(b"Failed to request irq %d (timer)\n\0".as_ptr(), IRQ_TIMER0);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
