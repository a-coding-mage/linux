// SPDX-License-Identifier: GPL-2.0
/*
 * scx_qmap: a demonstration and testing scheduler for sched_ext features.
 *
 * A simple scheduler that exercises a broad set of sched_ext features. Unlikely
 * to be useful for real workloads. It demonstrates:
 *
 * - BPF-side queueing using TIDs.
 * - BPF arena for scheduler state.
 * - Core-sched support.
 * - Hierarchical sub-scheduling: delegating cpus to child cgroup schedulers.
 *
 * Base design: Five FIFOs (arena-backed doubly-linked lists through per-task
 * context). A task is assigned to a FIFO by its compound weight. Each cpu
 * round-robins the FIFOs, dispatching more from higher ones.
 *
 * Sub-scheduling: Any qmap sched can delegate cpus to its own child cgroup
 * schedulers and keep the rest for its tasks. Terminology:
 *
 *   excl   - A cpu the delegatee owns wholly (ENQ_IMMED|ENQ|PREEMPT).
 *   shared - A cpu delegated as ENQ_IMMED only. Time-shared.
 *   held_excl / held_shared - What this node was handed by its parent.
 *            held-excl cpus are re-delegatable. A held-shared cpu is a
 *            time-share that stays self-local.
 *   self   - The excl cpus the node kept for itself, plus all of held_shared.
 *   owner  - Who holds a cid - a child slot, CID_SELF, or CID_NONE.
 *
 * The scheduler splits its held-excl cpus among self and the children in
 * proportion to each node's cpu.weight, handing each the floor of its share as
 * excl cpus. The leftover from rounding forms a shared pool the round-robin
 * timer hands around. With no excl cpu to delegate, the node evicts its
 * children.
 *
 * This policy is a demonstration only, not a practical one. The split
 * considers only direct children and is not work-conserving. It only exists to
 * drive sub-sched primitives with as simple logic as possible.
 *
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

// Dependencies supplied by scx/common.bpf.h and scx_qmap.h are referenced as
// extern symbols/types/constants in this source-level translation.

pub const ONE_SEC_IN_NS: u64 = 1000000000;
pub const ONE_MSEC_IN_NS: u64 = 1000000;
pub const LOWPRI_INTV_NS: u64 = 10 * ONE_MSEC_IN_NS;
pub const SHARED_DSQ: u64 = 0;
pub const HIGHPRI_DSQ: u64 = 1;
pub const LOWPRI_DSQ: u64 = 2;
pub const HIGHPRI_WEIGHT: u32 = 8668; /* this is what -20 maps to */
pub const IDLE_PICK_RETRIES: u32 = 16;

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe extern "C" {
    pub static slice_ns: u64;
    pub static stall_user_nth: u32;
    pub static stall_kernel_nth: u32;
    pub static dsp_inf_loop_after: u32;
    pub static dsp_batch: u32;
    pub static highpri_boosting: bool;
    pub static print_dsqs_and_events: bool;
    pub static print_msgs: bool;
    pub static sub_cgroup_id: u64;
    pub static disallow_tgid: s32;
    pub static suppress_dump: bool;
    pub static immed_stress_nth: u32;
    pub static max_tasks: u32;

    /* sub-sched: period for handing the round-robin cid pool to the next child */
    pub static round_robin_ns: u64;

    /*
     * Optional cid-override test harness. When cid_override_mode is non-zero,
     * qmap_init_cids() calls scx_bpf_cid_override() with the caller-supplied arrays
     * to exercise the kfunc's acceptance and error paths. See enum
     * qmap_cid_override for the modes.
     */
    pub static cid_override_mode: u32;
    pub static cid_override_nr_shards: u32;
}

unsafe extern "C" {
    pub static mut uei: scx_exit_info;
}

/*
 * All scheduler state - per-cpu context, stats counters, core-sched sequence
 * numbers, sub-sched cgroup ids - lives in this single BPF arena map. Userspace
 * reaches it via skel->arena->qa.
 */
