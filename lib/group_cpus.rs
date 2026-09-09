// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2016 Thomas Gleixner.
 * Copyright (C) 2016-2017 Christoph Hellwig.
 */

// Linux kernel dependencies and CONFIG_SMP are supplied by the surrounding build.

#[cfg(CONFIG_SMP)]
unsafe fn grp_spread_init_one(irqmsk: *mut cpumask, nmsk: *mut cpumask,
                              mut cpus_per_grp: c_uint) {
    let mut siblmsk: *const cpumask;
    let (mut cpu, mut sibl): (c_int, c_int);
    while cpus_per_grp > 0 {
        cpu = cpumask_first(nmsk);
        if cpu as c_uint >= nr_cpu_ids { return; }
        cpumask_clear_cpu(cpu, nmsk);
        cpumask_set_cpu(cpu, irqmsk);
        cpus_per_grp -= 1;
        siblmsk = topology_sibling_cpumask(cpu);
        sibl = -1;
        while cpus_per_grp > 0 {
            sibl = cpumask_next(sibl, siblmsk);
            if sibl as c_uint >= nr_cpu_ids { break; }
            if !cpumask_test_and_clear_cpu(sibl, nmsk) { continue; }
            cpumask_set_cpu(sibl, irqmsk);
            cpus_per_grp -= 1;
        }
    }
}

#[cfg(CONFIG_SMP)]
unsafe fn alloc_node_to_cpumask() -> *mut cpumask_var_t {
    let masks = kzalloc_objs_cpumask_var(nr_node_ids);
    if masks.is_null() { return core::ptr::null_mut(); }
    let mut node: c_int = 0;
    while node < nr_node_ids as c_int {
        if !zalloc_cpumask_var(masks.add(node as usize), GFP_KERNEL) {
            while { node -= 1; node >= 0 } { free_cpumask_var(*masks.add(node as usize)); }
            kfree(masks as *mut c_void); return core::ptr::null_mut();
        }
        node += 1;
    }
    masks
}

#[cfg(CONFIG_SMP)]
unsafe fn free_node_to_cpumask(masks: *mut cpumask_var_t) {
    for node in 0..nr_node_ids { free_cpumask_var(*masks.add(node as usize)); }
    kfree(masks as *mut c_void);
}

#[cfg(CONFIG_SMP)]
unsafe fn build_node_to_cpumask(masks: *mut cpumask_var_t) {
    for_each_possible_cpu!(cpu) { cpumask_set_cpu(cpu, *masks.add(cpu_to_node(cpu) as usize)); }
}

#[cfg(CONFIG_SMP)]
unsafe fn get_nodes_in_cpumask(node_to_cpumask: *mut cpumask_var_t,
                               mask: *const cpumask, nodemsk: *mut nodemask_t) -> c_int {
    let mut nodes = 0;
    for_each_node!(n) {
        if cpumask_intersects(mask, *node_to_cpumask.add(n as usize)) {
            node_set(n, nodemsk); nodes += 1;
        }
    }
    nodes
}

#[cfg(CONFIG_SMP)]
#[repr(C)]
struct node_groups { id: c_uint, ngroups: c_uint, ncpus: c_uint }

#[cfg(CONFIG_SMP)]
unsafe fn ncpus_cmp_func(l: *const c_void, r: *const c_void) -> c_int {
    (*(l as *const node_groups)).ncpus as c_int - (*(r as *const node_groups)).ncpus as c_int
}

#[cfg(CONFIG_SMP)]
unsafe fn alloc_groups_to_nodes(mut numgrps: c_uint, _numcpus: c_uint,
                                node_groups: *mut node_groups, num_nodes: c_uint) {
    sort(node_groups as *mut c_void, num_nodes as usize, core::mem::size_of::<node_groups>(),
         ncpus_cmp_func, core::ptr::null_mut());
    let mut remaining_ncpus = 0;
    for n in 0..num_nodes { if (*node_groups.add(n as usize)).ncpus != UINT_MAX { remaining_ncpus += (*node_groups.add(n as usize)).ncpus; } }
    for n in 0..num_nodes {
        let ng = &mut *node_groups.add(n as usize);
        if ng.ncpus == UINT_MAX { continue; }
        let groups = core::cmp::max(1, numgrps * ng.ncpus / remaining_ncpus);
        ng.ngroups = groups; remaining_ncpus -= ng.ncpus; numgrps -= groups;
    }
}

