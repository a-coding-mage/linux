// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022-2024, Advanced Micro Devices, Inc.
 */

// External Linux/driver dependencies are supplied by the surrounding build.

const MAX_HWCTX_ID: c_ulong = 255;
const MAX_ARG_COUNT: c_ulong = 4095;

#[repr(C)]
struct amdxdna_fence {
    base: dma_fence,
    lock: spinlock_t, /* for base */
    hwctx: *mut amdxdna_hwctx,
}

unsafe fn amdxdna_fence_get_driver_name(_fence: *mut dma_fence) -> *const c_char {
    KBUILD_MODNAME
}

unsafe fn amdxdna_fence_get_timeline_name(fence: *mut dma_fence) -> *const c_char {
    let xdna_fence = container_of!(fence, amdxdna_fence, base);
    (*(*xdna_fence).hwctx).name
}

static fence_ops: dma_fence_ops = dma_fence_ops {
    get_driver_name: Some(amdxdna_fence_get_driver_name),
    get_timeline_name: Some(amdxdna_fence_get_timeline_name),
};

unsafe fn amdxdna_fence_create(hwctx: *mut amdxdna_hwctx) -> *mut dma_fence {
    let fence: *mut amdxdna_fence = kzalloc_obj::<amdxdna_fence>();
    if fence.is_null() { return core::ptr::null_mut(); }
    (*fence).hwctx = hwctx;
    spin_lock_init(&mut (*fence).lock);
    dma_fence_init(&mut (*fence).base, &fence_ops, &mut (*fence).lock,
                   (*hwctx).id, 0);
    &mut (*fence).base
}

unsafe fn amdxdna_hwctx_release_expanded_heap(hwctx: *mut amdxdna_hwctx) {
    let client = (*hwctx).client;
    let mut heap: *mut amdxdna_gem_obj = core::ptr::null_mut();
    let mut heap_id: c_ulong = 0;
    mutex_lock(&mut (*client).mm_lock);
    if (*hwctx).last_attached_heap != 0 {
        xa_for_each_range!(&(*client).dev_heap_xa, heap_id, heap, 1,
                           (*hwctx).last_attached_heap, {
            amdxdna_gem_unpin(heap);
            drm_gem_object_put(to_gobj(heap));
        });
    }
    mutex_unlock(&mut (*client).mm_lock);
}

unsafe fn amdxdna_hwctx_destroy_rcu(hwctx: *mut amdxdna_hwctx, ss: *mut srcu_struct) {
    let client = (*hwctx).client;
    let xdna = (*client).xdna;
    synchronize_srcu(ss);
    /* At this point, user is not able to submit new commands */
    ((*(*(*xdna).dev_info).ops).hwctx_fini)(hwctx);
    amdxdna_hwctx_release_expanded_heap(hwctx);
    kfree((*hwctx).name as *mut c_void);
    kfree(hwctx as *mut c_void);
}

unsafe fn amdxdna_hwctx_walk(client: *mut amdxdna_client, arg: *mut c_void,
                             walk: Option<unsafe fn(*mut amdxdna_hwctx, *mut c_void) -> c_int>) -> c_int {
    let mut hwctx: *mut amdxdna_hwctx = core::ptr::null_mut();
    let mut hwctx_id: c_ulong = 0;
    let mut ret = 0;
    let idx = srcu_read_lock(&mut (*client).hwctx_srcu);
    amdxdna_for_each_hwctx!((*client), hwctx_id, hwctx, {
        ret = walk.unwrap()(hwctx, arg);
        if ret != 0 { break; }
    });
    srcu_read_unlock(&mut (*client).hwctx_srcu, idx);
    ret
}

