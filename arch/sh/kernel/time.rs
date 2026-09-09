// SPDX-License-Identifier: GPL-2.0
/*
 *  arch/sh/kernel/time.c
 *
 *  Copyright (C) 1999  Tetsuya Okada & Niibe Yutaka
 *  Copyright (C) 2000  Philipp Rumpf <prumpf@tux.org>
 *  Copyright (C) 2002 - 2009  Paul Mundt
 *  Copyright (C) 2002  M. R. Brown  <mrbrown@linux-sh.org>
 */

// Dependencies supplied by the surrounding kernel translation unit.
extern "C" {
    fn sh_early_platform_driver_register_all(class_str: *const core::ffi::c_char);
    fn sh_early_platform_driver_probe(
        class_str: *const core::ffi::c_char,
        nr: i32,
        start: i32,
    );
    fn timer_probe();
    fn clk_init();
    static mut late_time_init: Option<unsafe extern "C" fn()>;
}

unsafe extern "C" fn sh_late_time_init() {
    /*
     * Make sure all compiled-in early timers register themselves.
     *
     * Run probe() for two "earlytimer" devices, these will be the
     * clockevents and clocksource devices respectively. In the event
     * that only a clockevents device is available, we -ENODEV on the
     * clocksource and the jiffies clocksource is used transparently
     * instead. No error handling is necessary here.
     */
    unsafe {
        sh_early_platform_driver_register_all(b"earlytimer\0".as_ptr() as *const core::ffi::c_char);
        sh_early_platform_driver_probe(b"earlytimer\0".as_ptr() as *const core::ffi::c_char, 2, 0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn time_init() {
    unsafe {
        timer_probe();

        clk_init();

        late_time_init = Some(sh_late_time_init);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
