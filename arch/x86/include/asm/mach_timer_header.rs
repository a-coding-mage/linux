/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Machine specific calibrate_tsc() for generic.
 *  Split out from timer_tsc.c by Osamu Tomita <tomita@cinet.co.jp>
 */
/* ------ Calibrate the TSC -------
 * Return 2^32 * (1 / (TSC clocks per usec)) for do_fast_gettimeoffset().
 * Too much 64-bit arithmetic here to do this cleanly in C, and for
 * accuracy's sake we want to keep the overhead on the CTC speaker (channel 2)
 * output busy loop as low as possible. We avoid reading the CTC registers
 * directly because of the awkward 8-bit access mechanism of the 82C54
 * device.
 */

/* Supplied by the surrounding architecture-specific code. */
extern "C" {
    fn inb(port: u16) -> u8;
    fn inb_p(port: u16) -> u8;
    fn outb(value: u8, port: u16);
    fn outb_p(value: u8, port: u16);
}

pub const CALIBRATE_TIME_MSEC: u32 = 30; /* 30 msecs */
pub const CALIBRATE_LATCH: u32 =
    (PIT_TICK_RATE * CALIBRATE_TIME_MSEC + 1000 / 2) / 1000;

/* Supplied by the surrounding architecture-specific code. */
unsafe extern "C" {
    static PIT_TICK_RATE: u32;
}

pub unsafe fn mach_prepare_counter() {
    /* Set the Gate high, disable speaker */
    outb((inb(0x61) & !0x02) | 0x01, 0x61);

    /*
     * Now let's take care of CTC channel 2
     *
     * Set the Gate high, program CTC channel 2 for mode 0,
     * (interrupt on terminal count mode), binary count,
     * load 5 * LATCH count, (LSB and MSB) to begin countdown.
     *
     * Some devices need a delay here.
     */
    outb(0xb0, 0x43); /* binary, mode 0, LSB/MSB, Ch 2 */
    outb_p((CALIBRATE_LATCH & 0xff) as u8, 0x42); /* LSB of count */
    outb_p((CALIBRATE_LATCH >> 8) as u8, 0x42); /* MSB of count */
}

pub unsafe fn mach_countup(count_p: *mut usize) {
    let mut count: usize = 0;
    loop {
        count = count.wrapping_add(1);
        if (inb_p(0x61) & 0x20) != 0 {
            break;
        }
    }
    *count_p = count;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
