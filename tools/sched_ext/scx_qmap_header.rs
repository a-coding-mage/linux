/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shared definitions between scx_qmap.bpf.c and scx_qmap.c.
 *
 * The scheduler keeps all state in a single BPF arena map. struct
 * qmap_arena is the one object that lives at the base of the arena and is
 * mmap'd into userspace so the loader can read counters directly.
 *
 * Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2026 Tejun Heo <tj@kernel.org>
 */

/* C header dependencies:
 * - under __BPF__: <scx/bpf_arena_common.bpf.h>
 * - otherwise: <linux/types.h> and <scx/bpf_arena_common.h>
 */

pub const MAX_SUB_SCHEDS: usize = 8;
pub const MAX_PARTS: usize = MAX_SUB_SCHEDS + 1; /* participants: children + self */

/*
 * cpu_ctxs[] is sized to a fixed cap so the layout is shared between BPF and
 * userspace. Keep this in sync with NR_CPUS used by the BPF side.
 */
pub const SCX_QMAP_MAX_CPUS: usize = 1024;

/*
 * An owner id identifies who holds a cid: a child slot in [0, MAX_SUB_SCHEDS),
 * CID_SELF for this node, CID_NONE for a cid not currently held, or CID_SHARED
 * for a cid in the round-robin pool (its live holder is rr_slots[rr_pos]). Used
 * by the partition's cid_owner[].
 */
pub const CID_SELF: i32 = -1;
pub const CID_NONE: i32 = -2;
pub const CID_SHARED: i32 = -3;

