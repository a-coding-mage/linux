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
    let vec = &mut (*cp).pri_to_cpu[idx as usize];
    let mut skip = 0;

    if atomic_read(&vec.count) == 0 {
        skip = 1;
    }
    smp_rmb();
    if skip != 0 {
        return 0;
    }
    if cpumask_any_and(&(*p).cpus_mask, vec.mask) >= nr_cpu_ids {
        return 0;
    }
    if !lowest_mask.is_null() {
        cpumask_and(lowest_mask, &(*p).cpus_mask, vec.mask);
        cpumask_and(lowest_mask, lowest_mask, cpu_active_mask);
        if cpumask_empty(lowest_mask) != 0 {
            return 0;
        }
    }
    1
}

pub unsafe fn cpupri_find(cp: *mut cpupri, p: *mut task_struct, lowest_mask: *mut cpumask) -> i32 {
    cpupri_find_fitness(cp, p, lowest_mask, None)
}

pub unsafe fn cpupri_find_fitness(
    cp: *mut cpupri,
    p: *mut task_struct,
    lowest_mask: *mut cpumask,
    fitness_fn: Option<unsafe extern "C" fn(*mut task_struct, i32) -> bool>,
) -> i32 {
    let task_pri = convert_prio((*p).prio);
    let mut idx = 0;
    WARN_ON_ONCE(task_pri >= CPUPRI_NR_PRIORITIES);

    while idx < task_pri {
        if __cpupri_find(cp, p, lowest_mask, idx) == 0 {
            idx += 1;
            continue;
        }
        if lowest_mask.is_null() || fitness_fn.is_none() {
            return 1;
        }
        for_each_cpu!(cpu, lowest_mask, {
            if !fitness_fn.unwrap()(p, cpu) {
                cpumask_clear_cpu(cpu, lowest_mask);
            }
        });
        if cpumask_empty(lowest_mask) != 0 {
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

pub unsafe fn cpupri_set(cp: *mut cpupri, cpu: i32, mut newpri: i32) {
    let currpri = &mut (*cp).cpu_to_pri[cpu as usize];
    let oldpri = *currpri;
    let mut do_mb = 0;
    newpri = convert_prio(newpri);
    BUG_ON(newpri >= CPUPRI_NR_PRIORITIES);
    if newpri == oldpri { return; }
    if newpri != CPUPRI_INVALID {
        let vec = &mut (*cp).pri_to_cpu[newpri as usize];
        cpumask_set_cpu(cpu, vec.mask);
        smp_mb__before_atomic();
        atomic_inc(&vec.count);
        do_mb = 1;
    }
    if oldpri != CPUPRI_INVALID {
        let vec = &mut (*cp).pri_to_cpu[oldpri as usize];
        if do_mb != 0 { smp_mb__after_atomic(); }
        atomic_dec(&vec.count);
        smp_mb__after_atomic();
        cpumask_clear_cpu(cpu, vec.mask);
    }
    *currpri = newpri;
}

pub unsafe fn cpupri_init(cp: *mut cpupri) -> i32 {
    let mut i = 0;
    while i < CPUPRI_NR_PRIORITIES {
        let vec = &mut (*cp).pri_to_cpu[i as usize];
        atomic_set(&mut vec.count, 0);
        if !zalloc_cpumask_var(&mut vec.mask, GFP_KERNEL) {
            while i >= 0 {
                free_cpumask_var((*cp).pri_to_cpu[i as usize].mask);
                i -= 1;
            }
            return -ENOMEM;
        }
        i += 1;
    }
    (*cp).cpu_to_pri = kzalloc_objs::<i32>(nr_cpu_ids);
    if (*cp).cpu_to_pri.is_null() {
        while i >= 0 {
            free_cpumask_var((*cp).pri_to_cpu[i as usize].mask);
            i -= 1;
        }
        return -ENOMEM;
    }
    for_each_possible_cpu!(i, { (*cp).cpu_to_pri[i as usize] = CPUPRI_INVALID; });
    0
}

pub unsafe fn cpupri_cleanup(cp: *mut cpupri) {
    kfree((*cp).cpu_to_pri);
    let mut i = 0;
    while i < CPUPRI_NR_PRIORITIES {
        free_cpumask_var((*cp).pri_to_cpu[i as usize].mask);
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
