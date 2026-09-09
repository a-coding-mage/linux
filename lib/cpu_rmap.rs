// SPDX-License-Identifier: GPL-2.0-only
/*
 * cpu_rmap.c: CPU affinity reverse-map support
 * Copyright 2011 Solarflare Communications Inc.
 */

// External Linux kernel declarations supplied by other translation units:
// cpu_rmap, kref, gfp_t, cpumask, cpumask_t, irq_affinity_notify, and helpers.

/*
 * These functions maintain a mapping from CPUs to some ordered set of
 * objects with CPU affinities.  This can be seen as a reverse-map of
 * CPU affinity.  However, we do not assume that the object affinities
 * cover all CPUs in the system.  For those CPUs not directly covered
 * by object affinities, we attempt to find a nearest object based on
 * CPU topology.
 */

/// alloc_cpu_rmap - allocate CPU affinity reverse-map
/// @size: Number of objects to be mapped
/// @flags: Allocation flags e.g. %GFP_KERNEL
pub unsafe fn alloc_cpu_rmap(size: u32, flags: gfp_t) -> *mut cpu_rmap {
    let mut rmap: *mut cpu_rmap;
    let mut cpu: u32;
    let obj_offset: usize;

    /* This is a silly number of objects, and we use u16 indices. */
    if size > 0xffff {
        return core::ptr::null_mut();
    }

    /* Offset of object pointer array from base structure */
    obj_offset = ALIGN(
        core::mem::offset_of!(cpu_rmap, near) + nr_cpu_ids as usize * core::mem::size_of::<cpu_rmap_near>(),
        core::mem::size_of::<*mut core::ffi::c_void>(),
    );

    rmap = kzalloc(obj_offset + size as usize * core::mem::size_of::<*mut core::ffi::c_void>(), flags);
    if rmap.is_null() {
        return core::ptr::null_mut();
    }

    kref_init(&mut (*rmap).refcount);
    (*rmap).obj = (rmap as *mut u8).add(obj_offset) as *mut *mut core::ffi::c_void;

    /* Initially assign CPUs to objects on a rota, since we have
     * no idea where the objects are.  Use infinite distance, so
     * any object with known distance is preferable.  Include the
     * CPUs that are not present/online, since we definitely want
     * any newly-hotplugged CPUs to have some object assigned.
     */
    for_each_possible_cpu!(cpu) {
        (*rmap).near[cpu as usize].index = (cpu % size) as u16;
        (*rmap).near[cpu as usize].dist = CPU_RMAP_DIST_INF;
    }

    (*rmap).size = size;
    rmap
}

/// cpu_rmap_release - internal reclaiming helper called from kref_put
/// @ref: kref to struct cpu_rmap
unsafe fn cpu_rmap_release(ref_: *mut kref) {
    let rmap = container_of!(ref_, cpu_rmap, refcount);
    kfree(rmap);
}

/// cpu_rmap_get - internal helper to get new ref on a cpu_rmap
/// @rmap: reverse-map allocated with alloc_cpu_rmap()
pub unsafe fn cpu_rmap_get(rmap: *mut cpu_rmap) {
    kref_get(&mut (*rmap).refcount);
}

/// cpu_rmap_put - release ref on a cpu_rmap
/// @rmap: reverse-map allocated with alloc_cpu_rmap()
pub unsafe fn cpu_rmap_put(rmap: *mut cpu_rmap) -> i32 {
    kref_put(&mut (*rmap).refcount, cpu_rmap_release)
}

/* Reevaluate nearest object for given CPU, comparing with the given
 * neighbours at the given distance.
 */
unsafe fn cpu_rmap_copy_neigh(
    rmap: *mut cpu_rmap,
    cpu: u32,
    mask: *const cpumask,
    dist: u16,
) -> bool {
    let mut neigh: i32;

    for_each_cpu!(neigh, mask) {
        if (*rmap).near[cpu as usize].dist > dist
            && (*rmap).near[neigh as usize].dist <= dist
        {
            (*rmap).near[cpu as usize].index = (*rmap).near[neigh as usize].index;
            (*rmap).near[cpu as usize].dist = dist;
            return true;
        }
    }
    false
}

