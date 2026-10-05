// SPDX-License-Identifier: GPL-2.0-only
//! Resource lifetime and attributes from native kernel/workqueue.c.
//!
//! Authority: 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
//! Source-only overlay; configured native declarations, allocation profiling,
//! callback ABI, lock-class identity and build selection remain admission holds.

use super::common::*;
#[cfg(CONFIG_SMP)]
use super::completion::flush_work;
#[cfg(CONFIG_SMP)]
use super::dispatch::queue_work_on;
#[cfg(CONFIG_SMP)]
use super::execution::pool_allowed_cpus;
use super::completion::{
    cancel_work_sync,
    drain_workqueue, //
};
use super::dispatch::{
    first_idle_worker,
    bh_pool_irq_work,
    get_pwq,
    kick_pool,
    pwq_activate_first_inactive,
    pwq_is_empty,
    put_pwq_unlocked,
    wq_update_node_max_active,
    is_percpu_pool,
    mayday_cursor_func,
    unplug_oldest_pwq,
    wq_node_nr_active, //
};
use super::diagnostics::{
    show_one_workqueue,
    show_pwq,
    wq_watchdog_init,
    workqueue_sysfs_register,
    workqueue_sysfs_unregister, //
};
use super::execution::{
    alloc_worker,
    format_worker_id,
    rescuer_thread,
    unbind_worker,
    create_worker,
    detach_dying_workers,
    reap_dying_workers,
    set_worker_dying,
    idle_cull_fn,
    idle_worker_timeout,
    pool_mayday_timeout, //
};
use core::mem::{
    size_of,
    MaybeUninit, //
};
use core::ptr::{
    addr_of,
    addr_of_mut,
    null_mut, //
};

/// Releases an optional attrs allocation, native lines 4808–4815.
///
/// # Safety
/// `attrs` is null or exclusively owned, and its mask fields are initialized
/// or zeroed exactly as required by native free_cpumask_var.
pub(super) unsafe extern "C" fn free_workqueue_attrs(attrs: *mut workqueue_attrs) {
    // SAFETY: Allocation failure leaves zeroed mask fields; each owned native
    // mask is released before freeing its containing allocation.
    unsafe {
        if !attrs.is_null() {
            lupos_wq_free_attrs_cpumask(attrs);
            lupos_wq_free_attrs_pod_cpumask(attrs);
            kfree(attrs.cast::<c_void>());
        }
    }
}

/// Allocates default attributes, native lines 4825–4843.
///
/// # Safety
/// The caller may sleep for GFP_KERNEL allocation. The native leaf retains
/// each allocation site's configured type, overflow and profiling semantics.
pub(super) unsafe extern "C" fn alloc_workqueue_attrs_noprof() -> *mut workqueue_attrs {
    // SAFETY: Each failure path frees only zeroed or successfully allocated
    // native fields, including the first mask when allocating the second fails.
    unsafe {
        let attrs = lupos_wq_alloc_attrs_4829();
        if attrs.is_null() {
            return null_mut();
        }
        if !lupos_wq_alloc_attrs_cpumask_4832(attrs)
            || !lupos_wq_alloc_attrs_pod_cpumask_4834(attrs)
        {
            free_workqueue_attrs(attrs);
            return null_mut();
        }
        lupos_wq_cpumask_copy(lupos_wq_attrs_cpumask(attrs), lupos_wq_cpu_possible_mask());
        (*attrs).affn_scope = WQ_AFFN_DFL;
        attrs
    }
}

/// Copies pool and workqueue attrs fields, native lines 4845–4860.
///
/// # Safety
/// Both attrs and masks are live; the destination is exclusively writable.
pub(super) unsafe fn copy_workqueue_attrs(to: *mut workqueue_attrs, from: *const workqueue_attrs) {
    // SAFETY: Mask accessors preserve configured cpumask_var_t storage.
    unsafe {
        (*to).nice = (*from).nice;
        lupos_wq_cpumask_copy(lupos_wq_attrs_cpumask(to), lupos_wq_attrs_cpumask(from.cast_mut()));
        lupos_wq_cpumask_copy(lupos_wq_attrs_pod_cpumask(to), lupos_wq_attrs_pod_cpumask(from.cast_mut()));
        (*to).affn_strict = (*from).affn_strict;
        (*to).affn_scope = (*from).affn_scope;
        (*to).ordered = (*from).ordered;
    }
}

/// Removes workqueue-only identity fields, native lines 4866–4872.
///
/// # Safety
/// The caller exclusively owns the live attributes and allocated masks.
pub(super) unsafe fn wqattrs_clear_for_pool(attrs: *mut workqueue_attrs) {
    // SAFETY: Only unpublished pool attributes are modified.
    unsafe {
        (*attrs).affn_scope = WQ_AFFN_NR_TYPES;
        (*attrs).ordered = false;
        if (*attrs).affn_strict {
            lupos_wq_cpumask_copy(lupos_wq_attrs_cpumask(attrs), lupos_wq_cpu_possible_mask());
        }
    }
}

/// Hashes only native pool identity, native lines 4875–4887.
///
/// # Safety
/// Attrs and both masks remain stable; the byte extent is the configured
/// BITS_TO_LONGS(nr_cpumask_bits) * sizeof(long), including native tail bits.
pub(super) unsafe fn wqattrs_hash(attrs: *const workqueue_attrs) -> u32 {
    // SAFETY: The genuine jhash primitives consume exactly the native words;
    // strict affinity deliberately excludes the requested mask from identity.
    unsafe {
        let mut hash = lupos_wq_jhash_1word((*attrs).nice as u32, 0);
        hash = lupos_wq_jhash_1word((*attrs).affn_strict as u32, hash);
        hash = lupos_wq_jhash(
            lupos_wq_cpumask_bits(lupos_wq_attrs_pod_cpumask(attrs.cast_mut())).cast::<c_void>(),
            lupos_wq_cpumask_hash_bytes(),
            hash,
        );
        if !(*attrs).affn_strict {
            hash = lupos_wq_jhash(
                lupos_wq_cpumask_bits(lupos_wq_attrs_cpumask(attrs.cast_mut())).cast::<c_void>(),
                lupos_wq_cpumask_hash_bytes(),
                hash,
            );
        }
        hash
    }
}

/// Compares pool identity with the native short circuit, lines 4890–4902.
///
/// # Safety
/// Both attrs and their masks remain live and stable.
pub(super) unsafe fn wqattrs_equal(a: *const workqueue_attrs, b: *const workqueue_attrs) -> bool {
    // SAFETY: Workqueue-only fields are intentionally not read.
    unsafe {
        if (*a).nice != (*b).nice || (*a).affn_strict != (*b).affn_strict {
            return false;
        }
        if !lupos_wq_cpumask_equal(lupos_wq_attrs_pod_cpumask(a.cast_mut()), lupos_wq_attrs_pod_cpumask(b.cast_mut())) {
            return false;
        }
        if !(*a).affn_strict && !lupos_wq_cpumask_equal(lupos_wq_attrs_cpumask(a.cast_mut()), lupos_wq_attrs_cpumask(b.cast_mut())) {
            return false;
        }
        true
    }
}

/// Intersects allowed CPUs, falling back on an empty result, lines 4905–4916.
///
/// # Safety
/// Attrs is exclusively owned and both native masks remain live.
pub(super) unsafe fn wqattrs_actualize_cpumask(attrs: *mut workqueue_attrs, unbound_cpumask: *const cpumask) {
    // SAFETY: Native cpumask_and explicitly permits destination aliasing.
    unsafe {
        let mask = lupos_wq_attrs_cpumask(attrs);
        lupos_wq_cpumask_and(mask, mask, unbound_cpumask);
        if lupos_wq_unlikely_attrs_mask_empty_4914(lupos_wq_cpumask_empty(mask)) {
            lupos_wq_cpumask_copy(mask, unbound_cpumask);
        }
    }
}

/// Selects the initialized affinity pod type, native lines 4919–4946.
///
/// # Safety
/// The caller holds wq_pool_mutex and pins attrs. The early system pod exists.
pub(super) unsafe fn wqattrs_pod_type(attrs: *const workqueue_attrs) -> *const wq_pod_type {
    // SAFETY: The mutex serializes the default scope; only native static pod
    // addresses are returned. BUG_ON preserves the early-init invariant.
    unsafe {
        lupos_wq_assert_pool_mutex_held();
        let scope = if (*attrs).affn_scope == WQ_AFFN_DFL {
            lupos_wq_affn_dfl()
        } else {
            (*attrs).affn_scope
        };
        let pt = lupos_wq_pod_type(scope as c_int);
        if !lupos_wq_warn_attrs_scope_4934((*attrs).affn_scope == WQ_AFFN_NR_TYPES)
            && lupos_wq_likely_pods_ready_4935((*pt).nr_pods != 0)
        {
            return pt;
        }
        let pt = lupos_wq_pod_type(WQ_AFFN_SYSTEM as c_int);
        lupos_wq_bug_system_pod_missing_4944((*pt).nr_pods == 0);
        pt
    }
}

/// Initializes a zeroed pool even when attrs allocation fails, lines 4958–4992.
///
/// # Safety
/// The caller owns unpublished zeroed configured worker_pool storage. Every
/// callback and static lock class is supplied by its exact native-site leaf.
pub(super) unsafe fn init_worker_pool(pool: *mut worker_pool) -> c_int {
    // SAFETY: All destruction-visible fields precede the only fallible step.
    unsafe {
        lupos_wq_init_pool_lock_4960(pool);
        (*pool).id = -1;
        (*pool).cpu = -1;
        (*pool).node = NUMA_NO_NODE;
        (*pool).flags |= POOL_DISASSOCIATED as c_uint;
        (*pool).last_progress_ts = lupos_wq_read_jiffies();
        lupos_wq_init_list_head(addr_of_mut!((*pool).worklist));
        lupos_wq_init_list_head(addr_of_mut!((*pool).idle_list));
        lupos_wq_init_pool_busy_hash(pool);
        lupos_wq_init_idle_timer_4970(pool, Some(idle_worker_timeout));
        lupos_wq_init_idle_cull_work_4971(pool, Some(idle_cull_fn));
        lupos_wq_init_mayday_timer_4973(pool, Some(pool_mayday_timeout));
        lupos_wq_init_list_head(addr_of_mut!((*pool).workers));
        lupos_wq_ida_init(addr_of_mut!((*pool).worker_ida));
        lupos_wq_init_hlist_node(addr_of_mut!((*pool).hash_node));
        (*pool).refcnt = 1;
        #[cfg(CONFIG_PREEMPT_RT)]
        lupos_wq_init_pool_cb_lock_4981(pool);
        (*pool).attrs = lupos_wq_alloc_pool_attrs_4985();
        if (*pool).attrs.is_null() {
            return -(ENOMEM as c_int);
        }
        wqattrs_clear_for_pool((*pool).attrs);
        0
    }
}

/// Creates the queue's own lockdep map, native lines 4995–5007/5026–5028.
///
/// # Safety
/// The caller exclusively owns a live initialized queue name and key.
pub(super) unsafe fn wq_init_lockdep(wq: *mut workqueue_struct) {
    #[cfg(CONFIG_LOCKDEP)]
    // SAFETY: The key is registered before initializing the persistent map;
    // allocation failure uses the queue's own name with its existing lifetime.
    unsafe {
        lockdep_register_key(addr_of_mut!((*wq).key));
        let mut name = lupos_wq_alloc_lock_name_5000(wq);
        if name.is_null() {
            name = lupos_wq_wq_name(wq);
        }
        (*wq).lock_name = name;
        (*wq).lockdep_map = addr_of_mut!((*wq).__lockdep_map);
        lupos_wq_lockdep_init_map_5006(wq, name);
    }
    #[cfg(not(CONFIG_LOCKDEP))]
    let _ = wq;
}

/// Unregisters only an internally owned map, lines 5009–5015/5030–5032.
///
/// # Safety
/// The queue is live and no future operation can use its owned lockdep key.
pub(super) unsafe fn wq_unregister_lockdep(wq: *mut workqueue_struct) {
    #[cfg(CONFIG_LOCKDEP)]
    // SAFETY: External caller-supplied maps retain their own key ownership.
    unsafe {
        if (*wq).lockdep_map == addr_of_mut!((*wq).__lockdep_map) {
            lockdep_unregister_key(addr_of_mut!((*wq).key));
        }
    }
    #[cfg(not(CONFIG_LOCKDEP))]
    let _ = wq;
}

/// Frees only an allocated owned lockdep name, lines 5017–5024/5034–5036.
///
/// # Safety
/// The live queue has completed its lockdep-use and RCU lifetime obligations.
pub(super) unsafe fn wq_free_lockdep(wq: *mut workqueue_struct) {
    #[cfg(CONFIG_LOCKDEP)]
    // SAFETY: An external map and the inline name are never freed here.
    unsafe {
        if (*wq).lockdep_map == addr_of_mut!((*wq).__lockdep_map)
            && (*wq).lock_name != lupos_wq_wq_name(wq)
        {
            kfree((*wq).lock_name.cast::<c_void>());
        }
    }
    #[cfg(not(CONFIG_LOCKDEP))]
    let _ = wq;
}

