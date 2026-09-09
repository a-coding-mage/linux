// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 1991, 1992, 1995  Linus Torvalds
 *  Copyright (C) 2000, 2003  Maciej W. Rozycki
 *
 * This file contains the time handling details for PC-style clocks as
 * found in some MIPS systems.
 *
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn read_persistent_clock64(ts: *mut timespec64) {
    let (mut year, mut mon, mut day, mut hour, mut min, mut sec, mut real_year):
        (u32, u32, u32, u32, u32, u32, u32);
    let mut flags: c_ulong;

    spin_lock_irqsave(&raw mut rtc_lock, &mut flags);

    loop {
        sec = CMOS_READ(RTC_SECONDS);
        min = CMOS_READ(RTC_MINUTES);
        hour = CMOS_READ(RTC_HOURS);
        day = CMOS_READ(RTC_DAY_OF_MONTH);
        mon = CMOS_READ(RTC_MONTH);
        year = CMOS_READ(RTC_YEAR);
        /*
         * The PROM will reset the year to either '72 or '73.
         * Therefore we store the real year separately, in one
         * of unused BBU RAM locations.
         */
        real_year = CMOS_READ(RTC_DEC_YEAR);
        if sec == CMOS_READ(RTC_SECONDS) {
            break;
        }
    }

    spin_unlock_irqrestore(&raw mut rtc_lock, flags);

    if (CMOS_READ(RTC_CONTROL) & RTC_DM_BINARY == 0 || RTC_ALWAYS_BCD) {
        sec = bcd2bin(sec);
        min = bcd2bin(min);
        hour = bcd2bin(hour);
        day = bcd2bin(day);
        mon = bcd2bin(mon);
        year = bcd2bin(year);
    }

    year += real_year - 72 + 2000;

    (*ts).tv_sec = mktime64(year, mon, day, hour, min, sec);
    (*ts).tv_nsec = 0;
}

/*
 * In order to set the CMOS clock precisely, update_persistent_clock64 has to
 * be called 500 ms after the second nowtime has started, because when
 * nowtime is written into the registers of the CMOS clock, it will
 * jump to the next second precisely 500 ms later.  Check the Dallas
 * DS1287 data sheet for details.
 */
pub unsafe fn update_persistent_clock64(now: timespec64) -> c_int {
    let nowtime: time64_t = now.tv_sec;
    let mut retval: c_int = 0;
    let (mut real_seconds, mut real_minutes, mut cmos_minutes): (c_int, c_int, c_int);
    let (mut save_control, mut save_freq_select): (u8, u8);

    /* irq are locally disabled here */
    spin_lock(&raw mut rtc_lock);
    /* tell the clock it's being set */
    save_control = CMOS_READ(RTC_CONTROL);
    CMOS_WRITE(save_control | RTC_SET, RTC_CONTROL);

    /* stop and reset prescaler */
    save_freq_select = CMOS_READ(RTC_FREQ_SELECT);
    CMOS_WRITE(save_freq_select | RTC_DIV_RESET2, RTC_FREQ_SELECT);

    cmos_minutes = CMOS_READ(RTC_MINUTES);
    if save_control & RTC_DM_BINARY == 0 || RTC_ALWAYS_BCD {
        cmos_minutes = bcd2bin(cmos_minutes);
    }

    /*
     * since we're only adjusting minutes and seconds,
     * don't interfere with hour overflow. This avoids
     * messing with unknown time zones but requires your
     * RTC not to be off by more than 15 minutes
     */
    real_minutes = div_s64_rem(nowtime, 60, &mut real_seconds);
    if ((abs(real_minutes - cmos_minutes) + 15) / 30) & 1 != 0 {
        real_minutes += 30; /* correct for half hour time zone */
    }
    real_minutes %= 60;

    if abs(real_minutes - cmos_minutes) < 30 {
        if save_control & RTC_DM_BINARY == 0 || RTC_ALWAYS_BCD {
            real_seconds = bin2bcd(real_seconds);
            real_minutes = bin2bcd(real_minutes);
        }
        CMOS_WRITE(real_seconds, RTC_SECONDS);
        CMOS_WRITE(real_minutes, RTC_MINUTES);
    } else {
        printk_once!(KERN_NOTICE, "set_rtc_mmss: can't update from %d to %d\n", cmos_minutes, real_minutes);
        retval = -1;
    }

    /* The following flags have to be released exactly in this order,
     * otherwise the DS1287 will not reset the oscillator and will not
     * update precisely 500 ms later.  You won't find this mentioned
     * in the Dallas Semiconductor data sheets, but who believes data
     * sheets anyway ...                           -- Markus Kuhn
     */
    CMOS_WRITE(save_control, RTC_CONTROL);
    CMOS_WRITE(save_freq_select, RTC_FREQ_SELECT);
    spin_unlock(&raw mut rtc_lock);

    retval
}

pub unsafe fn plat_time_init() {
    let mut ioasic_clock: c_int = 0;
    let (mut start, mut end): (u32, u32);
    let mut i: c_int = HZ / 8;

    /* Set up the rate of periodic DS1287 interrupts. */
    ds1287_set_base_clock(HZ);

    /* On some I/O ASIC systems we have the I/O ASIC's counter.  */
    if IOASIC {
        ioasic_clock = (dec_ioasic_clocksource_init() == 0) as c_int;
    }
    if cpu_has_counter {
        ds1287_timer_state();
        while ds1287_timer_state() == 0 {}

        start = read_c0_count();

        while i != 0 {
            i -= 1;
            while ds1287_timer_state() == 0 {}
        }

        end = read_c0_count();

        mips_hpt_frequency = (end - start) * 8;
        printk!(KERN_INFO, "MIPS counter frequency %dHz\n", mips_hpt_frequency);

        /*
         * All R4k DECstations suffer from the CP0 Count erratum,
         * so we can't use the timer as a clock source, and a clock
         * event both at a time.  An accurate wall clock is more
         * important than a high-precision interval timer so only
         * use the timer as a clock source, and not a clock event
         * if there's no I/O ASIC counter available to serve as a
         * clock source.
         */
        if ioasic_clock == 0 {
            init_r4k_clocksource();
            mips_hpt_frequency = 0;
        }
    }

    ds1287_clockevent_init(dec_interrupt[DEC_IRQ_RTC]);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
