// SPDX-License-Identifier: GPL-2.0-only
/*
 * hangcheck-timer.c
 *
 * Driver for a little io fencing timer.
 *
 * Copyright (C) 2002, 2003 Oracle.  All rights reserved.
 *
 * Author: Joel Becker <joel.becker@oracle.com>
 */

/*
 * The hangcheck-timer driver uses the TSC to catch delays that
 * jiffies does not notice.  A timer is set.  When the timer fires, it
 * checks whether it was delayed and if that delay exceeds a given
 * margin of error.  The hangcheck_tick module parameter takes the timer
 * duration in seconds.  The hangcheck_margin parameter defines the
 * margin of error, in seconds.  The defaults are 60 seconds for the
 * timer and 180 seconds for the margin of error.  IOW, a timer is set
 * for 60 seconds.  When the timer fires, the callback checks the
 * actual duration that the timer waited.  If the duration exceeds the
 * allotted time and margin (here 60 + 180, or 240 seconds), the
 * machine is restarted.  A healthy machine will have the duration match
 * the expected timeout very closely.
 */

// Linux kernel headers provide the declarations used below.

const VERSION_STR: &str = "0.9.1";

const DEFAULT_IOFENCE_MARGIN: i32 = 60; // Default fudge factor, in seconds
const DEFAULT_IOFENCE_TICK: i32 = 180; // Default timer timeout, in seconds

static mut hangcheck_tick: i32 = DEFAULT_IOFENCE_TICK;
static mut hangcheck_margin: i32 = DEFAULT_IOFENCE_MARGIN;
static mut hangcheck_reboot: i32 = 0; // Defaults to not reboot
static mut hangcheck_dump_tasks: i32 = 0; // Defaults to not dumping SysRQ T

// options - modular
// module_param(hangcheck_tick, int, 0);
// MODULE_PARM_DESC(hangcheck_tick, "Timer delay.");
// module_param(hangcheck_margin, int, 0);
// MODULE_PARM_DESC(hangcheck_margin, "If the hangcheck timer has been delayed more than hangcheck_margin seconds, the driver will fire.");
// module_param(hangcheck_reboot, int, 0);
// MODULE_PARM_DESC(hangcheck_reboot, "If nonzero, the machine will reboot when the timer margin is exceeded.");
// module_param(hangcheck_dump_tasks, int, 0);
// MODULE_PARM_DESC(hangcheck_dump_tasks, "If nonzero, the machine will dump the system task state when the timer margin is exceeded.");

// MODULE_AUTHOR("Oracle");
// MODULE_DESCRIPTION("Hangcheck-timer detects when the system has gone out to lunch past a certain margin.");
// MODULE_LICENSE("GPL");
// MODULE_VERSION(VERSION_STR);

/* options - nonmodular */
// #ifndef MODULE

unsafe extern "C" {
    fn get_option(str: *mut *mut u8, int: *mut i32) -> i32;
}

unsafe fn hangcheck_parse_tick(str_: *mut u8) -> i32 {
    let mut par: i32 = 0;
    if get_option(&mut (str_ as *mut u8), &mut par) != 0 {
        hangcheck_tick = par;
    }
    1
}

unsafe fn hangcheck_parse_margin(str_: *mut u8) -> i32 {
    let mut par: i32 = 0;
    if get_option(&mut (str_ as *mut u8), &mut par) != 0 {
        hangcheck_margin = par;
    }
    1
}

unsafe fn hangcheck_parse_reboot(str_: *mut u8) -> i32 {
    let mut par: i32 = 0;
    if get_option(&mut (str_ as *mut u8), &mut par) != 0 {
        hangcheck_reboot = par;
    }
    1
}

unsafe fn hangcheck_parse_dump_tasks(str_: *mut u8) -> i32 {
    let mut par: i32 = 0;
    if get_option(&mut (str_ as *mut u8), &mut par) != 0 {
        hangcheck_dump_tasks = par;
    }
    1
}

// __setup("hcheck_tick", hangcheck_parse_tick);
// __setup("hcheck_margin", hangcheck_parse_margin);
// __setup("hcheck_reboot", hangcheck_parse_reboot);
// __setup("hcheck_dump_tasks", hangcheck_parse_dump_tasks);
// #endif /* not MODULE */

const TIMER_FREQ: u64 = 1_000_000_000u64;

/* Last time scheduled */
static mut hangcheck_tsc: u64 = 0;
static mut hangcheck_tsc_margin: u64 = 0;

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

extern "C" {
    fn ktime_get_ns() -> u64;
    fn mod_timer(timer: *mut timer_list, expires: usize) -> i32;
    fn timer_delete_sync(timer: *mut timer_list) -> i32;
    fn emergency_restart();
    static mut jiffies: usize;
    static HZ: usize;
}

static mut hangcheck_ticktock: timer_list = timer_list { _private: [] };

unsafe extern "C" fn hangcheck_fire(_unused: *mut timer_list) {
    let cur_tsc: u64 = ktime_get_ns();
    let tsc_diff: u64;

    if cur_tsc > hangcheck_tsc {
        tsc_diff = cur_tsc - hangcheck_tsc;
    } else {
        tsc_diff = cur_tsc.wrapping_add(!0u64 - hangcheck_tsc);
    }

    if tsc_diff > hangcheck_tsc_margin {
        if hangcheck_dump_tasks != 0 {
            // pr_crit!("Hangcheck: Task state:\n");
            // #ifdef CONFIG_MAGIC_SYSRQ
            // handle_sysrq('t');
            // #endif /* CONFIG_MAGIC_SYSRQ */
        }
        if hangcheck_reboot != 0 {
            // pr_crit!("Hangcheck: hangcheck is restarting the machine.\n");
            emergency_restart();
        } else {
            // pr_crit!("Hangcheck: hangcheck value past margin!\n");
        }
    }
    // #if 0
    /*
     * Enable to investigate delays in detail
     */
    // pr_debug!("Hangcheck: called %lld ns since last time (%lld ns overshoot)\n",
    //     tsc_diff, tsc_diff - hangcheck_tick*TIMER_FREQ);
    // #endif
    mod_timer(
        &raw mut hangcheck_ticktock,
        jiffies.wrapping_add((hangcheck_tick as usize).wrapping_mul(HZ)),
    );
    hangcheck_tsc = ktime_get_ns();
}

unsafe fn hangcheck_init() -> i32 {
    // pr_debug!("Hangcheck: starting hangcheck timer %s (tick is %d seconds, margin is %d seconds).\n",
    //     VERSION_STR, hangcheck_tick, hangcheck_margin);
    hangcheck_tsc_margin = (hangcheck_margin as u64).wrapping_add(hangcheck_tick as u64);
    hangcheck_tsc_margin = hangcheck_tsc_margin.wrapping_mul(TIMER_FREQ);

    hangcheck_tsc = ktime_get_ns();
    mod_timer(
        &raw mut hangcheck_ticktock,
        jiffies.wrapping_add((hangcheck_tick as usize).wrapping_mul(HZ)),
    );

    0
}

unsafe fn hangcheck_exit() {
    timer_delete_sync(&raw mut hangcheck_ticktock);
    // pr_debug!("Hangcheck: Stopped hangcheck timer.\n");
}

// module_init(hangcheck_init);
// module_exit(hangcheck_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
