// SPDX-License-Identifier: GPL-2.0
/*
 * i8253 PIT clocksource
 */

// Linux headers and build-time definitions are supplied by the surrounding tree.

/*
 * Protects access to I/O ports
 *
 * 0040-0043 : timer0, i8253 / i8254
 * 0061-0061 : NMI Control Register which contains two speaker control bits.
 */
pub static mut i8253_lock: raw_spinlock_t = raw_spinlock_t::new();

#[cfg(CONFIG_CLKSRC_I8253)]
/*
 * Since the PIT overflows every tick, its not very useful
 * to just read by itself. So use jiffies to emulate a free
 * running counter:
 */
unsafe fn i8253_read(cs: *mut clocksource) -> u64 {
    static mut old_count: i32 = 0;
    static mut old_jifs: u32 = 0;
    let mut flags: c_ulong = 0;
    let mut count: i32;
    let jifs: u32;

    raw_spin_lock_irqsave(&raw mut i8253_lock, &mut flags);
    /*
     * Although our caller may have the read side of jiffies_lock,
     * this is now a seqlock, and we are cheating in this routine
     * by having side effects on state that we cannot undo if
     * there is a collision on the seqlock and our caller has to
     * retry.  (Namely, old_jifs and old_count.)  So we must treat
     * jiffies as volatile despite the lock.  We read jiffies
     * before latching the timer count to guarantee that although
     * the jiffies value might be older than the count (that is,
     * the counter may underflow between the last point where
     * jiffies was incremented and the point where we latch the
     * count), it cannot be newer.
     */
    jifs = jiffies;
    outb_p(0x00, PIT_MODE); /* latch the count ASAP */
    count = inb_p(PIT_CH0) as i32; /* read the latched count */
    count |= (inb_p(PIT_CH0) as i32) << 8;

    /* VIA686a test code... reset the latch if count > max + 1 */
    if count > PIT_LATCH {
        outb_p(0x34, PIT_MODE);
        outb_p(PIT_LATCH & 0xff, PIT_CH0);
        outb_p(PIT_LATCH >> 8, PIT_CH0);
        count = PIT_LATCH - 1;
    }

    /*
     * It's possible for count to appear to go the wrong way for
     * a couple of reasons:
     *
     *  1. The timer counter underflows, but we haven't handled the
     *     resulting interrupt and incremented jiffies yet.
     *  2. Hardware problem with the timer, not giving us continuous time,
     *     the counter does small "jumps" upwards on some Pentium systems,
     *     (see c't 95/10 page 335 for Neptun bug.)
     *
     * Previous attempts to handle these cases intelligently were
     * buggy, so we just do the simple thing now.
     */
    if count > old_count && jifs == old_jifs {
        count = old_count;
    }

    old_count = count;
    old_jifs = jifs;

    raw_spin_unlock_irqrestore(&raw mut i8253_lock, flags);

    count = (PIT_LATCH - 1) - count;

    (jifs as u64).wrapping_mul(PIT_LATCH as u64).wrapping_add(count as u64)
}

static mut i8253_cs: clocksource = clocksource {
    name: b"pit\0".as_ptr() as *const c_char,
    rating: 110,
    read: Some(i8253_read),
    mask: CLOCKSOURCE_MASK(32),
};

pub unsafe fn clocksource_i8253_init() -> c_int {
    clocksource_register_hz(&raw mut i8253_cs, PIT_TICK_RATE)
}

#[cfg(CONFIG_CLKEVT_I8253)]
pub unsafe fn clockevent_i8253_disable() {
    let _guard = guard_raw_spinlock_irqsave(&raw mut i8253_lock);

    /*
     * Writing the MODE register should stop the counter, according to
     * the datasheet. This appears to work on real hardware (well, on
     * modern Intel and AMD boxes; I didn't dig the Pegasos out of the
     * shed).
     *
     * However, some virtual implementations differ, and the MODE change
     * doesn't have any effect until either the counter is written (KVM
     * in-kernel PIT) or the next interrupt (QEMU). And in those cases,
     * it may not stop the *count*, only the interrupts. Although in
     * the virt case, that probably doesn't matter, as the value of the
     * counter will only be calculated on demand if the guest reads it;
     * it's the interrupts which cause steal time.
     *
     * Hyper-V apparently has a bug where even in mode 0, the IRQ keeps
     * firing repeatedly if the counter is running. But it *does* do the
     * right thing when the MODE register is written.
     *
     * So: write the MODE and then load the counter, which ensures that
     * the IRQ is stopped on those buggy virt implementations. And then
     * write the MODE again, which is the right way to stop it.
     */
    outb_p(0x30, PIT_MODE);
    outb_p(0, PIT_CH0);
    outb_p(0, PIT_CH0);
    outb_p(0x30, PIT_MODE);
}

unsafe fn pit_shutdown(evt: *mut clock_event_device) -> c_int {
    if !clockevent_state_oneshot(evt) && !clockevent_state_periodic(evt) {
        return 0;
    }
    clockevent_i8253_disable();
    0
}

unsafe fn pit_set_oneshot(evt: *mut clock_event_device) -> c_int {
    raw_spin_lock(&raw mut i8253_lock);
    outb_p(0x38, PIT_MODE);
    raw_spin_unlock(&raw mut i8253_lock);
    0
}

unsafe fn pit_set_periodic(evt: *mut clock_event_device) -> c_int {
    raw_spin_lock(&raw mut i8253_lock);
    /* binary, mode 2, LSB/MSB, ch 0 */
    outb_p(0x34, PIT_MODE);
    outb_p(PIT_LATCH & 0xff, PIT_CH0); /* LSB */
    outb_p(PIT_LATCH >> 8, PIT_CH0); /* MSB */
    raw_spin_unlock(&raw mut i8253_lock);
    0
}

/*
 * Program the next event in oneshot mode
 *
 * Delta is given in PIT ticks
 */
unsafe fn pit_next_event(delta: c_ulong, evt: *mut clock_event_device) -> c_int {
    raw_spin_lock(&raw mut i8253_lock);
    outb_p(delta & 0xff, PIT_CH0); /* LSB */
    outb_p(delta >> 8, PIT_CH0); /* MSB */
    raw_spin_unlock(&raw mut i8253_lock);
    0
}

/*
 * On UP the PIT can serve all of the possible timer functions. On SMP systems
 * it can be solely used for the global tick.
 */
pub static mut i8253_clockevent: clock_event_device = clock_event_device {
    name: b"pit\0".as_ptr() as *const c_char,
    features: CLOCK_EVT_FEAT_PERIODIC,
    set_state_shutdown: Some(pit_shutdown),
    set_state_periodic: Some(pit_set_periodic),
    set_next_event: Some(pit_next_event),
};

/*
 * Initialize the conversion factor and the min/max deltas of the clock event
 * structure and register the clock event source with the framework.
 */
pub unsafe fn clockevent_i8253_init(oneshot: bool) {
    if oneshot {
        i8253_clockevent.features |= CLOCK_EVT_FEAT_ONESHOT;
        i8253_clockevent.set_state_oneshot = Some(pit_set_oneshot);
    }
    /*
     * Start pit with the boot cpu mask. x86 might make it global
     * when it is used as broadcast device later.
     */
    i8253_clockevent.cpumask = cpumask_of(smp_processor_id());
    clockevents_config_and_register(&raw mut i8253_clockevent, PIT_TICK_RATE, 0xF, 0x7FFF);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