#[cfg(feature = "DEBUG")]
unsafe fn debug_print_rmap(rmap: *const cpu_rmap, prefix: *const core::ffi::c_char) {
    let mut index: u32;
    let mut cpu: u32;

    pr_info!("cpu_rmap %p, %s:\n", rmap, prefix);

    for_each_possible_cpu!(cpu) {
        index = (*rmap).near[cpu as usize].index as u32;
        pr_info!(
            "cpu %d -> obj %u (distance %u)\n",
            cpu,
            index,
            (*rmap).near[cpu as usize].dist
        );
    }
}

#[cfg(not(feature = "DEBUG"))]
unsafe fn debug_print_rmap(_rmap: *const cpu_rmap, _prefix: *const core::ffi::c_char) {}

unsafe fn get_free_index(rmap: *mut cpu_rmap) -> i32 {
    let mut i: u32;

    for i in 0..(*rmap).size {
        if (*rmap).obj[i as usize].is_null() {
            return i as i32;
        }
    }

    -ENOSPC
}

/// cpu_rmap_add - add object to a rmap
/// @rmap: CPU rmap allocated with alloc_cpu_rmap()
/// @obj: Object to add to rmap
///
/// Return index of object or -ENOSPC if no free entry was found
pub unsafe fn cpu_rmap_add(rmap: *mut cpu_rmap, obj: *mut core::ffi::c_void) -> i32 {
    let index = get_free_index(rmap);

    if index < 0 {
        return index;
    }

    (*rmap).obj[index as usize] = obj;
    index
}

/// cpu_rmap_update - update CPU rmap following a change of object affinity
/// @rmap: CPU rmap to update
/// @index: Index of object whose affinity changed
/// @affinity: New CPU affinity of object
pub unsafe fn cpu_rmap_update(
    rmap: *mut cpu_rmap,
    index: u16,
    affinity: *const cpumask,
) -> i32 {
    let mut update_mask: cpumask_var_t = core::mem::zeroed();
    let mut cpu: u32;

    if !zalloc_cpumask_var(&mut update_mask, GFP_KERNEL) {
        return -ENOMEM;
    }

    /* Invalidate distance for all CPUs for which this used to be
     * the nearest object.  Mark those CPUs for update.
     */
    for_each_online_cpu!(cpu) {
        if (*rmap).near[cpu as usize].index == index {
            (*rmap).near[cpu as usize].dist = CPU_RMAP_DIST_INF;
            cpumask_set_cpu(cpu, update_mask);
        }
    }

    debug_print_rmap(rmap, c"after invalidating old distances");

    /* Set distance to 0 for all CPUs in the new affinity mask.
     * Mark all CPUs within their NUMA nodes for update.
     */
    for_each_cpu!(cpu, affinity) {
        (*rmap).near[cpu as usize].index = index;
        (*rmap).near[cpu as usize].dist = 0;
        cpumask_or(
            update_mask,
            update_mask,
            cpumask_of_node(cpu_to_node(cpu)),
        );
    }

    debug_print_rmap(rmap, c"after updating neighbours");

    /* Update distances based on topology */
    for_each_cpu!(cpu, update_mask) {
        if cpu_rmap_copy_neigh(rmap, cpu, topology_sibling_cpumask(cpu), 1) {
            continue;
        }
        if cpu_rmap_copy_neigh(rmap, cpu, topology_core_cpumask(cpu), 2) {
            continue;
        }
        if cpu_rmap_copy_neigh(rmap, cpu, cpumask_of_node(cpu_to_node(cpu)), 3) {
            continue;
        }
        /* We could continue into NUMA node distances, but for now
         * we give up.
         */
    }

    debug_print_rmap(rmap, c"after copying neighbours");

    free_cpumask_var(update_mask);
    0
}

