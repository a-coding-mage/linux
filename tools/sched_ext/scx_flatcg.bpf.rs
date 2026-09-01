/* SPDX-License-Identifier: GPL-2.0 */
/*
 * A demo sched_ext flattened cgroup hierarchy scheduler. It implements
 * hierarchical weight-based cgroup CPU control by flattening the cgroup
 * hierarchy into a single layer by compounding the active weight share at each
 * level. Consider the following hierarchy with weights in parentheses:
 *
 * R + A (100) + B (100)
 *   |         \ C (100)
 *   \ D (200)
 *
 * Ignoring the root and threaded cgroups, only B, C and D can contain tasks.
 * Let's say all three have runnable tasks. The total share that each of these
 * three cgroups is entitled to can be calculated by compounding its share at
 * each level.
 *
 * For example, B is competing against C and in that competition its share is
 * 100/(100+100) == 1/2. At its parent level, A is competing against D and A's
 * share in that competition is 100/(200+100) == 1/3. B's eventual share in the
 * system can be calculated by multiplying the two shares, 1/2 * 1/3 == 1/6. C's
 * eventual share is the same at 1/6. D is only competing at the top level and
 * its share is 200/(100+200) == 2/3.
 *
 * So, instead of hierarchically scheduling level-by-level, we can consider it
 * as B, C and D competing each other with respective share of 1/6, 1/6 and 2/3
 * and keep updating the eventual shares as the cgroups' runnable states change.
 *
 * This flattening of hierarchy can bring a substantial performance gain when
 * the cgroup hierarchy is nested multiple levels. in a simple benchmark using
 * wrk[8] on apache serving a CGI script calculating sha1sum of a small file, it
 * outperforms CFS by ~3% with CPU controller disabled and by ~10% with two
 * apache instances competing with 2:1 weight ratio nested four level deep.
 *
 * However, the gain comes at the cost of not being able to properly handle
 * thundering herd of cgroups. For example, if many cgroups which are nested
 * behind a low priority parent cgroup wake up around the same time, they may be
 * able to consume more CPU cycles than they are entitled to. In many use cases,
 * this isn't a real concern especially given the performance gain. Also, there
 * are ways to mitigate the problem further by e.g. introducing an extra
 * scheduling layer on cgroup delegation boundaries.
 *
 * The scheduler first picks the cgroup to run and then schedule the tasks
 * within by using nested weighted vtime scheduling by default. The
 * cgroup-internal scheduling can be switched to FIFO with the -f option.
 */

/* Dependencies from <scx/common.bpf.h> and "scx_flatcg.h" are expected. */

type u32 = __u32;
type u64 = __u64;
type s32 = __s32;
type s64 = __s64;

/*
 * Maximum amount of retries to find a valid cgroup.
 */
pub const FALLBACK_DSQ: u64 = 0;
pub const CGROUP_MAX_RETRIES: u32 = 1024;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static nr_cpus: u32 = 32; /* !0 for veristat, set during init */
#[no_mangle]
pub static cgrp_slice_ns: u64 = 0;
#[no_mangle]
pub static fifo_sched: bool = false;

#[no_mangle]
pub static mut cvtime_now: u64 = 0;

/* UEI_DEFINE(uei); */
extern "C" {
    static mut uei: uei;
}

#[repr(C)]
pub struct StatsMap {
    _priv: [u8; 0],
}

/* BPF_MAP_TYPE_PERCPU_ARRAY, key u32, value u64, max_entries FCG_NR_STATS */
#[no_mangle]
#[link_section = ".maps"]
pub static mut stats: StatsMap = StatsMap { _priv: [] };

unsafe fn stat_inc(idx: fcg_stat_idx) {
    let mut idx_v: u32 = idx as u32;

    let cnt_p = bpf_map_lookup_elem(&raw mut stats as *mut _ as *mut _, &mut idx_v as *mut _ as *mut _) as *mut u64;
    if !cnt_p.is_null() {
        *cnt_p = (*cnt_p).wrapping_add(1);
    }
}

#[repr(C)]
pub struct fcg_cpu_ctx {
    pub cur_cgid: u64,
    pub cur_at: u64,
}

#[repr(C)]
pub struct CpuCtxMap {
    _priv: [u8; 0],
}

/* BPF_MAP_TYPE_PERCPU_ARRAY, key u32, value struct fcg_cpu_ctx, max_entries 1 */
#[no_mangle]
#[link_section = ".maps"]
pub static mut cpu_ctx: CpuCtxMap = CpuCtxMap { _priv: [] };

#[repr(C)]
pub struct CgrpCtxMap {
    _priv: [u8; 0],
}

/* BPF_MAP_TYPE_CGRP_STORAGE, BPF_F_NO_PREALLOC, key int, value struct fcg_cgrp_ctx */
#[no_mangle]
#[link_section = ".maps"]
pub static mut cgrp_ctx: CgrpCtxMap = CgrpCtxMap { _priv: [] };

#[repr(C)]
pub struct cgv_node {
    pub rb_node: bpf_rb_node,
    pub cvtime: __u64,
    pub cgid: __u64,
}

