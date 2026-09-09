// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NET4: Sysctl interface to net af_unix subsystem.
 *
 * Authors: Mike Shaver.
 */

// Dependencies supplied by the surrounding kernel translation.

static UNIX_TABLE: [ctl_table; 1] = [ctl_table {
    procname: "max_dgram_qlen",
    data: unsafe { &mut (*(&raw mut init_net)).unx.sysctl_max_dgram_qlen as *mut _ },
    maxlen: core::mem::size_of::<i32>(),
    mode: 0o644,
    proc_handler: Some(proc_dointvec),
}];

unsafe fn unix_table_dup(net: *mut net) -> *mut ctl_table {
    let table: *mut ctl_table = kmemdup(
        UNIX_TABLE.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&UNIX_TABLE),
        GFP_KERNEL,
    ) as *mut ctl_table;
    if table.is_null() {
        return core::ptr::null_mut();
    }

    (*table).data = &mut (*net).unx.sysctl_max_dgram_qlen as *mut _;

    table
}

pub unsafe fn unix_sysctl_register(net: *mut net) -> i32 {
    let table: *const ctl_table;

    if net_eq(net, &raw const init_net as *const net) {
        table = UNIX_TABLE.as_ptr();
    } else {
        table = unix_table_dup(net);
        if table.is_null() {
            return -12;
        }
    }

    (*net).unx.ctl = register_net_sysctl_sz(
        net,
        "net/unix",
        table,
        UNIX_TABLE.len(),
    );
    if (*net).unx.ctl.is_null() {
        if !net_eq(net, &raw const init_net as *const net) {
            kfree(table as *mut core::ffi::c_void);
        }
        return -12;
    }

    0
}

pub unsafe fn unix_sysctl_unregister(net: *mut net) {
    let table: *const ctl_table = (*(*net).unx.ctl).ctl_table_arg;
    unregister_net_sysctl_table((*net).unx.ctl);
    if !net_eq(net, &raw const init_net as *const net) {
        kfree(table as *mut core::ffi::c_void);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
