// SPDX-License-Identifier: GPL-2.0-only
/*
 *  kernel/sched/cpudeadline.c
 *
 *  Global CPU deadline management
 *
 *  Author: Juri Lelli <j.lelli@sssup.it>
 */

#[inline]
unsafe fn parent(i: i32) -> i32 {
    (i - 1) >> 1
}

#[inline]
unsafe fn left_child(i: i32) -> i32 {
    (i << 1) + 1
}

#[inline]
unsafe fn right_child(i: i32) -> i32 {
    (i << 1) + 2
}

unsafe fn cpudl_heapify_down(cp: *mut cpudl, mut idx: i32) {
    let mut l: i32;
    let mut r: i32;
    let mut largest: i32;

    let orig_cpu = (*(*cp).elements.add(idx as usize)).cpu;
    let orig_dl = (*(*cp).elements.add(idx as usize)).dl;

    if left_child(idx) >= (*cp).size {
        return;
    }

    /* adapted from lib/prio_heap.c */
    loop {
        let mut largest_dl: u64;

        l = left_child(idx);
        r = right_child(idx);
        largest = idx;
        largest_dl = orig_dl;

        if l < (*cp).size && dl_time_before(orig_dl, (*(*cp).elements.add(l as usize)).dl) {
            largest = l;
            largest_dl = (*(*cp).elements.add(l as usize)).dl;
        }
        if r < (*cp).size && dl_time_before(largest_dl, (*(*cp).elements.add(r as usize)).dl) {
            largest = r;
        }

        if largest == idx {
            break;
        }

        /* pull largest child onto idx */
        (*(*cp).elements.add(idx as usize)).cpu = (*(*cp).elements.add(largest as usize)).cpu;
        (*(*cp).elements.add(idx as usize)).dl = (*(*cp).elements.add(largest as usize)).dl;
        let cpu = (*(*cp).elements.add(idx as usize)).cpu;
        (*(*cp).elements.add(cpu as usize)).idx = idx;
        idx = largest;
    }
    /* actual push down of saved original values orig_* */
    (*(*cp).elements.add(idx as usize)).cpu = orig_cpu;
    (*(*cp).elements.add(idx as usize)).dl = orig_dl;
    (*(*cp).elements.add(orig_cpu as usize)).idx = idx;
}

unsafe fn cpudl_heapify_up(cp: *mut cpudl, mut idx: i32) {
    let mut p: i32;
    let orig_cpu = (*(*cp).elements.add(idx as usize)).cpu;
    let orig_dl = (*(*cp).elements.add(idx as usize)).dl;

    if idx == 0 {
        return;
    }

    loop {
        p = parent(idx);
        if dl_time_before(orig_dl, (*(*cp).elements.add(p as usize)).dl) {
            break;
        }
        /* pull parent onto idx */
        (*(*cp).elements.add(idx as usize)).cpu = (*(*cp).elements.add(p as usize)).cpu;
        (*(*cp).elements.add(idx as usize)).dl = (*(*cp).elements.add(p as usize)).dl;
        let cpu = (*(*cp).elements.add(idx as usize)).cpu;
        (*(*cp).elements.add(cpu as usize)).idx = idx;
        idx = p;
        if idx == 0 {
            break;
        }
    }
    /* actual push up of saved original values orig_* */
    (*(*cp).elements.add(idx as usize)).cpu = orig_cpu;
    (*(*cp).elements.add(idx as usize)).dl = orig_dl;
    (*(*cp).elements.add(orig_cpu as usize)).idx = idx;
}

unsafe fn cpudl_heapify(cp: *mut cpudl, idx: i32) {
    if idx > 0 && dl_time_before((*(*cp).elements.add(parent(idx) as usize)).dl, (*(*cp).elements.add(idx as usize)).dl) {
        cpudl_heapify_up(cp, idx);
    } else {
        cpudl_heapify_down(cp, idx);
    }
}

#[inline]
unsafe fn cpudl_maximum(cp: *mut cpudl) -> i32 {
    (*(*cp).elements).cpu
}

/*
 * cpudl_find - find the best (later-dl) CPU in the system
 * @cp: the cpudl max-heap context
 * @p: the task
 * @later_mask: a mask to fill in with the selected CPUs (or NULL)
 *
 * Returns: int - CPUs were found
 */