#[no_mangle]
pub static mut cgv_tree_lock: bpf_spin_lock = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut cgv_tree: bpf_rb_root = unsafe { core::mem::zeroed() }; /* __contains(cgv_node, rb_node) */

#[repr(C)]
pub struct cgv_node_stash {
    pub node: *mut cgv_node, /* __kptr */
}

#[repr(C)]
pub struct CgvNodeStashMap {
    _priv: [u8; 0],
}

/* BPF_MAP_TYPE_HASH, max_entries 16384, key __u64, value struct cgv_node_stash */
#[no_mangle]
#[link_section = ".maps"]
pub static mut cgv_node_stash: CgvNodeStashMap = CgvNodeStashMap { _priv: [] };

#[repr(C)]
pub struct fcg_task_ctx {
    pub bypassed_at: u64,
}

#[repr(C)]
pub struct TaskCtxMap {
    _priv: [u8; 0],
}

/* BPF_MAP_TYPE_TASK_STORAGE, BPF_F_NO_PREALLOC, key int, value struct fcg_task_ctx */
#[no_mangle]
#[link_section = ".maps"]
pub static mut task_ctx: TaskCtxMap = TaskCtxMap { _priv: [] };

/* gets inc'd on weight tree changes to expire the cached hweights */
#[no_mangle]
pub static mut hweight_gen: u64 = 1;

unsafe fn div_round_up(dividend: u64, divisor: u64) -> u64 {
    dividend.wrapping_add(divisor).wrapping_sub(1) / divisor
}

unsafe extern "C" fn cgv_node_less(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let cgc_a = container_of_cgv_node_rb_node(a);
    let cgc_b = container_of_cgv_node_rb_node(b as *mut bpf_rb_node);

    (*cgc_a).cvtime < (*cgc_b).cvtime
}

unsafe fn find_cpu_ctx() -> *mut fcg_cpu_ctx {
    let mut idx: u32 = 0;

    let cpuc = bpf_map_lookup_elem(&raw mut cpu_ctx as *mut _ as *mut _, &mut idx as *mut _ as *mut _) as *mut fcg_cpu_ctx;
    if cpuc.is_null() {
        scx_bpf_error(c"cpu_ctx lookup failed".as_ptr());
        return core::ptr::null_mut();
    }
    cpuc
}

unsafe fn find_cgrp_ctx(cgrp: *mut cgroup) -> *mut fcg_cgrp_ctx {
    let cgc = bpf_cgrp_storage_get(&raw mut cgrp_ctx as *mut _ as *mut _, cgrp, 0, 0) as *mut fcg_cgrp_ctx;
    if cgc.is_null() {
        scx_bpf_error(c"cgrp_ctx lookup failed for cgid %llu".as_ptr(), (*(*cgrp).kn).id);
        return core::ptr::null_mut();
    }
    cgc
}

unsafe fn find_ancestor_cgrp_ctx(mut cgrp: *mut cgroup, level: i32) -> *mut fcg_cgrp_ctx {
    cgrp = bpf_cgroup_ancestor(cgrp, level);
    if cgrp.is_null() {
        scx_bpf_error(c"ancestor cgroup lookup failed".as_ptr());
        return core::ptr::null_mut();
    }

    let cgc = find_cgrp_ctx(cgrp);
    if cgc.is_null() {
        scx_bpf_error(c"ancestor cgrp_ctx lookup failed".as_ptr());
    }
    bpf_cgroup_release(cgrp);
    cgc
}

unsafe fn cgrp_refresh_hweight(cgrp: *mut cgroup, cgc: *mut fcg_cgrp_ctx) {
    if (*cgc).nr_active == 0 {
        stat_inc(FCG_STAT_HWT_SKIP);
        return;
    }

    if (*cgc).hweight_gen == hweight_gen {
        stat_inc(FCG_STAT_HWT_CACHE);
        return;
    }

    stat_inc(FCG_STAT_HWT_UPDATES);
    let mut level: i32 = 0;
    while level < (*cgrp).level + 1 {
        let cgc = find_ancestor_cgrp_ctx(cgrp, level);
        let is_active: bool;
        if cgc.is_null() {
            break;
        }

        if level == 0 {
            (*cgc).hweight = FCG_HWEIGHT_ONE;
            (*cgc).hweight_gen = hweight_gen;
        } else {
            let pcgc = find_ancestor_cgrp_ctx(cgrp, level - 1);
            if pcgc.is_null() {
                break;
            }

            /*
             * We can be opportunistic here and not grab the
             * cgv_tree_lock and deal with the occasional races.
             * However, hweight updates are already cached and
             * relatively low-frequency. Let's just do the
             * straightforward thing.
             */
            bpf_spin_lock(&raw mut cgv_tree_lock);
            is_active = (*cgc).nr_active != 0;
            if is_active {
                (*cgc).hweight_gen = (*pcgc).hweight_gen;
                (*cgc).hweight =
                    div_round_up((*pcgc).hweight.wrapping_mul((*cgc).weight as u64),
                                 (*pcgc).child_weight_sum);
            }
            bpf_spin_unlock(&raw mut cgv_tree_lock);

            if !is_active {
                stat_inc(FCG_STAT_HWT_RACE);
                break;
            }
        }
        level += 1;
    }
}

