// SPDX-License-Identifier: GPL-2.0-or-later
//
// Dependencies supplied by the corresponding kernel networking modules:
// linux/cache.h, linux/jiffies.h, linux/list.h, net/aligned_data.h,
// net/hotdata.h, net/ip.h, and net/proto_memory.h.

#[no_mangle]
pub static mut net_hotdata: net_hotdata = net_hotdata {
    // LIST_HEAD_INIT(net_hotdata.offload_base)
    offload_base: LIST_HEAD_INIT,
    gro_normal_batch: 8,

    netdev_budget: 300,
    /* Must be at least 2 jiffes to guarantee 1 jiffy timeout */
    netdev_budget_usecs: 2 * USEC_PER_SEC / HZ,

    tstamp_prequeue: 1,
    max_backlog: 1000,
    qdisc_max_burst: 1000,
    dev_tx_weight: 64,
    dev_rx_weight: 64,
    sysctl_max_skb_frags: MAX_SKB_FRAGS,
    sysctl_skb_defer_max: 128,
    sysctl_mem_pcpu_rsv: SK_MEMORY_PCPU_RESERVE,
};

// EXPORT_SYMBOL(net_hotdata);

#[no_mangle]
pub static mut net_aligned_data: net_aligned_data = net_aligned_data {};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
