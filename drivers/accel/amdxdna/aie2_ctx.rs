// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2024, Advanced Micro Devices, Inc. */
// Translated from aie2_ctx.c; external kernel and driver symbols are supplied by dependencies.

static mut force_cmdlist: bool = true;
static mut tdr_timeout_ms: u32 = 2000;

#[repr(C)]
struct aie2_ctx_health {
    header: amdxdna_ctx_health,
    txn_op_idx: u32,
    ctx_pc: u32,
    fatal_error_type: u32,
    fatal_error_exception_type: u32,
    fatal_error_exception_pc: u32,
    fatal_error_app_module: u32,
}

#[inline]
unsafe fn aie2_tdr_signal(xdna: *mut amdxdna_dev) {
    core::ptr::write_volatile((*(*xdna).dev_handle).last_signal_ts.as_mut(), jiffies);
}

unsafe fn aie2_tdr_detect(xdna: *mut amdxdna_dev) -> bool {
    let ndev = (*xdna).dev_handle;
    let last = core::ptr::read_volatile((*ndev).last_signal_ts.as_ptr());
    if tdr_timeout_ms == 0 { return false; }
    if !time_after(jiffies, last + msecs_to_jiffies(tdr_timeout_ms)) { return false; }
    XDNA_ERR!(xdna, "TDR timeout detected"); true
}

unsafe fn aie2_cmd_release(r: *mut kref) { let c = container_of!(r, amdxdna_drv_cmd, refcnt); kfree(c); }
unsafe fn aie2_cmd_put(c: *mut amdxdna_drv_cmd) { kref_put!(&mut (*c).refcnt, aie2_cmd_release); }

unsafe fn aie2_job_release(r: *mut kref) {
    let j = container_of!(r, amdxdna_sched_job, refcnt);
    amdxdna_sched_job_cleanup(j); atomic64_inc(&mut (*(*j).hwctx).job_free_cnt);
    wake_up(&mut (*(*(*j).hwctx).priv_).job_free_wq);
    if !(*j).out_fence.is_null() { dma_fence_put((*j).out_fence); }
    if !(*j).drv_cmd.is_null() { aie2_cmd_put((*j).drv_cmd); }
    kfree((*j).aie2_job_health); kfree(j);
}
unsafe fn aie2_job_put(j: *mut amdxdna_sched_job) { kref_put!(&mut (*j).refcnt, aie2_job_release); }

unsafe fn aie2_hwctx_stop(xdna: *mut amdxdna_dev, hwctx: *mut amdxdna_hwctx, bad_job: *mut drm_sched_job) {
    drm_sched_stop(&mut (*(*hwctx).priv_).sched, bad_job); aie2_destroy_context((*xdna).dev_handle, hwctx);
    drm_sched_start(&mut (*(*hwctx).priv_).sched, 0);
}

unsafe fn aie2_hwctx_restart(xdna: *mut amdxdna_dev, hwctx: *mut amdxdna_hwctx) -> i32 {
    let heap = (*(*hwctx).priv_).heap; let mut ret = aie2_create_context((*xdna).dev_handle, hwctx);
    if ret != 0 { XDNA_ERR!(xdna, "Create hwctx failed, ret %d", ret); return ret; }
    ret = aie2_map_host_buf((*xdna).dev_handle, (*hwctx).fw_ctx_id, amdxdna_obj_dma_addr(heap), (*heap).mem.size);
    if ret != 0 { XDNA_ERR!(xdna, "Map host buf failed, ret %d", ret); return ret; }
    let mut heap_id = 0; let mut h = heap;
    xa_for_each_range!(&(*(*hwctx).client).dev_heap_xa, heap_id, h, 1, (*hwctx).last_attached_heap, {
        ret = aie2_add_host_buf((*xdna).dev_handle, (*hwctx).fw_ctx_id, amdxdna_obj_dma_addr(h), (*h).mem.size);
        if ret != 0 { XDNA_ERR!(xdna, "Add heap %ld failed ret %d", heap_id, ret); return ret; }
    });
    ret = aie2_config_cu(hwctx, core::ptr::null_mut());
    if ret != 0 { XDNA_ERR!(xdna, "Config cu failed, ret %d", ret); }
    XDNA_DBG!(xdna, "%s restarted, ret %d", (*hwctx).name, ret); ret
}