unsafe fn cgrp_cap_budget(cgv_node: *mut cgv_node, cgc: *mut fcg_cgrp_ctx) {
    /*
     * A node which is on the rbtree can't be pointed to from elsewhere yet
     * and thus can't be updated and repositioned. Instead, we collect the
     * vtime deltas separately and apply it asynchronously here.
     */
    let delta = __sync_fetch_and_and(&raw mut (*cgc).cvtime_delta, 0);
    let mut cvtime = (*cgv_node).cvtime.wrapping_add(delta);

    /*
     * Allow a cgroup to carry the maximum budget proportional to its
     * hweight such that a full-hweight cgroup can immediately take up half
     * of the CPUs at the most while staying at the front of the rbtree.
     */
    let max_budget = cgrp_slice_ns
        .wrapping_mul(nr_cpus as u64)
        .wrapping_mul((*cgc).hweight)
        / (2 * FCG_HWEIGHT_ONE);
    if time_before(cvtime, cvtime_now.wrapping_sub(max_budget)) {
        cvtime = cvtime_now.wrapping_sub(max_budget);
    }

    (*cgv_node).cvtime = cvtime;
}

unsafe fn cgrp_enqueued(cgrp: *mut cgroup, cgc: *mut fcg_cgrp_ctx) {
    let cgid: u64 = (*(*cgrp).kn).id;

    /* paired with cmpxchg in try_pick_next_cgroup() */
    if __sync_val_compare_and_swap(&raw mut (*cgc).queued, 0, 1) != 0 {
        stat_inc(FCG_STAT_ENQ_SKIP);
        return;
    }

    let stash = bpf_map_lookup_elem(&raw mut cgv_node_stash as *mut _ as *mut _, &cgid as *const _ as *mut _) as *mut cgv_node_stash;
    if stash.is_null() {
        scx_bpf_error(c"cgv_node lookup failed for cgid %llu".as_ptr(), cgid);
        return;
    }

    /* NULL if the node is already on the rbtree */
    let cgv_node = bpf_kptr_xchg(&raw mut (*stash).node as *mut _, core::ptr::null_mut()) as *mut cgv_node;
    if cgv_node.is_null() {
        stat_inc(FCG_STAT_ENQ_RACE);
        return;
    }

    bpf_spin_lock(&raw mut cgv_tree_lock);
    cgrp_cap_budget(cgv_node, cgc);
    bpf_rbtree_add(&raw mut cgv_tree, &raw mut (*cgv_node).rb_node, Some(cgv_node_less));
    bpf_spin_unlock(&raw mut cgv_tree_lock);
}

unsafe fn set_bypassed_at(p: *mut task_struct, taskc: *mut fcg_task_ctx) {
    /*
     * Tell fcg_stopping() that this bypassed the regular scheduling path
     * and should be force charged to the cgroup. 0 is used to indicate that
     * the task isn't bypassing, so if the current runtime is 0, go back by
     * one nanosecond.
     */
    (*taskc).bypassed_at = if (*p).se.sum_exec_runtime != 0 {
        (*p).se.sum_exec_runtime
    } else {
        u64::MAX
    };
}

#[no_mangle]
pub unsafe extern "C" fn fcg_select_cpu(p: *mut task_struct, prev_cpu: s32, wake_flags: u64) -> s32 {
    let mut is_idle = false;

    let cpu = scx_bpf_select_cpu_dfl(p, prev_cpu, wake_flags, &mut is_idle);

    let taskc = bpf_task_storage_get(&raw mut task_ctx as *mut _ as *mut _, p, 0, 0) as *mut fcg_task_ctx;
    if taskc.is_null() {
        scx_bpf_error(c"task_ctx lookup failed".as_ptr());
        return cpu;
    }

    /*
     * If select_cpu_dfl() is recommending local enqueue, the target CPU is
     * idle. Follow it and charge the cgroup later in fcg_stopping() after
     * the fact.
     */
    if is_idle {
        set_bypassed_at(p, taskc);
        stat_inc(FCG_STAT_LOCAL);
        scx_bpf_dsq_insert(p, SCX_DSQ_LOCAL, SCX_SLICE_DFL, 0);
    }

    cpu
}