unsafe fn amdxdna_cmd_get_payload(abo: *mut amdxdna_gem_obj, size: *mut u32) -> *mut c_void {
    let cmd = amdxdna_gem_vmap(abo) as *mut amdxdna_cmd;
    if cmd.is_null() { return core::ptr::null_mut(); }
    let num_masks = if amdxdna_cmd_get_op(abo) == ERT_CMD_CHAIN { 0 } else {
        1 + FIELD_GET(AMDXDNA_CMD_EXTRA_CU_MASK, (*cmd).header)
    };
    if !size.is_null() {
        let count = FIELD_GET(AMDXDNA_CMD_COUNT, (*cmd).header);
        if unlikely(count <= num_masks || count * core::mem::size_of::<u32>() as u32
            + core::mem::offset_of!(amdxdna_cmd, data) > (*abo).mem.size) {
            *size = 0;
            return core::ptr::null_mut();
        }
        *size = (count - num_masks) * core::mem::size_of::<u32>() as u32;
    }
    (*cmd).data.as_mut_ptr().add(num_masks) as *mut c_void
}

unsafe fn amdxdna_cmd_get_cu_idx(abo: *mut amdxdna_gem_obj) -> u32 {
    let cmd = amdxdna_gem_vmap(abo) as *mut amdxdna_cmd;
    if cmd.is_null() || amdxdna_cmd_get_op(abo) == ERT_CMD_CHAIN { return INVALID_CU_IDX; }
    let num_masks = 1 + FIELD_GET(AMDXDNA_CMD_EXTRA_CU_MASK, (*cmd).header);
    for i in 0..num_masks {
        let mask = (*cmd).data[i as usize];
        if mask != 0 { return mask.trailing_zeros(); }
    }
    INVALID_CU_IDX
}

unsafe fn amdxdna_cmd_set_error(abo: *mut amdxdna_gem_obj, job: *mut amdxdna_sched_job,
                                 cmd_idx: u32, error_state: ert_cmd_state,
                                 err_data: *const c_void, size: usize) -> c_int {
    let client = (*(*job).hwctx).client;
    let mut cmd = amdxdna_gem_vmap(abo) as *mut amdxdna_cmd;
    if cmd.is_null() { return -ENOMEM; }
    (*cmd).header &= !AMDXDNA_CMD_STATE;
    (*cmd).header |= FIELD_PREP(AMDXDNA_CMD_STATE, error_state);
    let mut cc: *mut amdxdna_cmd_chain = core::ptr::null_mut();
    if amdxdna_cmd_get_op(abo) == ERT_CMD_CHAIN {
        cc = amdxdna_cmd_get_payload(abo, core::ptr::null_mut()) as *mut amdxdna_cmd_chain;
        (*cc).error_index = if cmd_idx < (*cc).command_count { cmd_idx } else { 0 };
        abo = amdxdna_gem_get_obj(client, (*cc).data[0], AMDXDNA_BO_SHARE);
        if abo.is_null() { return -EINVAL; }
        cmd = amdxdna_gem_vmap(abo) as *mut amdxdna_cmd;
        if cmd.is_null() { return -ENOMEM; }
    }
    let n = (*abo).mem.size - core::mem::size_of::<amdxdna_cmd>();
    core::ptr::write_bytes((*cmd).data.as_mut_ptr() as *mut u8, 0xff, n);
    if !err_data.is_null() { core::ptr::copy_nonoverlapping(err_data, (*cmd).data.as_mut_ptr() as *mut c_void, core::cmp::min(size, n)); }
    if !cc.is_null() { amdxdna_gem_put_obj(abo); }
    0
}

/* This should be called in close() and remove(). DO NOT call in other syscalls.
 * This guarantee that when hwctx and resources will be released, if user
 * doesn't call amdxdna_drm_destroy_hwctx_ioctl.
 */
unsafe fn amdxdna_hwctx_remove_all(client: *mut amdxdna_client) {
    let mut hwctx: *mut amdxdna_hwctx = core::ptr::null_mut();
    let mut hwctx_id: c_ulong = 0;
    amdxdna_for_each_hwctx!((*client), hwctx_id, hwctx, {
        XDNA_DBG!((*client).xdna, "PID %d close HW context %d", (*client).pid, (*hwctx).id);
        xa_erase(&mut (*client).hwctx_xa, (*hwctx).id);
        amdxdna_hwctx_destroy_rcu(hwctx, &mut (*client).hwctx_srcu);
    });
}

