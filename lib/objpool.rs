// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the Linux/kernel interfaces and other translation units.

/*
 * objpool: ring-array based lockless MPMC/FIFO queues
 *
 * Copyright: wuqiang.matt@bytedance.com,mhiramat@kernel.org
 */

/* initialize percpu objpool_slot */
unsafe fn objpool_init_percpu_slot(
    pool: *mut objpool_head,
    slot: *mut objpool_slot,
    nodes: i32,
    context: *mut core::ffi::c_void,
    objinit: objpool_init_obj_cb,
) -> i32 {
    let mut obj = (*slot).entries.as_mut_ptr().add((*pool).capacity as usize) as *mut u8;
    let mut i = 0;

    /* initialize elements of percpu objpool_slot */
    (*slot).mask = (*pool).capacity - 1;

    while i < nodes {
        if let Some(init) = objinit {
            let rc = init(obj as *mut core::ffi::c_void, context);
            if rc != 0 {
                return rc;
            }
        }
        let index = ((*slot).tail & (*slot).mask) as usize;
        *(*slot).entries.as_mut_ptr().add(index) = obj as *mut core::ffi::c_void;
        obj = obj.add((*pool).obj_size as usize);
        (*slot).tail += 1;
        (*slot).last = (*slot).tail;
        (*pool).nr_objs += 1;
        i += 1;
    }

    0
}

/* allocate and initialize percpu slots */
unsafe fn objpool_init_percpu_slots(
    pool: *mut objpool_head,
    nr_objs: i32,
    context: *mut core::ffi::c_void,
    objinit: objpool_init_obj_cb,
) -> i32 {
    let mut i = 0;
    let mut cpu_count = 0;

    while i < nr_cpu_ids {
        /* skip the cpu node which could never be present */
        if !cpu_possible(i) {
            i += 1;
            continue;
        }

        /* compute how many objects to be allocated with this slot */
        let mut nodes = nr_objs / (*pool).nr_possible_cpus;
        if cpu_count < (nr_objs % (*pool).nr_possible_cpus) {
            nodes += 1;
        }
        cpu_count += 1;

        let size = core::mem::size_of::<objpool_slot>()
            + (*pool).capacity as usize * core::mem::size_of::<*mut core::ffi::c_void>()
            + (*pool).obj_size as usize * nodes as usize;

        /*
         * here we allocate percpu-slot & objs together in a single
         * allocation to make it more compact, taking advantage of
         * warm caches and TLB hits. in default vmalloc is used
         * to reduce the pressure of kernel slab system. as we know,
         * mimimal size of vmalloc is one page since vmalloc would
         * always align the requested size to page size.
         * but if vmalloc fails or it is not available (e.g. GFP_ATOMIC)
         * allocate percpu slot with kmalloc.
         */
        let mut slot: *mut objpool_slot = core::ptr::null_mut();

        if ((*pool).gfp & (GFP_ATOMIC | GFP_KERNEL)) != GFP_ATOMIC {
            slot = __vmalloc_node(
                size,
                core::mem::size_of::<*mut core::ffi::c_void>(),
                (*pool).gfp,
                cpu_to_node(i),
                __builtin_return_address(0),
            ) as *mut objpool_slot;
        }

        if slot.is_null() {
            slot = kmalloc_node(size, (*pool).gfp, cpu_to_node(i)) as *mut objpool_slot;
            if slot.is_null() {
                return -ENOMEM;
            }
        }
        memset(slot as *mut core::ffi::c_void, 0, size);
        *(*pool).cpu_slots.add(i as usize) = slot;

        /* initialize the objpool_slot of cpu node i */
        let rc = objpool_init_percpu_slot(pool, slot, nodes, context, objinit);
        if rc != 0 {
            return rc;
        }
        i += 1;
    }

    0
}

/* cleanup all percpu slots of the object pool */
unsafe fn objpool_fini_percpu_slots(pool: *mut objpool_head) {
    if (*pool).cpu_slots.is_null() {
        return;
    }

    let mut i = 0;
    while i < nr_cpu_ids {
        kvfree(*(*pool).cpu_slots.add(i as usize) as *mut core::ffi::c_void);
        i += 1;
    }
    kfree((*pool).cpu_slots as *mut core::ffi::c_void);
}

/* initialize object pool and pre-allocate objects */
#[no_mangle]
pub unsafe extern "C" fn objpool_init(
    pool: *mut objpool_head,
    nr_objs: i32,
    mut object_size: i32,
    gfp: gfp_t,
    context: *mut core::ffi::c_void,
    objinit: objpool_init_obj_cb,
    release: objpool_fini_cb,
) -> i32 {
    /* check input parameters */
    if nr_objs <= 0 || nr_objs > OBJPOOL_NR_OBJECT_MAX || object_size <= 0 || object_size > OBJPOOL_OBJECT_SIZE_MAX {
        return -EINVAL;
    }

    /* align up to unsigned long size */
    object_size = ALIGN(object_size, core::mem::size_of::<c_long>() as i32);

    /* calculate capacity of percpu objpool_slot */
    let capacity = roundup_pow_of_two(nr_objs);
    if capacity == 0 {
        return -EINVAL;
    }

    /* initialize objpool pool */
    memset(pool as *mut core::ffi::c_void, 0, core::mem::size_of::<objpool_head>());
    (*pool).nr_possible_cpus = num_possible_cpus();
    (*pool).obj_size = object_size;
    (*pool).capacity = capacity;
    (*pool).gfp = gfp & !__GFP_ZERO;
    (*pool).context = context;
    (*pool).release = release;
    let slot_size = nr_cpu_ids as usize * core::mem::size_of::<*mut objpool_slot>();
    (*pool).cpu_slots = kzalloc(slot_size, (*pool).gfp) as *mut *mut objpool_slot;
    if (*pool).cpu_slots.is_null() {
        return -ENOMEM;
    }

    /* initialize per-cpu slots */
    let rc = objpool_init_percpu_slots(pool, nr_objs, context, objinit);
    if rc != 0 {
        objpool_fini_percpu_slots(pool);
    } else {
        refcount_set(&mut (*pool).refcount, (*pool).nr_objs + 1);
    }
    rc
}

/* release whole objpool forcely */
#[no_mangle]
pub unsafe extern "C" fn objpool_free(pool: *mut objpool_head) {
    if (*pool).cpu_slots.is_null() {
        return;
    }
    objpool_fini_percpu_slots(pool);
    if let Some(release) = (*pool).release {
        release(pool, (*pool).context);
    }
}

/* drop the allocated object, rather reclaim it to objpool */
#[no_mangle]
pub unsafe extern "C" fn objpool_drop(obj: *mut core::ffi::c_void, pool: *mut objpool_head) -> i32 {
    if obj.is_null() || pool.is_null() {
        return -EINVAL;
    }
    if refcount_dec_and_test(&mut (*pool).refcount) {
        objpool_free(pool);
        return 0;
    }
    -EAGAIN
}

/* drop unused objects and defref objpool for releasing */
#[no_mangle]
pub unsafe extern "C" fn objpool_fini(pool: *mut objpool_head) {
    let mut count = 1; /* extra ref for objpool itself */

    /* drop all remained objects from objpool */
    while !objpool_pop(pool).is_null() {
        count += 1;
    }

    if refcount_sub_and_test(count, &mut (*pool).refcount) {
        objpool_free(pool);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