#[no_mangle]
pub unsafe extern "C" fn fcg_enqueue(p: *mut task_struct, enq_flags: u64) {
    let taskc = bpf_task_storage_get(&raw mut task_ctx as *mut _ as *mut _, p, 0, 0) as *mut fcg_task_ctx;
    if taskc.is_null() {
        scx_bpf_error(c"task_ctx lookup failed".as_ptr());
        return;
    }

    /*
     * Use the direct dispatching and force charging to deal with tasks with
     * custom affinities so that we don't have to worry about per-cgroup
     * dq's containing tasks that can't be executed from some CPUs.
     */
    if (*p).nr_cpus_allowed != nr_cpus {
        set_bypassed_at(p, taskc);

        /*
         * The global dq is deprioritized as we don't want to let tasks
         * to boost themselves by constraining its cpumask. The
         * deprioritization is rather severe, so let's not apply that to
         * per-cpu kernel threads. This is ham-fisted. We probably wanna
         * implement per-cgroup fallback dq's instead so that we have
         * more control over when tasks with custom cpumask get issued.
         */
        if (*p).nr_cpus_allowed == 1 && ((*p).flags & PF_KTHREAD) != 0 {
            stat_inc(FCG_STAT_LOCAL);
            scx_bpf_dsq_insert(p, SCX_DSQ_LOCAL, SCX_SLICE_DFL, enq_flags);
        } else {
            stat_inc(FCG_STAT_GLOBAL);
            scx_bpf_dsq_insert(p, FALLBACK_DSQ, SCX_SLICE_DFL, enq_flags);
        }
        return;
    }

    let cgrp = scx_bpf_task_cgroup(p);
    let cgc = find_cgrp_ctx(cgrp);
    if cgc.is_null() {
        bpf_cgroup_release(cgrp);
        return;
    }

    if fifo_sched {
        scx_bpf_dsq_insert(p, (*(*cgrp).kn).id, SCX_SLICE_DFL, enq_flags);
    } else {
        let mut tvtime = (*p).scx.dsq_vtime;

        /*
         * Limit the amount of budget that an idling task can accumulate
         * to one slice.
         */
        if time_before(tvtime, (*cgc).tvtime_now.wrapping_sub(SCX_SLICE_DFL)) {
            tvtime = (*cgc).tvtime_now.wrapping_sub(SCX_SLICE_DFL);
        }

        scx_bpf_dsq_insert_vtime(p, (*(*cgrp).kn).id, SCX_SLICE_DFL, tvtime, enq_flags);
    }

    cgrp_enqueued(cgrp, cgc);
    bpf_cgroup_release(cgrp);
}

/*
 * Walk the cgroup tree to update the active weight sums as tasks wake up and
 * sleep. The weight sums are used as the base when calculating the proportion a
 * given cgroup or task is entitled to at each level.
 */
unsafe fn update_active_weight_sums(cgrp: *mut cgroup, runnable: bool) {
    let mut updated = false;

    let cgc = find_cgrp_ctx(cgrp);
    if cgc.is_null() {
        return;
    }

    /*
     * In most cases, a hot cgroup would have multiple threads going to
     * sleep and waking up while the whole cgroup stays active. In leaf
     * cgroups, ->nr_runnable which is updated with __sync operations gates
     * ->nr_active updates, so that we don't have to grab the cgv_tree_lock
     * repeatedly for a busy cgroup which is staying active.
     */
    if runnable {
        if __sync_fetch_and_add(&raw mut (*cgc).nr_runnable, 1) != 0 {
            return;
        }
        stat_inc(FCG_STAT_ACT);
    } else {
        if __sync_sub_and_fetch(&raw mut (*cgc).nr_runnable, 1) != 0 {
            return;
        }
        stat_inc(FCG_STAT_DEACT);
    }

    /*
     * If @cgrp is becoming runnable, its hweight should be refreshed after
     * it's added to the weight tree so that enqueue has the up-to-date
     * value. If @cgrp is becoming quiescent, the hweight should be
     * refreshed before it's removed from the weight tree so that the usage
     * charging which happens afterwards has access to the latest value.
     */
    if !runnable {
        cgrp_refresh_hweight(cgrp, cgc);
    }

    /* propagate upwards */
    let mut idx = 0;
    while idx < (*cgrp).level {
        let level = (*cgrp).level - idx;
        let cgc = find_ancestor_cgrp_ctx(cgrp, level);
        let mut pcgc: *mut fcg_cgrp_ctx = core::ptr::null_mut();
        let mut propagate = false;

        if cgc.is_null() {
            break;
        }
        if level != 0 {
            pcgc = find_ancestor_cgrp_ctx(cgrp, level - 1);
            if pcgc.is_null() {
                break;
            }
        }

        /*
         * We need the propagation protected by a lock to synchronize
         * against weight changes. There's no reason to drop the lock at
         * each level but bpf_spin_lock() doesn't want any function
         * calls while locked.
         */
        bpf_spin_lock(&raw mut cgv_tree_lock);

        if runnable {
            if (*cgc).nr_active == 0 {
                (*cgc).nr_active = (*cgc).nr_active.wrapping_add(1);
                updated = true;
                if !pcgc.is_null() {
                    propagate = true;
                    (*pcgc).child_weight_sum = (*pcgc).child_weight_sum.wrapping_add((*cgc).weight as u64);
                }
            } else {
                (*cgc).nr_active = (*cgc).nr_active.wrapping_add(1);
            }
        } else {
            (*cgc).nr_active = (*cgc).nr_active.wrapping_sub(1);
            if (*cgc).nr_active == 0 {
                updated = true;
                if !pcgc.is_null() {
                    propagate = true;
                    (*pcgc).child_weight_sum = (*pcgc).child_weight_sum.wrapping_sub((*cgc).weight as u64);
                }
            }
        }

        bpf_spin_unlock(&raw mut cgv_tree_lock);

        if !propagate {
            break;
        }
        idx += 1;
    }

    if updated {
        __sync_fetch_and_add(&raw mut hweight_gen, 1);
    }

    if runnable {
        cgrp_refresh_hweight(cgrp, cgc);
    }
}