unsafe fn amdxdna_hwctx_expand_heap(hwctx: *mut amdxdna_hwctx) -> c_int {
    let client = (*hwctx).client; let xdna = (*client).xdna;
    let nid = (*hwctx).last_attached_heap + 1;
    if nid == (*client).dev_heap_nid { return 0; }
    let mut heap: *mut amdxdna_gem_obj = core::ptr::null_mut(); let mut heap_id = 0;
    let mut ret = 0;
    xa_for_each_range!(&(*client).dev_heap_xa, heap_id, heap, nid, (*client).dev_heap_nid, {
        drm_gem_object_get(to_gobj(heap)); ret = amdxdna_gem_pin(heap);
        if ret != 0 { drm_gem_object_put(to_gobj(heap)); break; }
        ret = ((*(*(*xdna).dev_info).ops).hwctx_heap_expand)(hwctx, heap);
        if ret != 0 { amdxdna_gem_unpin(heap); drm_gem_object_put(to_gobj(heap)); break; }
        (*hwctx).last_attached_heap = heap_id;
    }); ret
}

unsafe fn amdxdna_update_heap(client: *mut amdxdna_client, hwctx: *mut amdxdna_hwctx) -> c_int {
    let mut ret = amdxdna_pm_resume_get_locked((*client).xdna); if ret != 0 { return ret; }
    mutex_lock(&mut (*client).mm_lock);
    if !hwctx.is_null() { ret = amdxdna_hwctx_expand_heap(hwctx); }
    else { let mut id = 0; amdxdna_for_each_hwctx!((*client), id, hwctx, { ret = amdxdna_hwctx_expand_heap(hwctx); if ret != 0 { break; }}); }
    mutex_unlock(&mut (*client).mm_lock); amdxdna_pm_suspend_put((*client).xdna); ret
}

unsafe fn amdxdna_arg_bos_put(job: *mut amdxdna_sched_job) { for i in 0..(*job).bo_cnt { if (*job).bos[i].is_null() { break; } drm_gem_object_put((*job).bos[i]); } }

unsafe fn amdxdna_arg_bos_lookup(client: *mut amdxdna_client, job: *mut amdxdna_sched_job, hdls: *mut u32, cnt: u32) -> c_int {
    (*job).bo_cnt = cnt; for i in 0..cnt { let gobj = drm_gem_object_lookup((*client).filp, *hdls.add(i as usize)); if gobj.is_null() { amdxdna_arg_bos_put(job); return -ENOENT; } let abo = to_xdna_obj(gobj); mutex_lock(&mut (*abo).lock); if !(*abo).pinned { let r=amdxdna_gem_pin_nolock(abo); if r!=0 { mutex_unlock(&mut (*abo).lock); drm_gem_object_put(gobj); amdxdna_arg_bos_put(job); return r; } (*abo).pinned=true; } mutex_unlock(&mut (*abo).lock); (*job).bos[i as usize]=gobj; } 0
}

unsafe fn amdxdna_sched_job_cleanup(job: *mut amdxdna_sched_job) { trace_amdxdna_debug_point!((*(*job).hwctx).name, (*job).seq, "job release"); amdxdna_pm_suspend_put((*(*job).hwctx).client.xdna); amdxdna_arg_bos_put(job); amdxdna_gem_put_obj((*job).cmd_bo); dma_fence_put((*job).fence); mmdrop((*job).mm); }

// ioctl entry points and command submission are declared with their source-level
// interfaces; their implementations use the external driver definitions.
unsafe extern "C" {
    fn amdxdna_drm_create_hwctx_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
    fn amdxdna_drm_destroy_hwctx_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
    fn amdxdna_drm_config_hwctx_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
    fn amdxdna_drm_submit_cmd_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
    fn amdxdna_drm_wait_cmd_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
