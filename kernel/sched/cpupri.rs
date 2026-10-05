// SPDX-License-Identifier: GPL-2.0-only
//! CPU-priority migration-map algorithms backed by configured native bindings.
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

use kernel::bindings::sched_support_native as b;

use b::{
    cpupri, cpumask, kfree, task_struct,
    lupos_support_pri_atomic_dec as atomic_dec,
    lupos_support_pri_atomic_inc as atomic_inc,
    lupos_support_pri_atomic_read as atomic_read,
    lupos_support_pri_atomic_set as atomic_set,
    lupos_support_pri_mask_and as cpumask_and,
    lupos_support_pri_mask_any_and as cpumask_any_and,
    lupos_support_pri_mask_clear as cpumask_clear_cpu,
    lupos_support_pri_mask_empty as cpumask_empty,
    lupos_support_pri_mask_set as cpumask_set_cpu,
    LUPOS_SUPPORT_CPUPRI_HIGHER as CPUPRI_HIGHER,
    LUPOS_SUPPORT_CPUPRI_INVALID as CPUPRI_INVALID,
    LUPOS_SUPPORT_CPUPRI_NORMAL as CPUPRI_NORMAL,
    LUPOS_SUPPORT_CPUPRI_NR_PRIORITIES as CPUPRI_NR_PRIORITIES,
    LUPOS_SUPPORT_ENOMEM as ENOMEM,
    LUPOS_SUPPORT_MAX_RT_PRIO as MAX_RT_PRIO,
};
use core::ptr::{addr_of, addr_of_mut};
use kernel::ffi::{c_int, c_void};

fn convert_prio(prio: c_int) -> c_int {
    let cpupri: c_int;
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
    idx: c_int,
) -> c_int {
    // SAFETY: The caller supplies live scheduler objects, a valid priority
    // index, and a null or writable destination mask under the native API's
    // lifetime and synchronization requirements. No exclusive borrow is made.
    unsafe {
        let vec = addr_of_mut!((*cp).pri_to_cpu[idx as usize]);
        let mut skip = 0;

        if atomic_read(addr_of!((*vec).count)) == 0 {
            skip = 1;
        }
        b::lupos_support_pri_read_barrier();
        if skip != 0 {
            return 0;
        }
        if cpumask_any_and(addr_of!((*p).cpus_mask), b::lupos_support_pri_mask(vec))
            >= b::lupos_support_pri_nr_cpu_ids()
        {
            return 0;
        }
        if !lowest_mask.is_null() {
            cpumask_and(
                lowest_mask,
                addr_of!((*p).cpus_mask),
                b::lupos_support_pri_mask(vec),
            );
            cpumask_and(lowest_mask, lowest_mask, b::lupos_support_pri_active_mask());
            if cpumask_empty(lowest_mask) {
                return 0;
            }
        }
        1
    }
}

/// Finds CPUs whose current priority is lower than the task's priority.
///
/// # Safety
///
/// `cp` and `p` must be live native scheduler objects. `lowest_mask` must be
/// null or writable for the configured mask size. The native scheduler API's
/// priority-domain, object-lifetime, and synchronization requirements apply.
#[no_mangle]
pub unsafe extern "C" fn cpupri_find(cp: *mut cpupri, p: *mut task_struct, lowest_mask: *mut cpumask) -> c_int {
    // SAFETY: The caller supplies the same objects and mask required by the
    // fitness search, and the absent callback adds no further requirements.
    unsafe { cpupri_find_fitness(cp, p, lowest_mask, None) }
}