#[no_mangle]
pub unsafe extern "C" fn fcg_runnable(p: *mut task_struct, _enq_flags: u64) {
    let cgrp = scx_bpf_task_cgroup(p);
    update_active_weight_sums(cgrp, true);
    bpf_cgroup_release(cgrp);
}

#[no_mangle]
pub unsafe extern "C" fn fcg_running(p: *mut task_struct) {
    if fifo_sched {
        return;
    }

    let cgrp = scx_bpf_task_cgroup(p);
    let cgc = find_cgrp_ctx(cgrp);
    if !cgc.is_null() {
        /*
         * @cgc->tvtime_now always progresses forward as tasks start
         * executing. The test and update can be performed concurrently
         * from multiple CPUs and thus racy. Any error should be
         * contained and temporary. Let's just live with it.
         */
        if time_before((*cgc).tvtime_now, (*p).scx.dsq_vtime) {
            (*cgc).tvtime_now = (*p).scx.dsq_vtime;
        }
    }
    bpf_cgroup_release(cgrp);
}

#[no_mangle]
pub unsafe extern "C" fn fcg_stopping(p: *mut task_struct, _runnable: bool) {
    /*
     * Scale the execution time by the inverse of the weight and charge.
     *
     * Note that the default yield implementation yields by setting
     * @p->scx.slice to zero and the following would treat the yielding task
     * as if it has consumed all its slice. If this penalizes yielding tasks
     * too much, determine the execution time by taking explicit timestamps
     * instead of depending on @p->scx.slice.
     */
    if !fifo_sched {
        let delta = scale_by_task_weight_inverse(p, SCX_SLICE_DFL.wrapping_sub((*p).scx.slice));

        scx_bpf_task_set_dsq_vtime(p, (*p).scx.dsq_vtime.wrapping_add(delta));
    }

    let taskc = bpf_task_storage_get(&raw mut task_ctx as *mut _ as *mut _, p, 0, 0) as *mut fcg_task_ctx;
    if taskc.is_null() {
        scx_bpf_error(c"task_ctx lookup failed".as_ptr());
        return;
    }

    if (*taskc).bypassed_at == 0 {
        return;
    }

    let cgrp = scx_bpf_task_cgroup(p);
    let cgc = find_cgrp_ctx(cgrp);
    if !cgc.is_null() {
        __sync_fetch_and_add(
            &raw mut (*cgc).cvtime_delta,
            ((*p).se.sum_exec_runtime.wrapping_sub((*taskc).bypassed_at))
                .wrapping_mul(FCG_HWEIGHT_ONE)
                / if (*cgc).hweight != 0 { (*cgc).hweight } else { 1 },
        );
        (*taskc).bypassed_at = 0;
    }
    bpf_cgroup_release(cgrp);
}

#[no_mangle]
pub unsafe extern "C" fn fcg_quiescent(p: *mut task_struct, _deq_flags: u64) {
    let cgrp = scx_bpf_task_cgroup(p);
    update_active_weight_sums(cgrp, false);
    bpf_cgroup_release(cgrp);
}

#[no_mangle]
pub unsafe extern "C" fn fcg_cgroup_set_weight(cgrp: *mut cgroup, weight: u32) {
    let cgc = find_cgrp_ctx(cgrp);
    let mut pcgc: *mut fcg_cgrp_ctx = core::ptr::null_mut();

    if cgc.is_null() {
        return;
    }

    if (*cgrp).level != 0 {
        pcgc = find_ancestor_cgrp_ctx(cgrp, (*cgrp).level - 1);
        if pcgc.is_null() {
            return;
        }
    }

    bpf_spin_lock(&raw mut cgv_tree_lock);
    if !pcgc.is_null() && (*cgc).nr_active != 0 {
        (*pcgc).child_weight_sum =
            ((*pcgc).child_weight_sum as s64).wrapping_add(weight as s64 - (*cgc).weight as s64) as u64;
    }
    (*cgc).weight = weight;
    bpf_spin_unlock(&raw mut cgv_tree_lock);

    /* expire cached hweights so the new weight propagates */
    __sync_fetch_and_add(&raw mut hweight_gen, 1);
}

