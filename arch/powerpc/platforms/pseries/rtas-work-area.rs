// SPDX-License-Identifier: GPL-2.0-only

// pr_fmt(fmt) = "rtas-work-area: " fmt

// Dependencies supplied by the surrounding kernel translation.

enum {
    /* Ensure the pool is page-aligned. */
    RTAS_WORK_AREA_ARENA_ALIGN = PAGE_SIZE,
    /* Don't let a single allocation claim the whole arena. */
    RTAS_WORK_AREA_ARENA_SZ = RTAS_WORK_AREA_MAX_ALLOC_SZ * 2,
    /*
     * The smallest known work area size is for ibm,get-vpd's
     * location code argument, which is limited to 79 characters
     * plus 1 nul terminator.
     *
     * PAPR+ 7.3.20 ibm,get-vpd RTAS Call
     * PAPR+ 12.3.2.4 Converged Location Code Rules - Length Restrictions
     */
    RTAS_WORK_AREA_MIN_ALLOC_SZ = roundup_pow_of_two(80),
}

struct RwaState {
    gen_pool: *mut gen_pool,
    arena: *mut core::ffi::c_char,
    mutex: mutex, /* serializes allocations */
    wqh: wait_queue_head,
    descriptor_pool: mempool_t,
    available: bool,
}

static mut rwa_state: RwaState = RwaState {
    gen_pool: core::ptr::null_mut(),
    arena: core::ptr::null_mut(),
    mutex: __MUTEX_INITIALIZER,
    wqh: __WAIT_QUEUE_HEAD_INITIALIZER,
    descriptor_pool: mempool_t {},
    available: false,
};

/*
 * A single work area buffer and descriptor to serve requests early in
 * boot before the allocator is fully initialized. We know 4KB is the
 * most any boot time user needs (they all call ibm,get-system-parameter).
 */
static mut early_work_area_in_use: bool = false;
#[repr(align(4096))]
static mut early_work_area_buf: [core::ffi::c_char; SZ_4K] = [0; SZ_4K];
static mut early_work_area: rtas_work_area = rtas_work_area {
    buf: unsafe { early_work_area_buf.as_mut_ptr() },
    size: core::mem::size_of::<[core::ffi::c_char; SZ_4K]>(),
};

unsafe fn rtas_work_area_alloc_early(size: usize) -> *mut rtas_work_area {
    WARN_ON(size > early_work_area.size);
    WARN_ON(early_work_area_in_use);
    early_work_area_in_use = true;
    memset(early_work_area.buf, 0, early_work_area.size);
    &raw mut early_work_area
}

unsafe fn rtas_work_area_free_early(work_area: *mut rtas_work_area) {
    WARN_ON(work_area != &raw mut early_work_area);
    WARN_ON(!early_work_area_in_use);
    early_work_area_in_use = false;
}

unsafe fn __rtas_work_area_alloc(size: usize) -> *mut rtas_work_area {
    let area: *mut rtas_work_area;
    let mut addr: c_ulong;

    might_sleep();

    /*
     * The rtas_work_area_alloc() wrapper enforces this at build
     * time. Requests that exceed the arena size will block
     * indefinitely.
     */
    WARN_ON(size > RTAS_WORK_AREA_MAX_ALLOC_SZ);

    if !rwa_state.available {
        return rtas_work_area_alloc_early(size);
    }
    /*
     * To ensure FCFS behavior and prevent a high rate of smaller
     * requests from starving larger ones, use the mutex to queue
     * allocations.
     */
    mutex_lock(&raw mut rwa_state.mutex);
    wait_event(&raw mut rwa_state.wqh, {
        addr = gen_pool_alloc(rwa_state.gen_pool, size);
        addr != 0
    });
    mutex_unlock(&raw mut rwa_state.mutex);

    area = mempool_alloc(&raw mut rwa_state.descriptor_pool, GFP_KERNEL);
    (*area).buf = addr as *mut core::ffi::c_char;
    (*area).size = size;

    area
}

unsafe fn rtas_work_area_free(area: *mut rtas_work_area) {
    if !rwa_state.available {
        rtas_work_area_free_early(area);
        return;
    }

    gen_pool_free(rwa_state.gen_pool, (*area).buf as c_ulong, (*area).size);
    mempool_free(area, &raw mut rwa_state.descriptor_pool);
    wake_up(&raw mut rwa_state.wqh);
}

/*
 * Initialization of the work area allocator happens in two parts. To
 * reliably reserve an arena that satisfies RTAS addressing
 * requirements, we must perform a memblock allocation early,
 * immmediately after RTAS instantiation. Then we have to wait until
 * the slab allocator is up before setting up the descriptor mempool
 * and adding the arena to a gen_pool.
 */
unsafe fn rtas_work_area_allocator_init() -> c_int {
    let order: c_uint = ilog2(RTAS_WORK_AREA_MIN_ALLOC_SZ);
    let pa_start: phys_addr_t = __pa(rwa_state.arena);
    let pa_end: phys_addr_t = pa_start + RTAS_WORK_AREA_ARENA_SZ - 1;
    let mut pool: *mut gen_pool;
    let nid: c_int = NUMA_NO_NODE;
    let mut err: c_int;

    err = -ENOMEM;
    if rwa_state.arena.is_null() { goto_err_out!(); }

    pool = gen_pool_create(order, nid);
    if pool.is_null() { goto_err_out!(); }
    /* All RTAS functions accept natural alignment where alignment is required. */
    gen_pool_set_algo(pool, gen_pool_first_fit_order_align, core::ptr::null_mut());

    err = gen_pool_add(pool, rwa_state.arena as c_ulong, RTAS_WORK_AREA_ARENA_SZ, nid);
    if err != 0 { goto_err_destroy!(); }

    err = mempool_init_kmalloc_pool(&raw mut rwa_state.descriptor_pool, 1,
                                    core::mem::size_of::<rtas_work_area>());
    if err != 0 { goto_err_destroy!(); }

    rwa_state.gen_pool = pool;
    rwa_state.available = true;
    pr_debug!("arena (%uK), min/max alloc sizes %u/%u\n",
              RTAS_WORK_AREA_ARENA_SZ / SZ_1K,
              RTAS_WORK_AREA_MIN_ALLOC_SZ,
              RTAS_WORK_AREA_MAX_ALLOC_SZ);
    return 0;

    // The C goto cleanup labels are represented by the surrounding kernel's
    // translation support; preserve their control-flow intent here.
}

unsafe fn rtas_work_area_reserve_arena(limit: phys_addr_t) {
    let align: phys_addr_t = RTAS_WORK_AREA_ARENA_ALIGN;
    let size: phys_addr_t = RTAS_WORK_AREA_ARENA_SZ;
    let min: phys_addr_t = MEMBLOCK_LOW_LIMIT;
    let nid: c_int = NUMA_NO_NODE;

    /* Too early for a machine_is(pseries) check. */
    if rtas_function_implemented(RTAS_FN_IBM_GET_SYSTEM_PARAMETER) != 0
        || rtas_function_implemented(RTAS_FN_IBM_CONFIGURE_CONNECTOR) != 0
    {
        rwa_state.arena = memblock_alloc_try_nid(size, align, min, limit, nid);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