/// Finds lower-priority CPUs, preferring those accepted by `fitness_fn`.
///
/// # Safety
///
/// The requirements of [`cpupri_find`] apply. A supplied callback must use the
/// native C ABI and accept `p` and each CPU selected from `lowest_mask`, with
/// the same lifetime and synchronization rules as the native callback API.
#[no_mangle]
pub unsafe extern "C" fn cpupri_find_fitness(
    cp: *mut cpupri,
    p: *mut task_struct,
    lowest_mask: *mut cpumask,
    fitness_fn: Option<unsafe extern "C" fn(*mut task_struct, c_int) -> bool>,
) -> c_int {
    // SAFETY: The caller maintains the native objects, mask, priority domain,
    // and optional callback contract throughout this search and its fallback.
    unsafe {
        let task_pri = convert_prio((*p).prio);
        let mut idx = 0;
        b::lupos_support_pri_warn_find_prio(task_pri);

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
            let mut cpu = b::lupos_support_pri_mask_first(lowest_mask);
            while cpu < b::lupos_support_pri_mask_limit() {
                if !fitness(p, cpu) {
                    cpumask_clear_cpu(cpu, lowest_mask);
                }
                cpu = b::lupos_support_pri_mask_next(cpu, lowest_mask);
            }
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
}

/// Updates a CPU's priority in the migration map.
///
/// # Safety
///
/// `cp` must be initialized and live, `cpu` must index its allocated CPU map,
/// and `newpri` must be in the native priority domain. The caller must hold
/// `cpu_rq(cpu)->lock` as required by the native scheduler API.
#[no_mangle]
pub unsafe extern "C" fn cpupri_set(cp: *mut cpupri, cpu: c_int, mut newpri: c_int) {
    // SAFETY: The caller provides the live initialized CPU map, valid indices,
    // and runqueue lock. Shared vectors use raw pointers and native atomics.
    unsafe {
        let currpri = (*cp).cpu_to_pri.add(cpu as usize);
        let oldpri = *currpri;
        let mut do_mb = 0;
        newpri = convert_prio(newpri);
        b::lupos_support_pri_bug_set_prio(newpri);
        if newpri == oldpri { return; }
        if b::lupos_support_pri_likely_new(newpri) {
            let vec = addr_of_mut!((*cp).pri_to_cpu[newpri as usize]);
            cpumask_set_cpu(cpu, b::lupos_support_pri_mask(vec));
            b::lupos_support_pri_before_publish();
            atomic_inc(addr_of_mut!((*vec).count));
            do_mb = 1;
        }
        if b::lupos_support_pri_likely_old(oldpri) {
            let vec = addr_of_mut!((*cp).pri_to_cpu[oldpri as usize]);
            if do_mb != 0 { b::lupos_support_pri_after_publish(); }
            atomic_dec(addr_of_mut!((*vec).count));
            b::lupos_support_pri_after_remove();
            cpumask_clear_cpu(cpu, b::lupos_support_pri_mask(vec));
        }
        *currpri = newpri;
    }
}

/// Initializes CPU-priority storage, unwinding allocations on failure.
///
/// # Safety
///
/// `cp` must point to writable native storage that is not concurrently used
/// or already initialized. The caller must permit GFP_KERNEL allocation.
#[no_mangle]
pub unsafe extern "C" fn cpupri_init(cp: *mut cpupri) -> c_int {
    // SAFETY: The caller owns writable initialization storage and permits
    // allocation. Unwind releases only the masks successfully allocated here.
    unsafe {
        let mut i: usize = 0;
        while i < CPUPRI_NR_PRIORITIES as usize {
            let vec = addr_of_mut!((*cp).pri_to_cpu[i]);
            atomic_set(addr_of_mut!((*vec).count), 0);
            if !b::lupos_support_pri_alloc_mask(vec) {
                // Only entries before i completed allocation, as in C's i--.
                while i != 0 {
                    i -= 1;
                    b::lupos_support_pri_free_mask(addr_of_mut!((*cp).pri_to_cpu[i]));
                }
                return -ENOMEM;
            }
            i += 1;
        }
        (*cp).cpu_to_pri = b::lupos_support_pri_alloc_priorities();
        if (*cp).cpu_to_pri.is_null() {
            while i != 0 {
                i -= 1;
                b::lupos_support_pri_free_mask(addr_of_mut!((*cp).pri_to_cpu[i]));
            }
            return -ENOMEM;
        }
        let mut cpu = b::lupos_support_pri_possible_first();
        while cpu < b::lupos_support_pri_possible_limit() {
            *(*cp).cpu_to_pri.add(cpu as usize) = CPUPRI_INVALID;
            cpu = b::lupos_support_pri_possible_next(cpu);
        }
        0
    }
}

/// Releases successfully initialized CPU-priority storage.
///
/// # Safety
///
/// `cp` must have completed [`cpupri_init`] successfully, must not have been
/// cleaned up already, and must no longer be accessible to concurrent users.
#[no_mangle]
pub unsafe extern "C" fn cpupri_cleanup(cp: *mut cpupri) {
    // SAFETY: The caller has ended all uses and transfers the initialized
    // allocations for their matching native release operations.
    unsafe {
        kfree((*cp).cpu_to_pri.cast::<c_void>());
        let mut i: usize = 0;
        while i < CPUPRI_NR_PRIORITIES as usize {
            b::lupos_support_pri_free_mask(addr_of_mut!((*cp).pri_to_cpu[i]));
            i += 1;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