unsafe fn try_pick_next_cgroup(cgidp: *mut u64) -> bool {
    /* pop the front cgroup and wind cvtime_now accordingly */
    bpf_spin_lock(&raw mut cgv_tree_lock);

    let mut rb_node = bpf_rbtree_first(&raw mut cgv_tree);
    if rb_node.is_null() {
        bpf_spin_unlock(&raw mut cgv_tree_lock);
        stat_inc(FCG_STAT_PNC_NO_CGRP);
        *cgidp = 0;
        return true;
    }

    rb_node = bpf_rbtree_remove(&raw mut cgv_tree, rb_node);
    bpf_spin_unlock(&raw mut cgv_tree_lock);

    if rb_node.is_null() {
        /*
         * This should never happen. bpf_rbtree_first() was called
         * above while the tree lock was held, so the node should
         * always be present.
         */
        scx_bpf_error(c"node could not be removed".as_ptr());
        return true;
    }

    let mut cgv_node = container_of_cgv_node_rb_node(rb_node);
    let cgid = (*cgv_node).cgid;

    if time_before(cvtime_now, (*cgv_node).cvtime) {
        cvtime_now = (*cgv_node).cvtime;
    }

    /*
     * If lookup fails, the cgroup's gone. Free and move on. See
     * fcg_cgroup_exit().
     */
    let cgrp = bpf_cgroup_from_id(cgid);
    if cgrp.is_null() {
        stat_inc(FCG_STAT_PNC_GONE);
        bpf_obj_drop(cgv_node as *mut _);
        return false;
    }

    let cgc = bpf_cgrp_storage_get(&raw mut cgrp_ctx as *mut _ as *mut _, cgrp, 0, 0) as *mut fcg_cgrp_ctx;
    if cgc.is_null() {
        bpf_cgroup_release(cgrp);
        stat_inc(FCG_STAT_PNC_GONE);
        bpf_obj_drop(cgv_node as *mut _);
        return false;
    }

    if !scx_bpf_dsq_move_to_local(cgid, 0) {
        bpf_cgroup_release(cgrp);
        stat_inc(FCG_STAT_PNC_EMPTY);

        let stash = bpf_map_lookup_elem(&raw mut cgv_node_stash as *mut _ as *mut _, &cgid as *const _ as *mut _) as *mut cgv_node_stash;
        if stash.is_null() {
            stat_inc(FCG_STAT_PNC_GONE);
            bpf_obj_drop(cgv_node as *mut _);
            return false;
        }

        /*
         * Paired with cmpxchg in cgrp_enqueued(). If they see the following
         * transition, they'll enqueue the cgroup. If they are earlier, we'll
         * see their task in the dq below and requeue the cgroup.
         */
        __sync_val_compare_and_swap(&raw mut (*cgc).queued, 1, 0);

        if scx_bpf_dsq_nr_queued(cgid) != 0 {
            bpf_spin_lock(&raw mut cgv_tree_lock);
            bpf_rbtree_add(&raw mut cgv_tree, &raw mut (*cgv_node).rb_node, Some(cgv_node_less));
            bpf_spin_unlock(&raw mut cgv_tree_lock);
            stat_inc(FCG_STAT_PNC_RACE);
        } else {
            cgv_node = bpf_kptr_xchg(&raw mut (*stash).node as *mut _, cgv_node as *mut _) as *mut cgv_node;
            if !cgv_node.is_null() {
                scx_bpf_error(c"unexpected !NULL cgv_node stash".as_ptr());
                bpf_obj_drop(cgv_node as *mut _);
                return false;
            }
        }

        return false;
    }

    /*
     * Successfully consumed from the cgroup. This will be our current
     * cgroup for the new slice. Refresh its hweight.
     */
    cgrp_refresh_hweight(cgrp, cgc);

    bpf_cgroup_release(cgrp);

    /*
     * As the cgroup may have more tasks, add it back to the rbtree. Note
     * that here we charge the full slice upfront and then exact later
     * according to the actual consumption. This prevents lowpri thundering
     * herd from saturating the machine.
     */
    bpf_spin_lock(&raw mut cgv_tree_lock);
    (*cgv_node).cvtime = (*cgv_node).cvtime.wrapping_add(
        cgrp_slice_ns.wrapping_mul(FCG_HWEIGHT_ONE) / if (*cgc).hweight != 0 { (*cgc).hweight } else { 1 },
    );
    cgrp_cap_budget(cgv_node, cgc);
    bpf_rbtree_add(&raw mut cgv_tree, &raw mut (*cgv_node).rb_node, Some(cgv_node_less));
    bpf_spin_unlock(&raw mut cgv_tree_lock);

    *cgidp = cgid;
    stat_inc(FCG_STAT_PNC_NEXT);
    true
}