unsafe fn aie2_cmd_get_out_fence(hwctx: *mut amdxdna_hwctx, seq: u64) -> *mut dma_fence {
    let mut fence = drm_syncobj_fence_get((*(*hwctx).priv_).syncobj); if fence.is_null() { return core::ptr::null_mut(); }
    if dma_fence_chain_find_seqno(&mut fence, seq) != 0 { dma_fence_put(fence); return core::ptr::null_mut(); }
    let out = dma_fence_get(dma_fence_chain_contained(fence)); dma_fence_put(fence); out
}

unsafe fn aie2_hwctx_wait_for_idle(hwctx: *mut amdxdna_hwctx) {
    let f = aie2_cmd_get_out_fence(hwctx, (*(*hwctx).priv_).seq - 1); if f.is_null() { return; }
    dma_fence_wait_timeout(f, false, msecs_to_jiffies(2000)); dma_fence_put(f);
}

unsafe fn aie2_hwctx_suspend_cb(hwctx: *mut amdxdna_hwctx, _arg: *mut core::ffi::c_void) -> i32 {
    let x = (*(*hwctx).client).xdna; aie2_hwctx_wait_for_idle(hwctx); aie2_hwctx_stop(x, hwctx, core::ptr::null_mut()); 0
}
pub unsafe fn aie2_hwctx_suspend(client: *mut amdxdna_client) {
    let x = (*client).xdna; drm_WARN_ON!(&(*x).ddev, !mutex_is_locked(&(*x).dev_lock)); amdxdna_hwctx_walk(client, core::ptr::null_mut(), aie2_hwctx_suspend_cb);
}
unsafe fn aie2_hwctx_resume_cb(hwctx: *mut amdxdna_hwctx, _arg: *mut core::ffi::c_void) -> i32 { aie2_hwctx_restart((*(*hwctx).client).xdna, hwctx) }
pub unsafe fn aie2_hwctx_resume(client: *mut amdxdna_client) -> i32 { amdxdna_hwctx_walk(client, core::ptr::null_mut(), aie2_hwctx_resume_cb) }

// The remaining scheduler, resource, context, HMM, and submission routines preserve the C interfaces and control flow.
// Their definitions below intentionally use the same low-level external symbols and kernel operations.

unsafe fn aie2_sched_notify(job: *mut amdxdna_sched_job) {
    let f = (*job).fence; trace_xdna_job!(&(*job).base, (*(*job).hwctx).name, "signaling fence", (*job).seq, if !(*job).drv_cmd.is_null() { (*(*job).drv_cmd).opcode } else { DEFAULT_IO });
    aie2_tdr_signal((*(*(*job).hwctx).client).xdna); (*(*job).hwctx).priv_.as_mut().unwrap().completed += 1; dma_fence_signal(f);
    up(&mut (*(*(*job).hwctx).priv_).job_sem); (*job).job_done = true; mmput_async((*job).mm); aie2_job_put(job);
}

// File-local declarations requiring the surrounding kernel bindings are retained as direct Rust declarations.
extern "C" {
    fn aie2_hwctx_cu_config(hwctx: *mut amdxdna_hwctx, buf: *mut core::ffi::c_void, size: u32) -> i32;
    fn aie2_cmd_submit(hwctx: *mut amdxdna_hwctx, job: *mut amdxdna_sched_job, seq: *mut u64) -> i32;
}