/// Releases every possible-node counter and fallback slot, lines 5039–5050.
///
/// # Safety
/// The array has nr_node_ids + 1 slots, initially zeroed, with exclusive access.
pub(super) unsafe fn free_node_nr_active(nna_ar: *mut *mut wq_node_nr_active) {
    // SAFETY: Native possible-node iteration handles holes; the fallback slot
    // is separate and each freed pointer is cleared for failure unwind.
    unsafe {
        let mut node = lupos_wq_first_possible_node();
        while node < lupos_wq_max_numnodes() {
            let slot = nna_ar.add(node as usize);
            kfree((*slot).cast::<c_void>());
            *slot = null_mut();
            node = lupos_wq_next_possible_node(node);
        }
        let slot = nna_ar.add(lupos_wq_nr_node_ids() as usize);
        kfree((*slot).cast::<c_void>());
        *slot = null_mut();
    }
}

/// Initializes one zeroed node-active counter, native lines 5052–5058.
///
/// # Safety
/// `nna` is exclusively owned unpublished configured native storage.
pub(super) unsafe fn init_node_nr_active(nna: *mut wq_node_nr_active) {
    // SAFETY: Native atomic/lock/list initialization precedes publication.
    unsafe {
        (*nna).max = WQ_DFL_MIN_ACTIVE as c_int;
        lupos_wq_atomic_set(addr_of_mut!((*nna).nr), 0);
        lupos_wq_init_nna_lock_5056(nna);
        lupos_wq_init_list_head(addr_of_mut!((*nna).pending_pwqs));
    }
}

/// Allocates node-local counters and the NUMA fallback, native lines 5064–5089.
///
/// # Safety
/// The caller owns a zeroed nr_node_ids + 1 pointer array and may sleep.
pub(super) unsafe fn alloc_node_nr_active(nna_ar: *mut *mut wq_node_nr_active) -> c_int {
    // SAFETY: Each successful allocation is initialized before storing it;
    // failures free all previous allocations and leave every used slot null.
    unsafe {
        let mut node = lupos_wq_first_possible_node();
        while node < lupos_wq_max_numnodes() {
            let nna = lupos_wq_alloc_nna_5070(node);
            if nna.is_null() {
                free_node_nr_active(nna_ar);
                return -(ENOMEM as c_int);
            }
            init_node_nr_active(nna);
            *nna_ar.add(node as usize) = nna;
            node = lupos_wq_next_possible_node(node);
        }
        let nna = lupos_wq_alloc_nna_fallback_5078();
        if nna.is_null() {
            free_node_nr_active(nna_ar);
            return -(ENOMEM as c_int);
        }
        init_node_nr_active(nna);
        *nna_ar.add(lupos_wq_nr_node_ids() as usize) = nna;
        0
    }
}

/// Frees a fully retired queue after its grace period, native lines 5091–5103.
///
/// # Safety
/// Called by RCU exactly once for a retired queue with no remaining PWQs.
pub(super) unsafe extern "C" fn rcu_free_wq(rcu: *mut rcu_head) {
    // SAFETY: Callback ownership covers the native flexible node array and
    // per-CPU storage; no Rust reference survives a corresponding free.
    unsafe {
        let wq = lupos_wq_wq_from_rcu(rcu);
        if (*wq).flags & WQ_UNBOUND as c_uint != 0 {
            free_node_nr_active(lupos_wq_node_nr_active_array(wq));
        }
        wq_free_lockdep(wq);
        lupos_wq_free_cpu_pwq(wq);
        free_workqueue_attrs((*wq).attrs);
        kfree(wq.cast::<c_void>());
    }
}

/// Frees a retired pool after its grace period, native lines 5105–5112.
///
/// # Safety
/// RCU invokes this once after workers and timers are gone and all refs dropped.
pub(super) unsafe extern "C" fn rcu_free_pool(rcu: *mut rcu_head) {
    // SAFETY: The caller's grace period ends all lookup references to the pool.
    unsafe {
        let pool = lupos_wq_pool_from_rcu(rcu);
        ida_destroy(addr_of_mut!((*pool).worker_ida));
        free_workqueue_attrs((*pool).attrs);
        kfree(pool.cast::<c_void>());
    }
}

/// Releases an unbound pool, including partly initialized pools, C5125–5191.
///
/// # Safety
/// The caller holds wq_pool_mutex and owns a pool reference. Pool initialization
/// has reached refcnt setup; workers, if present, follow the native lock rules.
pub(super) unsafe fn put_unbound_pool(pool: *mut worker_pool) {
    // SAFETY: The pool mutex serializes references/hash/IDR. Stack list storage
    // is initialized in place and remains pinned until all culled workers reap.
    unsafe {
        let mut cull_storage = MaybeUninit::<list_head>::uninit();
        let cull_list = cull_storage.as_mut_ptr();
        lupos_wq_init_list_head(cull_list);
        lupos_wq_assert_pool_mutex_held();
        (*pool).refcnt = (*pool).refcnt.wrapping_sub(1);
        if (*pool).refcnt != 0 {
            return;
        }
        if lupos_wq_warn_put_pool_percpu_5136(is_percpu_pool(pool))
            || lupos_wq_warn_put_pool_worklist_5137(!lupos_wq_list_empty(addr_of!((*pool).worklist)))
        {
            return;
        }
        if (*pool).id >= 0 {
            idr_remove(lupos_wq_worker_pool_idr(), (*pool).id);
        }
        lupos_wq_hash_del(addr_of_mut!((*pool).hash_node));

        loop {
            // This is the original rcuwait_wait_event expansion. The task-state
            // primitive retains the barrier paired with rcuwait_wake_up.
            let wait = lupos_wq_manager_wait();
            lupos_wq_prepare_to_rcuwait(wait);
            loop {
                lupos_wq_set_current_state(TASK_UNINTERRUPTIBLE as c_uint);
                if (*pool).flags & POOL_MANAGER_ACTIVE as c_uint == 0 {
                    break;
                }
                if lupos_wq_signal_pending_state(TASK_UNINTERRUPTIBLE as c_uint, lupos_wq_current()) {
                    break;
                }
                schedule();
            }
            finish_rcuwait(wait);
            lupos_wq_mutex_lock(lupos_wq_pool_attach_mutex());
            lupos_wq_pool_lock_irq(pool);
            if (*pool).flags & POOL_MANAGER_ACTIVE as c_uint == 0 {
                (*pool).flags |= POOL_MANAGER_ACTIVE as c_uint;
                break;
            }
            lupos_wq_pool_unlock_irq(pool);
            lupos_wq_mutex_unlock(lupos_wq_pool_attach_mutex());
        }
        loop {
            let worker = first_idle_worker(pool);
            if worker.is_null() {
                break;
            }
            set_worker_dying(worker, cull_list);
        }
        lupos_wq_warn_put_pool_workers_5175((*pool).nr_workers != 0 || (*pool).nr_idle != 0);
        lupos_wq_pool_unlock_irq(pool);
        detach_dying_workers(cull_list);
        lupos_wq_mutex_unlock(lupos_wq_pool_attach_mutex());
        reap_dying_workers(cull_list);
        lupos_wq_timer_delete_sync(addr_of_mut!((*pool).idle_timer));
        cancel_work_sync(addr_of_mut!((*pool).idle_cull_work));
        lupos_wq_timer_delete_sync(addr_of_mut!((*pool).mayday_timer));
        call_rcu(addr_of_mut!((*pool).rcu), Some(rcu_free_pool));
    }
}

/// Finds or creates a shared unbound pool, native lines 5207–5256.
///
/// # Safety
/// The caller holds wq_pool_mutex, may sleep, and keeps attrs/masks stable.
pub(super) unsafe fn get_unbound_pool(attrs: *const workqueue_attrs) -> *mut worker_pool {
    // SAFETY: Hash lookup/insertion/refcount are serialized by the pool mutex.
    // Failed initialized pools retain every prerequisite for put_unbound_pool.
    unsafe {
        let pt = lupos_wq_pod_type(WQ_AFFN_NUMA as c_int);
        let hash = wqattrs_hash(attrs);
        let mut node = NUMA_NO_NODE;
        lupos_wq_assert_pool_mutex_held();
        let head = lupos_wq_unbound_pool_hash_head(hash);
        let mut hnode = (*head).first;
        while !hnode.is_null() {
            let pool = lupos_wq_pool_from_hash_node(hnode);
            if wqattrs_equal((*pool).attrs, attrs) {
                (*pool).refcnt = (*pool).refcnt.wrapping_add(1);
                return pool;
            }
            hnode = (*hnode).next;
        }
        let mut pod: c_int = 0;
        while pod < (*pt).nr_pods {
            if lupos_wq_cpumask_subset(lupos_wq_attrs_pod_cpumask(attrs.cast_mut()), lupos_wq_pod_cpus(pt, pod)) {
                node = *(*pt).pod_node.add(pod as usize);
                break;
            }
            pod = pod.wrapping_add(1);
        }
        let pool = lupos_wq_alloc_pool_5233(node);
        if pool.is_null() {
            return null_mut();
        }
        if init_worker_pool(pool) < 0 {
            put_unbound_pool(pool);
            return null_mut();
        }
        (*pool).node = node;
        copy_workqueue_attrs((*pool).attrs, attrs);
        wqattrs_clear_for_pool((*pool).attrs);
        if worker_pool_assign_id(pool) < 0 {
            put_unbound_pool(pool);
            return null_mut();
        }
        if lupos_wq_online() && create_worker(pool).is_null() {
            put_unbound_pool(pool);
            return null_mut();
        }
        lupos_wq_unbound_pool_hash_add(pool, hash);
        pool
    }
}

/// Runs deferred PWQ destruction, native lines 5262–5313.
///
/// # Safety
/// The dedicated release worker invokes this exactly once after PWQ refs reach
/// zero. Linked PWQs retain the queue; an unlinked PWQ carries no such reference.
pub(super) unsafe extern "C" fn pwq_release_workfn(work: *mut kthread_work) {
    // SAFETY: Only pointer values are read before testing linkage. An unlinked
    // PWQ must have an empty pending_node; its possibly stale wq is never read.
    // Pending-node cleanup retains the native invariant and exact position
    // after the pool put; no extra queue reference is invented for this path.
    unsafe {
        let pwq = lupos_wq_pwq_from_release_work(work);
        let wq = (*pwq).wq;
        let pool = (*pwq).pool;
        let mut is_last = false;
        if !lupos_wq_list_empty(addr_of!((*pwq).pwqs_node)) {
            lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
            lupos_wq_list_del_rcu(addr_of_mut!((*pwq).pwqs_node));
            is_last = lupos_wq_list_empty(addr_of!((*wq).pwqs));
            if !is_last && (*wq).flags & __WQ_ORDERED as c_uint != 0 {
                unplug_oldest_pwq(wq);
            }
            lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
        }
        if !is_percpu_pool(pool) {
            lupos_wq_mutex_lock(lupos_wq_pool_mutex());
            put_unbound_pool(pool);
            lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        }
        if !lupos_wq_list_empty(addr_of!((*pwq).pending_node)) {
            let nna = wq_node_nr_active((*pwq).wq, (*(*pwq).pool).node);
            lupos_wq_nna_lock_irq(nna);
            lupos_wq_list_del_init(addr_of_mut!((*pwq).pending_node));
            lupos_wq_nna_unlock_irq(nna);
        }
        lupos_wq_kfree_rcu_pwq_5303(pwq);
        if is_last {
            wq_unregister_lockdep(wq);
            call_rcu(addr_of_mut!((*wq).rcu), Some(rcu_free_wq));
        }
    }
}

/// Initializes a newly allocated PWQ, native lines 5316–5345.
///
/// # Safety
/// PWQ is exclusive configured/aligned storage; the caller retains wq/pool.
pub(super) unsafe fn init_pwq(pwq: *mut pool_workqueue, wq: *mut workqueue_struct, pool: *mut worker_pool) {
    // SAFETY: Alignment is checked before zeroing; only configured native
    // storage is touched, and the mayday cursor is deliberately nonprocessable.
    unsafe {
        lupos_wq_bug_pwq_alignment_5319((pwq as c_ulong) & !(WORK_STRUCT_PWQ_MASK as c_ulong) != 0);
        core::ptr::write_bytes(pwq.cast::<u8>(), 0, size_of::<pool_workqueue>());
        (*pwq).pool = pool;
        (*pwq).wq = wq;
        (*pwq).flush_color = -1;
        (*pwq).refcnt = 1;
        lupos_wq_init_list_head(addr_of_mut!((*pwq).inactive_works));
        lupos_wq_init_list_head(addr_of_mut!((*pwq).pending_node));
        lupos_wq_init_list_head(addr_of_mut!((*pwq).pwqs_node));
        lupos_wq_init_list_head(addr_of_mut!((*pwq).mayday_node));
        lupos_wq_init_pwq_release_work_5331(pwq, Some(pwq_release_workfn));
        lupos_wq_init_mayday_cursor_5342(pwq, Some(mayday_cursor_func));
        lupos_wq_work_data_set(
            addr_of_mut!((*pwq).mayday_cursor),
            (pwq as c_ulong) | WORK_STRUCT_PENDING as c_ulong | WORK_STRUCT_PWQ as c_ulong | WORK_STRUCT_INACTIVE as c_ulong,
        );
    }
}

/// Links a PWQ once with its queue's current color, native lines 5348–5363.
///
/// # Safety
/// The caller holds wq->mutex and owns live PWQ, pool and queue references.
pub(super) unsafe fn link_pwq(pwq: *mut pool_workqueue) {
    // SAFETY: Queue mutex serializes color and RCU-list publication.
    unsafe {
        let wq = (*pwq).wq;
        lupos_wq_assert_wq_mutex_held(wq);
        if !lupos_wq_list_empty(addr_of!((*pwq).pwqs_node)) {
            return;
        }
        (*pwq).work_color = (*wq).work_color;
        lupos_wq_list_add_tail_rcu(addr_of_mut!((*pwq).pwqs_node), addr_of_mut!((*wq).pwqs));
    }
}

