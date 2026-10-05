// SPDX-License-Identifier: GPL-2.0-only
/*
 *  kernel/sched/cpupri.c
 *
 *  CPU priority management
 *
 *  Copyright (C) 2007-2008 Novell
 *
 *  Author: Gregory Haskins <ghaskins@novell.com>
 *
 *  This code tracks the priority of each CPU so that global migration
 *  decisions are easy to calculate. Each CPU can be in a state as follows:
 *
 *                 (INVALID), NORMAL, RT1, ... RT99, HIGHER
 *
 *  going from the lowest priority to the highest. CPUs in the INVALID state
 *  are not eligible for routing. The system maintains this state with
 *  a 2 dimensional bitmap (the first for priority class, the second for CPUs
 *  in that class).
 */

use core::ptr::{
    addr_of,
    addr_of_mut, //
};
use kernel::ffi::c_void;

unsafe fn convert_prio(prio: i32) -> i32 {
    let cpupri: i32;
    match prio {
        CPUPRI_INVALID => cpupri = CPUPRI_INVALID,
        0..=98 => cpupri = MAX_RT_PRIO - 1 - prio,
        x if x == MAX_RT_PRIO - 1 => cpupri = CPUPRI_NORMAL,
        x if x == MAX_RT_PRIO => cpupri = CPUPRI_HIGHER,
        _ => cpupri = 0,
    }
    cpupri
}

unsafe fn __cpupri_find(
    cp: *mut cpupri,
    p: *mut task_struct,
    lowest_mask: *mut cpumask,
    idx: i32,
) -> i32 {
    // The vector is shared with lockless readers and concurrent updaters.
    let vec = addr_of_mut!((*cp).pri_to_cpu[idx as usize]);
    let mut skip = 0;

    if atomic_read(addr_of!((*vec).count)) == 0 {
        skip = 1;
    }
    smp_rmb();
    if skip != 0 {
        return 0;
    }
    if cpumask_any_and(addr_of!((*p).cpus_mask), (*vec).mask) >= nr_cpu_ids {
        return 0;
    }
    if !lowest_mask.is_null() {
        cpumask_and(lowest_mask, addr_of!((*p).cpus_mask), (*vec).mask);
        cpumask_and(lowest_mask, lowest_mask, cpu_active_mask);
        if cpumask_empty(lowest_mask) {
            return 0;
        }
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn cpupri_find(cp: *mut cpupri, p: *mut task_struct, lowest_mask: *mut cpumask) -> i32 {
    cpupri_find_fitness(cp, p, lowest_mask, None)
}

#[no_mangle]
pub unsafe extern "C" fn cpupri_find_fitness(
    cp: *mut cpupri,
    p: *mut task_struct,
    lowest_mask: *mut cpumask,
    fitness_fn: Option<unsafe extern "C" fn(*mut task_struct, i32) -> bool>,
) -> i32 {
    let task_pri = convert_prio((*p).prio);
    let mut idx = 0;
    WARN_ON_ONCE(task_pri >= CPUPRI_NR_PRIORITIES as i32);

    while idx < task_pri {
        if __cpupri_find(cp, p, lowest_mask, idx) == 0 {
            idx += 1;
            continue;
        }
        if lowest_mask.is_null() {
            return 1;
        }
        let fitness = match fitness_fn {
            Some(fitness) => fitness,
            None => return 1,
        };
        for_each_cpu!(cpu, lowest_mask, {
            if !fitness(p, cpu) {
                cpumask_clear_cpu(cpu, lowest_mask);
            }
        });
        if cpumask_empty(lowest_mask) {
            idx += 1;
            continue;
        }
        return 1;
    }
    if fitness_fn.is_some() {
        return cpupri_find(cp, p, lowest_mask);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn cpupri_set(cp: *mut cpupri, cpu: i32, mut newpri: i32) {
    let currpri = (*cp).cpu_to_pri.add(cpu as usize);
    let oldpri = *currpri;
    let mut do_mb = 0;
    newpri = convert_prio(newpri);
    BUG_ON(newpri >= CPUPRI_NR_PRIORITIES as i32);
    if newpri == oldpri { return; }
    if newpri != CPUPRI_INVALID {
        let vec = addr_of_mut!((*cp).pri_to_cpu[newpri as usize]);
        cpumask_set_cpu(cpu, (*vec).mask);
        smp_mb__before_atomic();
        atomic_inc(addr_of_mut!((*vec).count));
        do_mb = 1;
    }
    if oldpri != CPUPRI_INVALID {
        let vec = addr_of_mut!((*cp).pri_to_cpu[oldpri as usize]);
        if do_mb != 0 { smp_mb__after_atomic(); }
        atomic_dec(addr_of_mut!((*vec).count));
        smp_mb__after_atomic();
        cpumask_clear_cpu(cpu, (*vec).mask);
    }
    *currpri = newpri;
}

#[no_mangle]
pub unsafe extern "C" fn cpupri_init(cp: *mut cpupri) -> i32 {
    let mut i: usize = 0;
    while i < CPUPRI_NR_PRIORITIES as usize {
        let vec = addr_of_mut!((*cp).pri_to_cpu[i]);
        atomic_set(addr_of_mut!((*vec).count), 0);
        if !zalloc_cpumask_var(addr_of_mut!((*vec).mask), GFP_KERNEL) {
            // Only entries before i completed allocation, as in C's i--.
            while i != 0 {
                i -= 1;
                free_cpumask_var((*cp).pri_to_cpu[i].mask);
            }
            return -ENOMEM;
        }
        i += 1;
    }
    (*cp).cpu_to_pri = kzalloc_objs::<i32>(nr_cpu_ids);
    if (*cp).cpu_to_pri.is_null() {
        while i != 0 {
            i -= 1;
            free_cpumask_var((*cp).pri_to_cpu[i].mask);
        }
        return -ENOMEM;
    }
    for_each_possible_cpu!(i, { *(*cp).cpu_to_pri.add(i as usize) = CPUPRI_INVALID; });
    0
}

#[no_mangle]
pub unsafe extern "C" fn cpupri_cleanup(cp: *mut cpupri) {
    kfree((*cp).cpu_to_pri.cast::<c_void>());
    let mut i: usize = 0;
    while i < CPUPRI_NR_PRIORITIES as usize {
        free_cpumask_var((*cp).pri_to_cpu[i].mask);
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