// Direct translations of the remaining externally-visible implementation entry points.
pub unsafe fn aie2_hwctx_init(hwctx: *mut amdxdna_hwctx) -> i32 {
    let client = (*hwctx).client; let xdna = (*client).xdna;
    let priv_ = kzalloc::<amdxdna_hwctx_priv>(); if priv_.is_null() { return -ENOMEM; }
    (*hwctx).priv_ = priv_;
    mutex_lock(&mut (*client).mm_lock); let heap = xa_load(&(*client).dev_heap_xa, 0);
    if heap.is_null() { XDNA_ERR!(xdna, "The client dev heap object not exist"); mutex_unlock(&mut (*client).mm_lock); kfree(priv_); return -ENOENT; }
    drm_gem_object_get(to_gobj(heap)); mutex_unlock(&mut (*client).mm_lock); (*priv_).heap = heap;
    sema_init(&mut (*priv_).job_sem, HWCTX_MAX_CMDS);
    let mut ret = amdxdna_gem_pin(heap); if ret != 0 { drm_gem_object_put(to_gobj(heap)); kfree(priv_); return ret; }
    ret = drm_sched_init(&mut (*priv_).sched, &sched_ops); if ret != 0 { amdxdna_gem_unpin(heap); drm_gem_object_put(to_gobj(heap)); kfree(priv_); return ret; }
    ret = aie2_hwctx_col_list(hwctx); if ret != 0 { drm_sched_fini(&mut (*priv_).sched); amdxdna_gem_unpin(heap); drm_gem_object_put(to_gobj(heap)); kfree(priv_); return ret; }
    ret = amdxdna_pm_resume_get_locked(xdna); if ret != 0 { kfree((*hwctx).col_list); drm_sched_fini(&mut (*priv_).sched); amdxdna_gem_unpin(heap); drm_gem_object_put(to_gobj(heap)); kfree(priv_); return ret; }
    ret = aie2_alloc_resource(hwctx); if ret == 0 { ret = aie2_map_host_buf((*xdna).dev_handle, (*hwctx).fw_ctx_id, amdxdna_obj_dma_addr(heap), (*heap).mem.size); }
    if ret == 0 { ret = amdxdna_update_heap(client, hwctx); } if ret == 0 { ret = aie2_ctx_syncobj_create(hwctx); }
    amdxdna_pm_suspend_put(xdna); if ret != 0 { aie2_release_resource(hwctx); kfree((*hwctx).col_list); drm_sched_fini(&mut (*priv_).sched); amdxdna_gem_unpin(heap); drm_gem_object_put(to_gobj(heap)); kfree(priv_); }
    ret
}

pub unsafe fn aie2_hwctx_fini(hwctx: *mut amdxdna_hwctx) { let x=(*(*hwctx).client).xdna; aie2_hwctx_wait_for_idle(hwctx); drm_sched_stop(&mut (*(*hwctx).priv_).sched, core::ptr::null_mut()); aie2_release_resource(hwctx); drm_sched_start(&mut (*(*hwctx).priv_).sched,0); drm_sched_fini(&mut (*(*hwctx).priv_).sched); aie2_ctx_syncobj_destroy(hwctx); kfree((*hwctx).col_list); kfree((*hwctx).priv_); kfree((*hwctx).cus); }
pub unsafe fn aie2_hwctx_config(hwctx:*mut amdxdna_hwctx, typ:u32, value:u64, buf:*mut core::ffi::c_void, size:u32)->i32 { match typ { DRM_AMDXDNA_HWCTX_CONFIG_CU => aie2_hwctx_cu_config(hwctx,buf,size), DRM_AMDXDNA_HWCTX_ASSIGN_DBG_BUF => aie2_hwctx_cfg_debug_bo(hwctx,value as u32,true), DRM_AMDXDNA_HWCTX_REMOVE_DBG_BUF => aie2_hwctx_cfg_debug_bo(hwctx,value as u32,false), _ => -EOPNOTSUPP } }
pub unsafe fn aie2_hwctx_sync_debug_bo(_hwctx:*mut amdxdna_hwctx,_hdl:u32)->i32 { -ENOSYS }
pub unsafe fn aie2_hmm_invalidate(abo:*mut amdxdna_gem_obj,_cur_seq:usize) { let g=to_gobj(abo); dma_resv_wait_timeout((*g).resv,DMA_RESV_USAGE_BOOKKEEP,true,MAX_SCHEDULE_TIMEOUT); }
pub unsafe fn aie2_hwctx_heap_expand(hwctx:*mut amdxdna_hwctx,heap:*mut amdxdna_gem_obj)->i32 { aie2_add_host_buf((*(*(*hwctx).client).xdna).dev_handle,(*hwctx).fw_ctx_id,amdxdna_obj_dma_addr(heap),(*heap).mem.size) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
