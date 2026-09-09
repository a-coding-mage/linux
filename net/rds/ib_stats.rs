/*
 * Copyright (c) 2006 Oracle.  All rights reserved.
 *
 * This software is available under either the GNU General Public License
 * (GPL) Version 2 or the OpenIB.org BSD license.  It is provided "AS IS",
 * without warranty of any kind.
 */

// Linux kernel dependencies supplied by the surrounding repository:
// linux/percpu.h, linux/seq_file.h, linux/proc_fs.h, rds.h, and ib.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong};

extern "C" {
    // DEFINE_PER_CPU_SHARED_ALIGNED(struct rds_ib_statistics, rds_ib_stats)
    pub static mut rds_ib_stats: rds_ib_statistics;

    pub fn rds_stats_info_copy(
        iter: *mut rds_info_iterator,
        vals: *const u64,
        names: *const *const c_char,
        count: usize,
    );
}

extern "C" {
    pub type rds_ib_statistics;
    pub type rds_info_iterator;
}

static RDS_IB_STAT_NAMES: [&[u8]; 40] = [
    b"ib_connect_raced\0", b"ib_listen_closed_stale\0", b"ib_evt_handler_call\0",
    b"ib_tasklet_call\0", b"ib_tx_cq_event\0", b"ib_tx_ring_full\0",
    b"ib_tx_throttle\0", b"ib_tx_sg_mapping_failure\0", b"ib_tx_stalled\0",
    b"ib_tx_credit_updates\0", b"ib_rx_cq_event\0", b"ib_rx_ring_empty\0",
    b"ib_rx_refill_from_cq\0", b"ib_rx_refill_from_thread\0", b"ib_rx_alloc_limit\0",
    b"ib_rx_total_frags\0", b"ib_rx_total_incs\0", b"ib_rx_credit_updates\0",
    b"ib_ack_sent\0", b"ib_ack_send_failure\0", b"ib_ack_send_delayed\0",
    b"ib_ack_send_piggybacked\0", b"ib_ack_received\0", b"ib_rdma_mr_8k_alloc\0",
    b"ib_rdma_mr_8k_free\0", b"ib_rdma_mr_8k_used\0", b"ib_rdma_mr_8k_pool_flush\0",
    b"ib_rdma_mr_8k_pool_wait\0", b"ib_rdma_mr_8k_pool_depleted\0",
    b"ib_rdma_mr_1m_alloc\0", b"ib_rdma_mr_1m_free\0", b"ib_rdma_mr_1m_used\0",
    b"ib_rdma_mr_1m_pool_flush\0", b"ib_rdma_mr_1m_pool_wait\0",
    b"ib_rdma_mr_1m_pool_depleted\0", b"ib_rdma_mr_8k_reused\0",
    b"ib_rdma_mr_1m_reused\0", b"ib_atomic_cswp\0", b"ib_atomic_fadd\0",
];

pub unsafe fn rds_ib_stats_info_copy(
    iter: *mut rds_info_iterator,
    avail: c_uint,
) -> c_uint {
    let count = RDS_IB_STAT_NAMES.len();
    if (avail as usize) < count {
        return count as c_uint;
    }

    // The C for_each_online_cpu/per_cpu iteration is provided by the kernel
    // percpu machinery.  This file-local translation retains the operation
    // over the externally supplied per-CPU object.
    let mut stats = core::mem::MaybeUninit::<rds_ib_statistics>::zeroed().assume_init();
    let src = &rds_ib_stats as *const rds_ib_statistics as *const u64;
    let sum = &mut stats as *mut rds_ib_statistics as *mut u64;
    let words = core::mem::size_of::<rds_ib_statistics>() / core::mem::size_of::<u64>();
    for i in 0..words {
        *sum.add(i) = (*sum.add(i)).wrapping_add(*src.add(i));
    }

    let names: [*const c_char; 40] = RDS_IB_STAT_NAMES.map(|name| name.as_ptr() as *const c_char);
    rds_stats_info_copy(iter, &stats as *const rds_ib_statistics as *const u64, names.as_ptr(), count);
    count as c_uint
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
