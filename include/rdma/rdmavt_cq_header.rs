/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/*
 * Copyright(c) 2016 - 2018 Intel Corporation.
 */

// C dependencies:
// <linux/kthread.h>, <rdma/ib_user_verbs.h>, <rdma/ib_verbs.h>, and
// <rdma/rvt-abi.h> provide the referenced types, constants, and primitives.

/*
 * Define an ib_cq_notify value that is not valid so we know when CQ
 * notifications are armed.
 */
pub const RVT_CQ_NONE: u32 = IB_CQ_NEXT_COMP + 1;

/*
 * Define read macro that apply smp_load_acquire memory barrier
 * when reading indice of circular buffer that mmaped to user space.
 */
#[macro_export]
macro_rules! RDMA_READ_UAPI_ATOMIC {
    ($member:expr) => {
        smp_load_acquire(core::ptr::addr_of!($member.val))
    };
}

/*
 * Define write macro that uses smp_store_release memory barrier
 * when writing indice of circular buffer that mmaped to user space.
 */
#[macro_export]
macro_rules! RDMA_WRITE_UAPI_ATOMIC {
    ($member:expr, $x:expr) => {
        smp_store_release(core::ptr::addr_of_mut!($member.val), $x)
    };
}

/*
 * This structure is used to contain the head pointer, tail pointer,
 * and completion queue entries as a single memory allocation so
 * it can be mmap'ed into user space.
 */
#[repr(C)]
pub struct rvt_k_cq_wc {
    pub head: u32,               /* index of next entry to fill */
    pub tail: u32,               /* index of next ib_poll_cq() entry */
    pub kqueue: [ib_wc; 0],
}

/*
 * The completion queue structure.
 */
#[repr(C)]
pub struct rvt_cq {
    pub ibcq: ib_cq,
    pub comptask: work_struct,
    pub lock: spinlock_t, /* protect changes in this struct */
    pub notify: u8,
    pub triggered: u8,
    pub cq_full: u8,
    pub comp_vector_cpu: i32,
    pub rdi: *mut rvt_dev_info,
    pub queue: *mut rvt_cq_wc,
    pub ip: *mut rvt_mmap_info,
    pub kqueue: *mut rvt_k_cq_wc,
}

pub unsafe fn ibcq_to_rvtcq(ibcq: *mut ib_cq) -> *mut rvt_cq {
    (ibcq as *mut u8).sub(core::mem::offset_of!(rvt_cq, ibcq)) as *mut rvt_cq
}

extern "C" {
    pub fn rvt_cq_enter(cq: *mut rvt_cq, entry: *mut ib_wc, solicited: bool) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