pub unsafe fn cpudl_find(cp: *mut cpudl, p: *mut task_struct, later_mask: *mut cpumask) -> i32 {
    let dl_se = &(*p).dl;
    if !later_mask.is_null() && cpumask_and(later_mask, (*cp).free_cpus, &(*p).cpus_mask) {
        let mut max_cap: c_ulong = 0;
        let mut max_cpu: i32 = -1;
        if !sched_asym_cpucap_active() { return 1; }
        for_each_cpu!(cpu, later_mask) {
            if !dl_task_fits_capacity(p, cpu) {
                cpumask_clear_cpu(cpu, later_mask);
                let cap = arch_scale_cpu_capacity(cpu);
                if cap > max_cap || (cpu == task_cpu(p) && cap == max_cap) {
                    max_cap = cap;
                    max_cpu = cpu;
                }
            }
        }
        if cpumask_empty(later_mask) { cpumask_set_cpu(max_cpu, later_mask); }
        1
    } else {
        let best_cpu = cpudl_maximum(cp);
        WARN_ON(best_cpu != -1 && !cpu_present(best_cpu));
        if cpumask_test_cpu(best_cpu, &(*p).cpus_mask) && dl_time_before(dl_se.deadline, (*(*cp).elements).dl) {
            if !later_mask.is_null() { cpumask_set_cpu(best_cpu, later_mask); }
            1
        } else { 0 }
    }
}

/*
 * cpudl_clear - remove a CPU from the cpudl max-heap
 * @cp: the cpudl max-heap context
 * @cpu: the target CPU
 * @online: the online state of the deadline runqueue
 *
 * Notes: assumes cpu_rq(cpu)->lock is locked
 *
 * Returns: (void)
 */
pub unsafe fn cpudl_clear(cp: *mut cpudl, cpu: i32, online: bool) {
    let mut flags: ulong = 0;
    WARN_ON(!cpu_present(cpu));
    raw_spin_lock_irqsave(&mut (*cp).lock, &mut flags);
    let old_idx = (*(*cp).elements.add(cpu as usize)).idx;
    if old_idx != IDX_INVALID {
        let new_cpu = (*(*cp).elements.add(((*cp).size - 1) as usize)).cpu;
        (*(*cp).elements.add(old_idx as usize)).dl = (*(*cp).elements.add(((*cp).size - 1) as usize)).dl;
        (*(*cp).elements.add(old_idx as usize)).cpu = new_cpu;
        (*cp).size -= 1;
        (*(*cp).elements.add(new_cpu as usize)).idx = old_idx;
        (*(*cp).elements.add(cpu as usize)).idx = IDX_INVALID;
        cpudl_heapify(cp, old_idx);
    }
    if likely(online) { __cpumask_set_cpu(cpu, (*cp).free_cpus); } else { __cpumask_clear_cpu(cpu, (*cp).free_cpus); }
    raw_spin_unlock_irqrestore(&mut (*cp).lock, flags);
}

/* cpudl_set - update the cpudl max-heap */
pub unsafe fn cpudl_set(cp: *mut cpudl, cpu: i32, dl: u64) {
    let mut flags: ulong = 0;
    WARN_ON(!cpu_present(cpu));
    raw_spin_lock_irqsave(&mut (*cp).lock, &mut flags);
    let old_idx = (*(*cp).elements.add(cpu as usize)).idx;
    if old_idx == IDX_INVALID {
        let new_idx = (*cp).size;
        (*cp).size += 1;
        (*(*cp).elements.add(new_idx as usize)).dl = dl;
        (*(*cp).elements.add(new_idx as usize)).cpu = cpu;
        (*(*cp).elements.add(cpu as usize)).idx = new_idx;
        cpudl_heapify_up(cp, new_idx);
        __cpumask_clear_cpu(cpu, (*cp).free_cpus);
    } else {
        (*(*cp).elements.add(old_idx as usize)).dl = dl;
        cpudl_heapify(cp, old_idx);
    }
    raw_spin_unlock_irqrestore(&mut (*cp).lock, flags);
}

/* cpudl_init - initialize the cpudl structure */
pub unsafe fn cpudl_init(cp: *mut cpudl) -> i32 {
    raw_spin_lock_init(&mut (*cp).lock);
    (*cp).size = 0;
    (*cp).elements = kzalloc_objs::<cpudl_item>(nr_cpu_ids);
    if (*cp).elements.is_null() { return -ENOMEM; }
    if !zalloc_cpumask_var(&mut (*cp).free_cpus, GFP_KERNEL) {
        kfree((*cp).elements);
        return -ENOMEM;
    }
    for_each_possible_cpu!(i) { (*(*cp).elements.add(i as usize)).idx = IDX_INVALID; }
    0
}

/* cpudl_cleanup - clean up the cpudl structure */
pub unsafe fn cpudl_cleanup(cp: *mut cpudl) {
    free_cpumask_var((*cp).free_cpus);
    kfree((*cp).elements);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
