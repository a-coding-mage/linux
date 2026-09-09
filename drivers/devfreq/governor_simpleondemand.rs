// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/drivers/devfreq/governor_simpleondemand.c
 *
 *  Copyright (C) 2011 Samsung Electronics
 *\tMyungJoo Ham <myungjoo.ham@samsung.com>
 */

// C header dependencies are supplied by the surrounding kernel translation.

/* Default constants for DevFreq-Simple-Ondemand (DFSO) */
const DFSO_UPTHRESHOLD: u32 = 90;
const DFSO_DOWNDIFFERENTIAL: u32 = 5;

unsafe fn devfreq_simple_ondemand_func(
    df: *mut devfreq,
    freq: *mut c_ulong,
) -> c_int {
    let mut err: c_int;
    let stat: *mut devfreq_dev_status;
    let mut a: u64;
    let mut b: u64;
    let mut dfso_upthreshold: u32 = DFSO_UPTHRESHOLD;
    let mut dfso_downdifferential: u32 = DFSO_DOWNDIFFERENTIAL;
    let data: *mut devfreq_simple_ondemand_data = (*df).data;

    err = devfreq_update_stats(df);
    if err != 0 {
        return err;
    }

    stat = &mut (*df).last_status;

    if !data.is_null() {
        if (*data).upthreshold != 0 {
            dfso_upthreshold = (*data).upthreshold;
        }
        if (*data).downdifferential != 0 {
            dfso_downdifferential = (*data).downdifferential;
        }
    }
    if dfso_upthreshold > 100 || dfso_upthreshold < dfso_downdifferential {
        return -EINVAL;
    }

    /* Assume MAX if it is going to be divided by zero */
    if (*stat).total_time == 0 {
        *freq = DEVFREQ_MAX_FREQ;
        return 0;
    }

    /* Prevent overflow */
    if (*stat).busy_time >= (1u << 24) || (*stat).total_time >= (1u << 24) {
        (*stat).busy_time >>= 7;
        (*stat).total_time >>= 7;
    }

    /* Set MAX if it's busy enough */
    if (*stat).busy_time * 100 > (*stat).total_time * (dfso_upthreshold as u64) {
        *freq = DEVFREQ_MAX_FREQ;
        return 0;
    }

    /* Set MAX if we do not know the initial frequency */
    if (*stat).current_frequency == 0 {
        *freq = DEVFREQ_MAX_FREQ;
        return 0;
    }

    /* Keep the current frequency */
    if (*stat).busy_time * 100
        > (*stat).total_time * (dfso_upthreshold - dfso_downdifferential) as u64
    {
        *freq = (*stat).current_frequency;
        return 0;
    }

    /* Set the desired frequency based on the load */
    a = (*stat).busy_time;
    a = a.wrapping_mul((*stat).current_frequency as u64);
    b = div_u64(a, (*stat).total_time);
    b = b.wrapping_mul(100);
    b = div_u64(
        b,
        (dfso_upthreshold - dfso_downdifferential / 2) as u64,
    );
    *freq = b as c_ulong;

    0
}

unsafe fn devfreq_simple_ondemand_handler(
    devfreq: *mut devfreq,
    event: c_uint,
    data: *mut c_void,
) -> c_int {
    match event {
        DEVFREQ_GOV_START => {
            devfreq_monitor_start(devfreq);
        }
        DEVFREQ_GOV_STOP => {
            devfreq_monitor_stop(devfreq);
        }
        DEVFREQ_GOV_UPDATE_INTERVAL => {
            devfreq_update_interval(devfreq, data as *mut c_uint);
        }
        DEVFREQ_GOV_SUSPEND => {
            devfreq_monitor_suspend(devfreq);
        }
        DEVFREQ_GOV_RESUME => {
            devfreq_monitor_resume(devfreq);
        }
        _ => {}
    }

    0
}

static mut devfreq_simple_ondemand: devfreq_governor = devfreq_governor {
    name: DEVFREQ_GOV_SIMPLE_ONDEMAND,
    attrs: DEVFREQ_GOV_ATTR_POLLING_INTERVAL | DEVFREQ_GOV_ATTR_TIMER,
    get_target_freq: Some(devfreq_simple_ondemand_func),
    event_handler: Some(devfreq_simple_ondemand_handler),
};

unsafe fn devfreq_simple_ondemand_init() -> c_int {
    devfreq_add_governor(&raw mut devfreq_simple_ondemand)
}

// subsys_initcall(devfreq_simple_ondemand_init);

unsafe fn devfreq_simple_ondemand_exit() {
    let ret: c_int;

    ret = devfreq_remove_governor(&raw mut devfreq_simple_ondemand);
    if ret != 0 {
        pr_err!("%s: failed remove governor %d\n", "devfreq_simple_ondemand_exit", ret);
    }

    return;
}

// module_exit(devfreq_simple_ondemand_exit);
// MODULE_DESCRIPTION("DEVFREQ Simple On-demand governor");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