#[no_mangle]
pub unsafe extern "C" fn fcg_dispatch(_cpu: s32, _prev: *mut task_struct) {
    let now: u64 = scx_bpf_now();
    let mut picked_next = false;

    let cpuc = find_cpu_ctx();
    if cpuc.is_null() {
        return;
    }

    if (*cpuc).cur_cgid != 0 {
        if time_before(now, (*cpuc).cur_at.wrapping_add(cgrp_slice_ns)) {
            if scx_bpf_dsq_move_to_local((*cpuc).cur_cgid, 0) {
                stat_inc(FCG_STAT_CNS_KEEP);
                return;
            }
            stat_inc(FCG_STAT_CNS_EMPTY);
        } else {
            stat_inc(FCG_STAT_CNS_EXPIRE);
        }

        /*
         * The current cgroup is expiring. It was already charged a full slice.
         * Calculate the actual usage and accumulate the delta.
         */
        let cgrp = bpf_cgroup_from_id((*cpuc).cur_cgid);
        if cgrp.is_null() {
            stat_inc(FCG_STAT_CNS_GONE);
        } else {
            let cgc = bpf_cgrp_storage_get(&raw mut cgrp_ctx as *mut _ as *mut _, cgrp, 0, 0) as *mut fcg_cgrp_ctx;
            if !cgc.is_null() {
                /*
                 * We want to update the vtime delta and then look for the next
                 * cgroup to execute but the latter needs to be done in a loop
                 * and we can't keep the lock held. Oh well...
                 */
                let delta: s64 = now.wrapping_sub((*cpuc).cur_at).wrapping_sub(cgrp_slice_ns) as s64;

                bpf_spin_lock(&raw mut cgv_tree_lock);
                /* keep the dividends positive, BPF division is unsigned */
                if delta >= 0 {
                    __sync_fetch_and_add(
                        &raw mut (*cgc).cvtime_delta,
                        (delta as u64).wrapping_mul(FCG_HWEIGHT_ONE)
                            / if (*cgc).hweight != 0 { (*cgc).hweight } else { 1 },
                    );
                } else {
                    __sync_fetch_and_sub(
                        &raw mut (*cgc).cvtime_delta,
                        ((-delta) as u64).wrapping_mul(FCG_HWEIGHT_ONE)
                            / if (*cgc).hweight != 0 { (*cgc).hweight } else { 1 },
                    );
                }
                bpf_spin_unlock(&raw mut cgv_tree_lock);
            } else {
                stat_inc(FCG_STAT_CNS_GONE);
            }

            bpf_cgroup_release(cgrp);
        }
    }

    (*cpuc).cur_at = now;

    if scx_bpf_dsq_move_to_local(FALLBACK_DSQ, 0) {
        (*cpuc).cur_cgid = 0;
        return;
    }

    let mut i = 0;
    while i < CGROUP_MAX_RETRIES {
        if try_pick_next_cgroup(&raw mut (*cpuc).cur_cgid) {
            picked_next = true;
            break;
        }
        i += 1;
    }

    /*
     * This only happens if try_pick_next_cgroup() races against enqueue
     * path for more than CGROUP_MAX_RETRIES times, which is extremely
     * unlikely and likely indicates an underlying bug. There shouldn't be
     * any stall risk as the race is against enqueue.
     */
    if !picked_next {
        stat_inc(FCG_STAT_PNC_FAIL);
    }
}

#[no_mangle]
pub unsafe extern "C" fn fcg_init_task(p: *mut task_struct, args: *mut scx_init_task_args) -> s32 {
    /*
     * @p is new. Let's ensure that its task_ctx is available. We can sleep
     * in this function and the following will automatically use GFP_KERNEL.
     */
    let taskc = bpf_task_storage_get(
        &raw mut task_ctx as *mut _ as *mut _,
        p,
        0,
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    ) as *mut fcg_task_ctx;
    if taskc.is_null() {
        return -ENOMEM;
    }

    (*taskc).bypassed_at = 0;

    let cgc = find_cgrp_ctx((*args).cgroup);
    if cgc.is_null() {
        return -ENOENT;
    }

    scx_bpf_task_set_dsq_vtime(p, (*cgc).tvtime_now);

    0
}

