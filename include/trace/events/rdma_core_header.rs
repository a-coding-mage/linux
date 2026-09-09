/* SPDX-License-Identifier: GPL-2.0-only */
/* Trace point definitions for core RDMA functions. */

// C dependencies: linux/tracepoint.h and rdma/ib_verbs.h.
// TRACE_SYSTEM is rdma_core.  TRACE_HEADER_MULTI_READ preserves the C
// header's repeated-inclusion behavior.

#[repr(i32)]
pub enum IbPollContext {
    Direct,
    Softirq,
    Workqueue,
    UnboundWorkqueue,
}

pub const IB_POLL_DIRECT: i32 = 0;
pub const IB_POLL_SOFTIRQ: i32 = 1;
pub const IB_POLL_WORKQUEUE: i32 = 2;
pub const IB_POLL_UNBOUND_WORKQUEUE: i32 = 3;

pub const IB_MR_TYPE_MEM_REG: i32 = 0;
pub const IB_MR_TYPE_SG_GAPS: i32 = 1;
pub const IB_MR_TYPE_DM: i32 = 2;
pub const IB_MR_TYPE_USER: i32 = 3;
pub const IB_MR_TYPE_DMA: i32 = 4;
pub const IB_MR_TYPE_INTEGRITY: i32 = 5;

// Equivalent symbolic names used by rdma_show_ib_poll_ctx and
// rdma_show_ib_mr_type in the tracepoint implementation.
pub static IB_POLL_CONTEXT_NAMES: [&str; 4] =
    ["DIRECT", "SOFTIRQ", "WORKQUEUE", "UNBOUND_WORKQUEUE"];
pub static IB_MR_TYPE_NAMES: [&str; 6] =
    ["MEM_REG", "SG_GAPS", "DM", "USER", "DMA", "INTEGRITY"];

#[inline]
pub fn rdma_show_ib_poll_ctx(x: usize) -> &'static str {
    IB_POLL_CONTEXT_NAMES[x]
}

#[inline]
pub fn rdma_show_ib_mr_type(x: usize) -> &'static str {
    IB_MR_TYPE_NAMES[x]
}

// The following repr(C) records are the TP_STRUCT__entry payloads generated
// by the C TRACE_EVENT declarations.  The trace-print formats are retained.

#[repr(C)]
pub struct CqScheduleEntry { pub cq_id: u32 }
#[repr(C)]
pub struct CqRescheduleEntry { pub cq_id: u32 }
#[repr(C)]
pub struct CqProcessEntry { pub cq_id: u32, pub interrupt: bool, pub latency: i64 }
#[repr(C)]
pub struct CqPollEntry { pub cq_id: u32, pub requested: i32, pub rc: i32 }
#[repr(C)]
pub struct CqDrainCompleteEntry { pub cq_id: u32 }
#[repr(C)]
pub struct CqModifyEntry { pub cq_id: u32, pub comps: u32, pub usec: u32 }
#[repr(C)]
pub struct CqAllocEntry {
    pub cq_id: u32, pub comps: i32, pub comp_vector: i32,
    pub poll_ctx: usize,
}
#[repr(C)]
pub struct CqAllocErrorEntry {
    pub rc: i32, pub nr_cqe: i32, pub comp_vector: i32,
    pub poll_ctx: usize,
}
#[repr(C)]
pub struct CqFreeEntry { pub cq_id: u32 }

#[repr(C)]
pub struct MrAllocEntry {
    pub pd_id: u32, pub mr_id: u32, pub max_num_sg: u32,
    pub rc: i32, pub mr_type: usize,
}
#[repr(C)]
pub struct MrIntegAllocEntry {
    pub pd_id: u32, pub mr_id: u32, pub max_num_data_sg: u32,
    pub max_num_meta_sg: u32, pub rc: i32,
}
#[repr(C)]
pub struct MrDeregEntry { pub id: u32 }

// C trace event interfaces (cq_schedule, cq_reschedule, cq_process, cq_poll,
// cq_drain_complete, cq_modify, cq_alloc, cq_alloc_error, cq_free, mr_alloc,
// mr_integ_alloc, and mr_dereg) are represented by the entry records above;
// their TP_PROTO/TP_ARGS and TP_fast_assign fields retain the following
// externally supplied types and operations: ib_cq::res.id, ib_cq::timestamp,
// ib_cq::interrupt, ib_pd::res.id, ib_mr::res.id, ktime_get,
// ktime_sub, ktime_to_us, IS_ERR, and PTR_ERR.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