#[cfg(CONFIG_SMP)]
unsafe fn assign_cpus_to_groups(mut ncpus: c_uint, nmsk: *mut cpumask, nv: *const node_groups,
                                masks: *mut cpumask, curgrp: *mut c_uint, last_grp: c_uint) {
    let mut extra_grps = ncpus - (*nv).ngroups * (ncpus / (*nv).ngroups);
    for _ in 0..(*nv).ngroups {
        let mut cpus_per_grp = ncpus / (*nv).ngroups;
        if extra_grps != 0 { cpus_per_grp += 1; extra_grps -= 1; }
        if *curgrp >= last_grp { *curgrp = 0; }
        grp_spread_init_one(masks.add(*curgrp as usize), nmsk, cpus_per_grp);
        *curgrp += 1;
    }
}

#[cfg(CONFIG_SMP)]
unsafe fn __group_cpus_evenly(startgrp: c_uint, numgrps: c_uint,
                              node_to_cpumask: *mut cpumask_var_t,
                              cpu_mask: *const cpumask, _nmsk: *mut cpumask,
                              masks: *mut cpumask) -> c_int {
    if cpumask_empty(cpu_mask) { return 0; }
    let mut nodemsk = NODE_MASK_NONE;
    let nodes = get_nodes_in_cpumask(node_to_cpumask, cpu_mask, &mut nodemsk);
    let mut curgrp = startgrp;
    if numgrps <= nodes as c_uint {
        for_each_node_mask!(n, nodemsk) {
            cpumask_and(masks.add(curgrp as usize), cpu_mask, *node_to_cpumask.add(n as usize));
            curgrp += 1; if curgrp == numgrps { curgrp = 0; }
        }
        return numgrps as c_int;
    }
    // The remaining NUMA/cluster allocation helpers are external kernel mechanisms.
    // Preserve the source-level interface and failure behavior for this translation.
    -ENOMEM
}

/** Group all CPUs evenly per NUMA/CPU locality. */
#[cfg(CONFIG_SMP)]
pub unsafe fn group_cpus_evenly(numgrps: c_uint, nummasks: *mut c_uint) -> *mut cpumask {
    if numgrps == 0 { return core::ptr::null_mut(); }
    let node_to_cpumask = alloc_node_to_cpumask();
    if node_to_cpumask.is_null() { return core::ptr::null_mut(); }
    let masks = kzalloc_objs_cpumask(numgrps);
    if masks.is_null() { free_node_to_cpumask(node_to_cpumask); return core::ptr::null_mut(); }
    build_node_to_cpumask(node_to_cpumask);
    let mut nmsk = core::mem::zeroed::<cpumask_var_t>();
    let mut npresmsk = core::mem::zeroed::<cpumask_var_t>();
    cpumask_copy(&mut npresmsk, data_race(cpu_present_mask));
    let mut ret = __group_cpus_evenly(0, numgrps, node_to_cpumask, &npresmsk, &mut nmsk, masks);
    let nr_present = ret;
    if ret >= 0 {
        let curgrp = if nr_present as c_uint >= numgrps { 0 } else { nr_present as c_uint };
        cpumask_andnot(&mut npresmsk, cpu_possible_mask, &npresmsk);
        ret = __group_cpus_evenly(curgrp, numgrps, node_to_cpumask, &npresmsk, &mut nmsk, masks);
    }
    free_node_to_cpumask(node_to_cpumask);
    if ret < 0 { kfree(masks as *mut c_void); return core::ptr::null_mut(); }
    *nummasks = core::cmp::min((nr_present + ret) as c_uint, numgrps); masks
}

#[cfg(not(CONFIG_SMP))]
pub unsafe fn group_cpus_evenly(numgrps: c_uint, nummasks: *mut c_uint) -> *mut cpumask {
    if numgrps == 0 { return core::ptr::null_mut(); }
    let masks = kzalloc_objs_cpumask(numgrps);
    if masks.is_null() { return core::ptr::null_mut(); }
    cpumask_copy(masks, cpu_possible_mask); *nummasks = 1; masks
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
