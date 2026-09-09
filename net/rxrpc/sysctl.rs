// SPDX-License-Identifier: GPL-2.0-or-later
/* sysctls for configuring RxRPC operating parameters
 *
 * Copyright (C) 2014 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel/RxRPC translation.

static mut rxrpc_sysctl_reg_table: *mut ctl_table_header = core::ptr::null_mut();
static rxrpc_rx_mtu_min: ::core::ffi::c_uint = 500;
static rxrpc_jumbo_max: ::core::ffi::c_uint = RXRPC_MAX_NR_JUMBO;
static four: ::core::ffi::c_uint = 4;
static max_backlog: ::core::ffi::c_uint = RXRPC_BACKLOG_MAX - 1;
static n_65535: ::core::ffi::c_uint = 65535;
static n_max_acks: ::core::ffi::c_uint = 255;
static one_ms: ::core::ffi::c_ulong = 1;
static max_ms: ::core::ffi::c_ulong = 1000;
static one_jiffy: ::core::ffi::c_ulong = 1;
static max_jiffies: ::core::ffi::c_ulong = MAX_JIFFY_OFFSET;
#[cfg(CONFIG_AF_RXRPC_INJECT_RX_DELAY)]
static max_500: ::core::ffi::c_ulong = 500;

/*
 * RxRPC operating parameters.
 *
 * See Documentation/networking/rxrpc.rst and the variable definitions for more
 * information on the individual parameters.
 */
static mut rxrpc_sysctl_table: [ctl_table; 9] = [
    /* Values measured in milliseconds */
    ctl_table {
        procname: "soft_ack_delay",
        data: unsafe { &mut rxrpc_soft_ack_delay as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_ulong>(),
        mode: 0o644,
        proc_handler: Some(proc_doulongvec_minmax),
        extra1: unsafe { &one_ms as *const _ as *mut ::core::ffi::c_void },
        extra2: unsafe { &max_ms as *const _ as *mut ::core::ffi::c_void },
    },
    ctl_table {
        procname: "idle_ack_delay",
        data: unsafe { &mut rxrpc_idle_ack_delay as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_ulong>(),
        mode: 0o644,
        proc_handler: Some(proc_doulongvec_minmax),
        extra1: unsafe { &one_ms as *const _ as *mut ::core::ffi::c_void },
        extra2: unsafe { &max_ms as *const _ as *mut ::core::ffi::c_void },
    },
    ctl_table {
        procname: "idle_conn_expiry",
        data: unsafe { &mut rxrpc_conn_idle_client_expiry as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_ulong>(),
        mode: 0o644,
        proc_handler: Some(proc_doulongvec_ms_jiffies_minmax),
        extra1: unsafe { &one_jiffy as *const _ as *mut ::core::ffi::c_void },
        extra2: unsafe { &max_jiffies as *const _ as *mut ::core::ffi::c_void },
    },
    ctl_table {
        procname: "idle_conn_fast_expiry",
        data: unsafe { &mut rxrpc_conn_idle_client_fast_expiry as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_ulong>(),
        mode: 0o644,
        proc_handler: Some(proc_doulongvec_ms_jiffies_minmax),
        extra1: unsafe { &one_jiffy as *const _ as *mut ::core::ffi::c_void },
        extra2: unsafe { &max_jiffies as *const _ as *mut ::core::ffi::c_void },
    },
    /* Values used in milliseconds */
    #[cfg(CONFIG_AF_RXRPC_INJECT_RX_DELAY)]
    ctl_table {
        procname: "inject_rx_delay",
        data: unsafe { &mut rxrpc_inject_rx_delay as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_ulong>(),
        mode: 0o644,
        proc_handler: Some(proc_doulongvec_minmax),
        extra1: SYSCTL_LONG_ZERO as *mut ::core::ffi::c_void,
        extra2: unsafe { &max_500 as *const _ as *mut ::core::ffi::c_void },
    },
    /* Non-time values */
    ctl_table {
        procname: "reap_client_conns",
        data: unsafe { &mut rxrpc_reap_client_connections as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_uint>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: SYSCTL_ONE as *mut ::core::ffi::c_void,
        extra2: unsafe { &n_65535 as *const _ as *mut ::core::ffi::c_void },
    },
    ctl_table {
        procname: "max_backlog",
        data: unsafe { &mut rxrpc_max_backlog as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_uint>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: unsafe { &four as *const _ as *mut ::core::ffi::c_void },
        extra2: unsafe { &max_backlog as *const _ as *mut ::core::ffi::c_void },
    },
    ctl_table {
        procname: "rx_window_size",
        data: unsafe { &mut rxrpc_rx_window_size as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_uint>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: SYSCTL_ONE as *mut ::core::ffi::c_void,
        extra2: unsafe { &n_max_acks as *const _ as *mut ::core::ffi::c_void },
    },
    ctl_table {
        procname: "rx_mtu",
        data: unsafe { &mut rxrpc_rx_mtu as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_uint>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: unsafe { &rxrpc_rx_mtu_min as *const _ as *mut ::core::ffi::c_void },
        extra2: unsafe { &n_65535 as *const _ as *mut ::core::ffi::c_void },
    },
    ctl_table {
        procname: "rx_jumbo_max",
        data: unsafe { &mut rxrpc_rx_jumbo_max as *mut _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_uint>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: SYSCTL_ONE as *mut ::core::ffi::c_void,
        extra2: unsafe { &rxrpc_jumbo_max as *const _ as *mut ::core::ffi::c_void },
    },
];

pub unsafe fn rxrpc_sysctl_init() -> ::core::ffi::c_int {
    rxrpc_sysctl_reg_table = register_net_sysctl(
        &mut init_net,
        "net/rxrpc",
        rxrpc_sysctl_table.as_mut_ptr(),
    );
    if rxrpc_sysctl_reg_table.is_null() {
        return -ENOMEM;
    }
    0
}

pub unsafe fn rxrpc_sysctl_exit() {
    if !rxrpc_sysctl_reg_table.is_null() {
        unregister_net_sysctl_table(rxrpc_sysctl_reg_table);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
