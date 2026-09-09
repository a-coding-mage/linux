// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007 Lemote, Inc. & Institute of Computing Technology
 * Author: Fuxin Zhang, zhangfx@lemote.com
 *
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// External declarations supplied by asm/mc146818-time.h, asm/time.h,
// asm/hpet.h, loongson.h, and cs5536/cs5536_mfgpt.h.
extern "C" {
    static mut mips_hpt_frequency: u64;
    static cpu_clock_freq: u64;

    fn setup_mfgpt0_timer();
    fn mc146818_get_cmos_time() -> i64;
}

#[repr(C)]
pub struct timespec64 {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

pub unsafe fn plat_time_init() {
    /* setup mips r4k timer */
    mips_hpt_frequency = cpu_clock_freq / 2;

    setup_mfgpt0_timer();
}

pub unsafe fn read_persistent_clock64(ts: *mut timespec64) {
    (*ts).tv_sec = mc146818_get_cmos_time();
    (*ts).tv_nsec = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
