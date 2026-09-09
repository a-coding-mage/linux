// SPDX-License-Identifier: GPL-2.0
/*
 * Lockless hierarchical page accounting & limiting
 *
 * Copyright (C) 2014 Red Hat, Inc., Johannes Weiner
 */

unsafe fn track_protection(c: *mut page_counter) -> bool {
    (*c).protection_support
}

unsafe fn propagate_protected_usage(c: *mut page_counter, usage: c_ulong) {
    if (*c).parent.is_null() {
        return;
    }

    let mut protected = core::cmp::min(usage, core::ptr::read_volatile(&(*c).min));
    let mut old_protected = atomic_long_read(&(*c).min_usage);
    if protected != old_protected {
        old_protected = atomic_long_xchg(&(*c).min_usage, protected);
        let delta = protected as c_long - old_protected as c_long;
        if delta != 0 {
            atomic_long_add(delta, &(*(*c).parent).children_min_usage);
        }
    }

    protected = core::cmp::min(usage, core::ptr::read_volatile(&(*c).low));
    old_protected = atomic_long_read(&(*c).low_usage);
    if protected != old_protected {
        old_protected = atomic_long_xchg(&(*c).low_usage, protected);
        let delta = protected as c_long - old_protected as c_long;
        if delta != 0 {
            atomic_long_add(delta, &(*(*c).parent).children_low_usage);
        }
    }
}

/// page_counter_cancel - take pages out of the local counter
pub unsafe fn page_counter_cancel(counter: *mut page_counter, nr_pages: c_ulong) {
    let mut new = atomic_long_sub_return(nr_pages, &(*counter).usage);
    if new < 0 {
        new = 0;
        atomic_long_set(&(*counter).usage, new);
    }
    if track_protection(counter) {
        propagate_protected_usage(counter, new as c_ulong);
    }
}

/// page_counter_charge - hierarchically charge pages
pub unsafe fn page_counter_charge(counter: *mut page_counter, nr_pages: c_ulong) {
    let protection = track_protection(counter);
    let mut c = counter;
    while !c.is_null() {
        let new = atomic_long_add_return(nr_pages, &(*c).usage);
        if protection {
            propagate_protected_usage(c, new as c_ulong);
        }
        if new > core::ptr::read_volatile(&(*c).local_watermark) {
            core::ptr::write_volatile(&mut (*c).local_watermark, new as c_ulong);
            if new > core::ptr::read_volatile(&(*c).watermark) {
                core::ptr::write_volatile(&mut (*c).watermark, new as c_ulong);
            }
        }
        c = (*c).parent;
    }
}

/// page_counter_try_charge - try to hierarchically charge pages
pub unsafe fn page_counter_try_charge(
    counter: *mut page_counter,
    nr_pages: c_ulong,
    fail: *mut *mut page_counter,
) -> bool {
    let protection = track_protection(counter);
    let track_failcnt = (*counter).track_failcnt;
    let mut c = counter;
    while !c.is_null() {
        let new = atomic_long_add_return(nr_pages, &(*c).usage);
        if new as c_ulong > (*c).max {
            atomic_long_sub(nr_pages, &(*c).usage);
            if track_failcnt {
                (*c).failcnt += 1;
            }
            *fail = c;
            let mut rollback = counter;
            while rollback != *fail {
                page_counter_cancel(rollback, nr_pages);
                rollback = (*rollback).parent;
            }
            return false;
        }
        if protection {
            propagate_protected_usage(c, new as c_ulong);
        }
        if new > core::ptr::read_volatile(&(*c).local_watermark) {
            core::ptr::write_volatile(&mut (*c).local_watermark, new as c_ulong);
            if new > core::ptr::read_volatile(&(*c).watermark) {
                core::ptr::write_volatile(&mut (*c).watermark, new as c_ulong);
            }
        }
        c = (*c).parent;
    }
    true
}

/// page_counter_uncharge - hierarchically uncharge pages
pub unsafe fn page_counter_uncharge(counter: *mut page_counter, nr_pages: c_ulong) {
    let mut c = counter;
    while !c.is_null() {
        page_counter_cancel(c, nr_pages);
        c = (*c).parent;
    }
}