/// Selects one static per-CPU pool, native lines 5366–5377.
///
/// # Safety
/// The queue remains live and cpu is an allocated possible CPU.
pub(super) unsafe fn get_percpu_pool(wq: *mut workqueue_struct, cpu: c_int) -> *mut worker_pool {
    // SAFETY: Accessors calculate addresses in original static per-CPU storage.
    unsafe {
        let highpri = ((*wq).flags & WQ_HIGHPRI as c_uint != 0) as c_int;
        if (*wq).flags & WQ_BH as c_uint != 0 {
            lupos_wq_bh_pool_at(cpu, highpri)
        } else {
            lupos_wq_cpu_pool(cpu, highpri)
        }
    }
}

/// Acquires a matching pool and associated PWQ, native lines 5380–5400.
///
/// # Safety
/// The caller holds wq_pool_mutex, may sleep, and retains wq/attrs.
pub(super) unsafe fn alloc_pwq(wq: *mut workqueue_struct, attrs: *const workqueue_attrs) -> *mut pool_workqueue {
    // SAFETY: On cache allocation failure the acquired pool reference is
    // dropped; a successful PWQ owns exactly that one pool reference.
    unsafe {
        lupos_wq_assert_pool_mutex_held();
        let pool = get_unbound_pool(attrs);
        if pool.is_null() {
            return null_mut();
        }
        let pwq = lupos_wq_alloc_pwq_5392((*pool).node);
        if pwq.is_null() {
            put_unbound_pool(pool);
            return null_mut();
        }
        init_pwq(pwq, wq, pool);
        pwq
    }
}

/// Computes a CPU pod's effective mask, native lines 5416–5428.
///
/// # Safety
/// The caller holds pool mutex and hotplug exclusion, owns attrs and passes a
/// possible CPU. Pod topology and online mask remain stable.
pub(super) unsafe fn wq_calc_pod_cpumask(attrs: *mut workqueue_attrs, cpu: c_int) {
    // SAFETY: The configured topology maps this CPU to a valid pod.
    unsafe {
        let pt = wqattrs_pod_type(attrs);
        let pod = *(*pt).cpu_pod.add(cpu as usize);
        let mask = lupos_wq_attrs_pod_cpumask(attrs);
        lupos_wq_cpumask_and(mask, lupos_wq_pod_cpus(pt, pod), lupos_wq_attrs_cpumask(attrs));
        if !lupos_wq_cpumask_intersects(mask, lupos_wq_online_cpumask()) {
            lupos_wq_cpumask_copy(mask, lupos_wq_attrs_cpumask(attrs));
        }
    }
}

/// Publishes one new PWQ and returns the previous slot, native C5431–5446.
///
/// # Safety
/// The caller holds pool mutex and wq mutex, and owns the new slot reference.
pub(super) unsafe fn install_unbound_pwq(wq: *mut workqueue_struct, cpu: c_int, pwq: *mut pool_workqueue) -> *mut pool_workqueue {
    // SAFETY: Linkage precedes release publication; the old reference transfers
    // to the caller and is not released while still in the published slot.
    unsafe {
        let slot = unbound_pwq_slot(wq, cpu);
        lupos_wq_assert_pool_mutex_held();
        lupos_wq_assert_wq_mutex_held(wq);
        link_pwq(pwq);
        let old = lupos_wq_pwq_rcu_access(slot);
        lupos_wq_pwq_rcu_assign(slot, pwq);
        old
    }
}

/// Releases prepared or replaced transactional resources, native C5458–5471.
///
/// # Safety
/// Ctx is null or exclusively owned; all non-null slots own a PWQ reference.
pub(super) unsafe fn apply_wqattrs_cleanup(ctx: *mut apply_wqattrs_ctx) {
    // SAFETY: A freshly allocated native flex context is zeroed; null PWQ puts
    // are explicitly supported by put_pwq_unlocked on partial prepare failure.
    unsafe {
        if ctx.is_null() {
            return;
        }
        let mut cpu = lupos_wq_first_possible_cpu();
        while cpu < lupos_wq_small_cpumask_bits() {
            put_pwq_unlocked(*lupos_wq_ctx_pwq_slot(ctx, cpu as c_int));
            cpu = lupos_wq_next_possible_cpu(cpu);
        }
        put_pwq_unlocked((*ctx).dfl_pwq);
        free_workqueue_attrs((*ctx).attrs);
        kfree(ctx.cast::<c_void>());
    }
}

/// Prepares all attrs/PWQs before changing a live queue, native C5474–5541.
///
/// # Safety
/// The caller holds pool mutex, may sleep, and retains wq/attrs/mask/topology.
pub(super) unsafe fn apply_wqattrs_prepare(
    wq: *mut workqueue_struct,
    attrs: *const workqueue_attrs,
    unbound_cpumask: *const cpumask,
) -> *mut apply_wqattrs_ctx {
    // SAFETY: No published state changes until the distinct commit step. The
    // context adopts new_attrs only after every fallible operation succeeds.
    unsafe {
        lupos_wq_assert_pool_mutex_held();
        if lupos_wq_warn_prepare_scope_5485(
            ((*attrs).affn_scope as c_int) < 0 || (*attrs).affn_scope >= WQ_AFFN_NR_TYPES,
        ) {
            return lupos_wq_err_ptr(-(EINVAL as c_long)).cast::<apply_wqattrs_ctx>();
        }
        let ctx = lupos_wq_alloc_apply_ctx_5489();
        let new_attrs = lupos_wq_alloc_prepare_attrs_5491();
        let mut prepared = !ctx.is_null() && !new_attrs.is_null();
        if prepared {
            copy_workqueue_attrs(new_attrs, attrs);
            wqattrs_actualize_cpumask(new_attrs, unbound_cpumask);
            lupos_wq_cpumask_copy(lupos_wq_attrs_pod_cpumask(new_attrs), lupos_wq_attrs_cpumask(new_attrs));
            (*ctx).dfl_pwq = alloc_pwq(wq, new_attrs);
            prepared = !(*ctx).dfl_pwq.is_null();
        }
        if prepared {
            let mut cpu = lupos_wq_first_possible_cpu();
            while cpu < lupos_wq_small_cpumask_bits() {
                let slot = lupos_wq_ctx_pwq_slot(ctx, cpu as c_int);
                if (*new_attrs).ordered {
                    (*(*ctx).dfl_pwq).refcnt = (*(*ctx).dfl_pwq).refcnt.wrapping_add(1);
                    *slot = (*ctx).dfl_pwq;
                } else {
                    wq_calc_pod_cpumask(new_attrs, cpu as c_int);
                    *slot = alloc_pwq(wq, new_attrs);
                    if (*slot).is_null() {
                        prepared = false;
                        break;
                    }
                }
                cpu = lupos_wq_next_possible_cpu(cpu);
            }
        }
        if !prepared {
            free_workqueue_attrs(new_attrs);
            apply_wqattrs_cleanup(ctx);
            return lupos_wq_err_ptr(-(ENOMEM as c_long)).cast::<apply_wqattrs_ctx>();
        }
        copy_workqueue_attrs(new_attrs, attrs);
        let mask = lupos_wq_attrs_cpumask(new_attrs);
        lupos_wq_cpumask_and(mask, mask, lupos_wq_cpu_possible_mask());
        lupos_wq_cpumask_copy(lupos_wq_attrs_pod_cpumask(new_attrs), mask);
        (*ctx).attrs = new_attrs;
        if (*wq).flags & __WQ_ORDERED as c_uint != 0 && !lupos_wq_list_empty(addr_of!((*wq).pwqs)) {
            (*(*ctx).dfl_pwq).plugged = true;
        }
        (*ctx).wq = wq;
        ctx
    }
}

/// Installs the prepared transaction and retains old refs, native C5544–5564.
///
/// # Safety
/// The caller holds pool mutex and owns a successfully prepared context.
pub(super) unsafe fn apply_wqattrs_commit(ctx: *mut apply_wqattrs_ctx) {
    // SAFETY: Every slot is prepared; swapping changes the context's ownership
    // from new to old refs, so cleanup can only release the replaced state.
    unsafe {
        let wq = (*ctx).wq;
        lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
        copy_workqueue_attrs((*wq).attrs, (*ctx).attrs);
        let mut cpu = lupos_wq_first_possible_cpu();
        while cpu < lupos_wq_small_cpumask_bits() {
            let slot = lupos_wq_ctx_pwq_slot(ctx, cpu as c_int);
            *slot = install_unbound_pwq(wq, cpu as c_int, *slot);
            cpu = lupos_wq_next_possible_cpu(cpu);
        }
        (*ctx).dfl_pwq = install_unbound_pwq(wq, -1, (*ctx).dfl_pwq);
        if (*wq).flags & WQ_UNBOUND as c_uint != 0 {
            wq_update_node_max_active(wq, -1);
        }
        lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
    }
}

/// Applies an attrs transaction under pool mutex, native C5566–5584.
///
/// # Safety
/// The caller holds pool mutex, may sleep, and retains wq/attrs.
pub(super) unsafe fn apply_workqueue_attrs_locked(wq: *mut workqueue_struct, attrs: *const workqueue_attrs) -> c_int {
    // SAFETY: Preparation is all-or-nothing; both successful and failed
    // transactions consume their temporary references exactly once.
    unsafe {
        if lupos_wq_warn_apply_bound_5572((*wq).flags & WQ_UNBOUND as c_uint == 0) {
            return -(EINVAL as c_int);
        }
        let ctx = apply_wqattrs_prepare(wq, attrs, lupos_wq_unbound_cpumask());
        if lupos_wq_is_err(ctx.cast::<c_void>()) {
            return lupos_wq_ptr_err(ctx.cast::<c_void>()) as c_int;
        }
        apply_wqattrs_commit(ctx);
        apply_wqattrs_cleanup(ctx);
        0
    }
}

/// Applies new unbound queue attributes, native C5601–5611.
///
/// # Safety
/// The caller retains queue/attrs and may sleep; attrs has allocated masks.
pub(super) unsafe extern "C" fn apply_workqueue_attrs(wq: *mut workqueue_struct, attrs: *const workqueue_attrs) -> c_int {
    // SAFETY: Pool mutex serializes all prepared and published pool state.
    unsafe {
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        let ret = apply_workqueue_attrs_locked(wq, attrs);
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        ret
    }
}

/// Updates one hotplug pod slot with default-PWQ fallback, native C5632–5680.
///
/// # Safety
/// Pool mutex and CPU-hotplug exclusion are held; wq/cpu/topology remain live.
pub(super) unsafe fn unbound_wq_update_pwq(wq: *mut workqueue_struct, cpu: c_int) {
    // SAFETY: Hotplug exclusion protects the one native scratch attrs object.
    // The fallback acquires its extra slot ref under the pool lock before use.
    unsafe {
        lupos_wq_assert_pool_mutex_held();
        if (*wq).flags & WQ_UNBOUND as c_uint == 0 || (*(*wq).attrs).ordered {
            return;
        }
        let target_attrs = lupos_wq_update_pwq_attrs_buf();
        copy_workqueue_attrs(target_attrs, (*wq).attrs);
        wqattrs_actualize_cpumask(target_attrs, lupos_wq_unbound_cpumask());
        wq_calc_pod_cpumask(target_attrs, cpu);
        if wqattrs_equal(target_attrs, (*(*unbound_pwq(wq, cpu)).pool).attrs) {
            return;
        }
        let mut pwq = alloc_pwq(wq, target_attrs);
        if pwq.is_null() {
            lupos_wq_warn_update_pwq_alloc_5660(wq);
            lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
            pwq = unbound_pwq(wq, -1);
            lupos_wq_pool_lock_irq((*pwq).pool);
            get_pwq(pwq);
            lupos_wq_pool_unlock_irq((*pwq).pool);
        } else {
            lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
        }
        let old = install_unbound_pwq(wq, cpu, pwq);
        lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
        put_pwq_unlocked(old);
    }
}

/// Allocates and links every per-CPU PWQ, native C5682–5704.
///
/// # Safety
/// The caller owns an unpublished queue with allocated zeroed cpu_pwq storage.
pub(super) unsafe fn alloc_and_link_percpu_pwqs(wq: *mut workqueue_struct) -> c_int {
    // SAFETY: Failure leaves previous slots visible only to the unpublished
    // queue's cleanup path, which performs native unlink and cache free.
    unsafe {
        let mut cpu = lupos_wq_first_possible_cpu();
        while cpu < lupos_wq_small_cpumask_bits() {
            let pool = get_percpu_pool(wq, cpu as c_int);
            let pwq = lupos_wq_alloc_percpu_pwq_5690((*pool).node);
            if pwq.is_null() {
                return -(ENOMEM as c_int);
            }
            init_pwq(pwq, wq, pool);
            lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
            link_pwq(pwq);
            lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
            lupos_wq_pwq_rcu_assign(lupos_wq_percpu_pwq_slot(wq, cpu as c_int), pwq);
            cpu = lupos_wq_next_possible_cpu(cpu);
        }
        0
    }
}

