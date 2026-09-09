// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2016 Thomas Gleixner.
 * Copyright (C) 2016-2017 Christoph Hellwig.
 */

unsafe fn default_calc_sets(affd: *mut irq_affinity, affvecs: c_uint) {
    (*affd).nr_sets = 1;
    (*affd).set_size[0] = affvecs;
}

/**
 * irq_create_affinity_masks - Create affinity masks for multiqueue spreading
 * @nvecs:      The total number of vectors
 * @affd:       Description of the affinity requirements
 *
 * Returns the irq_affinity_desc pointer or NULL if allocation failed.
 */
unsafe fn irq_create_affinity_masks(
    nvecs: c_uint,
    affd: *mut irq_affinity,
) -> *mut irq_affinity_desc {
    let mut affvecs: c_uint;
    let mut curvec: c_uint;
    let mut usedvecs: c_uint;
    let mut i: c_uint;
    let mut masks: *mut irq_affinity_desc = core::ptr::null_mut();

    /*
     * Determine the number of vectors which need interrupt affinities
     * assigned. If the pre/post request exhausts the available vectors
     * then nothing to do here except for invoking the calc_sets()
     * callback so the device driver can adjust to the situation.
     */
    if nvecs > (*affd).pre_vectors + (*affd).post_vectors {
        affvecs = nvecs - (*affd).pre_vectors - (*affd).post_vectors;
    } else {
        affvecs = 0;
    }

    /*
     * Simple invocations do not provide a calc_sets() callback. Install
     * the generic one.
     */
    if (*affd).calc_sets.is_none() {
        (*affd).calc_sets = Some(default_calc_sets);
    }

    /* Recalculate the sets */
    ((*affd).calc_sets.unwrap())(affd, affvecs);

    if (*affd).nr_sets > IRQ_AFFINITY_MAX_SETS {
        return core::ptr::null_mut();
    }

    /* Nothing to assign? */
    if affvecs == 0 {
        return core::ptr::null_mut();
    }

    masks = kzalloc_objs!(*masks, nvecs);
    if masks.is_null() {
        return core::ptr::null_mut();
    }

    /* Fill out vectors at the beginning that don't need affinity */
    curvec = 0;
    while curvec < (*affd).pre_vectors {
        cpumask_copy(
            &mut (*masks.add(curvec as usize)).mask,
            irq_default_affinity,
        );
        curvec += 1;
    }

    /*
     * Spread on present CPUs starting from affd->pre_vectors. If we
     * have multiple sets, build each sets affinity mask separately.
     */
    i = 0;
    usedvecs = 0;
    while i < (*affd).nr_sets {
        let mut nr_masks: c_uint = 0;
        let this_vecs: c_uint = (*affd).set_size[i as usize];
        let result: *mut cpumask = group_cpus_evenly(this_vecs, &mut nr_masks);

        if result.is_null() {
            kfree(masks as *mut core::ffi::c_void);
            return core::ptr::null_mut();
        }

        let mut j: c_uint = 0;
        while j < nr_masks {
            cpumask_copy(
                &mut (*masks.add((curvec + j) as usize)).mask,
                &*result.add(j as usize),
            );
            j += 1;
        }
        kfree(result as *mut core::ffi::c_void);

        curvec += nr_masks;
        usedvecs += nr_masks;
        i += 1;
    }

    /* Fill out vectors at the end that don't need affinity */
    if usedvecs >= affvecs {
        curvec = (*affd).pre_vectors + affvecs;
    } else {
        curvec = (*affd).pre_vectors + usedvecs;
    }
    while curvec < nvecs {
        cpumask_copy(
            &mut (*masks.add(curvec as usize)).mask,
            irq_default_affinity,
        );
        curvec += 1;
    }

    /* Mark the managed interrupts */
    i = (*affd).pre_vectors;
    while i < nvecs - (*affd).post_vectors {
        (*masks.add(i as usize)).is_managed = 1;
        i += 1;
    }

    masks
}

/**
 * irq_calc_affinity_vectors - Calculate the optimal number of vectors
 * @minvec:      The minimum number of vectors available
 * @maxvec:      The maximum number of vectors available
 * @affd:        Description of the affinity requirements
 */
unsafe fn irq_calc_affinity_vectors(
    minvec: c_uint,
    maxvec: c_uint,
    affd: *const irq_affinity,
) -> c_uint {
    let resv: c_uint = (*affd).pre_vectors + (*affd).post_vectors;
    let set_vecs: c_uint;

    if resv > minvec {
        return 0;
    }

    if (*affd).calc_sets.is_some() {
        set_vecs = maxvec - resv;
    } else {
        set_vecs = cpumask_weight(cpu_possible_mask);
    }

    resv + core::cmp::min(set_vecs, maxvec - resv)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
