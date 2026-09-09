// SPDX-License-Identifier: GPL-2.0
/* -*- linux-c -*-
 * sysctl_net_x25.c: sysctl interface to net X.25 subsystem.
 *
 * Begun April 1, 1996, Mike Shaver.
 * Added /proc/sys/net/x25 directory entry (empty =) ). [MS]
 */

// Dependencies supplied by the surrounding kernel translation.

static mut MIN_TIMER: [i32; 1] = [1 * HZ];
static mut MAX_TIMER: [i32; 1] = [300 * HZ];

static mut x25_table_header: *mut ctl_table_header = core::ptr::null_mut();

static mut x25_table: [ctl_table; 6] = [
    ctl_table {
        procname: "restart_request_timeout",
        data: &raw mut sysctl_x25_restart_request_timeout as *mut _,
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: &raw mut MIN_TIMER as *mut _,
        extra2: &raw mut MAX_TIMER as *mut _,
    },
    ctl_table {
        procname: "call_request_timeout",
        data: &raw mut sysctl_x25_call_request_timeout as *mut _,
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: &raw mut MIN_TIMER as *mut _,
        extra2: &raw mut MAX_TIMER as *mut _,
    },
    ctl_table {
        procname: "reset_request_timeout",
        data: &raw mut sysctl_x25_reset_request_timeout as *mut _,
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: &raw mut MIN_TIMER as *mut _,
        extra2: &raw mut MAX_TIMER as *mut _,
    },
    ctl_table {
        procname: "clear_request_timeout",
        data: &raw mut sysctl_x25_clear_request_timeout as *mut _,
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: &raw mut MIN_TIMER as *mut _,
        extra2: &raw mut MAX_TIMER as *mut _,
    },
    ctl_table {
        procname: "acknowledgement_hold_back_timeout",
        data: &raw mut sysctl_x25_ack_holdback_timeout as *mut _,
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: &raw mut MIN_TIMER as *mut _,
        extra2: &raw mut MAX_TIMER as *mut _,
    },
    ctl_table {
        procname: "x25_forward",
        data: &raw mut sysctl_x25_forward as *mut _,
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        extra1: core::ptr::null_mut(),
        extra2: core::ptr::null_mut(),
    },
];

pub unsafe extern "C" fn x25_register_sysctl() -> i32 {
    x25_table_header = register_net_sysctl(&raw mut init_net, "net/x25", x25_table.as_mut_ptr());
    if x25_table_header.is_null() {
        return -ENOMEM;
    }
    0
}

pub unsafe extern "C" fn x25_unregister_sysctl() {
    unregister_net_sysctl_table(x25_table_header);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
