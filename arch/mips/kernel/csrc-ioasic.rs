// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  DEC I/O ASIC's counter clocksource
 *
 *  Copyright (C) 2008 Yoichi Yuasa <yuasa@linux-mips.org>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/clocksource.h, linux/sched_clock.h, linux/init.h
// asm/ds1287.h, asm/time.h, asm/dec/ioasic.h, asm/dec/ioasic_addrs.h

unsafe extern "C" {
    fn ioasic_read(reg: u32) -> u64;
    fn ds1287_timer_state() -> u32;
    fn printk(format: *const u8, ...) -> i32;
    fn clocksource_register_hz(cs: *mut clocksource, hz: u32) -> i32;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: u32);
}

#[repr(C)]
pub struct clocksource {
    pub name: *const u8,
    pub read: unsafe extern "C" fn(*mut clocksource) -> u64,
    pub mask: u64,
    pub flags: u32,
    pub rating: u32,
}

const IO_REG_FCTR: u32 = 0; // Supplied by asm/dec/ioasic_addrs.h.
const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1; // Supplied by linux/clocksource.h.
const KERN_INFO: &[u8] = b"<6>\0";
const HZ: i32 = 100; // Supplied by the kernel configuration.
const ENXIO: i32 = 6;

unsafe extern "C" fn dec_ioasic_hpt_read(_cs: *mut clocksource) -> u64 {
    ioasic_read(IO_REG_FCTR)
}

static mut clocksource_dec: clocksource = clocksource {
    name: b"dec-ioasic\0".as_ptr(),
    read: dec_ioasic_hpt_read,
    mask: u64::MAX >> 32,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    rating: 0,
};

unsafe extern "C" fn dec_ioasic_read_sched_clock() -> u64 {
    ioasic_read(IO_REG_FCTR)
}

// __init
pub unsafe extern "C" fn dec_ioasic_clocksource_init() -> i32 {
    let mut freq: u32;
    let mut start: u32;
    let mut end: u32;
    let mut i: i32 = HZ / 8;

    ds1287_timer_state();
    while ds1287_timer_state() == 0 {
        // wait
    }

    start = dec_ioasic_hpt_read(&mut clocksource_dec) as u32;

    while {
        i -= 1;
        i >= 0
    } {
        while ds1287_timer_state() == 0 {
            // wait
        }
    }

    end = dec_ioasic_hpt_read(&mut clocksource_dec) as u32;

    freq = end.wrapping_sub(start).wrapping_mul(8);

    /* An early revision of the I/O ASIC didn't have the counter.  */
    if freq == 0 {
        return -ENXIO;
    }

    let message = b"<6>I/O ASIC clock frequency %dHz\n\0";
    printk(message.as_ptr(), freq as i32);

    clocksource_dec.rating = 200 + freq / 10000000;
    clocksource_register_hz(&mut clocksource_dec, freq);

    sched_clock_register(dec_ioasic_read_sched_clock, 32, freq);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
