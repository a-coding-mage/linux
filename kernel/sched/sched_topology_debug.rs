// SPDX-License-Identifier: GPL-2.0
// Continued from topology.rs; oracle topology.c:11–209 at 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
// Native metadata expands sd_flags.h. Rust retains diagnostic walks and decisions.

/// Acquires the native scheduler-domain mutex.
///
/// # Safety
/// The caller must be in sleepable context and obey the scheduler lock order.
#[no_mangle]
pub unsafe extern "C" fn sched_domains_mutex_lock() {
    // SAFETY: The caller supplies the native mutex's execution-context contract.
    unsafe { lupos_topology_domains_lock(); }
}

/// Releases the native scheduler-domain mutex.
///
/// # Safety
/// The current task must hold the mutex acquired by `sched_domains_mutex_lock`.
#[no_mangle]
pub unsafe extern "C" fn sched_domains_mutex_unlock() {
    // SAFETY: The caller owns the native mutex being released.
    unsafe { lupos_topology_domains_unlock(); }
}

/// Enables verbose topology diagnostics from the early boot parameter.
///
/// # Safety
/// Invoke only through the early-parameter path before init memory is discarded.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_topology_debug_setup(_str: *mut c_char) -> c_int {
    // SAFETY: Early boot serializes this write to the native diagnostic flag.
    unsafe { sched_debug_verbose = true; }
    0
}

unsafe fn sched_debug() -> bool {
    // SAFETY: Scheduler callers use the initialized native diagnostic flag.
    unsafe { sched_debug_verbose }
}

unsafe fn sched_domain_debug_one(sd: *mut sched_domain, cpu: c_int, level: c_int,
                                 groupmask: *mut cpumask) -> c_int {
    // SAFETY: The caller holds the domain mutex, owns the scratch mask, and keeps
    // the domain hierarchy and circular group/capacity objects live for the walk.
    unsafe {
        let mut group = (*sd).groups;
        let flags = (*sd).flags as c_ulong;
        lupos_topology_mask_clear(groupmask);
        lupos_topology_debug_domain(sd, level);
        if !lupos_topology_mask_test(cpu, lupos_topology_domain_span(sd)) {
            lupos_topology_debug_missing_cpu(cpu);
        }
        if !group.is_null() && !lupos_topology_mask_test(cpu, lupos_topology_group_span(group)) {
            lupos_topology_debug_missing_group_cpu(cpu);
        }
        for idx in 0..LUPOS_TOPOLOGY_SD_FLAG_COUNT {
            let flag = (1 as c_ulong) << idx;
            if flags & flag == 0 { continue; }
            let meta_flags = lupos_topology_flag_meta(idx);
            if meta_flags & LUPOS_TOPOLOGY_SDF_SHARED_CHILD != 0 && !(*sd).child.is_null()
                && ((*(*sd).child).flags as c_ulong & flag) == 0 {
                lupos_topology_debug_flag_child(idx);
            }
            if meta_flags & LUPOS_TOPOLOGY_SDF_SHARED_PARENT != 0 && !(*sd).parent.is_null()
                && ((*(*sd).parent).flags as c_ulong & flag) == 0 {
                lupos_topology_debug_flag_parent(idx);
            }
        }
        lupos_topology_debug_groups(level);
        loop {
            if group.is_null() {
                lupos_topology_debug_null_group();
                break;
            }
            if lupos_topology_mask_empty(lupos_topology_group_span(group)) {
                lupos_topology_debug_empty_group();
                break;
            }
            if (*sd).flags & SD_NUMA == 0
                && lupos_topology_mask_intersects(groupmask, lupos_topology_group_span(group)) {
                lupos_topology_debug_repeated_cpu();
                break;
            }
            lupos_topology_mask_or(groupmask, groupmask, lupos_topology_group_span(group));
            lupos_topology_debug_group(group);
            if (*sd).flags & SD_NUMA != 0
                && !lupos_topology_mask_equal(lupos_topology_balance_mask(group), lupos_topology_group_span(group)) {
                lupos_topology_debug_balance_mask(group);
            }
            if (*(*group).sgc).capacity != LUPOS_TOPOLOGY_CAPACITY_SCALE as c_ulong {
                lupos_topology_debug_capacity((*(*group).sgc).capacity);
            }
            if group == (*sd).groups && !(*sd).child.is_null()
                && !lupos_topology_mask_equal(lupos_topology_domain_span((*sd).child), lupos_topology_group_span(group)) {
                lupos_topology_debug_group_child_mismatch();
            }
            lupos_topology_debug_group_end();
            group = (*group).next;
            if group == (*sd).groups { break; }
            lupos_topology_debug_group_separator();
        }
        lupos_topology_debug_groups_end();
        if !lupos_topology_mask_equal(lupos_topology_domain_span(sd), groupmask) {
            lupos_topology_debug_group_span_mismatch();
        }
        if !(*sd).parent.is_null()
            && !lupos_topology_mask_subset(groupmask, lupos_topology_domain_span((*sd).parent)) {
            lupos_topology_debug_parent_span_mismatch();
        }
        0
    }
}

unsafe fn sched_domain_debug(mut sd: *mut sched_domain, cpu: c_int) {
    // SAFETY: The attach path holds the domain mutex and keeps the unpublished
    // hierarchy live; the same mutex serializes the shared native scratch mask.
    unsafe {
        let mut level = 0;
        if !sched_debug_verbose { return; }
        if sd.is_null() {
            lupos_topology_debug_attach_null(cpu);
            return;
        }
        lupos_topology_debug_attach(cpu);
        loop {
            if sched_domain_debug_one(sd, cpu, level, lupos_topology_tmpmask()) != 0 { break; }
            level += 1;
            sd = (*sd).parent;
            if sd.is_null() { break; }
        }
    }
}

unsafe fn sd_degenerate(sd: *mut sched_domain) -> bool {
    // SAFETY: The construction path supplies a live domain, its span allocation,
    // and its initialized circular group list while topology is serialized.
    unsafe {
        if lupos_topology_mask_weight(lupos_topology_domain_span(sd)) == 1 { return true; }
        if ((*sd).flags as c_uint) & lupos_topology_degenerate_groups_mask() != 0
            && (*sd).groups != (*(*sd).groups).next { return false; }
        if (*sd).flags & SD_WAKE_AFFINE != 0 { return false; }
        true
    }
}

unsafe fn sd_parent_degenerate(sd: *mut sched_domain, parent: *mut sched_domain) -> bool {
    // SAFETY: Both domains and their spans/groups remain live in the serialized
    // construction path; neither hierarchy is concurrently mutated here.
    unsafe {
        let cflags = (*sd).flags as c_ulong;
        let mut pflags = (*parent).flags as c_ulong;
        if sd_degenerate(parent) { return true; }
        if !lupos_topology_mask_equal(lupos_topology_domain_span(sd), lupos_topology_domain_span(parent)) {
            return false;
        }
        if (*parent).groups == (*(*parent).groups).next {
            // Match C: complement the unsigned-int mask before widening to unsigned long.
            pflags &= (!lupos_topology_degenerate_groups_mask()) as c_ulong;
        }
        if !cflags & pflags != 0 { return false; }
        true
    }
}
