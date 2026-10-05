// SPDX-License-Identifier: GPL-2.0

/// Skip a scheduler's subtree in an RCU-safe pre-order traversal.
///
/// # Safety
/// pos and root are live members of one tree, pos descends from root. The caller
/// holds scx_enable_mutex, scx_sched_lock, or an RCU read lock for the entire walk.
#[no_mangle]
pub unsafe extern "C" fn scx_skip_subtree_pre(mut pos: *mut scx_sched, root: *mut scx_sched) -> *mut scx_sched {
    // SAFETY: Caller pins the tree; native leaves retain RCU list accesses and
    // the original lockdep assertion. No Rust reference outlives that protection.
    unsafe {
        lupos_scx_sub_assert_skip_tree();
        while pos != root {
            let parent = lupos_scx_sub_parent(pos);
            let next = lupos_scx_sub_next_sibling(pos, parent);
            if !next.is_null() {
                return next;
            }
            pos = parent;
        }
        ptr::null_mut()
    }
}

/// Advance a pre-order traversal including root.
///
/// # Safety
/// root is live, or both root and pos are null for an empty tree. pos is null
/// to start or a previous result. Keep one of the tree protections documented
/// for scx_skip_subtree_pre held across iteration.
#[no_mangle]
pub unsafe extern "C" fn scx_next_descendant_pre(pos: *mut scx_sched, root: *mut scx_sched) -> *mut scx_sched {
    // SAFETY: Same protected tree and native RCU-list contract as the skip walk.
    unsafe {
        lupos_scx_sub_assert_next_tree();
        if pos.is_null() {
            return root;
        }
        let next = lupos_scx_sub_first_child(pos);
        if !next.is_null() {
            return next;
        }
        scx_skip_subtree_pre(pos, root)
    }
}

unsafe fn scx_find_sub_sched(cgroup_id: u64) -> *mut scx_sched {
    // SAFETY: Caller holds the RCU/enable protection required by the hash owner.
    unsafe { lupos_scx_sub_find(cgroup_id) }
}

/// Publish a task's scheduler using the native RCU assignment.
///
/// # Safety
/// p is pinned. Serialize transitions with its pi/rq locks, or own the not-yet-
/// published fork task while holding scx_fork_rwsem as in scx_fork. sch may be
/// null to detach; otherwise retain it through the corresponding reader grace
/// period. The native task state machine must permit this association change.
#[no_mangle]
pub unsafe extern "C" fn scx_set_task_sched(p: *mut task_struct, sch: *mut scx_sched) {
    // SAFETY: Native leaf preserves rcu_assign_pointer and the task's real layout.
    unsafe { lupos_scx_sub_set_task_sched(p, sch) };
}

/// Return the cgroup held by this scheduler.
///
/// # Safety
/// sch is allocated and pinned by enable/disable serialization, RCU, or its
/// exclusive final-release owner. Any non-null returned cgroup is borrowed;
/// use it only while sch still owns the cgroup reference.
#[no_mangle]
pub unsafe extern "C" fn sch_cgroup(sch: *mut scx_sched) -> *mut cgroup {
    // SAFETY: sch's cgroup reference is pinned by the caller.
    unsafe { (*sch).cgrp }
}

/// Point each live cgroup in a subtree at a scheduler.
///
/// # Safety
/// Caller holds scx_enable_mutex, scx_fork_rwsem for write, and cgroup_mutex.
/// cgrp is live. sch may be null to clear the association; otherwise keep sch
/// allocated through the grace period for readers of the published pointer.
#[no_mangle]
pub unsafe extern "C" fn set_cgroup_sched(cgrp: *mut cgroup, sch: *mut scx_sched) {
    // SAFETY: cgroup_mutex stabilizes css descendants and excludes teardown;
    // each native publication retains rcu_assign_pointer.
    unsafe {
        let root = lupos_scx_sub_cgroup_css(cgrp);
        let mut css = ptr::null_mut();
        loop {
            css = lupos_scx_sub_css_next(css, root);
            if css.is_null() {
                break;
            }
            let pos = lupos_scx_sub_live_cgroup(css);
            if !pos.is_null() {
                lupos_scx_sub_set_cgroup_sched(pos, sch);
            }
        }
    }
}

unsafe fn free_pshard(pshard: *mut scx_pshard) {
    // SAFETY: The shard is unpublished or has drained readers. Its saved
    // geometry, not mutable arena headers or newly published CID tables, sizes
    // the arena free. kfree(NULL) retains the native contract.
    unsafe {
        if pshard.is_null() {
            return;
        }
        let cu = ptr::addr_of_mut!((*pshard).caps_updated);
        if !(*cu).cmask_arena_out.is_null() {
            scx_arena_free((*pshard).sch, (*cu).cmask_arena_out.cast(),
                lupos_scx_sub_cmask_size((*pshard).nr_cids));
        }
        lupos_scx_sub_kfree(pshard.cast());
    }
}