/// Constructs all initial PWQ slots with native unwind, C5706–5759.
///
/// # Safety
/// The caller holds pool mutex and exclusively owns an unpublished queue.
pub(super) unsafe fn alloc_and_link_pwqs(wq: *mut workqueue_struct) -> c_int {
    // SAFETY: Per-CPU storage and PWQs are not reachable outside construction.
    // Unbound prepare failure has already dropped its own temporary pool refs.
    unsafe {
        let highpri = ((*wq).flags & WQ_HIGHPRI as c_uint != 0) as c_int;
        lupos_wq_assert_pool_mutex_held();
        if lupos_wq_alloc_cpu_pwq_5713(wq) {
            let ret;
            if (*wq).flags & WQ_UNBOUND as c_uint == 0 {
                ret = alloc_and_link_percpu_pwqs(wq);
            } else if (*wq).flags & __WQ_ORDERED as c_uint != 0 {
                ret = apply_workqueue_attrs_locked(wq, lupos_wq_ordered_attrs(highpri));
                let dfl_pwq = lupos_wq_pwq_rcu_access(addr_of!((*wq).dfl_pwq));
                let broken = ret == 0 && ((*wq).pwqs.next != addr_of_mut!((*dfl_pwq).pwqs_node)
                    || (*wq).pwqs.prev != addr_of_mut!((*dfl_pwq).pwqs_node));
                lupos_wq_warn_ordered_guarantee_5725(broken, wq);
            } else {
                ret = apply_workqueue_attrs_locked(wq, lupos_wq_std_unbound_attrs(highpri));
            }
            if ret == 0 {
                return 0;
            }
        }
        if lupos_wq_has_cpu_pwq(wq) {
            let mut cpu = lupos_wq_first_possible_cpu();
            while cpu < lupos_wq_small_cpumask_bits() {
                let pwq = lupos_wq_pwq_rcu_access(lupos_wq_percpu_pwq_slot(wq, cpu as c_int));
                if !pwq.is_null() {
                    if !lupos_wq_list_empty(addr_of!((*pwq).pwqs_node)) {
                        lupos_wq_list_del_rcu(addr_of_mut!((*pwq).pwqs_node));
                    }
                    lupos_wq_cache_free_pwq(pwq);
                }
                cpu = lupos_wq_next_possible_cpu(cpu);
            }
            lupos_wq_free_cpu_pwq(wq);
            lupos_wq_clear_cpu_pwq(wq);
        }
        -(ENOMEM as c_int)
    }
}

/// Warns and clamps max_active, native C5761–5769.
///
/// # Safety
/// Name is a live native NUL-terminated string throughout diagnostics.
pub(super) unsafe fn wq_clamp_max_active(max_active: c_int, _flags: c_uint, name: *const c_char) -> c_int {
    // SAFETY: Diagnostic formatting consumes only the supplied native string.
    unsafe {
        if max_active < 1 || max_active > WQ_MAX_ACTIVE as c_int {
            lupos_wq_warn_max_active_5765(max_active, name);
        }
        max_active.max(1).min(WQ_MAX_ACTIVE as c_int)
    }
}

/// Creates the memory-reclaim rescuer, native C5775–5816.
///
/// # Safety
/// The caller holds pool mutex, may sleep and owns the unpublished rescuer slot.
pub(super) unsafe fn init_rescuer(wq: *mut workqueue_struct) -> c_int {
    // SAFETY: The task adopts the allocated rescuer only after successful
    // creation; failed task creation returns its exact PTR_ERR and frees it.
    unsafe {
        lupos_wq_assert_pool_mutex_held();
        if (*wq).flags & WQ_MEM_RECLAIM as c_uint == 0 {
            return 0;
        }
        let rescuer = alloc_worker(NUMA_NO_NODE);
        if rescuer.is_null() {
            lupos_wq_err_rescuer_alloc_5788(wq);
            return -(ENOMEM as c_int);
        }
        (*rescuer).rescue_wq = wq;
        let mut id_buf = [0 as c_char; WORKER_ID_LEN as usize];
        format_worker_id(id_buf.as_mut_ptr(), id_buf.len(), rescuer, null_mut());
        (*rescuer).task = lupos_wq_create_rescuer_5796(Some(rescuer_thread), rescuer.cast::<c_void>(), id_buf.as_ptr());
        if lupos_wq_is_err((*rescuer).task.cast::<c_void>()) {
            let ret = lupos_wq_ptr_err((*rescuer).task.cast::<c_void>()) as c_int;
            lupos_wq_err_rescuer_task_5799(wq, ret);
            kfree(rescuer.cast::<c_void>());
            return ret;
        }
        (*wq).rescuer = rescuer;
        let mask = if lupos_wq_cpumask_intersects(lupos_wq_unbound_cpumask(), lupos_wq_cpu_active_mask()) {
            lupos_wq_unbound_cpumask()
        } else {
            lupos_wq_cpu_possible_mask()
        };
        kthread_bind_mask((*rescuer).task, mask);
        wake_up_process((*rescuer).task);
        0
    }
}

/// Updates active limits and fairly activates pending work, native C5826–5879.
///
/// # Safety
/// The caller holds wq mutex and the appropriate freeze/pool exclusion.
pub(super) unsafe fn wq_adjust_max_active(wq: *mut workqueue_struct) {
    // SAFETY: Published limits use native WRITE_ONCE. Each activation owns the
    // pool irqsave lock; round-robin PWQ traversal runs under wq mutex.
    unsafe {
        lupos_wq_assert_wq_mutex_held(wq);
        let (new_max, new_min) = if (*wq).flags & WQ_FREEZABLE as c_uint != 0 && lupos_wq_freezing() {
            (0, 0)
        } else {
            ((*wq).saved_max_active, (*wq).saved_min_active)
        };
        if (*wq).max_active == new_max && (*wq).min_active == new_min {
            return;
        }
        lupos_wq_write_once_int(addr_of_mut!((*wq).max_active), new_max);
        lupos_wq_write_once_int(addr_of_mut!((*wq).min_active), new_min);
        if (*wq).flags & WQ_UNBOUND as c_uint != 0 {
            wq_update_node_max_active(wq, -1);
        }
        if new_max == 0 {
            return;
        }
        loop {
            let mut activated = false;
            let mut pwq = lupos_wq_pwq_first(wq);
            while !pwq.is_null() {
                let pool = (*pwq).pool;
                let irq_flags = lupos_wq_pool_lock_irqsave(pool);
                if pwq_activate_first_inactive(pwq, true) {
                    activated = true;
                    kick_pool(pool);
                }
                lupos_wq_pool_unlock_irqrestore(pool, irq_flags);
                pwq = lupos_wq_pwq_next(wq, pwq);
            }
            if !activated {
                break;
            }
        }
    }
}

