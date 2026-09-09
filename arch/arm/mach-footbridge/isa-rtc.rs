// SPDX-License-Identifier: GPL-2.0
/*
 *  arch/arm/mach-footbridge/isa-rtc.c
 *
 *  Copyright (C) 1998 Russell King.
 *  Copyright (C) 1998 Phil Blundell
 *
 * CATS has a real-time clock, though the evaluation board doesn't.
 *
 * Changelog:
 *  21-Mar-1998 RMK Created
 *  27-Aug-1998 PJB CATS support
 *  28-Dec-1998 APH Made leds optional
 *  20-Jan-1999 RMK Started merge of EBSA285, CATS and NetWinder
 *  16-Mar-1999 RMK More support for EBSA285-like machines with RTCs in
 */

const RTC_ALWAYS_BCD: i32 = 0;

// Supplied by the RTC and platform dependencies.
extern "C" {
    fn CMOS_READ(reg: i32) -> i32;
    fn CMOS_WRITE(value: i32, reg: i32);
    fn printk(format: *const u8, ...);
}

// Supplied by the included RTC/platform headers.
extern "C" {
    static RTC_REG_A: i32;
    static RTC_REG_B: i32;
    static RTC_REG_D: i32;
    static RTC_REF_CLCK_32KHZ: i32;
    static KERN_WARNING: u8;
}

pub unsafe fn isa_rtc_init() {
    let reg_d: i32;
    let reg_b: i32;

    /*
     * Probe for the RTC.
     */
    reg_d = CMOS_READ(RTC_REG_D);

    /*
     * make sure the divider is set
     */
    CMOS_WRITE(RTC_REF_CLCK_32KHZ, RTC_REG_A);

    /*
     * Set control reg B
     *   (24 hour mode, update enabled)
     */
    reg_b = CMOS_READ(RTC_REG_B) & 0x7f;
    reg_b |= 2;
    CMOS_WRITE(reg_b, RTC_REG_B);

    if ((CMOS_READ(RTC_REG_A) & 0x7f) == RTC_REF_CLCK_32KHZ
        && CMOS_READ(RTC_REG_B) == reg_b)
    {
        /*
         * We have a RTC.  Check the battery
         */
        if ((reg_d & 0x80) == 0) {
            static WARNING: &[u8] = b"RTC: *** warning: CMOS battery bad\n\0";
            printk(WARNING.as_ptr());
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
