// SPDX-License-Identifier: GPL-2.0
// External kernel headers and local declarations are supplied by the surrounding translation.

unsafe fn io_init_wq_offload(ctx: *mut io_ring_ctx, task: *mut task_struct) -> *mut io_wq {
    let mut hash: *mut io_wq_hash;
    let mut data: io_wq_data;
    let concurrency: u32;

    mutex_lock(&mut (*ctx).uring_lock);
    hash = (*ctx).hash_map;
    if hash.is_null() {
        hash = kzalloc_obj::<io_wq_hash>();
        if hash.is_null() {
            mutex_unlock(&mut (*ctx).uring_lock);
            return ERR_PTR(-ENOMEM);
        }
        refcount_set(&mut (*hash).refs, 1);
        init_waitqueue_head(&mut (*hash).wait);
        (*ctx).hash_map = hash;
    }
    mutex_unlock(&mut (*ctx).uring_lock);

    data.hash = hash;
    data.task = task;

    /* Do QD, or 4 * CPUS, whatever is smallest */
    concurrency = core::cmp::min((*ctx).sq_entries, 4 * num_online_cpus());

    io_wq_create(concurrency, &mut data)
}

pub unsafe fn io_uring_free_tctx(tsk: *mut task_struct) {
    let tctx = (*tsk).io_uring;
    let mut node: *mut io_tctx_node = core::ptr::null_mut();
    let mut index: usize = 0;

    /*
     * Fault injection forcing allocation errors in the xa_store() path
     * can lead to xa_empty() returning false, even though no actual
     * node is stored in the xarray. Until that gets sorted out, attempt
     * an iteration here and warn if any entries are found.
     */
    if !tctx.is_null() {
        xa_for_each(&mut (*tctx).xa, &mut index, &mut node);
        if !node.is_null() { WARN_ON_ONCE(true); }
        WARN_ON_ONCE(!(*tctx).io_wq.is_null());
        WARN_ON_ONCE((*tctx).cached_refs != 0);

        percpu_counter_destroy(&mut (*tctx).inflight);
        kfree(tctx);
        (*tsk).io_uring = core::ptr::null_mut();
    }
}

pub unsafe fn __io_uring_free(tsk: *mut task_struct) {
    io_uring_free_tctx(tsk);
    if !(*tsk).io_uring_restrict.is_null() {
        io_put_bpf_filters((*tsk).io_uring_restrict);
        kfree((*tsk).io_uring_restrict);
        (*tsk).io_uring_restrict = core::ptr::null_mut();
    }
}

pub unsafe fn io_uring_alloc_task_context(task: *mut task_struct, ctx: *mut io_ring_ctx) -> *mut io_uring_task {
    let tctx = kzalloc_obj::<io_uring_task>();
    if tctx.is_null() { return ERR_PTR(-ENOMEM); }

    let mut ret = percpu_counter_init(&mut (*tctx).inflight, 0, GFP_KERNEL);
    if ret != 0 {
        kfree(tctx);
        return ERR_PTR(ret);
    }

    (*tctx).io_wq = io_init_wq_offload(ctx, task);
    if IS_ERR((*tctx).io_wq) {
        ret = PTR_ERR((*tctx).io_wq);
        percpu_counter_destroy(&mut (*tctx).inflight);
        kfree(tctx);
        return ERR_PTR(ret);
    }

    (*tctx).task = task;
    xa_init(&mut (*tctx).xa);
    init_waitqueue_head(&mut (*tctx).wait);
    atomic_set(&mut (*tctx).in_cancel, 0);
    atomic_set(&mut (*tctx).inflight_tracked, 0);
    mpscq_init(&mut (*tctx).task_list, &mut (*tctx).task_head);
    INIT_WORK(&mut (*tctx).fallback_work, io_tctx_fallback_work);
    init_task_work(&mut (*tctx).task_work, tctx_task_work);
    tctx
}

