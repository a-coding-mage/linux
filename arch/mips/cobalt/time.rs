// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Cobalt time initialization.
 *
 *  Copyright (C) 2007  Yoichi Yuasa <yuasa@linux-mips.org>
 */

// Supplied by the corresponding kernel headers/build environment.
const GT641XX_BASE_CLOCK: u32 = 50000000; /* 50MHz */

unsafe extern "C" {
    fn setup_pit_timer();
    fn gt641xx_set_base_clock(clock: u32);
    fn gt641xx_timer0_state() -> i32;
    fn read_c0_count() -> u32;
    fn printk(fmt: *const u8, ...) -> i32;
    static mut mips_hpt_frequency: u32;
    // HZ is a C preprocessor constant in the original source.
    static HZ: i32;
}

pub unsafe fn plat_time_init() {
    let mut start: u32;
    let mut end: u32;
    let mut i: i32 = HZ / 10;

    setup_pit_timer();

    gt641xx_set_base_clock(GT641XX_BASE_CLOCK);

    /*
     * MIPS counter frequency is measured during a 100msec interval
     * using GT64111 timer0.
     */
    while gt641xx_timer0_state() == 0 {}

    start = read_c0_count();

    while {
        i -= 1;
        i >= 0
    } {
        while gt641xx_timer0_state() == 0 {}
    }

    end = read_c0_count();

    mips_hpt_frequency = end.wrapping_sub(start).wrapping_mul(10);
    printk(
        b"MIPS counter frequency %dHz\n\0".as_ptr(),
        mips_hpt_frequency,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
