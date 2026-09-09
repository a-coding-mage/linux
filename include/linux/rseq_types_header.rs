/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/irq_work_types.h, linux/types.h, linux/workqueue_types.h
// Configuration conditions CONFIG_RSEQ, CONFIG_RSEQ_SLICE_EXTENSION, and
// CONFIG_SCHED_MM_CID are preserved below as cfg attributes/comments.

#[cfg(feature = "CONFIG_RSEQ")]
pub struct rseq;

#[cfg(feature = "CONFIG_RSEQ")]
pub const RSEQ_HAS_RSEQ_VERSION_MASK: u32 = 0xff;

#[cfg(feature = "CONFIG_RSEQ")]
#[repr(C)]
pub union rseq_event {
    pub all: u64,
    pub fields: rseq_event_fields,
}

#[cfg(feature = "CONFIG_RSEQ")]
#[repr(C)]
pub struct rseq_event_fields {
    pub events: rseq_event_events,
    pub has_rseq: u8,
    pub __pad: u8,
    pub error: rseq_event_error,
}

#[cfg(feature = "CONFIG_RSEQ")]
#[repr(C)]
pub union rseq_event_events {
    pub events: u32,
    pub flags: rseq_event_flags,
}

#[cfg(feature = "CONFIG_RSEQ")]
#[repr(C)]
pub struct rseq_event_flags {
    pub sched_switch: u8,
    pub ids_changed: u8,
    pub user_irq: u8,
}

#[cfg(feature = "CONFIG_RSEQ")]
#[repr(C)]
pub union rseq_event_error {
    pub error: u16,
    pub flags: rseq_event_error_flags,
}

#[cfg(feature = "CONFIG_RSEQ")]
#[repr(C)]
pub struct rseq_event_error_flags {
    pub fatal: u8,
    pub slowpath: u8,
}

#[cfg(feature = "CONFIG_RSEQ")]
#[repr(C)]
pub union rseq_ids {
    pub cpu_cid: u64,
    pub ids: rseq_ids_fields,
}

#[cfg(feature = "CONFIG_RSEQ")]
#[repr(C)]
pub struct rseq_ids_fields {
    pub cpu_id: u32,
    pub mm_cid: u32,
}

#[cfg(feature = "CONFIG_RSEQ")]
#[repr(C)]
pub union rseq_slice_state {
    pub state: u16,
    pub fields: rseq_slice_state_fields,
}

#[cfg(feature = "CONFIG_RSEQ")]
#[repr(C)]
pub struct rseq_slice_state_fields {
    pub enabled: u8,
    pub granted: u8,
}

#[cfg(feature = "CONFIG_RSEQ")]
#[repr(C)]
pub struct rseq_slice {
    pub state: rseq_slice_state,
    pub expires: u64,
    pub yielded: u8,
}

#[cfg(feature = "CONFIG_RSEQ")]
#[repr(C)]
pub struct rseq_data {
    pub usrptr: *mut rseq,
    pub len: u32,
    pub sig: u32,
    pub event: rseq_event,
    pub ids: rseq_ids,
    #[cfg(feature = "CONFIG_RSEQ_SLICE_EXTENSION")]
    pub slice: rseq_slice,
}

#[cfg(not(feature = "CONFIG_RSEQ"))]
#[repr(C)]
pub struct rseq_data {}

#[cfg(feature = "CONFIG_SCHED_MM_CID")]
pub const MM_CID_UNSET: u32 = 1 << 31;
#[cfg(feature = "CONFIG_SCHED_MM_CID")]
pub const MM_CID_ONCPU: u32 = 1 << 30;
#[cfg(feature = "CONFIG_SCHED_MM_CID")]
pub const MM_CID_TRANSIT: u32 = 1 << 29;

#[cfg(feature = "CONFIG_SCHED_MM_CID")]
#[repr(C)]
pub struct sched_mm_cid {
    pub active: ::core::ffi::c_uint,
    pub cid: ::core::ffi::c_uint,
    pub node: hlist_node,
}

#[cfg(feature = "CONFIG_SCHED_MM_CID")]
#[repr(C)]
pub struct mm_cid_pcpu {
    pub cid: ::core::ffi::c_uint,
}
// C attribute: ____cacheline_aligned_in_smp

#[cfg(feature = "CONFIG_SCHED_MM_CID")]
#[repr(C)]
pub struct mm_mm_cid {
    pub pcpu: *mut mm_cid_pcpu,
    pub mode: ::core::ffi::c_uint,
    pub max_cids: ::core::ffi::c_uint,
    pub irq_work: irq_work,
    pub work: work_struct,
    pub lock: raw_spinlock_t,
    pub mutex: mutex,
    pub user_list: hlist_head,
    pub nr_cpus_allowed: ::core::ffi::c_uint,
    pub users: ::core::ffi::c_uint,
    pub pcpu_thrs: ::core::ffi::c_uint,
    pub update_deferred: ::core::ffi::c_uint,
}
// C attribute: ____cacheline_aligned

#[cfg(not(feature = "CONFIG_SCHED_MM_CID"))]
#[repr(C)]
pub struct mm_mm_cid {}
#[cfg(not(feature = "CONFIG_SCHED_MM_CID"))]
#[repr(C)]
pub struct sched_mm_cid {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
