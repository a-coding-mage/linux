/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// #include <linux/clockchips.h>
// #include <linux/clocksource.h>
// #include <asm/loongarch.h>

extern "C" {
    pub static mut cpu_clock_freq: u64;
    pub static mut const_clock_freq: u64;

    pub fn save_counter();
    pub fn sync_counter();
}

pub unsafe fn calc_const_freq() -> u32 {
    let mut res: u32;
    let base_freq: u32;
    let cfm: u32;
    let cfd: u32;

    res = read_cpucfg(LOONGARCH_CPUCFG2);
    if (res & CPUCFG2_LLFTP) == 0 {
        return 0;
    }

    base_freq = read_cpucfg(LOONGARCH_CPUCFG4);
    res = read_cpucfg(LOONGARCH_CPUCFG5);
    cfm = res & 0xffff;
    cfd = (res >> 16) & 0xffff;

    if base_freq == 0 || cfm == 0 || cfd == 0 {
        return 0;
    }

    base_freq.wrapping_mul(cfm).wrapping_div(cfd)
}

/*
 * Initialize the calling CPU's timer interrupt as clockevent device
 */
extern "C" {
    pub fn constant_clockevent_init() -> i32;
    pub fn constant_clocksource_init() -> i32;
}

pub unsafe fn clockevent_set_clock(cd: *mut clock_event_device, clock: u32) {
    clockevents_calc_mult_shift(cd, clock, 4);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
