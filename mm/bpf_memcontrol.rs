// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Memory Controller-related BPF kfuncs and auxiliary code
 *
 * Author: Roman Gushchin <roman.gushchin@linux.dev>
 */

// Dependencies supplied by the surrounding kernel translation unit.

/// bpf_get_root_mem_cgroup - Returns a pointer to the root memory cgroup.
///
/// The function has KF_ACQUIRE semantics, even though the root memory cgroup
/// is never destroyed after being created and doesn't require reference
/// counting. And it's perfectly safe to pass it to bpf_put_mem_cgroup().
///
/// Return: A pointer to the root memory cgroup.
pub unsafe fn bpf_get_root_mem_cgroup() -> *mut mem_cgroup {
    if mem_cgroup_disabled() {
        return core::ptr::null_mut();
    }

    /* css_get() is not needed */
    root_mem_cgroup
}

/// bpf_get_mem_cgroup - Get a reference to a memory cgroup
/// @css: pointer to the css structure
///
/// It's fine to pass a css which belongs to any cgroup controller,
/// e.g. unified hierarchy's main css.
///
/// Implements KF_ACQUIRE semantics.
///
/// Return: A pointer to a mem_cgroup structure after bumping
/// the corresponding css's reference counter.
pub unsafe fn bpf_get_mem_cgroup(mut css: *mut cgroup_subsys_state) -> *mut mem_cgroup {
    let mut memcg: *mut mem_cgroup = core::ptr::null_mut();
    let mut rcu_unlock = false;

    if mem_cgroup_disabled() || root_mem_cgroup.is_null() {
        return core::ptr::null_mut();
    }

    if (*root_mem_cgroup).css.ss != (*css).ss {
        let cgroup: *mut cgroup = (*css).cgroup;
        let ssid: i32 = (*(*root_mem_cgroup).css.ss).id;

        rcu_read_lock();
        rcu_unlock = true;
        css = rcu_dereference_raw((*cgroup).subsys[ssid as usize]);
    }

    if !css.is_null() && css_tryget(css) {
        memcg = container_of!(css, mem_cgroup, css);
    }

    if rcu_unlock {
        rcu_read_unlock();
    }

    memcg
}

/// bpf_put_mem_cgroup - Put a reference to a memory cgroup
/// @memcg: memory cgroup to release
///
/// Releases a previously acquired memcg reference.
/// Implements KF_RELEASE semantics.
pub unsafe fn bpf_put_mem_cgroup(memcg: *mut mem_cgroup) {
    css_put(&mut (*memcg).css);
}

/// bpf_mem_cgroup_vm_events - Read memory cgroup's vm event counter
/// @memcg: memory cgroup
/// @event: event id
///
/// Allows to read memory cgroup event counters.
///
/// Return: The current value of the corresponding events counter.
pub unsafe fn bpf_mem_cgroup_vm_events(
    memcg: *mut mem_cgroup,
    event: vm_event_item,
) -> c_ulong {
    if !memcg_vm_event_item_valid(event) {
        return !0 as c_ulong;
    }

    memcg_events(memcg, event)
}

/// bpf_mem_cgroup_usage - Read memory cgroup's usage
/// @memcg: memory cgroup
///
/// Please, note that the root memory cgroup it special and is exempt
/// from the memory accounting. The returned value is a sum of sub-cgroup's
/// usages and it not reflecting the size of the root memory cgroup itself.
/// If you need to get an approximation, you can use root level statistics:
/// e.g. NR_FILE_PAGES + NR_ANON_MAPPED.
///
/// Return: The current memory cgroup size in bytes.
pub unsafe fn bpf_mem_cgroup_usage(memcg: *mut mem_cgroup) -> c_ulong {
    page_counter_read(&(*memcg).memory) * PAGE_SIZE
}

/// bpf_mem_cgroup_memory_events - Read memory cgroup's memory event value
/// @memcg: memory cgroup
/// @event: memory event id
///
/// Return: The current value of the memory event counter.
pub unsafe fn bpf_mem_cgroup_memory_events(
    memcg: *mut mem_cgroup,
    event: memcg_memory_event,
) -> c_ulong {
    if event >= MEMCG_NR_MEMORY_EVENTS {
        return !0 as c_ulong;
    }

    atomic_long_read(&(*memcg).memory_events[event as usize])
}

/// bpf_mem_cgroup_page_state - Read memory cgroup's page state counter
/// @memcg: memory cgroup
/// @idx: counter idx
///
/// Allows to read memory cgroup statistics. The output is in bytes.
///
/// Return: The value of the page state counter in bytes.
pub unsafe fn bpf_mem_cgroup_page_state(memcg: *mut mem_cgroup, idx: i32) -> c_ulong {
    if !memcg_stat_item_valid(idx) {
        return !0 as c_ulong;
    }

    memcg_page_state_output(memcg, idx)
}

/// bpf_mem_cgroup_flush_stats - Flush memory cgroup's statistics
/// @memcg: memory cgroup
///
/// Propagate memory cgroup's statistics up the cgroup tree.
pub unsafe fn bpf_mem_cgroup_flush_stats(memcg: *mut mem_cgroup) {
    mem_cgroup_flush_stats(memcg);
}

// BTF_KFUNCS_START(bpf_memcontrol_kfuncs)
// BTF_ID_FLAGS(func, bpf_get_root_mem_cgroup, KF_ACQUIRE | KF_RET_NULL)
// BTF_ID_FLAGS(func, bpf_get_mem_cgroup, KF_ACQUIRE | KF_RET_NULL | KF_RCU)
// BTF_ID_FLAGS(func, bpf_put_mem_cgroup, KF_RELEASE)
// BTF_ID_FLAGS(func, bpf_mem_cgroup_vm_events)
// BTF_ID_FLAGS(func, bpf_mem_cgroup_memory_events)
// BTF_ID_FLAGS(func, bpf_mem_cgroup_usage)
// BTF_ID_FLAGS(func, bpf_mem_cgroup_page_state)
// BTF_ID_FLAGS(func, bpf_mem_cgroup_flush_stats, KF_SLEEPABLE)
// BTF_KFUNCS_END(bpf_memcontrol_kfuncs)

// static const struct btf_kfunc_id_set bpf_memcontrol_kfunc_set = {
//     .owner = THIS_MODULE,
//     .set = &bpf_memcontrol_kfuncs,
// };

unsafe fn bpf_memcontrol_init() -> i32 {
    let err: i32 = register_btf_kfunc_id_set(BPF_PROG_TYPE_UNSPEC, &bpf_memcontrol_kfunc_set);
    if err != 0 {
        pr_warn!("error while registering bpf memcontrol kfuncs: {}", err);
    }
    err
}

// late_initcall(bpf_memcontrol_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
