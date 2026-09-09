/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */

/*
 * This file contains defines, structures, etc. that are used
 * to communicate between kernel and user code.
 */

/* C dependencies: linux/types.h and rdma/ib_user_verbs.h. */
/* RDMA_ATOMIC_UAPI(_type, _name) defaults to struct { _type val; } _name. */

#[repr(C)]
pub struct rvt_wqe_sge {
    pub addr: u64,
    pub length: u32,
    pub lkey: u32,
}

/*
 * This structure is used to contain the head pointer, tail pointer,
 * and completion queue entries as a single memory allocation so
 * it can be mmap'ed into user space.
 */
#[repr(C)]
pub struct rvt_cq_wc {
    /* index of next entry to fill */
    pub head: rvt_cq_wc_head,
    /* index of next ib_poll_cq() entry */
    pub tail: rvt_cq_wc_tail,

    /* these are actually size ibcq.cqe + 1 */
    pub uqueue: [ib_uverbs_wc; 0],
}

#[repr(C)]
pub struct rvt_cq_wc_head {
    pub val: u32,
}

#[repr(C)]
pub struct rvt_cq_wc_tail {
    pub val: u32,
}

/*
 * Receive work request queue entry.
 * The size of the sg_list is determined when the QP (or SRQ) is created
 * and stored in qp->r_rq.max_sge (or srq->rq.max_sge).
 */
#[repr(C)]
pub struct rvt_rwqe {
    pub wr_id: u64,
    pub num_sge: u8,
    pub padding: [u8; 7],
    pub sg_list: [rvt_wqe_sge; 0],
}

/*
 * This structure is used to contain the head pointer, tail pointer,
 * and receive work queue entries as a single memory allocation so
 * it can be mmap'ed into user space.
 * Note that the wq array elements are variable size so you can't
 * just index into the array to get the N'th element;
 * use get_rwqe_ptr() for user space and rvt_get_rwqe_ptr()
 * for kernel space.
 */
#[repr(C)]
pub struct rvt_rwq {
    /* new work requests posted to the head */
    pub head: rvt_rwq_head,
    /* receives pull requests from here. */
    pub tail: rvt_rwq_tail,
    pub wq: [rvt_rwqe; 0],
}

#[repr(C)]
pub struct rvt_rwq_head {
    pub val: u32,
}

#[repr(C)]
pub struct rvt_rwq_tail {
    pub val: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
