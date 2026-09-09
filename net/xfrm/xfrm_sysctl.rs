// SPDX-License-Identifier: GPL-2.0
//
// Dependencies are supplied by the corresponding kernel translation units.

unsafe fn __xfrm_sysctl_init(net: *mut net) {
    (*net).xfrm.sysctl_aevent_etime = XFRM_AE_ETIME;
    (*net).xfrm.sysctl_aevent_rseqth = XFRM_AE_SEQT_SIZE;
    (*net).xfrm.sysctl_larval_drop = 1;
    (*net).xfrm.sysctl_acq_expires = 30;
}

#[cfg(CONFIG_SYSCTL)]
static XFRM_TABLE: [ctl_table; 4] = [
    ctl_table {
        procname: "xfrm_aevent_etime",
        maxlen: core::mem::size_of::<u32>(),
        mode: 0o644,
        proc_handler: Some(proc_douintvec),
        ..unsafe { core::mem::zeroed() }
    },
    ctl_table {
        procname: "xfrm_aevent_rseqth",
        maxlen: core::mem::size_of::<u32>(),
        mode: 0o644,
        proc_handler: Some(proc_douintvec),
        ..unsafe { core::mem::zeroed() }
    },
    ctl_table {
        procname: "xfrm_larval_drop",
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        ..unsafe { core::mem::zeroed() }
    },
    ctl_table {
        procname: "xfrm_acq_expires",
        maxlen: core::mem::size_of::<i32>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        ..unsafe { core::mem::zeroed() }
    },
];

#[cfg(CONFIG_SYSCTL)]
unsafe fn xfrm_sysctl_init(net: *mut net) -> i32 {
    let mut table: *mut ctl_table;
    let mut table_size: usize = XFRM_TABLE.len();

    __xfrm_sysctl_init(net);

    table = kmemdup(
        XFRM_TABLE.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&XFRM_TABLE),
        GFP_KERNEL,
    ) as *mut ctl_table;
    if table.is_null() {
        return -ENOMEM;
    }
    (*table.add(0)).data = core::ptr::addr_of_mut!((*net).xfrm.sysctl_aevent_etime) as *mut _;
    (*table.add(1)).data = core::ptr::addr_of_mut!((*net).xfrm.sysctl_aevent_rseqth) as *mut _;
    (*table.add(2)).data = core::ptr::addr_of_mut!((*net).xfrm.sysctl_larval_drop) as *mut _;
    (*table.add(3)).data = core::ptr::addr_of_mut!((*net).xfrm.sysctl_acq_expires) as *mut _;

    // Don't export sysctls to unprivileged users
    if (*net).user_ns != core::ptr::addr_of_mut!(init_user_ns) {
        table_size = 0;
    }

    (*net).xfrm.sysctl_hdr = register_net_sysctl_sz(net, "net/core", table, table_size);
    if (*net).xfrm.sysctl_hdr.is_null() {
        kfree(table as *mut core::ffi::c_void);
        return -ENOMEM;
    }
    0
}

#[cfg(CONFIG_SYSCTL)]
unsafe fn xfrm_sysctl_fini(net: *mut net) {
    let table: *const ctl_table = (*net).xfrm.sysctl_hdr.ctl_table_arg;
    unregister_net_sysctl_table((*net).xfrm.sysctl_hdr);
    kfree(table as *mut core::ffi::c_void);
}

#[cfg(not(CONFIG_SYSCTL))]
unsafe fn xfrm_sysctl_init(net: *mut net) -> i32 {
    __xfrm_sysctl_init(net);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
