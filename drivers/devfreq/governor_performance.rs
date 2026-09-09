// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/drivers/devfreq/governor_performance.c
 *
 *  Copyright (C) 2011 Samsung Electronics
 *	MyungJoo Ham <myungjoo.ham@samsung.com>
 */

// Dependencies supplied by the surrounding kernel translation.
use crate::{
    devfreq, devfreq_governor, devfreq_add_governor, devfreq_remove_governor,
    update_devfreq, mutex_lock, mutex_unlock, pr_err, DEVFREQ_GOV_PERFORMANCE,
    DEVFREQ_GOV_START, DEVFREQ_MAX_FREQ,
};

unsafe fn devfreq_performance_func(df: *mut devfreq, freq: *mut ::core::ffi::c_ulong) -> i32 {
    /*
     * target callback should be able to get floor value as
     * said in devfreq.h
     */
    let _ = df;
    unsafe {
        *freq = DEVFREQ_MAX_FREQ;
    }
    0
}

unsafe fn devfreq_performance_handler(
    devfreq: *mut devfreq,
    event: ::core::ffi::c_uint,
    data: *mut ::core::ffi::c_void,
) -> i32 {
    let mut ret: i32 = 0;
    let _ = data;

    if event == DEVFREQ_GOV_START {
        unsafe {
            mutex_lock(&mut (*devfreq).lock);
            ret = update_devfreq(devfreq);
            mutex_unlock(&mut (*devfreq).lock);
        }
    }

    ret
}

static mut devfreq_performance: devfreq_governor = devfreq_governor {
    name: DEVFREQ_GOV_PERFORMANCE,
    get_target_freq: Some(devfreq_performance_func),
    event_handler: Some(devfreq_performance_handler),
};

unsafe fn devfreq_performance_init() -> i32 {
    unsafe { devfreq_add_governor(&mut devfreq_performance) }
}

// Equivalent of subsys_initcall(devfreq_performance_init).

unsafe fn devfreq_performance_exit() {
    let ret: i32;

    unsafe {
        ret = devfreq_remove_governor(&mut devfreq_performance);
        if ret != 0 {
            pr_err!("%s: failed remove governor %d\n", "devfreq_performance_exit", ret);
        }
    }

    return;
}

// Equivalent of module_exit(devfreq_performance_exit).
// MODULE_DESCRIPTION("DEVFREQ Performance governor");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