#[repr(C)]
pub struct ArenaMapDef {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
    pub map_extra: u64,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut arena: ArenaMapDef = ArenaMapDef {
    type_: BPF_MAP_TYPE_ARENA,
    map_flags: BPF_F_MMAPABLE,
    max_entries: 1 << 16, /* upper bound in pages */
    // C used 0x1ull << 32 on arm64/aarch64 and 0x1ull << 44 otherwise.
    #[cfg(any(target_arch = "aarch64", target_arch = "arm64"))]
    map_extra: 0x1u64 << 32, /* user/BPF mmap base */
    #[cfg(not(any(target_arch = "aarch64", target_arch = "arm64")))]
    map_extra: 0x1u64 << 44,
};

unsafe extern "C" {
    pub static mut qa: qmap_arena;
}

/* ensure that BPF and userspace are seeing the same size for qmap_cmask */
const _: () = {
    let _ = [(); QMAP_CMASK_WORDS as usize];
    let _ = [(); CMASK_NR_WORDS(SCX_QMAP_MAX_CPUS) as usize];
};

/* Per-queue locks. Each in its own .data section as bpf_res_spin_lock requires. */
#[unsafe(link_section = ".data.qa_q_lock0")]
#[unsafe(no_mangle)]
pub static mut qa_q_lock0: bpf_res_spin_lock = unsafe { core::mem::zeroed() };
#[unsafe(link_section = ".data.qa_q_lock1")]
#[unsafe(no_mangle)]
pub static mut qa_q_lock1: bpf_res_spin_lock = unsafe { core::mem::zeroed() };
#[unsafe(link_section = ".data.qa_q_lock2")]
#[unsafe(no_mangle)]
pub static mut qa_q_lock2: bpf_res_spin_lock = unsafe { core::mem::zeroed() };
#[unsafe(link_section = ".data.qa_q_lock3")]
#[unsafe(no_mangle)]
pub static mut qa_q_lock3: bpf_res_spin_lock = unsafe { core::mem::zeroed() };
#[unsafe(link_section = ".data.qa_q_lock4")]
#[unsafe(no_mangle)]
pub static mut qa_q_lock4: bpf_res_spin_lock = unsafe { core::mem::zeroed() };

unsafe fn qa_q_lock(qid: s32) -> *mut bpf_res_spin_lock {
    match qid {
        0 => &raw mut qa_q_lock0,
        1 => &raw mut qa_q_lock1,
        2 => &raw mut qa_q_lock2,
        3 => &raw mut qa_q_lock3,
        4 => &raw mut qa_q_lock4,
        _ => core::ptr::null_mut(),
    }
}

/*
 * If enabled, CPU performance target is set according to the queue index
 * according to the following table.
 */
static qidx_to_cpuperf_target: [u32; 5] = [
    SCX_CPUPERF_ONE * 0 / 4,
    SCX_CPUPERF_ONE * 1 / 4,
    SCX_CPUPERF_ONE * 2 / 4,
    SCX_CPUPERF_ONE * 3 / 4,
    SCX_CPUPERF_ONE * 4 / 4,
];

/*
 * Per-queue sequence numbers to implement core-sched ordering.
 *
 * Tail seq is assigned to each queued task and incremented. Head seq tracks the
 * sequence number of the latest dispatched task. The distance between the a
 * task's seq and the associated queue's head seq is called the queue distance
 * and used when comparing two tasks for ordering. See qmap_core_sched_before().
 */

/*
 * Per-task scheduling context. Allocated from the qa.task_ctxs[] slab in
 * arena. While the task is alive the entry is referenced from task_ctx_stor;
 * while it's free the entry sits on the free list singly-linked through
 * @next_free.
 *
 * When the task is queued on one of the five priority FIFOs, @q_idx is the
 * queue index and @q_next/@q_prev link it in the queue's doubly-linked list.
 * @q_idx is -1 when the task isn't on any queue.
 */
#[repr(C)]
pub struct task_ctx {
    pub next_free: *mut task_ctx, /* only valid on free list */
    pub q_next: *mut task_ctx,    /* queue link, NULL if tail */
    pub q_prev: *mut task_ctx,    /* queue link, NULL if head */
    pub fifo: *mut qmap_fifo,     /* queue we're on, NULL if not queued */
    pub tid: u64,
    pub pid: s32, /* for dump only */
    pub force_local: bool, /* Dispatch directly to local_dsq */
    pub highpri: bool,
    pub core_sched_seq: u64,
    pub cpus_allowed: scx_cmask, /* per-task affinity in cid space */
}

/*
 * Slab stride for task_ctx. cpus_allowed's flex array bits[] overlaps the
 * tail bytes appended per entry; struct_size() gives the actual per-entry
 * footprint.
 */
pub const TASK_CTX_STRIDE: usize =
    struct_size_task_ctx_cpus_allowed_bits(CMASK_NR_WORDS(SCX_QMAP_MAX_CPUS)) as usize;

/* All task_ctx pointers are arena pointers. */
pub type task_ctx_t = task_ctx;

/* Holds an arena pointer to the task's slab entry. */
#[repr(C)]
pub struct task_ctx_stor_val {
    pub taskc: *mut task_ctx_t,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut task_ctx_stor: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    max_entries: 0,
};

/* Protects the task_ctx slab free list. */
#[unsafe(link_section = ".data.qa_task_lock")]
#[unsafe(no_mangle)]
pub static mut qa_task_lock: bpf_res_spin_lock = unsafe { core::mem::zeroed() };

unsafe fn qmap_spin_lock(lock: *mut bpf_res_spin_lock) -> s32 {
    if bpf_res_spin_lock(lock) != 0 {
        scx_bpf_error(c"res_spin_lock failed".as_ptr());
        return -EBUSY;
    }
    0
}

/*
 * Try prev_cid, then scan cpus_allowed AND idle_cids AND self_cids round-robin
 * from prev_cid + 1. Atomic claim retries on race; bounded by
 * IDLE_PICK_RETRIES to keep the verifier's insn budget in check.
 */
unsafe fn pick_direct_dispatch_cid(
    _p: *mut task_struct,
    prev_cid: s32,
    taskc: *mut task_ctx_t,
) -> s32 {
    let nr_cids = scx_bpf_nr_cids();
    let mut cid: s32;

    if cmask_test(prev_cid, &raw const (*(&raw const qa)).self_cids.mask)
        && cmask_test_and_clear(prev_cid, &raw mut (*(&raw mut qa)).idle_cids.mask)
    {
        return prev_cid;
    }

    cid = prev_cid;
    for _i in 0..IDLE_PICK_RETRIES {
        cid = cmask_next_and2_set_wrap(
            &raw mut (*taskc).cpus_allowed,
            &raw mut (*(&raw mut qa)).idle_cids.mask,
            &raw mut (*(&raw mut qa)).self_cids.mask,
            cid + 1,
        );
        barrier_var(cid);
        if cid >= nr_cids as s32 {
            return -1;
        }
        if cmask_test_and_clear(cid, &raw mut (*(&raw mut qa)).idle_cids.mask) {
            return cid;
        }
    }
    -1
}

/*
 * Force a reference to the arena map. The verifier associates an arena with
 * a program by finding an LD_IMM64 instruction that loads the arena's BPF
 * map; programs that only use arena pointers returned from task-local
 * storage (like qmap_select_cpu) never reference @arena directly. Without
 * this, the verifier rejects addr_space_cast with "addr_space_cast insn
 * can only be used in a program that has an associated arena".
 */
unsafe fn qmap_touch_arena() {
    core::arch::asm!("", in(reg) &raw const arena, options(nostack, preserves_flags));
}

unsafe fn lookup_task_ctx(p: *mut task_struct) -> *mut task_ctx_t {
    let v: *mut task_ctx_stor_val;

    qmap_touch_arena();

    v = bpf_task_storage_get(&raw mut task_ctx_stor, p, core::ptr::null_mut(), 0)
        as *mut task_ctx_stor_val;
    if v.is_null() || (*v).taskc.is_null() {
        return core::ptr::null_mut();
    }
    (*v).taskc
}

/* Append @taskc to the tail of @fifo. Must not already be queued. */
unsafe fn qmap_fifo_enqueue(fifo: *mut qmap_fifo, taskc: *mut task_ctx_t) {
    let lock = qa_q_lock((*fifo).idx);

    if lock.is_null() || qmap_spin_lock(lock) != 0 {
        return;
    }
    (*taskc).fifo = fifo;
    (*taskc).q_next = core::ptr::null_mut();
    (*taskc).q_prev = (*fifo).tail;
    if !(*fifo).tail.is_null() {
        (*(*fifo).tail).q_next = taskc;
    } else {
        (*fifo).head = taskc;
    }
    (*fifo).tail = taskc;
    bpf_res_spin_unlock(lock);
}

/* Pop the head of @fifo. Returns NULL if empty. */
unsafe fn qmap_fifo_pop(fifo: *mut qmap_fifo) -> *mut task_ctx_t {
    let lock = qa_q_lock((*fifo).idx);
    let taskc: *mut task_ctx_t;

    if lock.is_null() || qmap_spin_lock(lock) != 0 {
        return core::ptr::null_mut();
    }
    taskc = (*fifo).head;
    if !taskc.is_null() {
        (*fifo).head = (*taskc).q_next;
        if !(*taskc).q_next.is_null() {
            (*(*taskc).q_next).q_prev = core::ptr::null_mut();
        } else {
            (*fifo).tail = core::ptr::null_mut();
        }
        (*taskc).q_next = core::ptr::null_mut();
        (*taskc).q_prev = core::ptr::null_mut();
        (*taskc).fifo = core::ptr::null_mut();
    }
    bpf_res_spin_unlock(lock);
    taskc
}

/* Remove @taskc from its fifo. No-op if not queued. */
unsafe fn qmap_fifo_remove(taskc: *mut task_ctx_t) {
    let fifo = (*taskc).fifo;
    let lock: *mut bpf_res_spin_lock;

    if fifo.is_null() {
        return;
    }

    lock = qa_q_lock((*fifo).idx);
    if lock.is_null() || qmap_spin_lock(lock) != 0 {
        return;
    }

    /* Re-check under lock - a concurrent pop may have cleared fifo. */
    if (*taskc).fifo != fifo {
        bpf_res_spin_unlock(lock);
        return;
    }

    if !(*taskc).q_next.is_null() {
        (*(*taskc).q_next).q_prev = (*taskc).q_prev;
    } else {
        (*fifo).tail = (*taskc).q_prev;
    }
    if !(*taskc).q_prev.is_null() {
        (*(*taskc).q_prev).q_next = (*taskc).q_next;
    } else {
        (*fifo).head = (*taskc).q_next;
    }
    (*taskc).q_next = core::ptr::null_mut();
    (*taskc).q_prev = core::ptr::null_mut();
    (*taskc).fifo = core::ptr::null_mut();
    bpf_res_spin_unlock(lock);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_select_cid(
    p: *mut task_struct,
    prev_cid: s32,
    _wake_flags: u64,
) -> s32 {
    let taskc = lookup_task_ctx(p);
    let cid: s32;

    if taskc.is_null() {
        return prev_cid;
    }

    if (*p).scx.weight < 2 && ((*p).flags & PF_KTHREAD) == 0 {
        return prev_cid;
    }

    cid = pick_direct_dispatch_cid(p, prev_cid, taskc);

    if cid >= 0 {
        (*taskc).force_local = true;
        cid
    } else {
        prev_cid
    }
}

/*
 * A received time-shared cid is held ENQ_IMMED-only, so inserts must set
 * SCX_ENQ_IMMED.
 */
unsafe fn needs_immed(cid: s32) -> u64 {
    if (*(&raw const qa)).cid_shared[cid as usize] != 0 {
        SCX_ENQ_IMMED
    } else {
        0
    }
}

/* first cid this node does NOT hold for fault injection, -1 if none */
unsafe fn first_unavail_cid() -> s32 {
    let nr_cids = qa.nr_cids;

    if nr_cids > SCX_QMAP_MAX_CPUS as s32 {
        scx_bpf_error(c"-ERANGE".as_ptr());
        return -1;
    }

    for c in 0..nr_cids {
        if !cmask_test(c, &raw const qa.held_excl.mask)
            && !cmask_test(c, &raw const qa.held_shared.mask)
        {
            return c;
        }
    }
    -1
}

fn weight_to_idx(weight: u32) -> s32 {
    /* Coarsely map the compound weight to a FIFO. */
    if weight <= 25 {
        0
    } else if weight <= 50 {
        1
    } else if weight < 200 {
        2
    } else if weight < 400 {
        3
    } else {
        4
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_enqueue(p: *mut task_struct, enq_flags: u64) {
    static mut user_cnt: u32 = 0;
    static mut kernel_cnt: u32 = 0;
    static mut inj_cnt: u32 = 0;
    static mut immed_stress_cnt: u32 = 0;
    let mut taskc: *mut task_ctx_t;
    let idx = weight_to_idx((*p).scx.weight);
    let cid: s32;

    if (enq_flags & SCX_ENQ_REENQ) != 0 {
        let reason = (*p).scx.flags & SCX_TASK_REENQ_REASON_MASK;

        __sync_fetch_and_add(&raw mut qa.nr_reenqueued, 1);
        if scx_bpf_task_cid(p) == 0 {
            __sync_fetch_and_add(&raw mut qa.nr_reenqueued_cid0, 1);
        }
        /* cap-loss and IMMED-handback bounces, relocated below */
        if reason == SCX_TASK_REENQ_CAP {
            __sync_fetch_and_add(&raw mut qa.nr_reenq_cap, 1);
        } else if reason == SCX_TASK_REENQ_IMMED {
            __sync_fetch_and_add(&raw mut qa.nr_reenq_immed, 1);
        }
    }

    if ((*p).flags & PF_KTHREAD) != 0 {
        if stall_kernel_nth != 0 {
            kernel_cnt = kernel_cnt.wrapping_add(1);
            if kernel_cnt % stall_kernel_nth == 0 {
                return;
            }
        }
    } else if stall_user_nth != 0 {
        user_cnt = user_cnt.wrapping_add(1);
        if user_cnt % stall_user_nth == 0 {
            return;
        }
    }

    if qa.test_error_cnt != 0 {
        qa.test_error_cnt -= 1;
        if qa.test_error_cnt == 0 {
            scx_bpf_error(c"test triggering error".as_ptr());
        }
    }

    taskc = lookup_task_ctx(p);
    if taskc.is_null() {
        return;
    }

    /*
     * All enqueued tasks must have their core_sched_seq updated for correct
     * core-sched ordering. Also, take a look at the end of qmap_dispatch().
     */
    (*taskc).core_sched_seq = qa.core_sched_tail_seqs[idx as usize];
    qa.core_sched_tail_seqs[idx as usize] = qa.core_sched_tail_seqs[idx as usize].wrapping_add(1);

    /*
     * A task of ours that can run on none of our self cids - the parent
     * didn't grant them or we delegated them to children - would starve in
     * SHARED/FIFO since we only pull from those on self cids.
     *
     * Force it onto its first allowed cid's local DSQ. If we hold that cid
     * it runs. Otherwise the insert carries SCX_ENQ_RESCUE and the kernel
     * diverts the task to its rescue path.
     */
    if !cmask_intersects(&raw mut (*taskc).cpus_allowed, &raw mut qa.self_cids.mask) {
        let c = cmask_next_set_wrap(&raw mut (*taskc).cpus_allowed, 0);

        if c >= 0 && c < scx_bpf_nr_cids() as s32 {
            (*taskc).force_local = false;
            __sync_fetch_and_add(&raw mut qa.nr_rescue_dsp, 1);
            scx_bpf_dsq_insert(
                p,
                SCX_DSQ_LOCAL_ON | c as u64,
                slice_ns,
                enq_flags | needs_immed(c) | SCX_ENQ_RESCUE,
            );
            return;
        }
    }

    /*
     * Fault injection: deliberately dispatch one of our own tasks to a cid
     * we don't hold. The inserts carry SCX_ENQ_RESCUE and divert to the
     * kernel rescue path, a deterministic rescue-traffic generator. Under
     * -B 0 the kernel cap check rejects and re-enqueues them instead, so
     * nr_inject_attempts tracks nr_reenq_cap 1:1 and proves delivery-time
     * enforcement. Throttled.
     */
    if qa.inject_mode == QMAP_INJ_WRONG_CID
        && (*p).nr_cpus_allowed > 1
        && (enq_flags & SCX_ENQ_REENQ) == 0
    {
        inj_cnt = inj_cnt.wrapping_add(1);
        if inj_cnt % 64 == 0 {
            let bad = first_unavail_cid();

            if bad >= 0 && cmask_test(bad, &raw mut (*taskc).cpus_allowed) {
                __sync_fetch_and_add(&raw mut qa.nr_inject_attempts, 1);
                __sync_fetch_and_add(&raw mut qa.nr_rescue_dsp, 1);
                scx_bpf_dsq_insert(
                    p,
                    SCX_DSQ_LOCAL_ON | bad as u64,
                    slice_ns,
                    enq_flags | SCX_ENQ_RESCUE,
                );
                return;
            }
        }
    }

    /*
     * IMMED stress testing: Every immed_stress_nth'th enqueue, dispatch
     * directly to prev_cpu's local DSQ even when busy to force dsq->nr > 1
     * and exercise the kernel IMMED reenqueue trigger paths.
     */
    if immed_stress_nth != 0 && (enq_flags & SCX_ENQ_REENQ) == 0 {
        immed_stress_cnt = immed_stress_cnt.wrapping_add(1);
        if immed_stress_cnt % immed_stress_nth == 0 {
            (*taskc).force_local = false;
            scx_bpf_dsq_insert(
                p,
                SCX_DSQ_LOCAL_ON | scx_bpf_task_cid(p) as u64,
                slice_ns,
                enq_flags,
            );
            return;
        }
    }

    /*
     * If qmap_select_cid() is telling us to or this is the last runnable
     * task on the CPU, enqueue locally.
     */
    if (*taskc).force_local {
        (*taskc).force_local = false;
        scx_bpf_dsq_insert(
            p,
            SCX_DSQ_LOCAL,
            slice_ns,
            enq_flags | needs_immed(scx_bpf_task_cid(p)),
        );
        return;
    }

    /* see lowpri_timerfn() */
    if __COMPAT_has_generic_reenq()
        && (*p).scx.weight < 2
        && ((*p).flags & PF_KTHREAD) == 0
        && (enq_flags & SCX_ENQ_REENQ) == 0
    {
        scx_bpf_dsq_insert(p, LOWPRI_DSQ, slice_ns, enq_flags);
        return;
    }

    /* if select_cid() wasn't called, try direct dispatch */
    if !__COMPAT_is_enq_cpu_selected(enq_flags) {
        cid = pick_direct_dispatch_cid(p, scx_bpf_task_cid(p), taskc);
        if cid >= 0 {
            __sync_fetch_and_add(&raw mut qa.nr_ddsp_from_enq, 1);
            scx_bpf_dsq_insert(
                p,
                SCX_DSQ_LOCAL_ON | cid as u64,
                slice_ns,
                enq_flags | needs_immed(cid),
            );
            return;
        }
    }

    /*
     * If the task was re-enqueued due to the CPU being preempted by a
     * higher priority scheduling class, just re-enqueue the task directly
     * on the global DSQ. As we want another CPU to pick it up, find and
     * kick an idle cid.
     */
    if (enq_flags & SCX_ENQ_REENQ) != 0 {
        let cid = cmask_next_and2_set_wrap(
            &raw mut (*taskc).cpus_allowed,
            &raw mut qa.idle_cids.mask,
            &raw mut qa.self_cids.mask,
            0,
        );
        scx_bpf_dsq_insert(p, SHARED_DSQ, 0, enq_flags);
        if cid < scx_bpf_nr_cids() as s32 {
            scx_bpf_kick_cid(cid, SCX_KICK_IDLE);
        }
        return;
    }

    /* Queue on the selected FIFO. */
    qmap_fifo_enqueue(&raw mut qa.fifos[idx as usize], taskc);

    if highpri_boosting && (*p).scx.weight >= HIGHPRI_WEIGHT {
        (*taskc).highpri = true;
        __sync_fetch_and_add(&raw mut qa.nr_highpri_queued, 1);
    }
    __sync_fetch_and_add(&raw mut qa.nr_enqueued, 1);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_dequeue(p: *mut task_struct, deq_flags: u64) {
    let taskc: *mut task_ctx_t;

    __sync_fetch_and_add(&raw mut qa.nr_dequeued, 1);
    if (deq_flags & SCX_DEQ_CORE_SCHED_EXEC) != 0 {
        __sync_fetch_and_add(&raw mut qa.nr_core_sched_execed, 1);
    }

    taskc = lookup_task_ctx(p);
    if !taskc.is_null() && !(*taskc).fifo.is_null() {
        if (*taskc).highpri {
            __sync_fetch_and_sub(&raw mut qa.nr_highpri_queued, 1);
        }
        qmap_fifo_remove(taskc);
    }
}

unsafe fn update_core_sched_head_seq(p: *mut task_struct) {
    let idx = weight_to_idx((*p).scx.weight);
    let taskc = lookup_task_ctx(p);

    if !taskc.is_null() {
        qa.core_sched_head_seqs[idx as usize] = (*taskc).core_sched_seq;
    }
}

/*
 * One pass over SHARED_DSQ: rescue stranded tasks and boost highpri ones. A
 * task whose cids were lost while it was queued in the fifos would strand on
 * SHARED_DSQ, which is consumed only on self cids it can't run on - move it to
 * the kernel rescue path. One whose cids were lost after the highpri cull is
 * likewise rescued out of HIGHPRI_DSQ below.
 *
 * To demonstrate the use of scx_bpf_dsq_move(), implement silly selective
 * priority boosting mechanism by moving highpri tasks to HIGHPRI_DSQ and then
 * consuming them first. This makes minor difference only when dsp_batch is
 * larger than 1.
 *
 * scx_bpf_dsq_move[_vtime]() are allowed both from ops.dispatch() and
 * non-rq-lock holding BPF programs. As demonstration, this function is called
 * from qmap_dispatch() and monitor_timerfn().
 */
unsafe fn scan_shared_dsq(from_timer: bool) -> bool {
    let this_cid = scx_bpf_this_cid();
    let nr_cids = scx_bpf_nr_cids();

    /* rescue strands and move highpri tasks to HIGHPRI_DSQ */
    bpf_for_each_scx_dsq!(p, SHARED_DSQ, 0, {
        static mut highpri_seq: u64 = 0;
        let taskc = lookup_task_ctx(p);
        let c: s32;

        if taskc.is_null() {
            return false;
        }

        /* stranded? rescue - it can't be dispatched here either way */
        if !cmask_intersects(&raw mut (*taskc).cpus_allowed, &raw mut qa.self_cids.mask) {
            c = cmask_next_set_wrap(&raw mut (*taskc).cpus_allowed, 0);
            if c >= 0 && c < scx_bpf_nr_cids() as s32 {
                __sync_fetch_and_add(&raw mut qa.nr_rescue_dsp, 1);
                scx_bpf_dsq_move(
                    BPF_FOR_EACH_ITER,
                    p,
                    SCX_DSQ_LOCAL_ON | c as u64,
                    needs_immed(c) | SCX_ENQ_RESCUE,
                );
            }
            continue;
        }

        if (*taskc).highpri {
            /* exercise the set_*() and vtime interface too */
            scx_bpf_dsq_move_set_slice(BPF_FOR_EACH_ITER, slice_ns * 2);
            scx_bpf_dsq_move_set_vtime(BPF_FOR_EACH_ITER, highpri_seq);
            highpri_seq = highpri_seq.wrapping_add(1);
            scx_bpf_dsq_move_vtime(BPF_FOR_EACH_ITER, p, HIGHPRI_DSQ, 0);
        }
    });

    /*
     * Scan HIGHPRI_DSQ and dispatch until a task that can run here is
     * found. Prefer this_cid if the task allows it; otherwise RR-scan the
     * task's cpus_allowed starting after this_cid.
     */
    bpf_for_each_scx_dsq!(p, HIGHPRI_DSQ, 0, {
        let taskc = lookup_task_ctx(p);
        let mut dispatched = false;
        let cid: s32;

        if taskc.is_null() {
            return false;
        }

        /* only run highpri tasks on cids this node holds, not delegated ones */
        if cmask_test(this_cid, &raw mut (*taskc).cpus_allowed)
            && cmask_test(this_cid, &raw mut qa.self_cids.mask)
        {
            cid = this_cid;
        } else {
            cid = cmask_next_and_set_wrap(
                &raw mut (*taskc).cpus_allowed,
                &raw mut qa.self_cids.mask,
                this_cid + 1,
            );
        }
        if cid >= nr_cids as s32 {
            /* stranded after the cull - rescue it from here */
            let c = cmask_next_set_wrap(&raw mut (*taskc).cpus_allowed, 0);

            if c >= 0 && c < nr_cids as s32 {
                __sync_fetch_and_add(&raw mut qa.nr_rescue_dsp, 1);
                scx_bpf_dsq_move(
                    BPF_FOR_EACH_ITER,
                    p,
                    SCX_DSQ_LOCAL_ON | c as u64,
                    needs_immed(c) | SCX_ENQ_RESCUE,
                );
            }
            continue;
        }

        if scx_bpf_dsq_move(
            BPF_FOR_EACH_ITER,
            p,
            SCX_DSQ_LOCAL_ON | cid as u64,
            SCX_ENQ_PREEMPT | needs_immed(cid),
        ) {
            if cid == this_cid {
                dispatched = true;
                __sync_fetch_and_add(&raw mut qa.nr_expedited_local, 1);
            } else {
                __sync_fetch_and_add(&raw mut qa.nr_expedited_remote, 1);
            }
            if from_timer {
                __sync_fetch_and_add(&raw mut qa.nr_expedited_from_timer, 1);
            }
        } else {
            __sync_fetch_and_add(&raw mut qa.nr_expedited_lost, 1);
        }

        if dispatched {
            return true;
        }
    });

    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_dispatch(cid: s32, prev: *mut task_struct) {
    let mut p: *mut task_struct;
    let cpuc: *mut cpu_ctx;
    let mut taskc: *mut task_ctx_t;
    let mut batch = if dsp_batch != 0 { dsp_batch } else { 1 };
    let owner: s32;

    if scan_shared_dsq(false) {
        return;
    }

    /*
     * Sub-sched routing: a child-owned cid goes to its owner. Never run
     * this node's own tasks on a delegated cid. Read without the guard.
     */
    owner = qa.part.cid_owner[cid as usize];
    if owner == CID_SHARED {
        /* route to the live rr holder (0 = self, runs below) */
        let pos = qa.part.rr_pos;
        let holder_cgid = if pos >= 0 && pos < MAX_PARTS {
            qa.part.rr_slots[pos as usize]
        } else {
            0
        };

        if holder_cgid != 0 {
            scx_bpf_sub_dispatch(holder_cgid);
            return;
        }
    } else if owner >= 0 && owner < MAX_SUB_SCHEDS {
        let cgid = qa.sub_sched_ctxs[owner as usize].cgroup_id;

        if cgid != 0 {
            if scx_bpf_sub_dispatch(cgid) {
                __sync_fetch_and_add(&raw mut qa.sub_sched_ctxs[owner as usize].nr_dsps, 1);
            }
            return;
        }
    }

    if qa.nr_highpri_queued == 0 && scx_bpf_dsq_move_to_local(SHARED_DSQ, needs_immed(cid)) {
        return;
    }

    if dsp_inf_loop_after != 0 && qa.nr_dispatched > dsp_inf_loop_after as u64 {
        /*
         * PID 2 should be kthreadd which should mostly be idle and off
         * the scheduler. Let's keep dispatching it to force the kernel
         * to call this function over and over again.
         */
        p = bpf_task_from_pid(2);
        if !p.is_null() {
            scx_bpf_dsq_insert(p, SCX_DSQ_LOCAL, slice_ns, 0);
            bpf_task_release(p);
            return;
        }
    }

    cpuc = &raw mut qa.cpu_ctxs[scx_bpf_this_cid() as usize];

    for _i in 0..5 {
        /* Advance the dispatch cursor and pick the fifo. */
        if (*cpuc).dsp_cnt == 0 {
            (*cpuc).dsp_idx = ((*cpuc).dsp_idx + 1) % 5;
            (*cpuc).dsp_cnt = 1 << (*cpuc).dsp_idx;
        }

        /* Dispatch or advance. */
        for _ in 0..BPF_MAX_LOOPS {
            taskc = qmap_fifo_pop(&raw mut qa.fifos[(*cpuc).dsp_idx as usize]);
            if taskc.is_null() {
                break;
            }

            p = scx_bpf_tid_to_task((*taskc).tid);
            if p.is_null() {
                continue;
            }

            if (*taskc).highpri {
                __sync_fetch_and_sub(&raw mut qa.nr_highpri_queued, 1);
            }

            update_core_sched_head_seq(p);
            __sync_fetch_and_add(&raw mut qa.nr_dispatched, 1);

            scx_bpf_dsq_insert(p, SHARED_DSQ, slice_ns, 0);

            /*
             * scx_qmap uses a global BPF queue that any CPU's
             * dispatch can pop from. If this CPU popped a task that
             * can't run here, it gets stranded on SHARED_DSQ after
             * consume_dispatch_q() skips it. Kick the task's home
             * CPU so it drains SHARED_DSQ.
             *
             * There's a race between the pop and the flush of the
             * buffered dsq_insert:
             *
             *  CPU 0 (dispatching)      CPU 1 (home, idle)
             *  ~~~~~~~~~~~~~~~~~~~      ~~~~~~~~~~~~~~~~~~~
             *  pop from BPF queue
             *  dsq_insert(buffered)
             *                           balance:
             *                             SHARED_DSQ empty
             *                             BPF queue empty
             *                             -> goes idle
             *  flush -> on SHARED
             *  kick CPU 1
             *                           wakes, drains task
             *
             * The kick prevents indefinite stalls but a per-CPU
             * kthread like ksoftirqd can be briefly stranded when
             * its home CPU enters idle with softirq pending,
             * triggering:
             *
             *  "NOHZ tick-stop error: local softirq work is pending, handler #N!!!"
             *
             * from report_idle_softirq(). The kick lands shortly
             * after and the home CPU drains the task. This could be
             * avoided by e.g. dispatching pinned tasks to local or
             * global DSQs, but the current code is left as-is to
             * document this class of issue -- other schedulers
             * seeing similar warnings can use this as a reference.
             */
            if !cmask_test(cid, &raw mut (*taskc).cpus_allowed) {
                scx_bpf_kick_cid(scx_bpf_task_cid(p), 0);
            }
            batch -= 1;
            (*cpuc).dsp_cnt -= 1;
            if batch == 0 || !scx_bpf_dispatch_nr_slots() {
                if scan_shared_dsq(false) {
                    return;
                }
                scx_bpf_dsq_move_to_local(SHARED_DSQ, needs_immed(cid));
                return;
            }
            if (*cpuc).dsp_cnt == 0 {
                break;
            }
        }

        (*cpuc).dsp_cnt = 0;
    }

    if scan_shared_dsq(false) {
        return;
    }

    /*
     * No other tasks. @prev will keep running. Update its core_sched_seq as
     * if the task were enqueued and dispatched immediately.
     */
    if !prev.is_null() {
        taskc = lookup_task_ctx(prev);
        if taskc.is_null() {
            return;
        }

        let idx = weight_to_idx((*prev).scx.weight) as usize;
        (*taskc).core_sched_seq = qa.core_sched_tail_seqs[idx];
        qa.core_sched_tail_seqs[idx] = qa.core_sched_tail_seqs[idx].wrapping_add(1);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_tick(p: *mut task_struct) {
    let cpuc = &raw mut qa.cpu_ctxs[scx_bpf_this_cid() as usize];
    let idx: s32;

    /*
     * Use the running avg of weights to select the target cpuperf level.
     * This is a demonstration of the cpuperf feature rather than a
     * practical strategy to regulate CPU frequency.
     */
    (*cpuc).avg_weight = (*cpuc).avg_weight * 3 / 4 + (*p).scx.weight / 4;
    idx = weight_to_idx((*cpuc).avg_weight);
    (*cpuc).cpuperf_target = qidx_to_cpuperf_target[idx as usize];

    scx_bpf_cidperf_set(scx_bpf_task_cid(p), (*cpuc).cpuperf_target);
}

/*
 * The distance from the head of the queue scaled by the weight of the queue.
 * The lower the number, the older the task and the higher the priority.
 */
unsafe fn task_qdist(p: *mut task_struct, taskc: *mut task_ctx_t) -> s64 {
    let idx = weight_to_idx((*p).scx.weight);
    let qdist: s64;

    qdist = (*taskc).core_sched_seq as s64 - qa.core_sched_head_seqs[idx as usize] as s64;

    /*
     * As queue index increments, the priority doubles. The queue w/ index 3
     * is dispatched twice more frequently than 2. Reflect the difference by
     * scaling qdists accordingly. Note that the shift amount needs to be
     * flipped depending on the sign to avoid flipping priority direction.
     */
    if qdist >= 0 {
        qdist << (4 - idx)
    } else {
        qdist << idx
    }
}

/*
 * This is called to determine the task ordering when core-sched is picking
 * tasks to execute on SMT siblings and should encode about the same ordering as
 * the regular scheduling path. Use the priority-scaled distances from the head
 * of the queues to compare the two tasks which should be consistent with the
 * dispatch path behavior.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_core_sched_before(
    a: *mut task_struct,
    b: *mut task_struct,
) -> bool {
    let taskc_a = lookup_task_ctx(a);
    let taskc_b = lookup_task_ctx(b);

    /*
     * A task delegated to a sub-scheduler has no task_ctx here. Order such
     * pairs by the kernel's default ordering - a running task after every
     * waiting task, then by runnable_at.
     */
    if taskc_a.is_null() || taskc_b.is_null() {
        if (*a).on_cpu != (*b).on_cpu {
            return (*b).on_cpu;
        }
        return time_before((*a).scx.runnable_at, (*b).scx.runnable_at);
    }

    task_qdist(a, taskc_a) < task_qdist(b, taskc_b)
}

/*
 * sched_switch tracepoint and cpu_release handlers are no longer needed.
 * With SCX_OPS_ALWAYS_ENQ_IMMED, wakeup_preempt_scx() reenqueues IMMED
 * tasks when a higher-priority scheduling class takes the CPU.
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_init_task(
    p: *mut task_struct,
    _args: *mut scx_init_task_args,
) -> s32 {
    let v: *mut task_ctx_stor_val;
    let taskc: *mut task_ctx_t;

    if qa.inject_mode == QMAP_INJ_INIT_FAIL && bpf_strncmp((*p).comm.as_ptr(), 6, c"qmfail".as_ptr()) == 0 {
        return -ENOMEM;
    }

    if (*p).tgid == disallow_tgid {
        (*p).scx.disallow = true;
    }

    /* pop a slab entry off the free list */
    if qmap_spin_lock(&raw mut qa_task_lock) != 0 {
        return -EBUSY;
    }
    taskc = qa.task_free_head;
    if !taskc.is_null() {
        qa.task_free_head = (*taskc).next_free;
    }
    bpf_res_spin_unlock(&raw mut qa_task_lock);
    if taskc.is_null() {
        scx_bpf_error(c"task_ctx slab exhausted (max_tasks=%u)".as_ptr(), max_tasks);
        return -ENOMEM;
    }

    (*taskc).next_free = core::ptr::null_mut();
    (*taskc).q_next = core::ptr::null_mut();
    (*taskc).q_prev = core::ptr::null_mut();
    (*taskc).fifo = core::ptr::null_mut();
    (*taskc).tid = (*p).scx.tid;
    (*taskc).pid = (*p).pid;
    (*taskc).force_local = false;
    (*taskc).highpri = false;
    (*taskc).core_sched_seq = 0;
    cmask_init(&raw mut (*taskc).cpus_allowed, 0, scx_bpf_nr_cids());
    bpf_rcu_read_lock();
    cmask_from_cpumask(&raw mut (*taskc).cpus_allowed, (*p).cpus_ptr);
    bpf_rcu_read_unlock();

    v = bpf_task_storage_get(
        &raw mut task_ctx_stor,
        p,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    ) as *mut task_ctx_stor_val;
    if v.is_null() {
        /* push back to the free list */
        if qmap_spin_lock(&raw mut qa_task_lock) == 0 {
            (*taskc).next_free = qa.task_free_head;
            qa.task_free_head = taskc;
            bpf_res_spin_unlock(&raw mut qa_task_lock);
        }
        return -ENOMEM;
    }
    (*v).taskc = taskc;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_exit_task(
    p: *mut task_struct,
    _args: *mut scx_exit_task_args,
) {
    let v: *mut task_ctx_stor_val;
    let taskc: *mut task_ctx_t;

    v = bpf_task_storage_get(&raw mut task_ctx_stor, p, core::ptr::null_mut(), 0)
        as *mut task_ctx_stor_val;
    if v.is_null() || (*v).taskc.is_null() {
        return;
    }
    taskc = (*v).taskc;
    (*v).taskc = core::ptr::null_mut();

    if qmap_spin_lock(&raw mut qa_task_lock) != 0 {
        return;
    }
    (*taskc).next_free = qa.task_free_head;
    qa.task_free_head = taskc;
    bpf_res_spin_unlock(&raw mut qa_task_lock);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_dump(_dctx: *mut scx_dump_ctx) {
    let mut taskc: *mut task_ctx_t;

    qmap_touch_arena();

    if suppress_dump {
        return;
    }

    /*
     * Walk the queue lists without locking - kfunc calls (scx_bpf_dump)
     * aren't in the verifier's kfunc_spin_allowed() list so we can't hold
     * a lock and dump. Best-effort; racing may print stale tids but the
     * walk is bounded by bpf_repeat() so it always terminates.
     */
    for i in 0..5 {
        scx_bpf_dump(c"QMAP FIFO[%d]:".as_ptr(), i);
        taskc = qa.fifos[i as usize].head;
        for _ in 0..4096 {
            if taskc.is_null() {
                break;
            }
            scx_bpf_dump(c" %d:%llu".as_ptr(), (*taskc).pid, (*taskc).tid);
            taskc = (*taskc).q_next;
        }
        scx_bpf_dump(c"\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_dump_cid(_dctx: *mut scx_dump_ctx, cid: s32, idle: bool) {
    let cpuc = &raw mut qa.cpu_ctxs[cid as usize];

    if suppress_dump || idle {
        return;
    }

    scx_bpf_dump(
        c"QMAP: dsp_idx=%llu dsp_cnt=%llu avg_weight=%u cpuperf_target=%u".as_ptr(),
        (*cpuc).dsp_idx,
        (*cpuc).dsp_cnt,
        (*cpuc).avg_weight,
        (*cpuc).cpuperf_target,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_dump_task(_dctx: *mut scx_dump_ctx, p: *mut task_struct) {
    let v: *mut task_ctx_stor_val;
    let taskc: *mut task_ctx_t;

    qmap_touch_arena();

    if suppress_dump {
        return;
    }
    v = bpf_task_storage_get(&raw mut task_ctx_stor, p, core::ptr::null_mut(), 0)
        as *mut task_ctx_stor_val;
    if v.is_null() || (*v).taskc.is_null() {
        return;
    }
    taskc = (*v).taskc;

    scx_bpf_dump(
        c"QMAP: force_local=%d core_sched_seq=%llu".as_ptr(),
        (*taskc).force_local as s32,
        (*taskc).core_sched_seq,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_cpuctl_init(
    cgrp: *mut cgroup,
    args: *mut scx_cgroup_init_args,
) -> s32 {
    qmap_touch_arena();

    if print_msgs {
        bpf_printk(
            c"CGRP INIT %llu weight=%u period=%lu quota=%ld burst=%lu".as_ptr(),
            (*(*cgrp).kn).id,
            (*args).weight,
            (*args).bw_period_us,
            (*args).bw_quota_us,
            (*args).bw_burst_us,
        );
    }

    if qa.inject_mode == QMAP_INJ_CGRP_INIT_FAIL {
        let mut name = [0i8; 7];

        bpf_probe_read_kernel_str(name.as_mut_ptr() as *mut _, name.len(), (*(*cgrp).kn).name);
        if bpf_strncmp(name.as_ptr(), 6, c"qmfail".as_ptr()) == 0 {
            return -ENOMEM;
        }
    }

    0
}

unsafe fn redistribute();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_cpuctl_set_weight(cgrp: *mut cgroup, weight: u32) {
    let cgid = (*(*cgrp).kn).id;

    qmap_touch_arena();

    if print_msgs {
        bpf_printk(c"CGRP SET %llu weight=%u".as_ptr(), cgid, weight);
    }

    /*
     * Knobs belong to the parent, so this op carries the child subs'
     * attach point weights. Adjust the matching sub's share of the cid
     * partition. Other cgroups don't participate in the split.
     */
    for i in 0..MAX_SUB_SCHEDS {
        if qa.sub_sched_ctxs[i as usize].cgroup_id != cgid {
            continue;
        }
        if qa.sub_sched_ctxs[i as usize].weight != weight {
            qa.sub_sched_ctxs[i as usize].weight = weight;
            redistribute();
        }
        break;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_cpuctl_set_bandwidth(
    cgrp: *mut cgroup,
    period_us: u64,
    quota_us: u64,
    burst_us: u64,
) {
    if print_msgs {
        bpf_printk(
            c"CGRP SET %llu period=%lu quota=%ld burst=%lu".as_ptr(),
            (*(*cgrp).kn).id,
            period_us,
            quota_us,
            burst_us,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_cpuctl_move(
    p: *mut task_struct,
    from: *mut cgroup,
    to: *mut cgroup,
) {
    if print_msgs {
        bpf_printk(
            c"CGRP MOVE %d %llu -> %llu".as_ptr(),
            (*p).pid,
            (*(*from).kn).id,
            (*(*to).kn).id,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_update_idle(cid: s32, idle: bool) {
    qmap_touch_arena();

    /*
     * The kernel delivers update_idle() for every cid this node holds
     * SCX_CAP_BASE on. Track every cid's idle state regardless of
     * delegation: the direct-dispatch pick masks idle_cids with self_cids
     * at selection, so a cid already idle when it returns to self needs no
     * reseed here.
     */
    if idle {
        cmask_set(cid, &raw mut qa.idle_cids.mask);
    } else {
        cmask_clear(cid, &raw mut qa.idle_cids.mask);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_set_cmask(p: *mut task_struct, cmask_in: *const scx_cmask) {
    let cmask = cmask_in as *mut scx_cmask;
    let taskc = lookup_task_ctx(p);

    if taskc.is_null() {
        return;
    }
    cmask_copy(&raw mut (*taskc).cpus_allowed, cmask);
}

#[repr(C)]
pub struct monitor_timer {
    pub timer: bpf_timer,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut monitor_timer: bpf_array_map_def = bpf_array_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

/*
 * Aggregate cidperf across the first nr_online_cids cids. Post-hotplug
 * the first-N-are-online invariant drifts, so some cap/cur values may
 * be stale. For this demo monitor that's fine; the scheduler exits on
 * the enable-time hotplug_seq mismatch and userspace restarts, which
 * rebuilds the layout.
 */
unsafe fn monitor_cpuperf() {
    let nr_online = scx_bpf_nr_online_cids();
    let mut cap_sum: u64 = 0;
    let mut cur_sum: u64 = 0;
    let mut cur_min: u64 = SCX_CPUPERF_ONE as u64;
    let mut cur_max: u64 = 0;
    let mut target_sum: u64 = 0;
    let mut target_min: u64 = SCX_CPUPERF_ONE as u64;
    let mut target_max: u64 = 0;

    qmap_touch_arena();

    for cid in 0..nr_online {
        let cpuc = &raw mut qa.cpu_ctxs[cid as usize];
        let cap = scx_bpf_cidperf_cap(cid as s32);
        let cur = scx_bpf_cidperf_cur(cid as s32);
        let target: u32;

        cur_min = if (cur as u64) < cur_min { cur as u64 } else { cur_min };
        cur_max = if (cur as u64) > cur_max { cur as u64 } else { cur_max };

        cur_sum += cur as u64 * cap as u64 / SCX_CPUPERF_ONE as u64;
        cap_sum += cap as u64;

        target = (*cpuc).cpuperf_target;
        target_sum += target as u64;
        target_min = if (target as u64) < target_min { target as u64 } else { target_min };
        target_max = if (target as u64) > target_max { target as u64 } else { target_max };
    }

    if nr_online == 0 || cap_sum == 0 {
        return;
    }

    qa.cpuperf_min = cur_min;
    qa.cpuperf_avg = cur_sum * SCX_CPUPERF_ONE as u64 / cap_sum;
    qa.cpuperf_max = cur_max;

    qa.cpuperf_target_min = target_min;
    qa.cpuperf_target_avg = target_sum / nr_online as u64;
    qa.cpuperf_target_max = target_max;
}

/*
 * Dump the currently queued tasks in the shared DSQ to demonstrate the usage of
 * scx_bpf_dsq_nr_queued() and DSQ iterator. Raise the dispatch batch count to
 * see meaningful dumps in the trace pipe.
 */
unsafe fn dump_shared_dsq() {
    let nr = scx_bpf_dsq_nr_queued(SHARED_DSQ);

    if nr == 0 {
        return;
    }

    bpf_printk(c"Dumping %d tasks in SHARED_DSQ in reverse order".as_ptr(), nr);

    bpf_rcu_read_lock();
    bpf_for_each_scx_dsq!(p, SHARED_DSQ, SCX_DSQ_ITER_REV, {
        bpf_printk(c"%s[%d]".as_ptr(), (*p).comm.as_ptr(), (*p).pid);
    });
    bpf_rcu_read_unlock();
}

unsafe extern "C" fn monitor_timerfn(
    _map: *mut core::ffi::c_void,
    _key: *mut s32,
    timer: *mut bpf_timer,
) -> s32 {
    bpf_rcu_read_lock();
    scan_shared_dsq(true);
    bpf_rcu_read_unlock();

    monitor_cpuperf();

    if print_dsqs_and_events {
        let mut events: scx_event_stats = core::mem::zeroed();

        dump_shared_dsq();

        __COMPAT_scx_bpf_events(&raw mut events, core::mem::size_of::<scx_event_stats>() as u32);

        bpf_printk(c"%35s: %lld".as_ptr(), c"SCX_EV_SELECT_CPU_FALLBACK".as_ptr(), scx_read_event(&events, SCX_EV_SELECT_CPU_FALLBACK));
        bpf_printk(c"%35s: %lld".as_ptr(), c"SCX_EV_DISPATCH_LOCAL_DSQ_OFFLINE".as_ptr(), scx_read_event(&events, SCX_EV_DISPATCH_LOCAL_DSQ_OFFLINE));
        bpf_printk(c"%35s: %lld".as_ptr(), c"SCX_EV_DISPATCH_KEEP_LAST".as_ptr(), scx_read_event(&events, SCX_EV_DISPATCH_KEEP_LAST));
        bpf_printk(c"%35s: %lld".as_ptr(), c"SCX_EV_ENQ_SKIP_EXITING".as_ptr(), scx_read_event(&events, SCX_EV_ENQ_SKIP_EXITING));
        bpf_printk(c"%35s: %lld".as_ptr(), c"SCX_EV_REFILL_SLICE_DFL".as_ptr(), scx_read_event(&events, SCX_EV_REFILL_SLICE_DFL));
        bpf_printk(c"%35s: %lld".as_ptr(), c"SCX_EV_BYPASS_DURATION".as_ptr(), scx_read_event(&events, SCX_EV_BYPASS_DURATION));
        bpf_printk(c"%35s: %lld".as_ptr(), c"SCX_EV_BYPASS_DISPATCH".as_ptr(), scx_read_event(&events, SCX_EV_BYPASS_DISPATCH));
        bpf_printk(c"%35s: %lld".as_ptr(), c"SCX_EV_BYPASS_ACTIVATE".as_ptr(), scx_read_event(&events, SCX_EV_BYPASS_ACTIVATE));
    }

    bpf_timer_start(timer, ONE_SEC_IN_NS, 0);
    0
}

#[repr(C)]
pub struct lowpri_timer {
    pub timer: bpf_timer,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut lowpri_timer: bpf_array_map_def = bpf_array_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

/*
 * Nice 19 tasks are put into the lowpri DSQ. Every 10ms, reenq is triggered and
 * the tasks are transferred to SHARED_DSQ.
 */
unsafe extern "C" fn lowpri_timerfn(
    _map: *mut core::ffi::c_void,
    _key: *mut s32,
    timer: *mut bpf_timer,
) -> s32 {
    scx_bpf_dsq_reenq(LOWPRI_DSQ, 0);
    bpf_timer_start(timer, LOWPRI_INTV_NS, 0);
    0
}

#[repr(C)]
pub struct round_robin_timer {
    pub timer: bpf_timer,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut round_robin_timer: bpf_array_map_def = bpf_array_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

/*
 * Partition update synchronization. qa.part can be written from concurrent
 * contexts. This single-runner guard admits one writer at a time without
 * holding a lock across the grant/revoke kfuncs. part_pending coalesces
 * repartition requests that arrive while it is held.
 *
 * They live in .bss, not the arena: rr_advance() runs from a bpf_timer
 * callback, where the verifier rejects atomic ops on arena memory.
 */
static mut part_busy: u64 = 0;
static mut part_pending: u64 = 0;

unsafe fn part_try_start() -> bool {
    /* set busy, report whether it was previously clear (we acquired it) */
    __sync_fetch_and_or(&raw mut part_busy, 1) == 0
}

unsafe fn part_end() {
    __sync_fetch_and_and(&raw mut part_busy, 0);
}

/*
 * compute_partition() scratch.
 *
 * The excl-held cids are handed out in cid order: position 0..nr_excl-1 over
 * the held cids is split into contiguous ranges, one per participant that gets
 * at least one excl cid. Range k is owned by cp_range_owner[k] and ends at the
 * cumulative position cp_range_end[k].
 */
static mut cp_range_owner: [s32; MAX_PARTS as usize] = [0; MAX_PARTS as usize]; /* exclusive range k: its owner id ... */
static mut cp_range_end: [s32; MAX_PARTS as usize] = [0; MAX_PARTS as usize]; /* ... and the cumulative position it ends at */

/* a participant in the partition: self or an attached child */
#[repr(C)]
pub struct participant {
    pub slot: s32,  /* child slot, or CID_SELF */
    pub weight: u32, /* cpu.weight */
}

/**
 * place_one - assign one excl-held cid to its owner
 * @cid: the excl-held cid to place
 * @n: its position among the excl-held cids, in [0, nr_excl)
 * @total_excl:	how many positions are owned exclusively (the rest are shared)
 *
 * Position @n below @total_excl is owned exclusively. It falls in the range
 * whose cumulative end it is under, owned by cp_range_owner[]. A position at or
 * above @total_excl is the rounding leftover which joins the shared pool.
 *
 * A separate __noinline function to help verification.
 */
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn place_one(cid: s32, n: s32, total_excl: s32) -> s32 {
    let mut owner = CID_SELF;
    let s: s32;

    if cid < 0 || cid >= SCX_QMAP_MAX_CPUS || n < 0 || n >= SCX_QMAP_MAX_CPUS || total_excl < 0 {
        scx_bpf_error(c"-ERANGE".as_ptr());
        return 0;
    }

    if n < total_excl {
        for i in 0..MAX_PARTS {
            if n < cp_range_end[i as usize] {
                owner = cp_range_owner[i as usize];
                break;
            }
        }
        qa.part.cid_owner[cid as usize] = owner;
    } else {
        s = n - total_excl;
        if s < 0 || s >= MAX_PARTS {
            scx_bpf_error(c"-ERANGE".as_ptr());
            return 0;
        }
        qa.part.shared_cids[s as usize] = cid;
        /* time-shared: dispatch resolves the live holder via rr_pos */
        qa.part.cid_owner[cid as usize] = CID_SHARED;
    }
    0
}

/**
 * compute_partition - build the cid partition from this node's held caps
 *
 * Decide each cid's owner, the shared pool and the rr rotation. __noinline to
 * help verification. See the comment at the top of the file.
 */
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn compute_partition() {
    let nr_cids = qa.nr_cids;
    let mut total_excl: s32 = 0;
    let mut nr_rr: s32 = 0;
    let mut sum_w: s32;
    let mut n: s32 = 0;
    let mut share: s32;
    let self_w: s32;
    let mut cgid_snap: [u64; MAX_SUB_SCHEDS as usize] = [0; MAX_SUB_SCHEDS as usize];
    let mut w_snap: [s32; MAX_SUB_SCHEDS as usize] = [0; MAX_SUB_SCHEDS as usize];

    if nr_cids > SCX_QMAP_MAX_CPUS {
        scx_bpf_error(c"-ERANGE".as_ptr());
        return;
    }

    /* find out the cids we hold */
    scx_bpf_sub_caps(0, SCX_CAP_ENQ, &raw mut qa.held_excl.mask);
    scx_bpf_sub_caps(0, SCX_CAP_ENQ_IMMED, &raw mut qa.held_shared.mask);
    cmask_andnot(&raw mut qa.held_shared.mask, &raw mut qa.held_excl.mask); /* held only as ENQ_IMMED */

    qa.part.nr_shared = 0;
    qa.part.nr_rr = 0;
    qa.part.rr_pos = 0;

    let nr_excl = cmask_weight(&raw mut qa.held_excl.mask);
    qa.part.nr_excl = nr_excl;

    /* no excl cid: held_shared stays self-local, the rest unheld */
    if nr_excl == 0 {
        for cid in 0..nr_cids {
            if cmask_test(cid, &raw mut qa.held_shared.mask) {
                qa.part.cid_owner[cid as usize] = CID_SELF;
            } else {
                qa.part.cid_owner[cid as usize] = CID_NONE;
            }
        }
        return;
    }

    /*
     * Snapshot membership and weights so the sum_w and share loops agree. A
     * mid-compute change would otherwise wrap nr_shared negative. The self
     * weight is fixed at the default: a cgroup's weight is its parent's
     * knob, not the scheduler's own business.
     */
    self_w = 100;
    for i in 0..MAX_SUB_SCHEDS {
        cgid_snap[i as usize] = qa.sub_sched_ctxs[i as usize].cgroup_id;
        w_snap[i as usize] = if cgid_snap[i as usize] != 0 {
            if qa.sub_sched_ctxs[i as usize].weight != 0 {
                qa.sub_sched_ctxs[i as usize].weight as s32
            } else {
                100
            }
        } else {
            0
        };
    }

    /*
     * Participants are self plus each child. Give each a fixed range/rr
     * slot: self at slot 0, child i at slot i+1.
     *
     * sum_w totals every participant's weight.
     */
    sum_w = self_w;
    for i in 0..MAX_SUB_SCHEDS {
        barrier_var(sum_w);
        sum_w += w_snap[i as usize];
    }

    /*
     * Split [0, nr_excl) into one contiguous range per participant, each
     * the floor of its weight share. cp_range_owner[]/cp_range_end[] record
     * each range's owner and cumulative end, total_excl counts the
     * exclusive slots, and the rest (nr_excl - total_excl) are shared.
     * rr_slots[] lists every participant for the round-robin.
     */
    share = (nr_excl as u64 * self_w as u64 / sum_w as u64) as s32;
    total_excl += share;
    cp_range_owner[0] = CID_SELF;
    cp_range_end[0] = total_excl;
    qa.part.rr_slots[nr_rr as usize] = 0; /* self holds slot 0 (cgid 0 = no grant) */
    nr_rr += 1;

    for i in 0..MAX_SUB_SCHEDS {
        let cgid = cgid_snap[i as usize];
        let w = w_snap[i as usize];

        barrier_var(total_excl);
        share = (nr_excl as u64 * w as u64 / sum_w as u64) as s32;
        total_excl += share;
        cp_range_owner[(i + 1) as usize] = if cgid != 0 { i } else { CID_NONE };
        cp_range_end[(i + 1) as usize] = total_excl;

        if cgid != 0 {
            barrier_var(nr_rr);
            if nr_rr < 0 || nr_rr >= MAX_PARTS {
                scx_bpf_error(c"-ERANGE".as_ptr());
                return;
            }
            qa.part.rr_slots[nr_rr as usize] = cgid;
            nr_rr += 1;
        }
    }

    /* assign each cid: held-excl by position, the rest self/none */
    for cid in 0..nr_cids {
        if cmask_test(cid, &raw mut qa.held_excl.mask) {
            place_one(cid, n, total_excl);
            n += 1;
            barrier_var(n);
        } else if cmask_test(cid, &raw mut qa.held_shared.mask) {
            qa.part.cid_owner[cid as usize] = CID_SELF; /* time-share, self-local */
        } else {
            qa.part.cid_owner[cid as usize] = CID_NONE; /* not held */
        }
    }

    qa.part.nr_shared = nr_excl - total_excl;
    qa.part.nr_rr = nr_rr;
}

/*
 * Charge elapsed wall time to each cid's current owner. Runs under the
 * partition guard before every ownership change and from the stats flush, so
 * alloc_ns[] reflects the layout that was in effect. Shared-pool time is
 * charged to the live round-robin holder.
 */
#[inline(never)]
unsafe fn account_alloc() {
    let now = bpf_ktime_get_ns();
    let mut rr_owner = CID_SELF;
    let nr_cids = qa.nr_cids;
    let delta: u64;

    if nr_cids < 0 || nr_cids > SCX_QMAP_MAX_CPUS {
        scx_bpf_error(c"-ERANGE".as_ptr());
        return;
    }

    /* first call starts the clock */
    if qa.alloc_ts == 0 {
        qa.alloc_ts = now;
        return;
    }
    delta = now - qa.alloc_ts;
    qa.alloc_ts = now;
    qa.alloc_window_ns += delta;

    /* resolve the live shared-pool holder to an owner id */
    if qa.part.nr_shared != 0 && qa.part.nr_rr != 0 {
        let pos = qa.part.rr_pos as u32;
        let cgid = if pos < MAX_PARTS as u32 {
            qa.part.rr_slots[pos as usize]
        } else {
            0
        };

        if cgid != 0 {
            rr_owner = CID_NONE;
            for i in 0..MAX_SUB_SCHEDS {
                if qa.sub_sched_ctxs[i as usize].cgroup_id == cgid {
                    rr_owner = i;
                }
            }
        }
    }

    for cid in 0..nr_cids {
        let mut owner = qa.part.cid_owner[cid as usize];

        if owner == CID_SHARED {
            owner = rr_owner;
        }
        if owner >= 0 && owner < MAX_SUB_SCHEDS {
            qa.alloc_ns[owner as usize] += delta;
        } else if owner == CID_SELF {
            qa.self_alloc_ns += delta;
        }
    }
}

/*
 * apply_partition - execute the plan compute_partition() built
 *
 * Turn the owner map into the per-child, shared and self cmasks and issue the
 * grant/revoke kfuncs as a delta against each child's previous grant. If no
 * excl cid, evict every child.
 */
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn apply_partition() {
    let nr_cids = qa.nr_cids;
    let nr_shared = qa.part.nr_shared;

    if nr_cids < 0 || nr_cids > SCX_QMAP_MAX_CPUS || nr_shared < 0 || nr_shared > MAX_PARTS {
        scx_bpf_error(c"-ERANGE".as_ptr());
        return;
    }

    /* no excl cpu: run own tasks on the held shares, evict children */
    if qa.part.nr_excl == 0 {
        cmask_copy(&raw mut qa.self_cids.mask, &raw mut qa.held_shared.mask);
        for i in 0..MAX_SUB_SCHEDS {
            if qa.sub_sched_ctxs[i as usize].cgroup_id != 0 {
                scx_bpf_sub_kill(
                    qa.sub_sched_ctxs[i as usize].cgroup_id,
                    c"parent holds no excl cpu to distribute".as_ptr(),
                );
            }
        }
        return;
    }

    /*
     * Snapshot the old pool. The per-child revoke below clears ENQ_IMMED on
     * the previously-granted pool, so a cid that left the pool (now a
     * sibling's excl) doesn't keep a stale ENQ_IMMED on its last holder.
     */
    cmask_copy(&raw mut qa.prev_rr_cids.mask, &raw mut qa.rr_cids.mask);

    /* turn the owner map into the rr pool, per-child excl, and self sets */
    cmask_init(&raw mut qa.rr_cids.mask, 0, nr_cids as u32);
    cmask_init(&raw mut qa.self_cids.mask, 0, nr_cids as u32);

    /* snapshot each child's grant, then rebuild the new sets below */
    for i in 0..MAX_SUB_SCHEDS {
        cmask_copy(
            &raw mut qa.sub_sched_ctxs[i as usize].prev_granted.mask,
            &raw mut qa.sub_sched_ctxs[i as usize].granted_cids.mask,
        );
        cmask_init(&raw mut qa.sub_sched_ctxs[i as usize].granted_cids.mask, 0, nr_cids as u32);
    }

    for i in 0..nr_shared {
        cmask_set(qa.part.shared_cids[i as usize], &raw mut qa.rr_cids.mask);
    }
    for cid in 0..nr_cids {
        let o = qa.part.cid_owner[cid as usize];

        if cmask_test(cid, &raw mut qa.rr_cids.mask) {
            continue;
        }
        if o >= 0 && o < MAX_SUB_SCHEDS {
            cmask_set(cid, &raw mut qa.sub_sched_ctxs[o as usize].granted_cids.mask);
        } else if o == CID_SELF {
            cmask_set(cid, &raw mut qa.self_cids.mask);
        }
    }

    /*
     * Apply each child's exclusive cids as a delta against its previous
     * grant. Separately clear the previous shared grant (ENQ_IMMED on the
     * old pool), covering cids still pooled and cids that left for a
     * sibling's excl. The current holder is granted the new pool below.
     */
    for i in 0..MAX_SUB_SCHEDS {
        let ssc = &raw mut qa.sub_sched_ctxs[i as usize];
        let cgid = (*ssc).cgroup_id;

        if cgid == 0 {
            continue;
        }

        cmask_copy(&raw mut qa.to_revoke_cids.mask, &raw mut (*ssc).prev_granted.mask);
        cmask_andnot(&raw mut qa.to_revoke_cids.mask, &raw mut (*ssc).granted_cids.mask);
        cmask_copy(&raw mut qa.to_grant_cids.mask, &raw mut (*ssc).granted_cids.mask);
        cmask_andnot(&raw mut qa.to_grant_cids.mask, &raw mut (*ssc).prev_granted.mask);

        scx_bpf_sub_revoke(cgid, SCX_CAP_ENQ_IMMED | SCX_CAP_PERF, &raw mut qa.prev_rr_cids.mask);
        scx_bpf_sub_revoke(
            cgid,
            SCX_CAP_ENQ | SCX_CAP_PREEMPT | SCX_CAP_ENQ_IMMED | SCX_CAP_PERF,
            &raw mut qa.to_revoke_cids.mask,
        );
        scx_bpf_sub_grant(
            cgid,
            SCX_CAP_ENQ | SCX_CAP_PREEMPT | SCX_CAP_ENQ_IMMED | SCX_CAP_PERF,
            &raw mut qa.to_grant_cids.mask,
            core::ptr::null_mut(),
        );
    }

    /* the current holder of the shared pool gets ENQ_IMMED on all of it */
    if nr_shared != 0 {
        let pos = qa.part.rr_pos;
        let holder_cgid: u64;

        if pos < 0 || pos >= MAX_PARTS {
            scx_bpf_error(c"-ERANGE".as_ptr());
            return;
        }

        holder_cgid = qa.part.rr_slots[pos as usize]; /* 0 = self, nothing to grant */
        if holder_cgid != 0 {
            scx_bpf_sub_grant(
                holder_cgid,
                SCX_CAP_ENQ_IMMED | SCX_CAP_PERF,
                &raw mut qa.rr_cids.mask,
                core::ptr::null_mut(),
            );
        }
    }
}

/*
 * Recompute the split off the node's held caps and apply it. The contexts this
 * runs from (the sub-sched and cgroup callbacks, the rr timer) are not
 * serialized by the kernel, so a single runner does the work. A caller that
 * finds the guard held leaves part_pending set; the holder drains it before
 * releasing, with the rr timer as a backstop.
 */
unsafe fn redistribute() {
    __sync_fetch_and_or(&raw mut part_pending, 1);

    if !part_try_start() {
        return;
    }

    for _i in 0..1024 {
        __sync_fetch_and_and(&raw mut part_pending, 0);
        /* charge elapsed time to the current partition before rebuilding it */
        account_alloc();
        compute_partition();
        apply_partition();
        if __sync_fetch_and_or(&raw mut part_pending, 0) == 0 {
            break;
        }
    }

    part_end();
}

/*
 * Userspace pokes this (PROG_RUN) to bring alloc_ns[] current before reading
 * it for the stats display. Skipping when the partition guard is held is
 * fine - alloc_ts is untouched, so the elapsed time is charged next time.
 */
#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn flush_alloc(_ctx: *mut core::ffi::c_void) -> s32 {
    if part_try_start() {
        account_alloc();
        part_end();
    }
    0
}

/*
 * Hand the shared pool to the next participant in the rotation. Self's turn
 * just revokes the pool back to this sched. A child's turn grants it ENQ_IMMED
 * on the entire pool. As only excl-held cids are time-shared, a wall-clock
 * rotation works. Driven by the round-robin timer.
 */
unsafe fn rr_advance() {
    let nr_shared: s32;
    let old_pos: s32;
    let new_pos: s32;
    let old_cgid: u64;
    let new_cgid: u64;
    let nr_rr: u32; /* unsigned for % */

    /* a redistribute holds the partition and rebuilds the pool, so skip */
    if !part_try_start() {
        return;
    }

    nr_rr = qa.part.nr_rr as u32;
    nr_shared = qa.part.nr_shared;

    if nr_shared < 0 || nr_shared > MAX_PARTS {
        scx_bpf_error(c"-ERANGE".as_ptr());
        return;
    }

    if nr_shared != 0 && nr_rr >= 2 {
        /* close out the outgoing holder's pool time */
        account_alloc();

        old_pos = qa.part.rr_pos;
        new_pos = ((old_pos as u32 + 1) % nr_rr) as s32;
        old_cgid = qa.part.rr_slots[old_pos as usize];
        new_cgid = qa.part.rr_slots[new_pos as usize];
        qa.part.rr_pos = new_pos;

        /*
         * Move the ENQ_IMMED cap to the next participant. The shared
         * cids stay marked CID_SHARED. qmap_dispatch() resolves the
         * live holder via rr_pos without the guard, so a dispatch
         * racing this handoff may reenqueue a task once. Harmless for a
         * time-share.
         */
        if old_cgid != 0 {
            scx_bpf_sub_revoke(
                old_cgid,
                SCX_CAP_ENQ_IMMED | SCX_CAP_PERF,
                &raw mut qa.rr_cids.mask,
            );
        }
        if new_cgid != 0 {
            scx_bpf_sub_grant(
                new_cgid,
                SCX_CAP_ENQ_IMMED | SCX_CAP_PERF,
                &raw mut qa.rr_cids.mask,
                core::ptr::null_mut(),
            );
        }
    }

    part_end();

    /* a resplit queued while we held the guard supersedes this rotation */
    if __sync_fetch_and_or(&raw mut part_pending, 0) != 0 {
        redistribute();
    }
}

/* advance the time-shared cid pool every round_robin_ns */
unsafe extern "C" fn round_robin_timerfn(
    _map: *mut core::ffi::c_void,
    _key: *mut s32,
    timer: *mut bpf_timer,
) -> s32 {
    rr_advance();
    bpf_timer_start(timer, round_robin_ns, 0);
    0
}

/*
 * Custom cid layout for the cid-override test. On invalid input the kfunc
 * scx_error()s and aborts the scheduler.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_init_cids() -> s32 {
    let nr_cpu_ids = scx_bpf_nr_cpu_ids();

    if cid_override_mode == 0 {
        return 0;
    }

    /* the arena arrays are sized SCX_QMAP_MAX_CPUS */
    if nr_cpu_ids > SCX_QMAP_MAX_CPUS as u32 {
        scx_bpf_error(
            c"nr_cpu_ids=%u exceeds SCX_QMAP_MAX_CPUS=%d".as_ptr(),
            nr_cpu_ids,
            SCX_QMAP_MAX_CPUS,
        );
        return -EINVAL;
    }

    scx_bpf_cid_override(
        qa.cid_override_cpu_to_cid.as_mut_ptr(),
        nr_cpu_ids,
        qa.cid_override_shard_start.as_mut_ptr(),
        cid_override_nr_shards,
    );
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_init() -> s32 {
    let slab: *mut u8;
    let nr_pages: u32;
    let mut key: u32 = 0;
    let nr_cids: u32;
    let nr_cpu_ids: u32;
    let mut timer: *mut bpf_timer;
    let mut ret: s32;

    nr_cids = scx_bpf_nr_cids();
    nr_cpu_ids = scx_bpf_nr_cpu_ids();

    if nr_cids > SCX_QMAP_MAX_CPUS as u32 {
        scx_bpf_error(
            c"nr_cids=%u exceeds SCX_QMAP_MAX_CPUS=%d".as_ptr(),
            nr_cids,
            SCX_QMAP_MAX_CPUS,
        );
        return -EINVAL;
    }
    if nr_cpu_ids > SCX_QMAP_MAX_CPUS as u32 {
        scx_bpf_error(
            c"nr_cpu_ids=%u exceeds SCX_QMAP_MAX_CPUS=%d".as_ptr(),
            nr_cpu_ids,
            SCX_QMAP_MAX_CPUS,
        );
        return -EINVAL;
    }

    /*
     * Allocate the task_ctx slab in arena and thread the entire slab onto
     * the free list. max_tasks is set by userspace before load. Each entry
     * is TASK_CTX_STRIDE bytes - task_ctx's trailing cpus_allowed flex
     * array extends into the stride tail.
     */
    if max_tasks == 0 {
        scx_bpf_error(c"max_tasks must be > 0".as_ptr());
        return -EINVAL;
    }

    nr_pages = (max_tasks * TASK_CTX_STRIDE as u32 + PAGE_SIZE - 1) / PAGE_SIZE;
    slab = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), nr_pages, NUMA_NO_NODE, 0)
        as *mut u8;
    if slab.is_null() {
        scx_bpf_error(c"failed to allocate task_ctx slab".as_ptr());
        return -ENOMEM;
    }
    qa.task_ctxs = slab as *mut task_ctx_t;

    for i in 0..5 {
        qa.fifos[i as usize].idx = i;
    }

    for i in 0..max_tasks {
        let cur = slab.add(i as usize * TASK_CTX_STRIDE) as *mut task_ctx_t;
        let next = if i + 1 < max_tasks {
            slab.add((i + 1) as usize * TASK_CTX_STRIDE) as *mut task_ctx_t
        } else {
            core::ptr::null_mut()
        };
        (*cur).next_free = next;
    }
    qa.task_free_head = slab as *mut task_ctx_t;

    /* cache the cid count, trusted to be <= SCX_QMAP_MAX_CPUS hereafter */
    qa.nr_cids = nr_cids as s32;

    /* cmasks are embedded in qa, so they only need initializing */
    cmask_init(&raw mut qa.idle_cids.mask, 0, nr_cids);
    cmask_init(&raw mut qa.rr_cids.mask, 0, nr_cids);
    cmask_init(&raw mut qa.prev_rr_cids.mask, 0, nr_cids);
    cmask_init(&raw mut qa.self_cids.mask, 0, nr_cids);
    cmask_init(&raw mut qa.to_revoke_cids.mask, 0, nr_cids);
    cmask_init(&raw mut qa.to_grant_cids.mask, 0, nr_cids);
    cmask_init(&raw mut qa.held_excl.mask, 0, nr_cids);
    cmask_init(&raw mut qa.held_shared.mask, 0, nr_cids);

    scx_bpf_sub_caps(0, SCX_CAP_ENQ, &raw mut qa.held_excl.mask);
    scx_bpf_sub_caps(0, SCX_CAP_ENQ_IMMED, &raw mut qa.held_shared.mask);
    cmask_andnot(&raw mut qa.held_shared.mask, &raw mut qa.held_excl.mask);

    for i in 0..MAX_SUB_SCHEDS {
        cmask_init(&raw mut qa.sub_sched_ctxs[i as usize].granted_cids.mask, 0, nr_cids);
        cmask_init(&raw mut qa.sub_sched_ctxs[i as usize].prev_granted.mask, 0, nr_cids);
    }

    /*
     * The root starts holding every cid. qmap_sub_ecaps_updated() maintains
     * per-cid shared state as effective caps settle, and redistribute()
     * rebuilds owner and self from held caps. A non-root node starts with
     * nothing.
     */
    for i in 0..nr_cids {
        if sub_cgroup_id == 0 {
            cmask_set(i as s32, &raw mut qa.self_cids.mask);
            qa.part.cid_owner[i as usize] = CID_SELF;
        } else {
            qa.part.cid_owner[i as usize] = CID_NONE;
        }
    }
    qa.part.nr_shared = 0;

    ret = scx_bpf_create_dsq(SHARED_DSQ, -1);
    if ret != 0 {
        scx_bpf_error(c"failed to create DSQ %d (%d)".as_ptr(), SHARED_DSQ, ret);
        return ret;
    }

    ret = scx_bpf_create_dsq(HIGHPRI_DSQ, -1);
    if ret != 0 {
        scx_bpf_error(c"failed to create DSQ %d (%d)".as_ptr(), HIGHPRI_DSQ, ret);
        return ret;
    }

    ret = scx_bpf_create_dsq(LOWPRI_DSQ, -1);
    if ret != 0 {
        return ret;
    }

    timer = bpf_map_lookup_elem(&raw mut monitor_timer, &raw mut key) as *mut bpf_timer;
    if timer.is_null() {
        return -ESRCH;
    }
    bpf_timer_init(timer, &raw mut monitor_timer, CLOCK_MONOTONIC);
    bpf_timer_set_callback(timer, monitor_timerfn);
    ret = bpf_timer_start(timer, ONE_SEC_IN_NS, 0);
    if ret != 0 {
        return ret;
    }

    if __COMPAT_has_generic_reenq() {
        /* see lowpri_timerfn() */
        timer = bpf_map_lookup_elem(&raw mut lowpri_timer, &raw mut key) as *mut bpf_timer;
        if timer.is_null() {
            return -ESRCH;
        }
        bpf_timer_init(timer, &raw mut lowpri_timer, CLOCK_MONOTONIC);
        bpf_timer_set_callback(timer, lowpri_timerfn);
        ret = bpf_timer_start(timer, LOWPRI_INTV_NS, 0);
        if ret != 0 {
            return ret;
        }
    }

    /* sub-sched: drive the boundary-cid round-robin from a bpf timer */
    timer = bpf_map_lookup_elem(&raw mut round_robin_timer, &raw mut key) as *mut bpf_timer;
    if timer.is_null() {
        return -ESRCH;
    }
    bpf_timer_init(timer, &raw mut round_robin_timer, CLOCK_MONOTONIC);
    bpf_timer_set_callback(timer, round_robin_timerfn);
    ret = bpf_timer_start(timer, round_robin_ns, 0);
    if ret != 0 {
        return ret;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_exit(ei: *mut scx_exit_info) {
    UEI_RECORD(&raw mut uei, ei);
}

/*
 * Seed a new sub slot with the cgroup's current weight. The kernel delivers
 * ops.cpuctl_set_weight() only on value-changing writes, so a weight set
 * before the sub attached would otherwise go unnoticed.
 */
unsafe fn cgrp_cur_weight(cgid: u64) -> u32 {
    let css: *mut cgroup_subsys_state;
    let cgrp: *mut cgroup;
    let mut weight: u32 = 100;

    cgrp = bpf_cgroup_from_id(cgid);
    if cgrp.is_null() {
        return weight;
    }

    css = BPF_CORE_READ_cgroup_subsys(cgrp, cpu_cgrp_id);
    if !css.is_null() {
        let tg = container_of_task_group_from_css(css);
        let w = BPF_CORE_READ_task_group_scx_weight(tg);

        if w != 0 {
            weight = w;
        }
    }
    bpf_cgroup_release(cgrp);
    weight
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_sub_attach(args: *mut scx_sub_attach_args) -> s32 {
    /* as long as there is at least one excl cpu, children can attach */
    if cmask_weight(&raw mut qa.held_excl.mask) == 0 {
        return -ENOSPC;
    }

    for i in 0..MAX_SUB_SCHEDS {
        if qa.sub_sched_ctxs[i as usize].cgroup_id != 0 {
            continue;
        }

        qa.sub_sched_ctxs[i as usize].cgroup_id = (*(*args).ops).sub_cgroup_id;
        qa.sub_sched_ctxs[i as usize].weight = cgrp_cur_weight((*(*args).ops).sub_cgroup_id);
        qa.nr_sub_scheds += 1;
        bpf_printk(c"attaching sub-sched[%d] on %s".as_ptr(), i, (*args).cgroup_path);
        redistribute();
        return 0;
    }

    -ENOSPC
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_sub_detach(args: *mut scx_sub_detach_args) {
    for i in 0..MAX_SUB_SCHEDS {
        if qa.sub_sched_ctxs[i as usize].cgroup_id != (*(*args).ops).sub_cgroup_id {
            continue;
        }

        qa.sub_sched_ctxs[i as usize].cgroup_id = 0;
        qa.sub_sched_ctxs[i as usize].weight = 100;
        cmask_init(&raw mut qa.sub_sched_ctxs[i as usize].granted_cids.mask, 0, qa.nr_cids as u32);
        qa.nr_sub_scheds -= 1;
        bpf_printk(c"detaching sub-sched[%d] on %s".as_ptr(), i, (*args).cgroup_path);
        redistribute();
        break;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_sub_caps_updated(_cmask: *const scx_cmask, _caps: u64) {
    /* our held caps changed, redistribute */
    redistribute();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn qmap_sub_ecaps_updated(cid: s32, _before: u64, after: u64) {
    /*
     * Effective caps updated. Track which cids hold shared caps so a self
     * task placed there enqueues IMMED.
     */
    if (after & SCX_CAP_ENQ_IMMED) != 0 {
        qa.cid_shared[cid as usize] = if (after & SCX_CAP_ENQ) != 0 { 0 } else { 1 };
    } else {
        qa.cid_shared[cid as usize] = 0;
    }
}

#[repr(C)]
pub struct scx_ops_cid {
    pub flags: u64,
    pub select_cid: *const core::ffi::c_void,
    pub enqueue: *const core::ffi::c_void,
    pub dequeue: *const core::ffi::c_void,
    pub dispatch: *const core::ffi::c_void,
    pub tick: *const core::ffi::c_void,
    pub core_sched_before: *const core::ffi::c_void,
    pub set_cmask: *const core::ffi::c_void,
    pub update_idle: *const core::ffi::c_void,
    pub init_task: *const core::ffi::c_void,
    pub exit_task: *const core::ffi::c_void,
    pub dump: *const core::ffi::c_void,
    pub dump_cid: *const core::ffi::c_void,
    pub dump_task: *const core::ffi::c_void,
    pub cpuctl_init: *const core::ffi::c_void,
    pub cpuctl_set_weight: *const core::ffi::c_void,
    pub cpuctl_set_bandwidth: *const core::ffi::c_void,
    pub cpuctl_move: *const core::ffi::c_void,
    pub sub_attach: *const core::ffi::c_void,
    pub sub_detach: *const core::ffi::c_void,
    pub sub_caps_updated: *const core::ffi::c_void,
    pub sub_ecaps_updated: *const core::ffi::c_void,
    pub init_cids: *const core::ffi::c_void,
    pub init: *const core::ffi::c_void,
    pub exit: *const core::ffi::c_void,
    pub timeout_ms: u32,
    pub name: [u8; 5],
}

#[unsafe(no_mangle)]
pub static qmap_ops: scx_ops_cid = scx_ops_cid {
    flags: SCX_OPS_ENQ_EXITING | SCX_OPS_TID_TO_TASK,
    select_cid: qmap_select_cid as *const core::ffi::c_void,
    enqueue: qmap_enqueue as *const core::ffi::c_void,
    dequeue: qmap_dequeue as *const core::ffi::c_void,
    dispatch: qmap_dispatch as *const core::ffi::c_void,
    tick: qmap_tick as *const core::ffi::c_void,
    core_sched_before: qmap_core_sched_before as *const core::ffi::c_void,
    set_cmask: qmap_set_cmask as *const core::ffi::c_void,
    update_idle: qmap_update_idle as *const core::ffi::c_void,
    init_task: qmap_init_task as *const core::ffi::c_void,
    exit_task: qmap_exit_task as *const core::ffi::c_void,
    dump: qmap_dump as *const core::ffi::c_void,
    dump_cid: qmap_dump_cid as *const core::ffi::c_void,
    dump_task: qmap_dump_task as *const core::ffi::c_void,
    cpuctl_init: qmap_cpuctl_init as *const core::ffi::c_void,
    cpuctl_set_weight: qmap_cpuctl_set_weight as *const core::ffi::c_void,
    cpuctl_set_bandwidth: qmap_cpuctl_set_bandwidth as *const core::ffi::c_void,
    cpuctl_move: qmap_cpuctl_move as *const core::ffi::c_void,
    sub_attach: qmap_sub_attach as *const core::ffi::c_void,
    sub_detach: qmap_sub_detach as *const core::ffi::c_void,
    sub_caps_updated: qmap_sub_caps_updated as *const core::ffi::c_void,
    sub_ecaps_updated: qmap_sub_ecaps_updated as *const core::ffi::c_void,
    init_cids: qmap_init_cids as *const core::ffi::c_void,
    init: qmap_init as *const core::ffi::c_void,
    exit: qmap_exit as *const core::ffi::c_void,
    timeout_ms: 5000u32,
    name: *b"qmap\0",
};