/// Free every shard using the scheduler's allocation-time shard count.
///
/// # Safety
/// sch owns the array and every shard exclusively; all task/RCU users have
/// drained. Call at most once before the scheduler storage is destroyed.
#[no_mangle]
pub unsafe extern "C" fn scx_free_pshards(sch: *mut scx_sched) {
    // SAFETY: Caller owns all allocation elements; count was saved at allocation.
    unsafe {
        let shards = (*sch).pshard;
        if shards.is_null() {
            return;
        }
        for si in 0..(*sch).nr_pshards as usize {
            free_pshard(*shards.add(si));
        }
        lupos_scx_sub_kfree(shards.cast());
    }
}

unsafe fn alloc_pshard(sch: *mut scx_sched, shard_idx: c_int, node: c_int) -> *mut scx_pshard {
    // SAFETY: enable mutex pins published CID geometry. Each allocation remains
    // private until fully initialized; the arena is already set up for sch.
    unsafe {
        let shard = lupos_scx_sub_shard_range(shard_idx);
        let size = lupos_scx_sub_cmask_size((*shard).nr_cids as u32);
        let ps = lupos_scx_sub_alloc_pshard(node);
        if ps.is_null() {
            return ps;
        }
        lupos_scx_sub_init_pshard_lock(ps);
        (*ps).sch = sch;
        (*ps).base = (*shard).base_cid as u32;
        (*ps).nr_cids = (*shard).nr_cids as u32;
        for i in 0..__SCX_NR_CAPS {
            lupos_scx_sub_cmask_init(lupos_scx_sub_cap_cmask(ps, i), (*ps).base, (*ps).nr_cids);
        }
        let cu = ptr::addr_of_mut!((*ps).caps_updated);
        lupos_scx_sub_init_updated(cu);
        lupos_scx_sub_cmask_init_capacity(lupos_scx_sub_updated_cmask(cu), (*ps).base, (*ps).nr_cids,
            LUPOS_SCX_SUB_SHARD_MAX_CPUS);
        (*cu).cmask_arena_out = scx_arena_alloc(sch, size).cast();
        if (*cu).cmask_arena_out.is_null() {
            free_pshard(ps);
            return ptr::null_mut();
        }
        lupos_scx_sub_cmask_init((*cu).cmask_arena_out, (*ps).base, (*ps).nr_cids);
        ps
    }
}

/// Allocate every per-shard capability owner, then publish the completed array.
///
/// # Safety
/// Caller holds scx_enable_mutex, has initialized CID tables and sch's arena,
/// and has not previously published a shard array for sch. Sleep is permitted.
#[no_mangle]
pub unsafe extern "C" fn scx_alloc_pshards(sch: *mut scx_sched) -> c_int {
    // SAFETY: The unpublished array is exclusively owned; failure frees only
    // initialized elements. Publication retains smp_wmb followed by WRITE_ONCE.
    unsafe {
        if !(*sch).is_cid_type || (*sch).arena_pool.is_null() {
            return 0;
        }
        let nodes = lupos_scx_sub_shard_nodes();
        let count = lupos_scx_sub_nr_shards();
        let shards = lupos_scx_sub_alloc_pshard_array(count);
        if shards.is_null() {
            return -(LUPOS_SCX_SUB_ENOMEM as c_int);
        }
        for si in 0..count as usize {
            *shards.add(si) = alloc_pshard(sch, si as c_int, *nodes.add(si));
            if (*shards.add(si)).is_null() {
                for built in (0..si).rev() {
                    free_pshard(*shards.add(built));
                }
                lupos_scx_sub_kfree(shards.cast());
                return -(LUPOS_SCX_SUB_ENOMEM as c_int);
            }
        }
        (*sch).nr_pshards = count;
        lupos_scx_sub_publish_pshards(sch, shards);
        0
    }
}

/// Seed root ownership of every CID and capability.
///
/// # Safety
/// Caller holds the enable mutex and owns a fully allocated root scheduler
/// which is not yet live; no cap reader or writer may observe partial seeding.
#[no_mangle]
pub unsafe extern "C" fn scx_init_root_caps(sch: *mut scx_sched) {
    // SAFETY: Both loop bounds come from native allocation geometry/constants.
    unsafe {
        for si in 0..(*sch).nr_pshards as usize {
            let ps = *(*sch).pshard.add(si);
            for i in 0..__SCX_NR_CAPS {
                scx_cmask_fill(lupos_scx_sub_cap_cmask(ps, i));
            }
        }
    }
}
