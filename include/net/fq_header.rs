/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 Qualcomm Atheros, Inc
 *
 * Based on net/sched/sch_fq_codel.c
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/skbuff.h, linux/spinlock.h, and linux/types.h

use core::ffi::c_void;

pub struct fq_tin;

/**
 * struct fq_flow - per traffic flow queue
 *
 * @tin: owner of this flow. Used to manage collisions, i.e. when a packet
 *	hashes to an index which points to a flow that is already owned by a
 *	different tin the packet is destined to. In such case the implementer
 *	must provide a fallback flow
 * @flowchain: can be linked to fq_tin's new_flows or old_flows. Used for DRR++
 *	(deficit round robin) based round robin queuing similar to the one
 *	found in net/sched/sch_fq_codel.c
 * @queue: sk_buff queue to hold packets
 * @backlog: number of bytes pending in the queue. The number of packets can be
 *	found in @queue.qlen
 * @deficit: used for DRR++
 */
#[repr(C)]
pub struct fq_flow {
    pub tin: *mut fq_tin,
    pub flowchain: list_head,
    pub queue: sk_buff_head,
    pub backlog: u32,
    pub deficit: i32,
}

/**
 * struct fq_tin - a logical container of fq_flows
 *
 * Used to group fq_flows into a logical aggregate. DRR++ scheme is used to
 * pull interleaved packets out of the associated flows.
 *
 * @new_flows: linked list of fq_flow
 * @old_flows: linked list of fq_flow
 */
#[repr(C)]
pub struct fq_tin {
    pub new_flows: list_head,
    pub old_flows: list_head,
    pub tin_list: list_head,
    pub default_flow: fq_flow,
    pub backlog_bytes: u32,
    pub backlog_packets: u32,
    pub overlimit: u32,
    pub collisions: u32,
    pub flows: u32,
    pub tx_bytes: u32,
    pub tx_packets: u32,
}

/**
 * struct fq - main container for fair queuing purposes
 *
 * @limit: max number of packets that can be queued across all flows
 * @backlog: number of packets queued across all flows
 */
#[repr(C)]
pub struct fq {
    pub flows: *mut fq_flow,
    pub flows_bitmap: *mut c_ulong,

    pub tin_backlog: list_head,
    pub lock: spinlock_t,
    pub flows_cnt: u32,
    pub limit: u32,
    pub memory_limit: u32,
    pub memory_usage: u32,
    pub quantum: u32,
    pub backlog: u32,
    pub overlimit: u32,
    pub overmemory: u32,
    pub collisions: u32,
}

pub type fq_tin_dequeue_t = unsafe extern "C" fn(
    *mut fq,
    *mut fq_tin,
    *mut fq_flow,
) -> *mut sk_buff;

pub type fq_skb_free_t = unsafe extern "C" fn(
    *mut fq,
    *mut fq_tin,
    *mut fq_flow,
    *mut sk_buff,
);

/* Return %true to filter (drop) the frame. */
pub type fq_skb_filter_t = unsafe extern "C" fn(
    *mut fq,
    *mut fq_tin,
    *mut fq_flow,
    *mut sk_buff,
    *mut c_void,
) -> bool;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
