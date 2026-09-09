// SPDX-License-Identifier: GPL-2.0-or-later
/* rxrpc network namespace handling.
 *
 * Copyright (C) 2017 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependency declarations and kernel-provided symbols are supplied by other
// translation units.

pub static mut rxrpc_net_id: libc::c_uint = 0;

unsafe fn rxrpc_service_conn_reap_timeout(timer: *mut timer_list) {
    let rxnet: *mut rxrpc_net = container_of_service_conn_reap_timer(timer);

    if (*rxnet).live {
        rxrpc_queue_work(&mut (*rxnet).service_conn_reaper);
    }
}

unsafe fn rxrpc_peer_keepalive_timeout(timer: *mut timer_list) {
    let rxnet: *mut rxrpc_net = container_of_peer_keepalive_timer(timer);

    if (*rxnet).live {
        rxrpc_queue_work(&mut (*rxnet).peer_keepalive_work);
    }
}

/*
 * Initialise a per-network namespace record.
 */
unsafe fn rxrpc_init_net(net: *mut net) -> libc::c_int {
    let rxnet: *mut rxrpc_net = rxrpc_net(net);
    let mut ret: libc::c_int;
    let mut i: usize;

    (*rxnet).live = true;
    get_random_bytes(
        &mut (*rxnet).epoch as *mut _ as *mut libc::c_void,
        core::mem::size_of_val(&(*rxnet).epoch),
    );
    (*rxnet).epoch |= RXRPC_RANDOM_EPOCH;

    INIT_LIST_HEAD(&mut (*rxnet).calls);
    spin_lock_init(&mut (*rxnet).call_lock);
    atomic_set(&mut (*rxnet).nr_calls, 1);

    atomic_set(&mut (*rxnet).nr_conns, 1);
    INIT_LIST_HEAD(&mut (*rxnet).bundle_proc_list);
    INIT_LIST_HEAD(&mut (*rxnet).conn_proc_list);
    INIT_LIST_HEAD(&mut (*rxnet).service_conns);
    rwlock_init(&mut (*rxnet).conn_lock);
    INIT_WORK(&mut (*rxnet).service_conn_reaper, rxrpc_service_connection_reaper);
    timer_setup(
        &mut (*rxnet).service_conn_reap_timer,
        rxrpc_service_conn_reap_timeout,
        0,
    );

    atomic_set(&mut (*rxnet).nr_client_conns, 0);

    INIT_HLIST_HEAD(&mut (*rxnet).local_endpoints);
    mutex_init(&mut (*rxnet).local_mutex);

    hash_init(&mut (*rxnet).peer_hash);
    spin_lock_init(&mut (*rxnet).peer_hash_lock);
    i = 0;
    while i < ARRAY_SIZE(&(*rxnet).peer_keepalive) {
        INIT_LIST_HEAD(&mut (*rxnet).peer_keepalive[i]);
        i += 1;
    }
    INIT_LIST_HEAD(&mut (*rxnet).peer_keepalive_new);
    timer_setup(
        &mut (*rxnet).peer_keepalive_timer,
        rxrpc_peer_keepalive_timeout,
        0,
    );
    INIT_WORK(&mut (*rxnet).peer_keepalive_work, rxrpc_peer_keepalive_worker);
    (*rxnet).peer_keepalive_base = ktime_get_seconds();

    ret = -ENOMEM;
    (*rxnet).proc_net = proc_net_mkdir(net, "rxrpc", (*net).proc_net);
    if (*rxnet).proc_net.is_null() {
        (*rxnet).live = false;
        return ret;
    }

    proc_create_net(
        "calls", 0o444, (*rxnet).proc_net, &rxrpc_call_seq_ops,
        core::mem::size_of::<seq_net_private>(),
    );
    proc_create_net(
        "conns", 0o444, (*rxnet).proc_net, &rxrpc_connection_seq_ops,
        core::mem::size_of::<seq_net_private>(),
    );
    proc_create_net(
        "bundles", 0o444, (*rxnet).proc_net, &rxrpc_bundle_seq_ops,
        core::mem::size_of::<seq_net_private>(),
    );
    proc_create_net(
        "peers", 0o444, (*rxnet).proc_net, &rxrpc_peer_seq_ops,
        core::mem::size_of::<seq_net_private>(),
    );
    proc_create_net(
        "locals", 0o444, (*rxnet).proc_net, &rxrpc_local_seq_ops,
        core::mem::size_of::<seq_net_private>(),
    );
    proc_create_net_single_write(
        "stats", S_IFREG | 0o644, (*rxnet).proc_net,
        rxrpc_stats_show, rxrpc_stats_clear, core::ptr::null_mut(),
    );
    0
}

/*
 * Clean up a per-network namespace record.
 */
unsafe fn rxrpc_exit_net(net: *mut net) {
    let rxnet: *mut rxrpc_net = rxrpc_net(net);

    (*rxnet).live = false;
    timer_delete_sync(&mut (*rxnet).peer_keepalive_timer);
    cancel_work_sync(&mut (*rxnet).peer_keepalive_work);
    /* Remove the timer again as the worker may have restarted it. */
    timer_delete_sync(&mut (*rxnet).peer_keepalive_timer);
    rxrpc_destroy_all_calls(rxnet);
    rxrpc_destroy_all_connections(rxnet);
    rxrpc_destroy_all_peers(rxnet);
    rxrpc_destroy_all_locals(rxnet);
    proc_remove((*rxnet).proc_net);
}

pub static mut rxrpc_net_ops: pernet_operations = pernet_operations {
    init: Some(rxrpc_init_net),
    exit: Some(rxrpc_exit_net),
    id: unsafe { &mut rxrpc_net_id },
    size: core::mem::size_of::<rxrpc_net>(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
