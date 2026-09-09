// SPDX-License-Identifier: GPL-2.0
/*
 * consolidates trace point definitions
 *
 * Copyright (C) 2009 Neil Horman <nhorman@tuxdriver.com>
 */

// Kernel dependencies supplied by other translation units:
// linux/netdevice.h, linux/etherdevice.h, linux/string.h, linux/if_arp.h,
// linux/inetdevice.h, linux/inet.h, linux/interrupt.h, linux/export.h,
// linux/netpoll.h, linux/sched.h, linux/delay.h, linux/rcupdate.h,
// linux/types.h, linux/workqueue.h, linux/netlink.h, linux/net_dropmon.h,
// linux/slab.h, linux/unaligned.h, and asm/bitops.h.

// #define CREATE_TRACE_POINTS
// Trace-event definitions supplied by other translation units:
// trace/events/skb.h, net.h, napi.h, sock.h, udp.h, tcp.h, fib.h, qdisc.h,
// neigh.h.

// The following declarations correspond to EXPORT_TRACEPOINT_SYMBOL_GPL
// invocations in the original source. The tracepoint objects are defined by
// the imported trace-event declarations and exported by the kernel build.

#[cfg(feature = "bridge")]
mod bridge_tracepoint_exports {
    // #include <trace/events/bridge.h>
    // EXPORT_TRACEPOINT_SYMBOL_GPL(br_fdb_add);
    // EXPORT_TRACEPOINT_SYMBOL_GPL(br_fdb_external_learn_add);
    // EXPORT_TRACEPOINT_SYMBOL_GPL(fdb_delete);
    // EXPORT_TRACEPOINT_SYMBOL_GPL(br_fdb_update);
    // EXPORT_TRACEPOINT_SYMBOL_GPL(br_mdb_full);
}

#[cfg(feature = "page_pool")]
mod page_pool_tracepoints {
    // #include <trace/events/page_pool.h>
}

// EXPORT_TRACEPOINT_SYMBOL_GPL(neigh_update);
// EXPORT_TRACEPOINT_SYMBOL_GPL(neigh_update_done);
// EXPORT_TRACEPOINT_SYMBOL_GPL(neigh_timer_handler);
// EXPORT_TRACEPOINT_SYMBOL_GPL(neigh_event_send_done);
// EXPORT_TRACEPOINT_SYMBOL_GPL(neigh_event_send_dead);
// EXPORT_TRACEPOINT_SYMBOL_GPL(neigh_cleanup_and_release);

// EXPORT_TRACEPOINT_SYMBOL_GPL(kfree_skb);

// EXPORT_TRACEPOINT_SYMBOL_GPL(napi_poll);

// EXPORT_TRACEPOINT_SYMBOL_GPL(tcp_send_reset);
// EXPORT_TRACEPOINT_SYMBOL_GPL(tcp_bad_csum);

// EXPORT_TRACEPOINT_SYMBOL_GPL(udp_fail_queue_rcv_skb);

// EXPORT_TRACEPOINT_SYMBOL_GPL(sk_data_ready);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