unsafe fn io_tctx_install_node(ctx: *mut io_ring_ctx, tctx: *mut io_uring_task) -> i32 {
    if !xa_load(&mut (*tctx).xa, ctx as usize).is_null() { return 0; }
    let node = kmalloc_obj::<io_tctx_node>();
    if node.is_null() { return -ENOMEM; }
    (*node).ctx = ctx;
    (*node).task = current;
    let ret = xa_err(xa_store(&mut (*tctx).xa, ctx as usize, node, GFP_KERNEL));
    if ret != 0 { kfree(node); return ret; }
    mutex_lock(&mut (*ctx).tctx_lock);
    list_add(&mut (*node).ctx_node, &mut (*ctx).tctx_list);
    mutex_unlock(&mut (*ctx).tctx_lock);
    0
}

pub unsafe fn __io_uring_add_tctx_node(ctx: *mut io_ring_ctx) -> i32 {
    let mut tctx = (*current).io_uring;
    let mut new_tctx = false;
    let mut ret: i32;
    if tctx.is_null() {
        tctx = io_uring_alloc_task_context(current, ctx);
        if IS_ERR(tctx) { return PTR_ERR(tctx); }
        new_tctx = true;
        if data_race((*ctx).int_flags) & IO_RING_F_IOWQ_LIMITS_SET != 0 {
            let mut limits = [0u32; 2];
            mutex_lock(&mut (*ctx).uring_lock);
            limits[0] = (*ctx).iowq_limits[0]; limits[1] = (*ctx).iowq_limits[1];
            mutex_unlock(&mut (*ctx).uring_lock);
            ret = io_wq_max_workers((*tctx).io_wq, limits.as_mut_ptr());
            if ret != 0 { return ret; }
        }
    }
    if !(*tctx).io_wq.is_null() { io_wq_set_exit_on_idle((*tctx).io_wq, false); }
    if new_tctx { (*current).io_uring = tctx; }
    ret = io_tctx_install_node(ctx, tctx);
    if ret == 0 { return 0; }
    if new_tctx {
        (*current).io_uring = core::ptr::null_mut();
        if !(*tctx).io_wq.is_null() { io_wq_exit_start((*tctx).io_wq); io_wq_put_and_exit((*tctx).io_wq); }
        percpu_counter_destroy(&mut (*tctx).inflight); kfree(tctx);
    }
    ret
}

pub unsafe fn __io_uring_add_tctx_node_from_submit(ctx: *mut io_ring_ctx) -> i32 {
    if (*ctx).flags & IORING_SETUP_SINGLE_ISSUER != 0 && (*ctx).submitter_task != current { return -EEXIST; }
    let ret = __io_uring_add_tctx_node(ctx); if ret != 0 { return ret; }
    (*(*current).io_uring).last = ctx; 0
}

/* Remove this io_uring_file -> task mapping. */
pub unsafe fn io_uring_del_tctx_node(index: usize) {
    let tctx = (*current).io_uring; if tctx.is_null() { return; }
    let node = xa_erase(&mut (*tctx).xa, index); if node.is_null() { return; }
    WARN_ON_ONCE(current != (*node).task); WARN_ON_ONCE(list_empty(&mut (*node).ctx_node));
    mutex_lock(&mut (*(*node).ctx).tctx_lock); list_del(&mut (*node).ctx_node); mutex_unlock(&mut (*(*node).ctx).tctx_lock);
    if (*tctx).last == (*node).ctx { (*tctx).last = core::ptr::null_mut(); }
    kfree(node);
    if xa_empty(&mut (*tctx).xa) && !(*tctx).io_wq.is_null() { io_wq_set_exit_on_idle((*tctx).io_wq, true); }
}

pub unsafe fn io_uring_clean_tctx(tctx: *mut io_uring_task) {
    let wq = (*tctx).io_wq; let mut node = core::ptr::null_mut(); let mut index = 0usize;
    while xa_for_each(&mut (*tctx).xa, &mut index, &mut node) { io_uring_del_tctx_node(index); cond_resched(); }
    if !wq.is_null() { io_wq_put_and_exit(wq); (*tctx).io_wq = core::ptr::null_mut(); }
}

