/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2024-2025 Intel Corporation */

/* Dependency supplied by the surrounding kernel/Rust environment:
 * #include <linux/workqueue.h>
 */

/* Stats */

/**
 * struct libeth_rq_napi_stats - "hot" counters to update in Rx polling loop
 * @packets: received frames counter
 * @bytes: sum of bytes of received frames above
 * @fragments: sum of fragments of received S/G frames
 * @hsplit: number of frames the device performed the header split for
 * @raw: alias to access all the fields as an array
 */
#[repr(C)]
pub struct libeth_rq_napi_stats {
    pub values: libeth_rq_napi_stats_values,
    pub raw: [u32; 0],
}

#[repr(C)]
pub struct libeth_rq_napi_stats_values {
    pub packets: u32,
    pub bytes: u32,
    pub fragments: u32,
    pub hsplit: u32,
}

/**
 * struct libeth_sq_napi_stats - "hot" counters to update in Tx completion loop
 * @packets: completed frames counter
 * @bytes: sum of bytes of completed frames above
 * @raw: alias to access all the fields as an array
 */
#[repr(C)]
pub struct libeth_sq_napi_stats {
    pub values: libeth_sq_napi_stats_values,
    pub raw: [u32; 0],
}

#[repr(C)]
pub struct libeth_sq_napi_stats_values {
    pub packets: u32,
    pub bytes: u32,
}

/**
 * struct libeth_xdpsq_napi_stats - "hot" counters to update in XDP Tx
 *                                    completion loop
 * @packets: completed frames counter
 * @bytes: sum of bytes of completed frames above
 * @fragments: sum of fragments of completed S/G frames
 * @raw: alias to access all the fields as an array
 */
#[repr(C)]
pub struct libeth_xdpsq_napi_stats {
    pub values: libeth_xdpsq_napi_stats_values,
    pub raw: [u32; 0],
}

#[repr(C)]
pub struct libeth_xdpsq_napi_stats_values {
    pub packets: u32,
    pub bytes: u32,
    pub fragments: u32,
}

/* XDP */

/*
 * The following structures should be embedded into driver's queue structure
 * and passed to the libeth_xdp helpers, never used directly.
 */

/* XDPSQ sharing */

/**
 * struct libeth_xdpsq_lock - locking primitive for sharing XDPSQs
 * @lock: spinlock for locking the queue
 * @share: whether this particular queue is shared
 */
#[repr(C)]
pub struct libeth_xdpsq_lock {
    pub lock: spinlock_t,
    pub share: bool,
}

/* XDPSQ clean-up timers */

/**
 * struct libeth_xdpsq_timer - timer for cleaning up XDPSQs w/o interrupts
 * @xdpsq: queue this timer belongs to
 * @lock: lock for the queue
 * @dwork: work performing cleanups
 *
 * XDPSQs not using interrupts but lazy cleaning, i.e. only when there's no
 * space for sending the current queued frame/bulk, must fire up timers to
 * make sure there are no stale buffers to free.
 */
#[repr(C)]
pub struct libeth_xdpsq_timer {
    pub xdpsq: *mut core::ffi::c_void,
    pub lock: *mut libeth_xdpsq_lock,

    pub dwork: delayed_work,
}

/* Rx polling path */

/**
 * struct libeth_xdp_buff_stash - struct for stashing &xdp_buff onto a queue
 * @data: pointer to the start of the frame, xdp_buff.data
 * @headroom: frame headroom, xdp_buff.data - xdp_buff.data_hard_start
 * @len: frame linear space length, xdp_buff.data_end - xdp_buff.data
 * @frame_sz: truesize occupied by the frame, xdp_buff.frame_sz
 * @flags: xdp_buff.flags
 *
 * &xdp_buff is 56 bytes long on x64, &libeth_xdp_buff is 64 bytes. This
 * structure carries only necessary fields to save/restore a partially built
 * frame on the queue structure to finish it during the next NAPI poll.
 */
#[repr(C, align(8))]
pub struct libeth_xdp_buff_stash {
    pub data: *mut core::ffi::c_void,
    pub headroom: u16,
    pub len: u16,

    /* C bitfields: frame_sz occupies bits 0..24 and flags occupies bits 24..32. */
    pub frame_sz_flags: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