/// Owns allocation/flags/lifetime policy behind native va_list, C5881–6014.
///
/// # Safety
/// The native adapter supplies fmt and an opaque live pointer to its own
/// va_list object. Only the native formatting leaf interprets that pointer;
/// Rust never projects or copies a va_list layout. The caller may sleep.
pub(super) unsafe extern "C" fn __alloc_workqueue(
    fmt: *const c_char,
    mut flags: c_uint,
    mut max_active: c_int,
    native_args: *mut c_void,
) -> *mut workqueue_struct {
    // SAFETY: All allocations are unpublished until complete PWQ setup and
    // list insertion. Each failure branch performs the native partial unwind.
    unsafe {
        if flags & WQ_BH as c_uint != 0 {
            if lupos_wq_warn_alloc_bh_flags_5891(flags & !(__WQ_BH_ALLOWS as c_uint) != 0) {
                return null_mut();
            }
            if lupos_wq_warn_alloc_bh_active_5893(max_active != 0) {
                return null_mut();
            }
        }
        if flags & WQ_POWER_EFFICIENT as c_uint != 0 && lupos_wq_power_efficient() {
            flags = (flags & !(WQ_PERCPU as c_uint)) | WQ_UNBOUND as c_uint;
        }
        let wq_size = if flags & WQ_UNBOUND as c_uint != 0 {
            lupos_wq_unbound_wq_size()
        } else {
            size_of::<workqueue_struct>()
        };
        let wq = lupos_wq_alloc_wq_noprof_5907(wq_size);
        if wq.is_null() {
            return null_mut();
        }
        (*wq).attrs = alloc_workqueue_attrs_noprof();
        if (*wq).attrs.is_null() {
            free_workqueue_attrs((*wq).attrs);
            kfree(wq.cast::<c_void>());
            return null_mut();
        }
        let name_len = lupos_wq_format_wq_name_5915(wq, fmt, native_args);
        if name_len >= WQ_NAME_LEN as c_int {
            lupos_wq_warn_name_truncated_5918(wq);
        }
        if lupos_wq_unlikely_alloc_no_binding_5928(flags & (WQ_UNBOUND | WQ_PERCPU) as c_uint == 0) {
            lupos_wq_warn_alloc_no_binding_5929(wq);
            flags |= WQ_PERCPU as c_uint;
        } else if lupos_wq_unlikely_alloc_both_bindings_5932(
            flags & WQ_PERCPU as c_uint != 0 && flags & WQ_UNBOUND as c_uint != 0,
        ) {
            lupos_wq_warn_alloc_both_bindings_5933(wq);
            flags &= !(WQ_PERCPU as c_uint);
        }
        if flags & WQ_BH as c_uint != 0 {
            max_active = INT_MAX as c_int;
        } else {
            if max_active == 0 {
                max_active = WQ_DFL_ACTIVE as c_int;
            }
            max_active = wq_clamp_max_active(max_active, flags, lupos_wq_wq_name(wq));
        }
        (*wq).flags = flags;
        (*wq).max_active = max_active;
        (*wq).min_active = max_active.min(WQ_DFL_MIN_ACTIVE as c_int);
        (*wq).saved_max_active = (*wq).max_active;
        (*wq).saved_min_active = (*wq).min_active;
        lupos_wq_init_wq_mutex_5955(wq);
        lupos_wq_atomic_set(addr_of_mut!((*wq).nr_pwqs_to_flush), 0);
        lupos_wq_init_list_head(addr_of_mut!((*wq).pwqs));
        lupos_wq_init_list_head(addr_of_mut!((*wq).flusher_queue));
        lupos_wq_init_list_head(addr_of_mut!((*wq).flusher_overflow));
        lupos_wq_init_list_head(addr_of_mut!((*wq).maydays));
        lupos_wq_init_list_head(addr_of_mut!((*wq).list));
        if flags & WQ_UNBOUND as c_uint != 0 && alloc_node_nr_active(lupos_wq_node_nr_active_array(wq)) < 0 {
            free_workqueue_attrs((*wq).attrs);
            kfree(wq.cast::<c_void>());
            return null_mut();
        }
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        if alloc_and_link_pwqs(wq) < 0 {
            lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
            if (*wq).flags & WQ_UNBOUND as c_uint != 0 {
                kthread_flush_worker(lupos_wq_pwq_release_worker());
                free_node_nr_active(lupos_wq_node_nr_active_array(wq));
            }
            free_workqueue_attrs((*wq).attrs);
            kfree(wq.cast::<c_void>());
            return null_mut();
        }
        lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
        wq_adjust_max_active(wq);
        lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
        lupos_wq_list_add_tail_rcu(addr_of_mut!((*wq).list), lupos_wq_workqueues());
        if lupos_wq_online() && init_rescuer(wq) < 0 {
            lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
            destroy_workqueue(wq);
            return null_mut();
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        if (*wq).flags & WQ_SYSFS as c_uint != 0 && workqueue_sysfs_register(wq) != 0 {
            destroy_workqueue(wq);
            return null_mut();
        }
        wq
    }
}

/// Adds owned lockdep state after allocation, native C6016–6029.
///
/// # Safety
/// The native compiler-ABI adapter supplies __alloc_workqueue's opaque args.
pub(super) unsafe extern "C" fn alloc_workqueue_va(fmt: *const c_char, flags: c_uint, max_active: c_int, native_args: *mut c_void) -> *mut workqueue_struct {
    // SAFETY: The args remain native-owned and live until the adapter va_end.
    unsafe {
        let wq = __alloc_workqueue(fmt, flags, max_active, native_args);
        if !wq.is_null() {
            wq_init_lockdep(wq);
        }
        wq
    }
}

/// Releases the device-managed queue, native C6047–6050.
///
/// # Safety
/// Devres invokes this exactly once for the registered live queue.
pub(super) unsafe extern "C" fn devm_workqueue_release(res: *mut c_void) {
    // SAFETY: The devres action retains destruction ownership of this queue.
    unsafe { destroy_workqueue(res.cast::<workqueue_struct>()) }
}

/// Finishes devres registration after the native va_end, C6062–6071.
///
/// # Safety
/// Native devm_alloc_workqueue_noprof calls this after its compiler-ABI-only
/// va_start/alloc_workqueue_va/va_end phase, with null or newly allocated wq.
pub(super) unsafe extern "C" fn finish_devm_workqueue(dev: *mut device, wq: *mut workqueue_struct) -> *mut workqueue_struct {
    // SAFETY: The genuine action-or-reset operation destroys wq on failure;
    // returning null must not free it a second time.
    unsafe {
        if wq.is_null() {
            return null_mut();
        }
        if lupos_wq_devm_add_action_or_reset_6066(dev, Some(devm_workqueue_release), wq.cast::<c_void>()) != 0 {
            return null_mut();
        }
        wq
    }
}

/// Installs a caller-owned lockdep map after native va_end, C6086–6092.
///
/// # Safety
/// Native alloc_workqueue_lockdep_map invokes this with null or a new queue
/// from __alloc_workqueue; map outlives all queue operations.
#[cfg(CONFIG_LOCKDEP)]
pub(super) unsafe extern "C" fn finish_workqueue_lockdep_map(wq: *mut workqueue_struct, map: *mut lockdep_map) -> *mut workqueue_struct {
    // SAFETY: No owned lockdep map/key was registered by __alloc_workqueue.
    unsafe {
        if !wq.is_null() {
            (*wq).lockdep_map = map;
        }
        wq
    }
}

/// Checks destruction-time PWQ activity, native C6096–6110.
///
/// # Safety
/// Queue/pool locks serialize the live PWQ and in-flight/color/reference state.
pub(super) unsafe fn pwq_busy(pwq: *mut pool_workqueue) -> bool {
    // SAFETY: Every native color slot is read within its configured extent.
    unsafe {
        let mut i: c_int = 0;
        while i < WORK_NR_COLORS as c_int {
            if *addr_of!((*pwq).nr_in_flight).cast::<c_int>().add(i as usize) != 0 {
                return true;
            }
            i = i.wrapping_add(1);
        }
        if pwq != lupos_wq_pwq_rcu_access(addr_of!((*(*pwq).wq).dfl_pwq)) && (*pwq).refcnt > 1 {
            return true;
        }
        !pwq_is_empty(pwq)
    }
}

/// Drains and retires a queue, native C6129–6201.
///
/// # Safety
/// The caller owns destruction, can sleep, and has canceled delayed work that
/// is still only timer-linked. Such delayed work is not owned by this queue.
pub(super) unsafe extern "C" fn destroy_workqueue(wq: *mut workqueue_struct) {
    // SAFETY: Drain precedes teardown; sanity failure leaves the diagnosed
    // queue allocated. RCU protects the final loop while each base ref is put.
    unsafe {
        workqueue_sysfs_unregister(wq);
        lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
        (*wq).flags |= __WQ_DESTROYING as c_uint;
        lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
        drain_workqueue(wq);
        if !(*wq).rescuer.is_null() {
            kthread_stop((*(*wq).rescuer).task);
            kfree((*wq).rescuer.cast::<c_void>());
            (*wq).rescuer = null_mut();
        }
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
        let mut pwq = lupos_wq_pwq_first(wq);
        while !pwq.is_null() {
            let pool = (*pwq).pool;
            lupos_wq_pool_lock_irq(pool);
            if lupos_wq_warn_destroy_busy_6164(pwq_busy(pwq)) {
                lupos_wq_warn_destroy_busy_name_6165(wq);
                show_pwq(pwq);
                lupos_wq_pool_unlock_irq(pool);
                lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
                lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
                show_one_workqueue(wq);
                return;
            }
            lupos_wq_pool_unlock_irq(pool);
            pwq = lupos_wq_pwq_next(wq, pwq);
        }
        lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
        lupos_wq_list_del_rcu(addr_of_mut!((*wq).list));
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        lupos_wq_rcu_read_lock();
        let mut cpu = lupos_wq_first_possible_cpu();
        while cpu < lupos_wq_small_cpumask_bits() {
            put_pwq_unlocked(unbound_pwq(wq, cpu as c_int));
            lupos_wq_pwq_rcu_init(unbound_pwq_slot(wq, cpu as c_int), null_mut());
            cpu = lupos_wq_next_possible_cpu(cpu);
        }
        put_pwq_unlocked(unbound_pwq(wq, -1));
        lupos_wq_pwq_rcu_init(unbound_pwq_slot(wq, -1), null_mut());
        lupos_wq_rcu_read_unlock();
    }
}

/// Changes the saved max-active setting, native C6215–6235.
///
/// # Safety
/// The caller retains wq and is outside IRQ context; the function may sleep.
pub(super) unsafe extern "C" fn workqueue_set_max_active(wq: *mut workqueue_struct, mut max_active: c_int) {
    // SAFETY: Wq mutex protects saved settings and active-limit adjustment.
    unsafe {
        if lupos_wq_warn_set_max_bh_6218((*wq).flags & WQ_BH as c_uint != 0) {
            return;
        }
        if lupos_wq_warn_set_max_ordered_6221((*wq).flags & __WQ_ORDERED as c_uint != 0) {
            return;
        }
        max_active = wq_clamp_max_active(max_active, (*wq).flags, lupos_wq_wq_name(wq));
        lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
        (*wq).saved_max_active = max_active;
        if (*wq).flags & WQ_UNBOUND as c_uint != 0 {
            (*wq).saved_min_active = (*wq).saved_min_active.min(max_active);
        }
        wq_adjust_max_active(wq);
        lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
    }
}

/// Changes a nonordered unbound queue's saved minimum, native C6252–6263.
///
/// # Safety
/// The caller retains wq and may sleep for its mutex.
pub(super) unsafe extern "C" fn workqueue_set_min_active(wq: *mut workqueue_struct, min_active: c_int) {
    // SAFETY: Native flags are validated before touching saved settings.
    unsafe {
        if lupos_wq_warn_set_min_flags_6255(
            (*wq).flags & (WQ_BH | WQ_UNBOUND | __WQ_ORDERED) as c_uint != WQ_UNBOUND as c_uint,
        ) {
            return;
        }
        lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
        (*wq).saved_min_active = min_active.max(0).min((*wq).saved_max_active);
        wq_adjust_max_active(wq);
        lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
    }
}

/// Disassociates this CPU's threaded pools, native C6804–6850.
///
/// # Safety
/// Runs on the offlining local CPU with hotplug exclusion; worker membership
/// is protected by attach mutex and flags/counters by the pool irq lock.
#[cfg(CONFIG_SMP)]
pub(super) unsafe fn unbind_workers(cpu: c_int) {
    // SAFETY: UNBOUND is set before nr_running reset, while no local scheduler
    // callback can interleave; actual task affinity changes occur after unlock.
    unsafe {
        let mut index: c_int = 0;
        while index < NR_STD_WORKER_POOLS as c_int {
            let pool = lupos_wq_cpu_pool(cpu, index);
            lupos_wq_mutex_lock(lupos_wq_pool_attach_mutex());
            lupos_wq_pool_lock_irq(pool);
            let head = addr_of_mut!((*pool).workers);
            let mut node = (*head).next;
            while node != head {
                let worker = lupos_wq_worker_from_node(node);
                lupos_wq_assert_pool_worker_6821();
                (*worker).flags |= WORKER_UNBOUND as c_uint;
                node = (*node).next;
            }
            (*pool).flags |= POOL_DISASSOCIATED as c_uint;
            (*pool).nr_running = 0;
            kick_pool(pool);
            lupos_wq_pool_unlock_irq(pool);
            let mut node = (*head).next;
            while node != head {
                lupos_wq_assert_pool_worker_6845();
                unbind_worker(lupos_wq_worker_from_node(node));
                node = (*node).next;
            }
            lupos_wq_mutex_unlock(lupos_wq_pool_attach_mutex());
            index = index.wrapping_add(1);
        }
    }
}

/// Restores local task affinity then concurrency tracking, native C6858–6906.
///
/// # Safety
/// Attach mutex and CPU-online exclusion are held for the live threaded pool.
#[cfg(CONFIG_SMP)]
pub(super) unsafe fn rebind_workers(pool: *mut worker_pool) {
    // SAFETY: Every worker is first moved to its CPU's runqueue. One WRITE_ONCE
    // replaces UNBOUND with REBOUND so an unlocked scheduler test cannot see a
    // transient running state and incorrectly change nr_running.
    unsafe {
        lupos_wq_assert_pool_attach_mutex_held();
        let head = addr_of_mut!((*pool).workers);
        let mut node = (*head).next;
        while node != head {
            let worker = lupos_wq_worker_from_node(node);
            lupos_wq_assert_pool_worker_6871();
            kthread_set_per_cpu((*worker).task, (*pool).cpu);
            lupos_wq_warn_rebind_affinity_6873(set_cpus_allowed_ptr((*worker).task, pool_allowed_cpus(pool)) < 0);
            node = (*node).next;
        }
        lupos_wq_pool_lock_irq(pool);
        (*pool).flags &= !(POOL_DISASSOCIATED as c_uint);
        let mut node = (*head).next;
        while node != head {
            let worker = lupos_wq_worker_from_node(node);
            lupos_wq_assert_pool_worker_6881();
            let mut flags = (*worker).flags;
            lupos_wq_warn_rebind_unbound_6899(flags & WORKER_UNBOUND as c_uint == 0);
            flags |= WORKER_REBOUND as c_uint;
            flags &= !(WORKER_UNBOUND as c_uint);
            lupos_wq_write_once_uint(addr_of_mut!((*worker).flags), flags);
            node = (*node).next;
        }
        lupos_wq_pool_unlock_irq(pool);
    }
}

/// Restores unbound workers' online affinity, native C6918–6934.
///
/// # Safety
/// The caller holds attach mutex and CPU-online exclusion; pool is unbound.
#[cfg(CONFIG_SMP)]
pub(super) unsafe fn restore_unbound_workers_cpumask(pool: *mut worker_pool, cpu: c_int) {
    // SAFETY: The native static scratch mask is serialized by attach mutex.
    unsafe {
        lupos_wq_assert_pool_attach_mutex_held();
        let allowed = lupos_wq_attrs_cpumask((*pool).attrs);
        if !lupos_wq_cpumask_test_cpu(cpu, allowed) {
            return;
        }
        let mask = lupos_wq_restore_cpumask();
        lupos_wq_cpumask_and(mask, allowed, lupos_wq_cpu_online_mask());
        let head = addr_of_mut!((*pool).workers);
        let mut node = (*head).next;
        while node != head {
            let worker = lupos_wq_worker_from_node(node);
            lupos_wq_assert_pool_worker_6932();
            lupos_wq_warn_restore_affinity_6933(set_cpus_allowed_ptr((*worker).task, mask) < 0);
            node = (*node).next;
        }
    }
}

/// Creates missing threaded workers before CPU online, native C6936–6947.
///
/// # Safety
/// The CPU-hotplug caller supplies an allocated possible CPU and may sleep.
#[cfg(CONFIG_SMP)]
pub(super) unsafe extern "C" fn workqueue_prepare_cpu(cpu: c_uint) -> c_int {
    // SAFETY: Hotplug serialization stabilizes the per-CPU pool array.
    unsafe {
        let mut index: c_int = 0;
        while index < NR_STD_WORKER_POOLS as c_int {
            let pool = lupos_wq_cpu_pool(cpu as c_int, index);
            if (*pool).nr_workers == 0 && create_worker(pool).is_null() {
                return -(ENOMEM as c_int);
            }
            index = index.wrapping_add(1);
        }
        0
    }
}

/// Rebinds pools and updates unbound pods after CPU online, C6949–6991.
///
/// # Safety
/// The CPU-hotplug core calls this with online transition exclusion.
#[cfg(CONFIG_SMP)]
pub(super) unsafe extern "C" fn workqueue_online_cpu(cpu: c_uint) -> c_int {
    // SAFETY: Pool mutex protects IDR/topology/queue traversal; attach mutex
    // protects task rebinding and queue mutex protects active-limit updates.
    unsafe {
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        lupos_wq_cpumask_set_cpu(cpu, lupos_wq_online_cpumask().cast_mut());
        let mut pi: c_int = 0;
        loop {
            let pool = idr_get_next(lupos_wq_worker_pool_idr(), addr_of_mut!(pi)).cast::<worker_pool>();
            if pool.is_null() {
                break;
            }
            lupos_wq_assert_each_pool_6959();
            if (*pool).flags & POOL_BH as c_uint == 0 {
                lupos_wq_mutex_lock(lupos_wq_pool_attach_mutex());
                if (*pool).cpu as c_uint == cpu {
                    rebind_workers(pool);
                } else if (*pool).cpu < 0 {
                    restore_unbound_workers_cpumask(pool, cpu as c_int);
                }
                lupos_wq_mutex_unlock(lupos_wq_pool_attach_mutex());
            }
            pi = pi.wrapping_add(1);
        }
        let head = lupos_wq_workqueues();
        let mut node = (*head).next;
        while node != head {
            let wq = lupos_wq_wq_from_list(node);
            if (*wq).flags & WQ_UNBOUND as c_uint != 0 {
                let pt = wqattrs_pod_type((*wq).attrs);
                let pod = *(*pt).cpu_pod.add(cpu as usize);
                let mask = lupos_wq_pod_cpus(pt, pod);
                let mut tcpu = lupos_wq_first_cpu_in_mask(mask);
                while tcpu < lupos_wq_small_cpumask_bits() {
                    unbound_wq_update_pwq(wq, tcpu as c_int);
                    tcpu = lupos_wq_next_cpu_in_mask(tcpu, mask);
                }
                lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
                wq_update_node_max_active(wq, -1);
                lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
            }
            node = (*node).next;
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        0
    }
}

/// Disassociates local pools and updates affected pods, native C6993–7026.
///
/// # Safety
/// The CPU-hotplug core calls this on the CPU being offlined.
#[cfg(CONFIG_SMP)]
pub(super) unsafe extern "C" fn workqueue_offline_cpu(cpu: c_uint) -> c_int {
    // SAFETY: Local CPU validation precedes any state transition; all pod and
    // active-count changes retain the original lock and CPU-offline ordering.
    unsafe {
        if lupos_wq_warn_offline_cpu_6998(cpu != lupos_wq_smp_processor_id()) {
            return -1;
        }
        unbind_workers(cpu as c_int);
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        lupos_wq_cpumask_clear_cpu(cpu, lupos_wq_online_cpumask().cast_mut());
        let head = lupos_wq_workqueues();
        let mut node = (*head).next;
        while node != head {
            let wq = lupos_wq_wq_from_list(node);
            if (*wq).flags & WQ_UNBOUND as c_uint != 0 {
                let pt = wqattrs_pod_type((*wq).attrs);
                let pod = *(*pt).cpu_pod.add(cpu as usize);
                let mask = lupos_wq_pod_cpus(pt, pod);
                let mut tcpu = lupos_wq_first_cpu_in_mask(mask);
                while tcpu < lupos_wq_small_cpumask_bits() {
                    unbound_wq_update_pwq(wq, tcpu as c_int);
                    tcpu = lupos_wq_next_cpu_in_mask(tcpu, mask);
                }
                lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
                wq_update_node_max_active(wq, cpu as c_int);
                lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
            }
            node = (*node).next;
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        0
    }
}

/// Executes a synchronous CPU callback, native C7035–7040.
///
/// # Safety
/// The queued stack work_for_cpu and its non-null callback/argument remain live.
#[cfg(CONFIG_SMP)]
pub(super) unsafe extern "C" fn work_for_cpu_fn(work: *mut work_struct) {
    // SAFETY: work_on_cpu_key initializes the native callback field and waits
    // for completion before destroying its stack work or reading the result.
    unsafe {
        let wfc = lupos_wq_work_for_cpu_from_work(work);
        let func = lupos_wq_work_for_cpu_callback(wfc).unwrap_unchecked();
        (*wfc).ret = func((*wfc).arg);
    }
}

/// Runs a function synchronously on a pinned-online CPU, native C7054–7064.
///
/// # Safety
/// Func is non-null; arg satisfies its contract. Caller prevents CPU offline
/// and holds no lock that would block the callback from completing.
#[cfg(CONFIG_SMP)]
pub(super) unsafe extern "C" fn work_on_cpu_key(
    cpu: c_int,
    func: Option<unsafe extern "C" fn(*mut c_void) -> c_long>,
    arg: *mut c_void,
    key: *mut lock_class_key,
) -> c_long {
    // SAFETY: Native stack storage stays at its initialized address through
    // schedule/flush and native debug destruction; callback writes happen-before
    // the return-value read after flush.
    unsafe {
        let mut storage = MaybeUninit::<work_for_cpu>::zeroed();
        let wfc = storage.as_mut_ptr();
        lupos_wq_work_for_cpu_set_callback(wfc, func);
        (*wfc).arg = arg;
        lupos_wq_init_work_on_cpu_7059(wfc, Some(work_for_cpu_fn), key);
        queue_work_on(cpu, lupos_wq_system_percpu_wq(), addr_of_mut!((*wfc).work));
        flush_work(addr_of_mut!((*wfc).work));
        lupos_wq_destroy_work_on_stack(addr_of_mut!((*wfc).work));
        (*wfc).ret
    }
}

/// Suppresses freezable queues' active limits, native C7080–7096.
///
/// # Safety
/// The freezer invokes this in the native freeze protocol and may sleep.
#[cfg(CONFIG_FREEZER)]
pub(super) unsafe extern "C" fn freeze_workqueues_begin() {
    // SAFETY: Pool mutex serializes freeze state and queue list; each queue
    // mutex serializes its active limits without changing the saved settings.
    unsafe {
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        lupos_wq_warn_freeze_already_7086(lupos_wq_freezing());
        lupos_wq_set_freezing(true);
        let head = lupos_wq_workqueues();
        let mut node = (*head).next;
        while node != head {
            let wq = lupos_wq_wq_from_list(node);
            lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
            wq_adjust_max_active(wq);
            lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
            node = (*node).next;
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
    }
}

/// Reports remaining active work while freezing, native C7111–7142.
///
/// # Safety
/// Called between freeze_workqueues_begin and thaw_workqueues; may sleep.
#[cfg(CONFIG_FREEZER)]
pub(super) unsafe extern "C" fn freeze_workqueues_busy() -> bool {
    // SAFETY: nr_active only decreases in this interval. Native source reads
    // it without pool lock; RCU protects PWQs while the list is traversed.
    unsafe {
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        lupos_wq_warn_freeze_not_started_7119(!lupos_wq_freezing());
        let head = lupos_wq_workqueues();
        let mut node = (*head).next;
        while node != head {
            let wq = lupos_wq_wq_from_list(node);
            if (*wq).flags & WQ_FREEZABLE as c_uint != 0 {
                lupos_wq_rcu_read_lock();
                let mut pwq = lupos_wq_pwq_first(wq);
                while !pwq.is_null() {
                    lupos_wq_warn_freeze_negative_7130((*pwq).nr_active < 0);
                    if (*pwq).nr_active != 0 {
                        lupos_wq_rcu_read_unlock();
                        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
                        return true;
                    }
                    pwq = lupos_wq_pwq_next(wq, pwq);
                }
                lupos_wq_rcu_read_unlock();
            }
            node = (*node).next;
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        false
    }
}

/// Restores saved active limits after freezing, native C7153–7173.
///
/// # Safety
/// Invoked by the freezer in process context with queue lifetime established.
#[cfg(CONFIG_FREEZER)]
pub(super) unsafe extern "C" fn thaw_workqueues() {
    // SAFETY: Global freeze state is cleared before activating queued work.
    unsafe {
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        if lupos_wq_freezing() {
            lupos_wq_set_freezing(false);
            let head = lupos_wq_workqueues();
            let mut node = (*head).next;
            while node != head {
                let wq = lupos_wq_wq_from_list(node);
                lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
                wq_adjust_max_active(wq);
                lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
                node = (*node).next;
            }
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
    }
}

/// Atomically prepares and commits every global affinity change, C7176–7228.
///
/// # Safety
/// Pool mutex is held; the caller keeps the input mask live and may sleep.
pub(super) unsafe fn workqueue_apply_unbound_cpumask(unbound_cpumask: *const cpumask) -> c_int {
    // SAFETY: Contexts own every temporary ref until all prepare operations
    // succeed. Cached next pointers permit cleanup without dereferencing a
    // freed context; no queue commits at all if one prepare fails.
    unsafe {
        let mut storage = MaybeUninit::<list_head>::uninit();
        let ctxs = storage.as_mut_ptr();
        lupos_wq_init_list_head(ctxs);
        let mut ret: c_int = 0;
        lupos_wq_assert_pool_mutex_held();
        let head = lupos_wq_workqueues();
        let mut node = (*head).next;
        while node != head {
            let wq = lupos_wq_wq_from_list(node);
            if (*wq).flags & WQ_UNBOUND as c_uint != 0 && (*wq).flags & __WQ_DESTROYING as c_uint == 0 {
                let ctx = apply_wqattrs_prepare(wq, (*wq).attrs, unbound_cpumask);
                if lupos_wq_is_err(ctx.cast::<c_void>()) {
                    ret = lupos_wq_ptr_err(ctx.cast::<c_void>()) as c_int;
                    break;
                }
                lupos_wq_list_add_tail(addr_of_mut!((*ctx).list), ctxs);
            }
            node = (*node).next;
        }
        let mut node = (*ctxs).next;
        while node != ctxs {
            let next = (*node).next;
            let ctx = lupos_wq_ctx_from_list(node);
            if ret == 0 {
                apply_wqattrs_commit(ctx);
            }
            apply_wqattrs_cleanup(ctx);
            node = next;
        }
        if ret == 0 {
            lupos_wq_mutex_lock(lupos_wq_pool_attach_mutex());
            lupos_wq_cpumask_copy(lupos_wq_unbound_cpumask().cast_mut(), unbound_cpumask);
            let mut node = (*head).next;
            while node != head {
                let wq = lupos_wq_wq_from_list(node);
                if !(*wq).rescuer.is_null() && (*(*wq).rescuer).pool.is_null() {
                    unbind_worker((*wq).rescuer);
                }
                node = (*node).next;
            }
            let mut cpu = lupos_wq_first_possible_cpu();
            while cpu < lupos_wq_small_cpumask_bits() {
                let mut index: c_int = 0;
                while index < NR_STD_WORKER_POOLS as c_int {
                    let pool = lupos_wq_cpu_pool(cpu as c_int, index);
                    if (*pool).flags & POOL_DISASSOCIATED as c_uint != 0 {
                        let workers = addr_of_mut!((*pool).workers);
                        let mut node = (*workers).next;
                        while node != workers {
                            lupos_wq_assert_pool_worker_7221();
                            unbind_worker(lupos_wq_worker_from_node(node));
                            node = (*node).next;
                        }
                    }
                    index = index.wrapping_add(1);
                }
                cpu = lupos_wq_next_possible_cpu(cpu);
            }
            lupos_wq_mutex_unlock(lupos_wq_pool_attach_mutex());
        }
        ret
    }
}

/// Applies the requested-affinity/housekeeping intersection, native C7240–7268.
///
/// # Safety
/// Hk is a live native cpumask and caller may sleep for allocation/mutex.
pub(super) unsafe extern "C" fn workqueue_unbound_housekeeping_update(hk: *const cpumask) -> c_int {
    // SAFETY: The temporary uses configured cpumask_var_t storage, with only
    // address/value leaves interpreting its pointer-versus-array representation.
    unsafe {
        let mut storage = MaybeUninit::<cpumask_var_t>::uninit();
        if !lupos_wq_zalloc_hk_cpumask_7245(storage.as_mut_ptr()) {
            return -(ENOMEM as c_int);
        }
        let mask = lupos_wq_cpumask_var_value(storage.as_ptr());
        let mut ret: c_int = 0;
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        if !lupos_wq_cpumask_and(mask, lupos_wq_requested_unbound_cpumask(), hk) {
            lupos_wq_cpumask_copy(mask, lupos_wq_requested_unbound_cpumask());
        }
        if !lupos_wq_cpumask_equal(mask, lupos_wq_unbound_cpumask()) {
            ret = workqueue_apply_unbound_cpumask(mask);
        }
        if ret == 0 {
            lupos_wq_cpumask_andnot(lupos_wq_isolated_cpumask(), lupos_wq_cpu_possible_mask(), hk);
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        lupos_wq_free_cpumask_var(storage.as_mut_ptr());
        ret
    }
}

/// Matches a native affinity-scope parameter string, native C7270–7273.
///
/// # Safety
/// Val is a live native NUL-terminated parameter string.
pub(super) unsafe fn parse_affn_scope(val: *const c_char) -> c_int {
    // SAFETY: The native array retains its configured size and string objects.
    unsafe { lupos_wq_sysfs_match_affn_names(val) }
}

/// Changes default scope and refreshes online CPU slots, native C7275–7300.
///
/// # Safety
/// Parameter core supplies live val/kp; callback may sleep for hotplug exclusion.
pub(super) unsafe extern "C" fn wq_affn_dfl_set(val: *const c_char, _kp: *const kernel_param) -> c_int {
    // SAFETY: cpus_read_lock precedes pool mutex, stabilizing online CPU and
    // pod traversal. The configured native global remains its sole owner.
    unsafe {
        let affn = parse_affn_scope(val);
        if affn < 0 {
            return affn;
        }
        if affn == WQ_AFFN_DFL as c_int {
            return -(EINVAL as c_int);
        }
        lupos_wq_cpus_read_lock();
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        lupos_wq_set_affn_dfl(affn);
        let head = lupos_wq_workqueues();
        let mut node = (*head).next;
        while node != head {
            let wq = lupos_wq_wq_from_list(node);
            let mut cpu = lupos_wq_first_online_cpu();
            while cpu >= 0 {
                unbound_wq_update_pwq(wq, cpu);
                cpu = lupos_wq_next_online_cpu(cpu);
            }
            node = (*node).next;
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        lupos_wq_cpus_read_unlock();
        0
    }
}

/// Formats the native affinity-scope parameter, native C7302–7305.
///
/// # Safety
/// Parameter core supplies a writable PAGE_SIZE buffer and valid kp.
pub(super) unsafe extern "C" fn wq_affn_dfl_get(buffer: *mut c_char, _kp: *const kernel_param) -> c_int {
    // SAFETY: Native scnprintf retains PAGE_SIZE and "%s\n" varargs ABI.
    unsafe { lupos_wq_format_affn_dfl_7304(buffer) }
}

/// Constructs a complete native pod type, native C8342–8378.
///
/// # Safety
/// Called during topology init with unpublished pt and stable CPU topology;
/// the callback is a genuine non-null topology predicate for possible CPUs.
pub(super) unsafe fn init_pod_type(pt: *mut wq_pod_type, cpus_share_pod: unsafe extern "C" fn(c_int, c_int) -> bool) {
    // SAFETY: Every native table/mask is allocated before indexed writes.
    // Earlier possible CPUs already have assigned pod IDs when reused.
    unsafe {
        (*pt).nr_pods = 0;
        let allocated = lupos_wq_alloc_topology_cpu_pod_8350(pt);
        lupos_wq_bug_topology_cpu_pod_8351(!allocated);
        let mut cur = lupos_wq_first_possible_cpu();
        while cur < lupos_wq_small_cpumask_bits() {
            let mut pre = lupos_wq_first_possible_cpu();
            while pre < lupos_wq_small_cpumask_bits() {
                if pre >= cur {
                    *(*pt).cpu_pod.add(cur as usize) = (*pt).nr_pods;
                    (*pt).nr_pods = (*pt).nr_pods.wrapping_add(1);
                    break;
                }
                if cpus_share_pod(cur as c_int, pre as c_int) {
                    *(*pt).cpu_pod.add(cur as usize) = *(*pt).cpu_pod.add(pre as usize);
                    break;
                }
                pre = lupos_wq_next_possible_cpu(pre);
            }
            cur = lupos_wq_next_possible_cpu(cur);
        }
        let cpus_ok = lupos_wq_alloc_topology_pod_cpus_8367(pt);
        let nodes_ok = lupos_wq_alloc_topology_pod_node_8368(pt);
        lupos_wq_bug_topology_pod_arrays_8369(!cpus_ok || !nodes_ok);
        let mut pod: c_int = 0;
        while pod < (*pt).nr_pods {
            let allocated = lupos_wq_zalloc_topology_pod_mask_8372(pt, pod);
            lupos_wq_bug_topology_pod_mask_8372(!allocated);
            pod = pod.wrapping_add(1);
        }
        let mut cpu = lupos_wq_first_possible_cpu();
        while cpu < lupos_wq_small_cpumask_bits() {
            let pod = *(*pt).cpu_pod.add(cpu as usize);
            lupos_wq_cpumask_set_cpu(cpu, lupos_wq_pod_cpus(pt, pod).cast_mut());
            *(*pt).pod_node.add(pod as usize) = lupos_wq_cpu_to_node(cpu as c_int);
            cpu = lupos_wq_next_possible_cpu(cpu);
        }
    }
}

/// Separates every CPU into its own pod, native C8380–8383.
///
/// # Safety
/// The init-only callback address must not outlive the native init section.
unsafe extern "C" fn cpus_dont_share(_cpu0: c_int, _cpu1: c_int) -> bool {
    false
}

/// Tests SMT sibling membership, native C8385–8388.
///
/// # Safety
/// Both CPUs are possible and the topology is initialized and stable.
unsafe extern "C" fn cpus_share_smt(cpu0: c_int, cpu1: c_int) -> bool {
    // SAFETY: Native topology supplies the sibling mask for a valid CPU.
    unsafe { lupos_wq_cpumask_test_cpu(cpu0, lupos_wq_cpu_smt_mask(cpu1)) }
}

/// Tests native NUMA-node identity, native C8390–8393.
///
/// # Safety
/// Both CPUs are possible and topology is initialized and stable.
unsafe extern "C" fn cpus_share_numa(cpu0: c_int, cpu1: c_int) -> bool {
    // SAFETY: The genuine CPU-to-node primitive handles configured NUMA/UP.
    unsafe { lupos_wq_cpu_to_node(cpu0) == lupos_wq_cpu_to_node(cpu1) }
}

/// Counts one leader per SMT core in an LLC pod, native C8406–8423.
///
/// # Safety
/// Pod masks and the SMT topology tables are initialized and stable.
pub(super) unsafe fn llc_count_cores(pod_cpus: *const cpumask, smt_pods: *mut wq_pod_type) -> c_int {
    // SAFETY: A CPU's SMT pod is valid; exactly its first member is counted.
    unsafe {
        let mut nr_cores: c_int = 0;
        let mut cpu = lupos_wq_first_cpu_in_mask(pod_cpus);
        while cpu < lupos_wq_small_cpumask_bits() {
            let siblings = lupos_wq_pod_cpus(smt_pods, *(*smt_pods).cpu_pod.add(cpu as usize));
            if lupos_wq_cpumask_first(siblings) == cpu {
                nr_cores = nr_cores.wrapping_add(1);
            }
            cpu = lupos_wq_next_cpu_in_mask(cpu, pod_cpus);
        }
        nr_cores
    }
}

/// Computes one shard's target number of cores, native C8432–8440.
pub(super) fn llc_shard_size(shard_id: c_int, cores_per_shard: c_int, nr_large_shards: c_int) -> c_int {
    if shard_id < nr_large_shards {
        cores_per_shard.wrapping_add(1)
    } else {
        cores_per_shard
    }
}

/// Calculates nearest shard count and balanced sizes, native C8450–8460.
///
/// # Safety
/// Called after precompute_cache_shard_ids has made shard size nonzero;
/// nr_cores is a nonnegative count of possible CPUs, bounded by native NR_CPUS.
pub(super) unsafe fn llc_calc_shard_layout(nr_cores: c_int) -> llc_shard_layout {
    // SAFETY: DIV_ROUND_CLOSEST mixes signed nr_cores and unsigned shard size,
    // so the addition/division is explicitly unsigned and wrapping. The result
    // is at least one and fits c_int for a valid possible-CPU count.
    unsafe {
        let size = lupos_wq_cache_shard_size();
        let rounded = (nr_cores as c_uint).wrapping_add(size / 2) / size;
        let nr_shards = rounded.max(1) as c_int;
        llc_shard_layout {
            nr_large_shards: nr_cores % nr_shards,
            cores_per_shard: nr_cores / nr_shards,
            nr_shards,
        }
    }
}

/// Checks exactly the native shard-capacity equality, C8470–8475.
///
/// # Safety
/// Layout is a live initialized native shard-layout value.
pub(super) unsafe fn llc_shard_is_full(cores_in_shard: c_int, shard_id: c_int, layout: *const llc_shard_layout) -> bool {
    // SAFETY: The initialized layout is read only for this comparison.
    unsafe { cores_in_shard == llc_shard_size(shard_id, (*layout).cores_per_shard, (*layout).nr_large_shards) }
}

/// Assigns shard IDs while keeping SMT siblings together, native C8486–8530.
///
/// # Safety
/// LLC/SMT topology is stable during init; cpu_shard_id native initdata storage
/// remains live and shard size was corrected to nonzero before calling.
pub(super) unsafe fn llc_populate_cpu_shard_id(pod_cpus: *const cpumask, smt_pods: *mut wq_pod_type, nr_cores: c_int) {
    // SAFETY: CPU iteration is ascending, so every sibling leader's entry has
    // already been assigned. Native UP bounds warning precedes leader access.
    unsafe {
        let layout = llc_calc_shard_layout(nr_cores);
        let mut cores_in_shard: c_int = 0;
        let mut shard_id: c_int = 0;
        let mut cpu = lupos_wq_first_cpu_in_mask(pod_cpus);
        while cpu < lupos_wq_small_cpumask_bits() {
            let siblings = lupos_wq_pod_cpus(smt_pods, *(*smt_pods).cpu_pod.add(cpu as usize));
            if lupos_wq_cpumask_first(siblings) == cpu {
                if llc_shard_is_full(cores_in_shard, shard_id, addr_of!(layout)) {
                    shard_id = shard_id.wrapping_add(1);
                    cores_in_shard = 0;
                }
                cores_in_shard = cores_in_shard.wrapping_add(1);
                *lupos_wq_cpu_shard_id_slot(cpu as c_int) = shard_id;
            } else {
                let leader = lupos_wq_cpumask_first(siblings);
                if !lupos_wq_warn_shard_leader_8523(leader >= lupos_wq_nr_cpu_ids()) {
                    *lupos_wq_cpu_shard_id_slot(cpu as c_int) = *lupos_wq_cpu_shard_id_slot(leader as c_int);
                }
            }
            cpu = lupos_wq_next_cpu_in_mask(cpu, pod_cpus);
        }
        lupos_wq_warn_shard_count_8529(shard_id != layout.nr_shards.wrapping_sub(1));
    }
}

/// Computes all cache shard IDs, native C8539–8559.
///
/// # Safety
/// CACHE and SMT pod types are initialized; this is the sole initdata writer.
pub(super) unsafe fn precompute_cache_shard_ids() {
    // SAFETY: Zero user shard size is corrected before any division; all
    // per-LLC core counts and SMT assignments are then computed in Rust.
    unsafe {
        let llc = lupos_wq_pod_type(WQ_AFFN_CACHE as c_int);
        let smt = lupos_wq_pod_type(WQ_AFFN_SMT as c_int);
        if lupos_wq_cache_shard_size() == 0 {
            lupos_wq_warn_zero_shard_size_8548();
            lupos_wq_set_cache_shard_size(1);
        }
        let mut pod: c_int = 0;
        while pod < (*llc).nr_pods {
            let cpus = lupos_wq_pod_cpus(llc, pod);
            let nr_cores = llc_count_cores(cpus, smt);
            llc_populate_cpu_shard_id(cpus, smt, nr_cores);
            pod = pod.wrapping_add(1);
        }
    }
}

/// Tests both cache identity and within-cache shard, native C8567–8573.
///
/// # Safety
/// CPU IDs and initdata shard entries are valid in the topology-init phase.
unsafe extern "C" fn cpus_share_cache_shard(cpu0: c_int, cpu1: c_int) -> bool {
    // SAFETY: Shard IDs alone are not globally unique; cache identity is first.
    unsafe { cpus_share_cache(cpu0, cpu1) && *lupos_wq_cpu_shard_id_slot(cpu0) == *lupos_wq_cpu_shard_id_slot(cpu1) }
}

/// Completes the third workqueue initialization stage, native C8582–8614.
///
/// # Safety
/// SMP topology is finalized; native early/full workqueue init already ran.
pub(super) unsafe extern "C" fn workqueue_init_topology() {
    // SAFETY: Native pod storage is owned here until initialized publication;
    // pool mutex then serializes every existing queue's replacement slots.
    unsafe {
        init_pod_type(lupos_wq_pod_type(WQ_AFFN_CPU as c_int), cpus_dont_share);
        init_pod_type(lupos_wq_pod_type(WQ_AFFN_SMT as c_int), cpus_share_smt);
        init_pod_type(lupos_wq_pod_type(WQ_AFFN_CACHE as c_int), cpus_share_cache);
        precompute_cache_shard_ids();
        init_pod_type(lupos_wq_pod_type(WQ_AFFN_CACHE_SHARD as c_int), cpus_share_cache_shard);
        init_pod_type(lupos_wq_pod_type(WQ_AFFN_NUMA as c_int), cpus_share_numa);
        lupos_wq_set_topo_initialized(true);
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        let head = lupos_wq_workqueues();
        let mut node = (*head).next;
        while node != head {
            let wq = lupos_wq_wq_from_list(node);
            let mut cpu = lupos_wq_first_online_cpu();
            while cpu >= 0 {
                unbound_wq_update_pwq(wq, cpu);
                cpu = lupos_wq_next_online_cpu(cpu);
            }
            if (*wq).flags & WQ_UNBOUND as c_uint != 0 {
                lupos_wq_mutex_lock(addr_of_mut!((*wq).mutex));
                wq_update_node_max_active(wq, -1);
                lupos_wq_mutex_unlock(addr_of_mut!((*wq).mutex));
            }
            node = (*node).next;
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
    }
}

/// Parses boot affinity while consuming the setup option, native C8623–8631.
///
/// # Safety
/// The boot parser supplies a mutable native NUL-terminated string before init.
pub(super) unsafe extern "C" fn workqueue_unbound_cpus_setup(string: *mut c_char) -> c_int {
    // SAFETY: Parse failure restores an empty command-line mask, issues the
    // original warning, and still returns 1 to mark the boot option handled.
    unsafe {
        if lupos_wq_cpulist_parse(string, lupos_wq_cmdline_cpumask()) < 0 {
            lupos_wq_cpumask_clear(lupos_wq_cmdline_cpumask());
            lupos_wq_warn_bad_unbound_cpus_8627();
        }
        1
    }
}

/// Raises the normal BH softirq, native C8081–8084.
///
/// # Safety
/// Native irq_work invokes this with local IRQs disabled.
pub(super) unsafe extern "C" fn bh_pool_kick_normal(_work: *mut irq_work) {
    // SAFETY: The callback's native IRQ context satisfies the softirq primitive.
    unsafe { lupos_wq_raise_softirq_irqoff(TASKLET_SOFTIRQ as c_uint) }
}

/// Raises the high-priority BH softirq, native C8086–8089.
///
/// # Safety
/// Native irq_work invokes this with local IRQs disabled.
pub(super) unsafe extern "C" fn bh_pool_kick_highpri(_work: *mut irq_work) {
    // SAFETY: The callback's native IRQ context satisfies the softirq primitive.
    unsafe { lupos_wq_raise_softirq_irqoff(HI_SOFTIRQ as c_uint) }
}

/// Restricts early unbound CPUs only if intersection survives, C8091–8100.
///
/// # Safety
/// Called in exclusive early init with allocated unbound mask and live name.
pub(super) unsafe fn restrict_unbound_cpumask(name: *const c_char, mask: *const cpumask) {
    // SAFETY: The global mask has one native owner; empty restrictions warn
    // without changing it, preserving fallback availability.
    unsafe {
        let unbound = lupos_wq_unbound_cpumask().cast_mut();
        if !lupos_wq_cpumask_intersects(unbound, mask) {
            lupos_wq_warn_restrict_mask_8094(name, mask);
            return;
        }
        lupos_wq_cpumask_and(unbound, unbound, mask);
    }
}

/// Initializes one early static CPU/BH pool, native C8102–8116.
///
/// # Safety
/// Exclusive early init owns the zeroed native per-CPU pool and possible CPU.
pub(super) unsafe fn init_cpu_worker_pool(pool: *mut worker_pool, cpu: c_int, nice: c_int) {
    // SAFETY: Fatal initialization checks precede dereferences of allocated
    // attrs; pool ID publication remains under pool mutex.
    unsafe {
        lupos_wq_bug_init_cpu_pool_8104(init_worker_pool(pool) != 0);
        (*pool).cpu = cpu;
        lupos_wq_cpumask_copy(lupos_wq_attrs_cpumask((*pool).attrs), lupos_wq_cpumask_of(cpu));
        lupos_wq_cpumask_copy(lupos_wq_attrs_pod_cpumask((*pool).attrs), lupos_wq_cpumask_of(cpu));
        (*(*pool).attrs).nice = nice;
        (*(*pool).attrs).affn_strict = true;
        (*pool).node = lupos_wq_cpu_to_node(cpu);
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        lupos_wq_bug_init_cpu_id_8114(worker_pool_assign_id(pool) != 0);
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
    }
}

/// Builds pools and system queues without starting workers, C8128–8234.
///
/// # Safety
/// Runs once in native early-init order after allocator/cpumask/IDR readiness.
/// None of these entrypoints may be published until all owner lanes compose.
pub(super) unsafe extern "C" fn workqueue_init_early() {
    // SAFETY: Native static storage, allocation-site metadata and callbacks
    // retain their sole common owner; Rust performs each initialization phase
    // and queue policy in the original order before any worker can execute.
    unsafe {
        let pt = lupos_wq_pod_type(WQ_AFFN_SYSTEM as c_int);
        let std_nice: [c_int; NR_STD_WORKER_POOLS as usize] = [0, HIGHPRI_NICE_LEVEL as c_int];
        let irq_fns: [Option<unsafe extern "C" fn(*mut irq_work)>; NR_STD_WORKER_POOLS as usize] = [
            Some(bh_pool_kick_normal), Some(bh_pool_kick_highpri),
        ];
        lupos_wq_build_bug_pwq_alignment_8136();
        lupos_wq_bug_alloc_online_mask_8138(!lupos_wq_alloc_online_mask_8138());
        lupos_wq_bug_alloc_unbound_mask_8139(!lupos_wq_alloc_unbound_mask_8139());
        lupos_wq_bug_alloc_requested_mask_8140(!lupos_wq_alloc_requested_mask_8140());
        lupos_wq_bug_alloc_isolated_mask_8141(!lupos_wq_alloc_isolated_mask_8141());
        lupos_wq_cpumask_copy(lupos_wq_online_cpumask().cast_mut(), lupos_wq_cpu_online_mask());
        lupos_wq_cpumask_copy(lupos_wq_unbound_cpumask().cast_mut(), lupos_wq_cpu_possible_mask());
        restrict_unbound_cpumask(c"HK_TYPE_DOMAIN".as_ptr().cast::<c_char>(), lupos_wq_housekeeping_cpumask(HK_TYPE_DOMAIN));
        if !lupos_wq_cpumask_empty(lupos_wq_cmdline_cpumask()) {
            restrict_unbound_cpumask(c"workqueue.unbound_cpus".as_ptr().cast::<c_char>(), lupos_wq_cmdline_cpumask());
        }
        lupos_wq_cpumask_copy(lupos_wq_requested_unbound_cpumask().cast_mut(), lupos_wq_unbound_cpumask());
        lupos_wq_cpumask_andnot(lupos_wq_isolated_cpumask(), lupos_wq_cpu_possible_mask(), lupos_wq_housekeeping_cpumask(HK_TYPE_DOMAIN));
        lupos_wq_set_pwq_cache(lupos_wq_create_pwq_cache_8152());
        lupos_wq_set_update_pwq_attrs_buf(lupos_wq_alloc_early_update_attrs_8154());
        lupos_wq_bug_update_attrs_8155(lupos_wq_update_pwq_attrs_buf().is_null());
        if lupos_wq_housekeeping_enabled(HK_TYPE_TICK) {
            lupos_wq_set_power_efficient(true);
        }
        let cpus_ok = lupos_wq_alloc_system_pod_cpus_8165(pt);
        let nodes_ok = lupos_wq_alloc_system_pod_node_8166(pt);
        let mapping_ok = lupos_wq_alloc_system_cpu_pod_8167(pt);
        lupos_wq_bug_system_pod_arrays_8168(!cpus_ok || !nodes_ok || !mapping_ok);
        lupos_wq_bug_system_pod_mask_8170(!lupos_wq_zalloc_system_pod_mask_8170(pt));
        (*pt).nr_pods = 1;
        lupos_wq_cpumask_copy(lupos_wq_pod_cpus(pt, 0).cast_mut(), lupos_wq_cpu_possible_mask());
        *(*pt).pod_node = NUMA_NO_NODE;
        *(*pt).cpu_pod = 0;
        let mut cpu = lupos_wq_first_possible_cpu();
        while cpu < lupos_wq_small_cpumask_bits() {
            let mut index: c_int = 0;
            while index < NR_STD_WORKER_POOLS as c_int {
                let pool = lupos_wq_bh_pool_at(cpu as c_int, index);
                init_cpu_worker_pool(pool, cpu as c_int, *std_nice.as_ptr().add(index as usize));
                (*pool).flags |= POOL_BH as c_uint;
                lupos_wq_init_irq_work_8185(bh_pool_irq_work(pool), *irq_fns.as_ptr().add(index as usize));
                index = index.wrapping_add(1);
            }
            let mut index: c_int = 0;
            while index < NR_STD_WORKER_POOLS as c_int {
                init_cpu_worker_pool(lupos_wq_cpu_pool(cpu as c_int, index), cpu as c_int, *std_nice.as_ptr().add(index as usize));
                index = index.wrapping_add(1);
            }
            cpu = lupos_wq_next_possible_cpu(cpu);
        }
        let mut index: c_int = 0;
        while index < NR_STD_WORKER_POOLS as c_int {
            let attrs = lupos_wq_alloc_std_attrs_8198();
            lupos_wq_bug_std_attrs_8198(attrs.is_null());
            (*attrs).nice = *std_nice.as_ptr().add(index as usize);
            lupos_wq_set_std_unbound_attrs(index, attrs);
            let attrs = lupos_wq_alloc_ordered_attrs_8206();
            lupos_wq_bug_ordered_attrs_8206(attrs.is_null());
            (*attrs).nice = *std_nice.as_ptr().add(index as usize);
            (*attrs).ordered = true;
            lupos_wq_set_ordered_attrs(index, attrs);
            index = index.wrapping_add(1);
        }
        *lupos_wq_system_wq_slot() = lupos_wq_alloc_system_8212(c"events".as_ptr().cast::<c_char>(), (WQ_PERCPU | __WQ_DEPRECATED) as c_uint, 0);
        *lupos_wq_system_percpu_wq_slot() = lupos_wq_alloc_system_8213(c"events".as_ptr().cast::<c_char>(), WQ_PERCPU as c_uint, 0);
        *lupos_wq_system_highpri_wq_slot() = lupos_wq_alloc_system_8214(c"events_highpri".as_ptr().cast::<c_char>(), (WQ_HIGHPRI | WQ_PERCPU) as c_uint, 0);
        *lupos_wq_system_long_wq_slot() = lupos_wq_alloc_system_8216(c"events_long".as_ptr().cast::<c_char>(), WQ_PERCPU as c_uint, 0);
        *lupos_wq_system_unbound_wq_slot() = lupos_wq_alloc_system_8217(c"events_unbound".as_ptr().cast::<c_char>(), (WQ_UNBOUND | __WQ_DEPRECATED) as c_uint, WQ_MAX_ACTIVE as c_int);
        *lupos_wq_system_dfl_wq_slot() = lupos_wq_alloc_system_8218(c"events_unbound".as_ptr().cast::<c_char>(), WQ_UNBOUND as c_uint, WQ_MAX_ACTIVE as c_int);
        *lupos_wq_system_freezable_wq_slot() = lupos_wq_alloc_system_8219(c"events_freezable".as_ptr().cast::<c_char>(), (WQ_FREEZABLE | WQ_PERCPU) as c_uint, 0);
        *lupos_wq_system_power_efficient_wq_slot() = lupos_wq_alloc_system_8221(c"events_power_efficient".as_ptr().cast::<c_char>(), (WQ_POWER_EFFICIENT | WQ_PERCPU) as c_uint, 0);
        *lupos_wq_system_freezable_power_efficient_wq_slot() = lupos_wq_alloc_system_8223(c"events_freezable_pwr_efficient".as_ptr().cast::<c_char>(), (WQ_FREEZABLE | WQ_POWER_EFFICIENT | WQ_PERCPU) as c_uint, 0);
        *lupos_wq_system_bh_wq_slot() = lupos_wq_alloc_system_8225(c"events_bh".as_ptr().cast::<c_char>(), (WQ_BH | WQ_PERCPU) as c_uint, 0);
        *lupos_wq_system_bh_highpri_wq_slot() = lupos_wq_alloc_system_8226(c"events_bh_highpri".as_ptr().cast::<c_char>(), (WQ_BH | WQ_HIGHPRI | WQ_PERCPU) as c_uint, 0);
        *lupos_wq_system_dfl_long_wq_slot() = lupos_wq_alloc_system_8228(c"events_dfl_long".as_ptr().cast::<c_char>(), WQ_UNBOUND as c_uint, WQ_MAX_ACTIVE as c_int);
        lupos_wq_bug_system_queues_8229(
            (*lupos_wq_system_wq_slot()).is_null()
                || (*lupos_wq_system_percpu_wq_slot()).is_null()
                || (*lupos_wq_system_highpri_wq_slot()).is_null()
                || (*lupos_wq_system_long_wq_slot()).is_null()
                || (*lupos_wq_system_unbound_wq_slot()).is_null()
                || (*lupos_wq_system_freezable_wq_slot()).is_null()
                || (*lupos_wq_system_dfl_wq_slot()).is_null()
                || (*lupos_wq_system_power_efficient_wq_slot()).is_null()
                || (*lupos_wq_system_freezable_power_efficient_wq_slot()).is_null()
                || (*lupos_wq_system_bh_wq_slot()).is_null()
                || (*lupos_wq_system_bh_highpri_wq_slot()).is_null()
                || (*lupos_wq_system_dfl_long_wq_slot()).is_null(),
        );
    }
}

/// Creates the release worker and calibrates the CPU threshold, C8236–8273.
///
/// # Safety
/// Called once at second-stage init after kthread creation becomes available.
pub(super) unsafe fn wq_cpu_intensive_thresh_init() {
    // SAFETY: The release worker must exist even when the user supplied a
    // threshold. Unsigned-long scaling uses native width and wrapping math.
    unsafe {
        let release = lupos_wq_run_release_worker_8241();
        lupos_wq_set_pwq_release_worker(release);
        lupos_wq_bug_release_worker_8242(lupos_wq_is_err(release.cast::<c_void>()));
        if lupos_wq_cpu_intensive_thresh_us() != ULONG_MAX as c_ulong {
            return;
        }
        let mut thresh = (10 as c_ulong).wrapping_mul(USEC_PER_MSEC as c_ulong);
        let lpj = lupos_wq_loops_per_jiffy();
        let bogo = (lpj / 500000).wrapping_mul(lupos_wq_hz()).max(1);
        if bogo < 4000 {
            thresh = (thresh.wrapping_mul(4000) / bogo).min(USEC_PER_SEC as c_ulong);
        }
        lupos_wq_debug_cpu_thresh_8269(lupos_wq_loops_per_jiffy(), bogo, thresh);
        lupos_wq_set_cpu_intensive_thresh_us(thresh);
    }
}

/// Starts initial workers and enables future worker creation, C8284–8335.
///
/// # Safety
/// Native second-stage init runs after early setup and scheduler/kthread ready.
pub(super) unsafe extern "C" fn workqueue_init() {
    // SAFETY: Node hints and early rescuers are fixed under pool mutex. BH
    // pseudo workers cover all possible CPUs; threaded workers only online
    // CPUs. The online flag is published only after all workers are created.
    unsafe {
        wq_cpu_intensive_thresh_init();
        lupos_wq_mutex_lock(lupos_wq_pool_mutex());
        let mut cpu = lupos_wq_first_possible_cpu();
        while cpu < lupos_wq_small_cpumask_bits() {
            let mut index: c_int = 0;
            while index < NR_STD_WORKER_POOLS as c_int {
                (*lupos_wq_bh_pool_at(cpu as c_int, index)).node = lupos_wq_cpu_to_node(cpu as c_int);
                index = index.wrapping_add(1);
            }
            let mut index: c_int = 0;
            while index < NR_STD_WORKER_POOLS as c_int {
                (*lupos_wq_cpu_pool(cpu as c_int, index)).node = lupos_wq_cpu_to_node(cpu as c_int);
                index = index.wrapping_add(1);
            }
            cpu = lupos_wq_next_possible_cpu(cpu);
        }
        let head = lupos_wq_workqueues();
        let mut node = (*head).next;
        while node != head {
            let wq = lupos_wq_wq_from_list(node);
            lupos_wq_warn_early_rescuer_8306(init_rescuer(wq) != 0, wq);
            node = (*node).next;
        }
        lupos_wq_mutex_unlock(lupos_wq_pool_mutex());
        let mut cpu = lupos_wq_first_possible_cpu();
        while cpu < lupos_wq_small_cpumask_bits() {
            let mut index: c_int = 0;
            while index < NR_STD_WORKER_POOLS as c_int {
                lupos_wq_bug_bh_worker_8321(create_worker(lupos_wq_bh_pool_at(cpu as c_int, index)).is_null());
                index = index.wrapping_add(1);
            }
            cpu = lupos_wq_next_possible_cpu(cpu);
        }
        let mut cpu = lupos_wq_first_online_cpu();
        while cpu >= 0 {
            let mut index: c_int = 0;
            while index < NR_STD_WORKER_POOLS as c_int {
                let pool = lupos_wq_cpu_pool(cpu as c_int, index);
                (*pool).flags &= !(POOL_DISASSOCIATED as c_uint);
                lupos_wq_bug_cpu_worker_8326(create_worker(pool).is_null());
                index = index.wrapping_add(1);
            }
            cpu = lupos_wq_next_online_cpu(cpu);
        }
        let mut bucket: c_uint = 0;
        while bucket < (1 as c_uint) << UNBOUND_POOL_HASH_ORDER {
            let mut hnode = (*lupos_wq_unbound_pool_bucket(bucket)).first;
            while !hnode.is_null() {
                let pool = lupos_wq_pool_from_hash_node(hnode);
                lupos_wq_bug_unbound_worker_8331(create_worker(pool).is_null());
                hnode = (*hnode).next;
            }
            bucket = bucket.wrapping_add(1);
        }
        lupos_wq_set_online(true);
        wq_watchdog_init();
    }
}