#[no_mangle]
pub unsafe extern "C" fn fcg_cgroup_init(cgrp: *mut cgroup, args: *mut scx_cgroup_init_args) -> i32 {
    let mut empty_stash: cgv_node_stash = core::mem::zeroed();
    let cgid: u64 = (*(*cgrp).kn).id;

    /*
     * Technically incorrect as cgroup ID is full 64bit while dsq ID is
     * 63bit. Should not be a problem in practice and easy to spot in the
     * unlikely case that it breaks.
     */
    let mut ret = scx_bpf_create_dsq(cgid, -1);
    if ret != 0 {
        scx_bpf_error(c"scx_bpf_create_dsq failed (%d)".as_ptr(), ret);
        return ret;
    }

    let cgc = bpf_cgrp_storage_get(
        &raw mut cgrp_ctx as *mut _ as *mut _,
        cgrp,
        0,
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    ) as *mut fcg_cgrp_ctx;
    if cgc.is_null() {
        ret = -ENOMEM;
        scx_bpf_destroy_dsq(cgid);
        return ret;
    }

    (*cgc).weight = (*args).weight;
    (*cgc).hweight = FCG_HWEIGHT_ONE;

    ret = bpf_map_update_elem(
        &raw mut cgv_node_stash as *mut _ as *mut _,
        &cgid as *const _ as *mut _,
        &mut empty_stash as *mut _ as *mut _,
        BPF_NOEXIST,
    );
    if ret != 0 {
        if ret != -ENOMEM {
            scx_bpf_error(c"unexpected stash creation error (%d)".as_ptr(), ret);
        }
        scx_bpf_destroy_dsq(cgid);
        return ret;
    }

    let stash = bpf_map_lookup_elem(&raw mut cgv_node_stash as *mut _ as *mut _, &cgid as *const _ as *mut _) as *mut cgv_node_stash;
    if stash.is_null() {
        scx_bpf_error(c"unexpected cgv_node stash lookup failure".as_ptr());
        ret = -ENOENT;
        bpf_map_delete_elem(&raw mut cgv_node_stash as *mut _ as *mut _, &cgid as *const _ as *mut _);
        scx_bpf_destroy_dsq(cgid);
        return ret;
    }

    let mut cgv_node = bpf_obj_new_cgv_node();
    if cgv_node.is_null() {
        ret = -ENOMEM;
        bpf_map_delete_elem(&raw mut cgv_node_stash as *mut _ as *mut _, &cgid as *const _ as *mut _);
        scx_bpf_destroy_dsq(cgid);
        return ret;
    }

    (*cgv_node).cgid = cgid;
    (*cgv_node).cvtime = cvtime_now;

    cgv_node = bpf_kptr_xchg(&raw mut (*stash).node as *mut _, cgv_node as *mut _) as *mut cgv_node;
    if !cgv_node.is_null() {
        scx_bpf_error(c"unexpected !NULL cgv_node stash".as_ptr());
        ret = -EBUSY;
        bpf_obj_drop(cgv_node as *mut _);
        bpf_map_delete_elem(&raw mut cgv_node_stash as *mut _ as *mut _, &cgid as *const _ as *mut _);
        scx_bpf_destroy_dsq(cgid);
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn fcg_cgroup_exit(cgrp: *mut cgroup) {
    let cgid: u64 = (*(*cgrp).kn).id;

    /*
     * For now, there's no way find and remove the cgv_node if it's on the
     * cgv_tree. Let's drain them in the dispatch path as they get popped
     * off the front of the tree.
     */
    bpf_map_delete_elem(&raw mut cgv_node_stash as *mut _ as *mut _, &cgid as *const _ as *mut _);
    scx_bpf_destroy_dsq(cgid);
}

#[no_mangle]
pub unsafe extern "C" fn fcg_cgroup_move(p: *mut task_struct, from: *mut cgroup, to: *mut cgroup) {
    /* find_cgrp_ctx() triggers scx_bpf_error() on lookup failures */
    let from_cgc = find_cgrp_ctx(from);
    let to_cgc = find_cgrp_ctx(to);
    if from_cgc.is_null() || to_cgc.is_null() {
        return;
    }

    let delta: s64 = time_delta((*p).scx.dsq_vtime, (*from_cgc).tvtime_now);
    scx_bpf_task_set_dsq_vtime(p, ((*to_cgc).tvtime_now as s64).wrapping_add(delta) as u64);
}

#[no_mangle]
pub unsafe extern "C" fn fcg_init() -> s32 {
    let ret = scx_bpf_create_dsq(FALLBACK_DSQ, -1);
    if ret != 0 {
        scx_bpf_error(c"failed to create DSQ %d (%d)".as_ptr(), FALLBACK_DSQ, ret);
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn fcg_exit(ei: *mut scx_exit_info) {
    UEI_RECORD(&raw mut uei, ei);
}

/* SCX_OPS_DEFINE(flatcg_ops,
 *        .select_cpu          = (void *)fcg_select_cpu,
 *        .enqueue             = (void *)fcg_enqueue,
 *        .dispatch            = (void *)fcg_dispatch,
 *        .runnable            = (void *)fcg_runnable,
 *        .running             = (void *)fcg_running,
 *        .stopping            = (void *)fcg_stopping,
 *        .quiescent           = (void *)fcg_quiescent,
 *        .init_task           = (void *)fcg_init_task,
 *        .cgroup_set_weight   = (void *)fcg_cgroup_set_weight,
 *        .cgroup_init         = (void *)fcg_cgroup_init,
 *        .cgroup_exit         = (void *)fcg_cgroup_exit,
 *        .cgroup_move         = (void *)fcg_cgroup_move,
 *        .init                = (void *)fcg_init,
 *        .exit                = (void *)fcg_exit,
 *        .flags               = SCX_OPS_ENQ_EXITING,
 *        .name                = "flatcg");
 */
#[no_mangle]
pub static mut flatcg_ops: scx_ops = scx_ops {
    select_cpu: Some(fcg_select_cpu),
    enqueue: Some(fcg_enqueue),
    dispatch: Some(fcg_dispatch),
    runnable: Some(fcg_runnable),
    running: Some(fcg_running),
    stopping: Some(fcg_stopping),
    quiescent: Some(fcg_quiescent),
    init_task: Some(fcg_init_task),
    cgroup_set_weight: Some(fcg_cgroup_set_weight),
    cgroup_init: Some(fcg_cgroup_init),
    cgroup_exit: Some(fcg_cgroup_exit),
    cgroup_move: Some(fcg_cgroup_move),
    init: Some(fcg_init),
    exit: Some(fcg_exit),
    flags: SCX_OPS_ENQ_EXITING,
    name: *b"flatcg\0",
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
