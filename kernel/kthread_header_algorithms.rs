// SPDX-License-Identifier: GPL-2.0-only
// Header algorithms used by kthread.rs. Authorities are the unchanged
// include/linux/{sched/mm.h,sched/task.h,freezer.h,cpuset.h,cgroup_refcnt.h,
// percpu-refcount.h,string.h} at source checkpoint 6b28b101b790.
// Native helpers retain only the individual primitives and ABI identities;
// this module owns the branches and the order of their invocation.

#[inline]
unsafe fn header_mmdrop(mm: *mut mm_struct) {
    // This exact native decrement supplies the full membarrier barrier.
    if lupos_kthread_mm_count_dec_and_test(mm) {
        // Existing public Rust fork provider, not a companion implementation.
        __mmdrop(mm);
    }
}

#[inline]
unsafe fn header_mmgrab_lazy_tlb(mm: *mut mm_struct) {
    #[cfg(CONFIG_MMU_LAZY_TLB_REFCOUNT)]
    lupos_kthread_mmgrab(mm);
    #[cfg(not(CONFIG_MMU_LAZY_TLB_REFCOUNT))]
    let _ = mm;
}

#[inline]
unsafe fn header_mmdrop_lazy_tlb(mm: *mut mm_struct) {
    #[cfg(CONFIG_MMU_LAZY_TLB_REFCOUNT)]
    header_mmdrop(mm);
    #[cfg(not(CONFIG_MMU_LAZY_TLB_REFCOUNT))]
    {
        let _ = mm;
        // Even without lazy references this call is a full barrier.
        lupos_kthread_mb();
    }
}

#[inline]
unsafe fn header_put_task_struct(task: *mut task_struct) {
    if !lupos_kthread_task_usage_dec_and_test(task) {
        return;
    }
    // sched/task.h queues on both RT and !RT. The native callback leaf
    // selects the existing public Rust __put_task_struct_rcu_cb identity.
    lupos_kthread_task_release_rcu(task);
}

#[inline]
unsafe fn header_freezing(task: *mut task_struct) -> bool {
    #[cfg(CONFIG_FREEZER)]
    if lupos_kthread_freezer_active() {
        // This separately owned freezer provider remains an explicit gap.
        return freezing_slow_path(task);
    }
    #[cfg(not(CONFIG_FREEZER))]
    let _ = task;
    false
}

#[inline]
unsafe fn header_try_to_freeze() -> bool {
    #[cfg(CONFIG_FREEZER)]
    {
        lupos_kthread_might_sleep();
        if !header_freezing(current()) {
            return false;
        }
        if (*current()).flags & PF_NOFREEZE == 0 {
            lupos_kthread_debug_no_locks_held();
        }
        // __refrigerator is separately owned; the !FREEZER stub is never
        // reached here, matching try_to_freeze's no-op disabled definition.
        return lupos_kthread_refrigerator(false);
    }
    #[cfg(not(CONFIG_FREEZER))]
    false
}

#[inline]
unsafe fn header_set_mems_allowed() {
    #[cfg(CONFIG_CPUSETS)]
    {
        // set_mems_allowed(node_states[N_MEMORY]) passes a value: snapshot
        // before task_lock, rather than reading the global inside the lock.
        let nodemask = lupos_kthread_memory_nodes();
        lupos_kthread_task_lock(current());
        let flags = lupos_kthread_irq_save();
        lupos_kthread_mems_seq_begin(current());
        (*current()).mems_allowed = nodemask;
        lupos_kthread_mems_seq_end(current());
        lupos_kthread_irq_restore(flags);
        lupos_kthread_task_unlock(current());
    }
}

#[cfg(CONFIG_BLK_CGROUP)]
#[inline]
unsafe fn header_percpu_ref_get_many(reference: *mut percpu_ref, nr: c_ulong) {
    lupos_kthread_rcu_lock();
    // __ref_is_percpu: test and use ONE READ_ONCE snapshot. Keeping the
    // native value as the per-CPU leaf's argument preserves the dependency
    // on the load paired with __percpu_ref_switch_to_percpu's release store.
    let pointer = lupos_kthread_percpu_ref_read_mode(reference);
    if pointer & LUPOS_KTHREAD_PERCPU_REF_ATOMIC_DEAD as c_ulong == 0 {
        lupos_kthread_percpu_ref_cpu_add(pointer, nr);
    } else {
        lupos_kthread_percpu_ref_atomic_add(reference, nr);
    }
    lupos_kthread_rcu_unlock();
}

#[cfg(CONFIG_BLK_CGROUP)]
#[inline]
unsafe fn header_percpu_ref_put_many(reference: *mut percpu_ref, nr: c_ulong) {
    lupos_kthread_rcu_lock();
    let pointer = lupos_kthread_percpu_ref_read_mode(reference);
    // DEAD can be visible without ATOMIC during kill. Test both flags,
    // rather than masking one flag off a pointer or loading it again.
    if pointer & LUPOS_KTHREAD_PERCPU_REF_ATOMIC_DEAD as c_ulong == 0 {
        lupos_kthread_percpu_ref_cpu_sub(pointer, nr);
    } else if lupos_kthread_percpu_ref_atomic_sub_and_test(reference, nr) {
        // Keep release within the RCU scope. Do not dereference the ref
        // after this native CFI call: the release callback may free it.
        lupos_kthread_percpu_ref_release(reference);
    }
    lupos_kthread_rcu_unlock();
}

#[cfg(CONFIG_BLK_CGROUP)]
#[inline]
unsafe fn header_css_get(css: *mut cgroup_subsys_state) {
    if lupos_kthread_css_flags(css) & LUPOS_KTHREAD_CSS_NO_REF == 0 {
        header_percpu_ref_get_many(lupos_kthread_css_refcount(css), 1);
    }
}

#[cfg(CONFIG_BLK_CGROUP)]
#[inline]
unsafe fn header_css_put(css: *mut cgroup_subsys_state) {
    if lupos_kthread_css_flags(css) & LUPOS_KTHREAD_CSS_NO_REF == 0 {
        header_percpu_ref_put_many(lupos_kthread_css_refcount(css), 1);
    }
}

#[inline]
unsafe fn header_sized_strscpy_pad(dst: *mut c_char, src: *const c_char, count: usize) -> isize {
    // sized_strscpy remains a separately owned string-library provider.
    let wrote = lupos_kthread_strscpy(dst, src, count) as isize;
    if wrote >= 0 && (wrote as usize) < count {
        let wrote = wrote as usize;
        core::ptr::write_bytes(dst.add(wrote + 1), 0, count - wrote - 1);
    }
    wrote
}