/* Glue between IRQ affinity notifiers and CPU rmaps */

#[repr(C)]
struct irq_glue {
    notify: irq_affinity_notify,
    rmap: *mut cpu_rmap,
    index: u16,
}

/// free_irq_cpu_rmap - free a CPU affinity reverse-map used for IRQs
/// @rmap: Reverse-map allocated with alloc_irq_cpu_map(), or %NULL
///
/// Must be called in process context, before freeing the IRQs.
pub unsafe fn free_irq_cpu_rmap(rmap: *mut cpu_rmap) {
    if rmap.is_null() {
        return;
    }

    for index in 0..(*rmap).size as u16 {
        let glue = (*rmap).obj[index as usize] as *mut irq_glue;
        if !glue.is_null() {
            irq_set_affinity_notifier((*glue).notify.irq, core::ptr::null_mut());
        }
    }

    cpu_rmap_put(rmap);
}

/// irq_cpu_rmap_notify - callback for IRQ subsystem when IRQ affinity updated
/// @notify: struct irq_affinity_notify passed by irq/manage.c
/// @mask: cpu mask for new SMP affinity
///
/// This is executed in workqueue context.
unsafe fn irq_cpu_rmap_notify(
    notify: *mut irq_affinity_notify,
    mask: *const cpumask_t,
) {
    let glue = container_of!(notify, irq_glue, notify);
    let rc = cpu_rmap_update((*glue).rmap, (*glue).index, mask as *const cpumask);
    if rc != 0 {
        pr_warn!("irq_cpu_rmap_notify: update failed: %d\n", rc);
    }
}

/// irq_cpu_rmap_release - reclaiming callback for IRQ subsystem
/// @ref: kref to struct irq_affinity_notify passed by irq/manage.c
unsafe fn irq_cpu_rmap_release(ref_: *mut kref) {
    let glue = container_of!(ref_, irq_glue, notify.kref);

    (*glue).rmap.as_mut().unwrap().obj[(*glue).index as usize] = core::ptr::null_mut();
    cpu_rmap_put((*glue).rmap);
    kfree(glue);
}

/// irq_cpu_rmap_remove - remove an IRQ from a CPU affinity reverse-map
/// @rmap: The reverse-map
/// @irq: The IRQ number
pub unsafe fn irq_cpu_rmap_remove(_rmap: *mut cpu_rmap, irq: i32) -> i32 {
    irq_set_affinity_notifier(irq, core::ptr::null_mut())
}

/// irq_cpu_rmap_add - add an IRQ to a CPU affinity reverse-map
/// @rmap: The reverse-map
/// @irq: The IRQ number
///
/// This adds an IRQ affinity notifier that will update the reverse-map
/// automatically.
///
/// Must be called in process context, after the IRQ is allocated but
/// before it is bound with request_irq().
pub unsafe fn irq_cpu_rmap_add(rmap: *mut cpu_rmap, irq: i32) -> i32 {
    let glue = kzalloc_obj::<irq_glue>();
    let mut rc: i32;

    if glue.is_null() {
        return -ENOMEM;
    }
    (*glue).notify.notify = Some(irq_cpu_rmap_notify);
    (*glue).notify.release = Some(irq_cpu_rmap_release);
    (*glue).rmap = rmap;
    cpu_rmap_get(rmap);
    rc = cpu_rmap_add(rmap, glue as *mut core::ffi::c_void);
    if rc < 0 {
        cpu_rmap_put((*glue).rmap);
        kfree(glue);
        return rc;
    }

    (*glue).index = rc as u16;
    rc = irq_set_affinity_notifier(irq, &mut (*glue).notify);
    if rc != 0 {
        (*rmap).obj[(*glue).index as usize] = core::ptr::null_mut();
        cpu_rmap_put((*glue).rmap);
        kfree(glue);
        return rc;
    }

    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