/// page_counter_set_max - set the maximum number of pages allowed
pub unsafe fn page_counter_set_max(counter: *mut page_counter, nr_pages: c_ulong) -> c_int {
    loop {
        let usage = page_counter_read(counter);
        if usage > nr_pages as c_long {
            return -EBUSY;
        }
        let old = core::mem::replace(&mut (*counter).max, nr_pages);
        if page_counter_read(counter) <= usage || nr_pages >= old {
            return 0;
        }
        (*counter).max = old;
        cond_resched();
    }
}

/// page_counter_set_min - set the amount of protected memory
pub unsafe fn page_counter_set_min(counter: *mut page_counter, nr_pages: c_ulong) {
    core::ptr::write_volatile(&mut (*counter).min, nr_pages);
    let mut c = counter;
    while !c.is_null() {
        propagate_protected_usage(c, atomic_long_read(&(*c).usage) as c_ulong);
        c = (*c).parent;
    }
}

/// page_counter_set_low - set the amount of protected memory
pub unsafe fn page_counter_set_low(counter: *mut page_counter, nr_pages: c_ulong) {
    core::ptr::write_volatile(&mut (*counter).low, nr_pages);
    let mut c = counter;
    while !c.is_null() {
        propagate_protected_usage(c, atomic_long_read(&(*c).usage) as c_ulong);
        c = (*c).parent;
    }
}

/// page_counter_memparse - memparse() for page counter limits
pub unsafe fn page_counter_memparse(
    buf: *const c_char,
    max: *const c_char,
    nr_pages: *mut c_ulong,
) -> c_int {
    if strcmp(buf, max) == 0 {
        *nr_pages = PAGE_COUNTER_MAX;
        return 0;
    }
    let mut end: *mut c_char = core::ptr::null_mut();
    let bytes = memparse(buf, &mut end);
    if *end != 0 {
        return -EINVAL;
    }
    *nr_pages = core::cmp::min(bytes / PAGE_SIZE, PAGE_COUNTER_MAX as u64) as c_ulong;
    0
}

// Preserved from the source: compiled when CONFIG_MEMCG or CONFIG_CGROUP_DMEM is enabled.
#[cfg(any(feature = "CONFIG_MEMCG", feature = "CONFIG_CGROUP_DMEM"))]
unsafe fn effective_protection(
    usage: c_ulong,
    parent_usage: c_ulong,
    setting: c_ulong,
    parent_effective: c_ulong,
    siblings_protected: c_ulong,
    recursive_protection: bool,
) -> c_ulong {
    let protected = core::cmp::min(usage, setting);
    if siblings_protected > parent_effective {
        return protected * parent_effective / siblings_protected;
    }
    let mut ep = protected;
    if !recursive_protection {
        return ep;
    }
    if parent_effective > siblings_protected
        && parent_usage > siblings_protected
        && usage > protected
    {
        let mut unclaimed = parent_effective - siblings_protected;
        unclaimed *= usage - protected;
        unclaimed /= parent_usage - siblings_protected;
        ep += unclaimed;
    }
    ep
}

#[cfg(any(feature = "CONFIG_MEMCG", feature = "CONFIG_CGROUP_DMEM"))]
pub unsafe fn page_counter_calculate_protection(
    root: *mut page_counter,
    counter: *mut page_counter,
    recursive_protection: bool,
) {
    let parent = (*counter).parent;
    if root == counter {
        return;
    }
    let usage = page_counter_read(counter) as c_ulong;
    if usage == 0 {
        return;
    }
    if parent == root {
        (*counter).emin = core::ptr::read_volatile(&(*counter).min);
        (*counter).elow = core::ptr::read_volatile(&(*counter).low);
        return;
    }
    let parent_usage = page_counter_read(parent) as c_ulong;
    core::ptr::write_volatile(&mut (*counter).emin, effective_protection(
        usage, parent_usage, core::ptr::read_volatile(&(*counter).min),
        core::ptr::read_volatile(&(*parent).emin), atomic_long_read(&(*parent).children_min_usage) as c_ulong,
        recursive_protection));
    core::ptr::write_volatile(&mut (*counter).elow, effective_protection(
        usage, parent_usage, core::ptr::read_volatile(&(*counter).low),
        core::ptr::read_volatile(&(*parent).elow), atomic_long_read(&(*parent).children_low_usage) as c_ulong,
        recursive_protection));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