/* -C cid-override test modes. Selects cid_override_mode in scx_qmap.bpf.c. */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum qmap_cid_override {
    QMAP_CID_OVR_OFF = 0,     /* disabled */
    QMAP_CID_OVR_SHUFFLE = 1, /* valid reversed cpu->cid mapping */
    QMAP_CID_OVR_BAD_DUP = 2, /* invalid: duplicate cid assignment */
    QMAP_CID_OVR_BAD_RANGE = 3, /* invalid: out-of-range cid */
    QMAP_CID_OVR_BAD_MONO = 4, /* invalid: non-monotonic shard_start */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_ctx {
    pub dsp_idx: u64, /* dispatch index */
    pub dsp_cnt: u64, /* remaining count */
    pub avg_weight: u32,
    pub cpuperf_target: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmap_fifo {
    pub head: *mut task_ctx,
    pub tail: *mut task_ctx,
    pub idx: i32,
}

/* -J fault-injection modes. Selects inject_mode in struct qmap_arena. */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum qmap_inject {
    QMAP_INJ_OFF = 0,
    QMAP_INJ_WRONG_CID = 1,      /* dispatch to a cid we don't hold */
    QMAP_INJ_INIT_FAIL = 2,      /* fail init_task for "qmfail*" comms */
    QMAP_INJ_CGRP_INIT_FAIL = 3, /* fail cpuctl_init for "qmfail*" cgroups */
}

/*
 * scx_cmask's are embedded in struct qmap_arena with inline backing storage.
 * The bpf side uses &field.mask with the normal cmask_* helpers. Userspace
 * doesn't have access to the type definition and sees same-sized opaque words.
 * _Static_assert()'s in .bpf.c ensure that they are in sync.
 */
pub const QMAP_CMASK_WORDS: usize = (SCX_QMAP_MAX_CPUS + 63) / 64 + 1;

#[cfg(__BPF__)]
#[repr(C)]
pub union qmap_cmask {
    pub mask: scx_cmask,
    pub words: [u64; QMAP_CMASK_WORDS + 2],
}

#[cfg(not(__BPF__))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmap_cmask {
    pub words: [u64; QMAP_CMASK_WORDS + 2],
}

/* Opaque to userspace; defined in scx_qmap.bpf.c. */
pub enum task_ctx {}

/* per-direct-child state for the sub-scheduler */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sub_sched_ctx {
    pub cgroup_id: u64,
    pub weight: u32, /* cpu.weight, seeded at attach, then set_weight */
    pub nr_dsps: u64,
    pub granted_cids: qmap_cmask, /* cids granted excl to this child */
    pub prev_granted: qmap_cmask, /* last grant, for delta calculation */
}

/*
 * compute_partition() builds the following from this node's held caps, and
 * apply_partition()/rr_advance() execute it. Userspace only reads for the
 * hierarchy display.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmap_partition {
    pub nr_excl: u32, /* number of excl-held (delegatable) cids */
    pub cid_owner: [i32; SCX_QMAP_MAX_CPUS], /* per cid: owner id, or CID_NONE */
    pub shared_cids: [i32; MAX_PARTS], /* the round-robin cid pool */
    pub nr_shared: u32, /* number of shared_cids entries */
    pub rr_slots: [u64; MAX_PARTS], /* rotation order: holder cgroup_id, 0 = self */
    pub nr_rr: u32, /* number of rr_slots entries */
    pub rr_pos: u32, /* current rotation index */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmap_arena {
    /* userspace-visible stats */
    pub nr_enqueued: u64,
    pub nr_dispatched: u64,
    pub nr_reenqueued: u64,
    pub nr_reenqueued_cid0: u64,
    pub nr_dequeued: u64,
    pub nr_ddsp_from_enq: u64,
    pub nr_core_sched_execed: u64,
    pub nr_expedited_local: u64,
    pub nr_expedited_remote: u64,
    pub nr_expedited_lost: u64,
    pub nr_expedited_from_timer: u64,
    pub nr_highpri_queued: u64,
    pub test_error_cnt: u32,
    pub cpuperf_min: u32,
    pub cpuperf_avg: u32,
    pub cpuperf_max: u32,
    pub cpuperf_target_min: u32,
    pub cpuperf_target_avg: u32,
    pub cpuperf_target_max: u32,

    /* kernel-side runtime state */
    pub core_sched_head_seqs: [u64; 5],
    pub core_sched_tail_seqs: [u64; 5],

    pub cpu_ctxs: [cpu_ctx; SCX_QMAP_MAX_CPUS],

    /* cid-override test input, populated by the loader before attach */
    pub cid_override_cpu_to_cid: [i32; SCX_QMAP_MAX_CPUS],
    pub cid_override_shard_start: [i32; SCX_QMAP_MAX_CPUS],

    /* task_ctx slab; allocated and threaded by qmap_init() */
    pub task_ctxs: *mut task_ctx,
    pub task_free_head: *mut task_ctx,

    /* five priority FIFOs, each a doubly-linked list through task_ctx */
    pub fifos: [qmap_fifo; 5],

    /*
     * Hierarchical sub-scheduling state. See the design comment at the top
     * of scx_qmap.bpf.c.
     */
    pub nr_cids: u32, /* cid count, cached at init */

    /* bpf-owned partition: read by userspace for display */
    pub part: qmap_partition,

    pub sub_sched_ctxs: [sub_sched_ctx; MAX_SUB_SCHEDS], /* per-child context */
    pub nr_sub_scheds: u64, /* number of attached children */

    /* bpf-internal per-cid state */
    pub cid_shared: [u8; SCX_QMAP_MAX_CPUS], /* per cid: 1 if held shared (ENQ_IMMED-only) */

    /* allocated cid-time, charged per owner by account_alloc() */
    pub alloc_ns: [u64; MAX_SUB_SCHEDS], /* per child slot */
    pub self_alloc_ns: u64,
    pub alloc_ts: u64, /* last accounting timestamp */
    pub alloc_window_ns: u64, /* total accounted time, the alloc denominator */

    /* bpf-internal cmasks (embedded, see struct qmap_cmask) */
    pub self_cids: qmap_cmask, /* cids this node runs its own tasks on */
    pub idle_cids: qmap_cmask, /* idle state of all cids regardless of delegation */
    pub rr_cids: qmap_cmask, /* the shared pool, as a mask for grant/revoke */

    /* scratch cmasks */
    pub to_revoke_cids: qmap_cmask, /* delta cids to revoke */
    pub to_grant_cids: qmap_cmask, /* delta cids to grant */
    pub prev_rr_cids: qmap_cmask, /* previous shared pool, to clear stale grants */
    pub held_excl: qmap_cmask, /* cids held excl (ENQ): delegatable */
    pub held_shared: qmap_cmask, /* cids held shared (ENQ_IMMED only): self-local */

    /* bpf -> userspace: stats */
    pub nr_reenq_cap: u64, /* SCX_TASK_REENQ_CAP bounces */
    pub nr_reenq_immed: u64, /* SCX_TASK_REENQ_IMMED bounces */
    pub nr_inject_attempts: u64, /* fault-injection: dispatches to an unheld cid */
    pub nr_rescue_dsp: u64, /* SCX_ENQ_RESCUE dispatch attempts */
    pub inject_mode: u32, /* fault-injection mode (QMAP_INJ_*) */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
