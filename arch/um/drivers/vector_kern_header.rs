/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 2007 Jeff Dike (jdike@{addtoit,linux.intel.com})
 */

// C dependencies supplied by other translation units/headers:
// linux/netdevice.h, linux/platform_device.h, linux/skbuff.h, linux/socket.h,
// linux/list.h, linux/ctype.h, linux/workqueue.h, linux/interrupt.h,
// asm/atomic.h, and vector_user.h.

/* Queue structure specially adapted for multiple enqueue/dequeue
 * in a mmsgrecv/mmsgsend context
 */

/* Dequeue method */

pub const QUEUE_SENDMSG: i32 = 0;
pub const QUEUE_SENDMMSG: i32 = 1;

pub const VECTOR_RX: i32 = 1;
pub const VECTOR_TX: i32 = 1 << 1;
pub const VECTOR_BPF: i32 = 1 << 2;
pub const VECTOR_QDISC_BYPASS: i32 = 1 << 3;
pub const VECTOR_BPF_FLASH: i32 = 1 << 4;

pub const ETH_MAX_PACKET: i32 = 1500;
pub const ETH_HEADER_OTHER: i32 = 32; /* just in case someone decides to go mad on QnQ */

pub const MAX_FILTER_PROG: i32 = 2 << 16;

#[repr(C)]
pub struct vector_queue {
    pub mmsg_vector: *mut mmsghdr,
    pub skbuff_vector: *mut *mut core::ffi::c_void,
    /* backlink to device which owns us */
    pub dev: *mut net_device,
    pub head_lock: spinlock_t,
    pub tail_lock: spinlock_t,
    pub queue_depth: atomic_t,
    pub head: i32,
    pub tail: i32,
    pub max_depth: i32,
    pub max_iov_frags: i32,
    pub options: i16,
}

#[repr(C)]
pub struct vector_estats {
    pub rx_queue_max: u64,
    pub rx_queue_running_average: u64,
    pub tx_queue_max: u64,
    pub tx_queue_running_average: u64,
    pub rx_encaps_errors: u64,
    pub tx_timeout_count: u64,
    pub tx_restart_queue: u64,
    pub tx_kicks: u64,
    pub tx_flow_control_xon: u64,
    pub tx_flow_control_xoff: u64,
    pub rx_csum_offload_good: u64,
    pub rx_csum_offload_errors: u64,
    pub sg_ok: u64,
    pub sg_linearized: u64,
}

pub const VERIFY_HEADER_NOK: i32 = -1;
pub const VERIFY_HEADER_OK: i32 = 0;
pub const VERIFY_CSUM_OK: i32 = 1;

#[repr(C)]
pub struct vector_private {
    pub list: list_head,
    pub dev: *mut net_device,
    pub napi: napi_struct,

    pub unit: i32,

    /* Timeout timer in TX */
    pub tl: timer_list,

    /* Scheduled "remove device" work */
    pub reset_tx: work_struct,
    pub fds: *mut vector_fds,

    pub rx_queue: *mut vector_queue,
    pub tx_queue: *mut vector_queue,

    pub rx_irq: i32,
    pub tx_irq: i32,

    pub parsed: *mut arglist,

    pub transport_data: *mut core::ffi::c_void, /* transport specific params if needed */

    pub max_packet: i32,
    pub req_size: i32, /* different from max packet - used for TSO */
    pub headroom: i32,

    pub options: i32,

    /* remote address if any - some transports will leave this as null */

    pub header_size: i32,
    pub rx_header_size: i32,
    pub coalesce: i32,

    pub header_rxbuffer: *mut core::ffi::c_void,
    pub header_txbuffer: *mut core::ffi::c_void,

    pub form_header: Option<unsafe extern "C" fn(
        header: *mut u8,
        skb: *mut sk_buff,
        vp: *mut vector_private,
    ) -> i32>,
    pub verify_header: Option<unsafe extern "C" fn(
        header: *mut u8,
        skb: *mut sk_buff,
        vp: *mut vector_private,
    ) -> i32>,

    pub stats_lock: spinlock_t,

    pub rexmit_scheduled: bool,
    pub opened: bool,
    pub in_write_poll: bool,
    pub in_error: bool,

    /* guest allowed to use ethtool flash to load bpf */
    pub bpf_via_flash: bool,

    /* ethtool stats */

    pub estats: vector_estats,
    pub bpf: *mut sock_fprog,

    pub user: [core::ffi::c_char; 0],
}

unsafe extern "C" {
    pub fn build_transport_data(vp: *mut vector_private) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