pub unsafe fn io_uring_unreg_ringfd() {
    let tctx = (*current).io_uring;
    for i in 0..IO_RINGFD_REG_MAX { if !(*tctx).registered_rings[i].is_null() { fput((*tctx).registered_rings[i]); (*tctx).registered_rings[i] = core::ptr::null_mut(); } }
}

pub unsafe fn io_ring_add_registered_file(tctx: *mut io_uring_task, file: *mut file, start: i32, end: i32) -> i32 {
    for offset in start..end { let idx = array_index_nospec(offset, IO_RINGFD_REG_MAX); if !(*tctx).registered_rings[idx].is_null() { continue; } (*tctx).registered_rings[idx] = file; return idx as i32; }
    -EBUSY
}

unsafe fn io_ring_add_registered_fd(tctx: *mut io_uring_task, fd: i32, start: i32, end: i32) -> i32 {
    let file = fget(fd); if file.is_null() { return -EBADF; }
    if !io_is_uring_fops(file) { fput(file); return -EOPNOTSUPP; }
    let offset = io_ring_add_registered_file(tctx, file, start, end); if offset < 0 { fput(file); } offset
}

pub unsafe fn io_ringfd_register(ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void, nr_args: u32) -> i32 {
    if nr_args == 0 || nr_args > IO_RINGFD_REG_MAX as u32 { return -EINVAL; }
    mutex_unlock(&mut (*ctx).uring_lock); let ret = __io_uring_add_tctx_node(ctx); mutex_lock(&mut (*ctx).uring_lock); if ret != 0 { return ret; }
    let tctx = (*current).io_uring; let mut i = 0;
    while i < nr_args { let reg = &mut *(arg as *mut io_uring_rsrc_update).add(i as usize); if copy_from_user(reg, reg, core::mem::size_of::<io_uring_rsrc_update>()) != 0 { return if i != 0 { i as i32 } else { -EFAULT }; } if reg.resv != 0 { return if i != 0 { i as i32 } else { -EINVAL }; }
        let (start,end) = if reg.offset == -1u32 { (0,IO_RINGFD_REG_MAX as i32) } else { if reg.offset >= IO_RINGFD_REG_MAX as u32 { return -EINVAL; } (reg.offset as i32, reg.offset as i32 + 1) };
        let n = io_ring_add_registered_fd(tctx, reg.data, start, end); if n < 0 { return if i != 0 { i as i32 } else { n }; } reg.offset = n as u32; i += 1;
    } i as i32
}

pub unsafe fn io_ringfd_unregister(_ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void, nr_args: u32) -> i32 {
    if nr_args == 0 || nr_args > IO_RINGFD_REG_MAX as u32 { return -EINVAL; } let tctx = (*current).io_uring; if tctx.is_null() { return 0; }
    for i in 0..nr_args { let reg = &*(arg as *mut io_uring_rsrc_update).add(i as usize); if reg.resv != 0 || reg.data != 0 || reg.offset >= IO_RINGFD_REG_MAX as u32 { return if i != 0 { i as i32 } else { -EINVAL }; } let idx = array_index_nospec(reg.offset as i32, IO_RINGFD_REG_MAX); if !(*tctx).registered_rings[idx].is_null() { fput((*tctx).registered_rings[idx]); (*tctx).registered_rings[idx] = core::ptr::null_mut(); } } nr_args as i32
}

pub unsafe fn __io_uring_fork(tsk: *mut task_struct) -> i32 {
    let src = (*tsk).io_uring_restrict; /* Don't leave it dangling on error */ (*tsk).io_uring_restrict = core::ptr::null_mut();
    let res = kzalloc_obj::<io_restriction>(GFP_KERNEL_ACCOUNT); if res.is_null() { return -ENOMEM; }
    (*tsk).io_uring_restrict = res; io_restriction_clone(res, src); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
