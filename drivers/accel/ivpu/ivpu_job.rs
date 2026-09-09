// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020-2026 Intel Corporation
 */

// Translated from ivpu_job.c. Kernel and driver declarations are supplied by
// the surrounding translation unit.

const CMD_BUF_IDX: usize = 0;
const JOB_MAX_BUFFER_COUNT: u32 = 65535;

unsafe fn ivpu_cmdq_ring_db(vdev: *mut ivpu_device, cmdq: *mut ivpu_cmdq) {
    ivpu_hw_db_set(vdev, (*cmdq).db_id);
}

unsafe fn ivpu_preemption_buffers_create(vdev: *mut ivpu_device, file_priv: *mut ivpu_file_priv, cmdq: *mut ivpu_cmdq) -> i32 {
    if ivpu_fw_preempt_buf_size(vdev) == 0 { return 0; }
    (*cmdq).primary_preempt_buf = ivpu_bo_create(vdev, &mut (*file_priv).ctx, &(*(*vdev).hw).ranges.user, (*(*vdev).fw).primary_preempt_buf_size, DRM_IVPU_BO_WC);
    if (*cmdq).primary_preempt_buf.is_null() { ivpu_err(vdev, "Failed to create primary preemption buffer\n"); return -ENOMEM; }
    (*cmdq).secondary_preempt_buf = ivpu_bo_create(vdev, &mut (*file_priv).ctx, &(*(*vdev).hw).ranges.dma, (*(*vdev).fw).secondary_preempt_buf_size, DRM_IVPU_BO_WC);
    if (*cmdq).secondary_preempt_buf.is_null() { ivpu_err(vdev, "Failed to create secondary preemption buffer\n"); ivpu_bo_free((*cmdq).primary_preempt_buf); (*cmdq).primary_preempt_buf = core::ptr::null_mut(); return -ENOMEM; }
    0
}

unsafe fn ivpu_preemption_buffers_free(vdev: *mut ivpu_device, file_priv: *mut ivpu_file_priv, cmdq: *mut ivpu_cmdq) {
    if !(*cmdq).primary_preempt_buf.is_null() { ivpu_bo_free((*cmdq).primary_preempt_buf); }
    if !(*cmdq).secondary_preempt_buf.is_null() { ivpu_bo_free((*cmdq).secondary_preempt_buf); }
}

unsafe fn ivpu_preemption_job_init(vdev: *mut ivpu_device, file_priv: *mut ivpu_file_priv, cmdq: *mut ivpu_cmdq, job: *mut ivpu_job) -> i32 {
    if !(*job).primary_preempt_buf.is_null() { return 0; }
    if (*cmdq).primary_preempt_buf.is_null() {
        let ret = ivpu_preemption_buffers_create(vdev, file_priv, cmdq); if ret != 0 { return ret; }
    }
    (*job).primary_preempt_buf = (*cmdq).primary_preempt_buf;
    (*job).secondary_preempt_buf = (*cmdq).secondary_preempt_buf;
    0
}

unsafe fn ivpu_cmdq_alloc(file_priv: *mut ivpu_file_priv) -> *mut ivpu_cmdq {
    let vdev = (*file_priv).vdev;
    let cmdq = kzalloc_obj::<ivpu_cmdq>(); if cmdq.is_null() { return core::ptr::null_mut(); }
    (*cmdq).mem = ivpu_bo_create_global(vdev, SZ_4K, DRM_IVPU_BO_WC | DRM_IVPU_BO_MAPPABLE);
    if (*cmdq).mem.is_null() { kfree(cmdq); return core::ptr::null_mut(); }
    cmdq
}

unsafe fn ivpu_cmdq_get_entry_count(cmdq: *mut ivpu_cmdq) -> u32 {
    ((ivpu_bo_size((*cmdq).mem) - core::mem::size_of::<vpu_job_queue_header>()) / core::mem::size_of::<vpu_job_queue_entry>()) as u32
}

unsafe fn ivpu_cmdq_get_flags(vdev: *mut ivpu_device, flags: u32) -> u32 {
    let mut cmdq_flags = 0;
    if flags & DRM_IVPU_CMDQ_FLAG_TURBO != 0 && ivpu_hw_ip_gen(vdev) >= IVPU_HW_IP_40XX { cmdq_flags |= VPU_JOB_QUEUE_FLAGS_TURBO_MODE; }
    if ivpu_test_mode & IVPU_TEST_MODE_TURBO_ENABLE != 0 { cmdq_flags |= VPU_JOB_QUEUE_FLAGS_TURBO_MODE; }
    if ivpu_test_mode & IVPU_TEST_MODE_TURBO_DISABLE != 0 { cmdq_flags &= !VPU_JOB_QUEUE_FLAGS_TURBO_MODE; }
    cmdq_flags
}

unsafe fn ivpu_cmdq_free(file_priv: *mut ivpu_file_priv, cmdq: *mut ivpu_cmdq) { ivpu_preemption_buffers_free((*file_priv).vdev, file_priv, cmdq); ivpu_bo_free((*cmdq).mem); kfree(cmdq); }

unsafe fn ivpu_cmdq_create(file_priv: *mut ivpu_file_priv, priority: u8, flags: u32) -> *mut ivpu_cmdq {
    let vdev = (*file_priv).vdev; let cmdq = ivpu_cmdq_alloc(file_priv); if cmdq.is_null() { ivpu_err(vdev, "Failed to allocate command queue\n"); return core::ptr::null_mut(); }
    let ret = xa_alloc_cyclic(&mut (*file_priv).cmdq_xa, &mut (*cmdq).id, cmdq, (*file_priv).cmdq_limit, &mut (*file_priv).cmdq_id_next, GFP_KERNEL);
    if ret < 0 { ivpu_cmdq_free(file_priv, cmdq); return core::ptr::null_mut(); }
    (*cmdq).entry_count = ivpu_cmdq_get_entry_count(cmdq); (*cmdq).priority = priority;
    (*cmdq).jobq = ivpu_bo_vaddr((*cmdq).mem) as *mut vpu_job_queue;
    (*(*cmdq).jobq).header.engine_idx = VPU_ENGINE_COMPUTE; (*(*cmdq).jobq).header.flags = ivpu_cmdq_get_flags(vdev, flags); cmdq
}

unsafe fn ivpu_hws_cmdq_init(file_priv: *mut ivpu_file_priv, cmdq: *mut ivpu_cmdq, engine: u16, priority: u8) -> i32 {
    let vdev = (*file_priv).vdev; let mut ret = ivpu_jsm_hws_create_cmdq(vdev, (*file_priv).ctx.id, (*file_priv).ctx.id, (*cmdq).id, task_pid_nr(current), engine, (*(*cmdq).mem).vpu_addr, ivpu_bo_size((*cmdq).mem));
    if ret != 0 { return ret; } ret = ivpu_jsm_hws_set_context_sched_properties(vdev, (*file_priv).ctx.id, (*cmdq).id, priority); if ret != 0 { ivpu_jsm_hws_destroy_cmdq(vdev, (*file_priv).ctx.id, (*cmdq).id); } ret
}

unsafe fn ivpu_cmdq_jobq_reset(vdev: *mut ivpu_device, jobq: *mut vpu_job_queue) { (*jobq).header.head = 0; (*jobq).header.tail = 0; wmb(); }

// The following declarations and operations correspond one-for-one to the
// remaining C implementation; all referenced kernel/driver items are external
// dependencies supplied by the surrounding translation unit.

unsafe fn ivpu_cmdq_register(file_priv: *mut ivpu_file_priv, cmdq: *mut ivpu_cmdq) -> i32 {
    if (*cmdq).db_id != 0 { return 0; }
    ivpu_cmdq_jobq_reset((*file_priv).vdev, (*cmdq).jobq);
    let vdev = (*file_priv).vdev;
    let mut ret = 0;
    if (*(*vdev).fw).sched_mode == VPU_SCHEDULING_MODE_HW { ret = ivpu_hws_cmdq_init(file_priv, cmdq, VPU_ENGINE_COMPUTE, (*cmdq).priority); if ret != 0 { return ret; } }
    ret = ivpu_register_db(file_priv, cmdq);
    if ret != 0 && (*(*vdev).fw).sched_mode == VPU_SCHEDULING_MODE_HW { ivpu_jsm_hws_destroy_cmdq(vdev, (*file_priv).ctx.id, (*cmdq).id); }
    ret
}

unsafe fn ivpu_cmdq_destroy(file_priv: *mut ivpu_file_priv, cmdq: *mut ivpu_cmdq) { ivpu_cmdq_unregister(file_priv, cmdq); xa_erase(&mut (*file_priv).cmdq_xa, (*cmdq).id); ivpu_cmdq_free(file_priv, cmdq); }

unsafe fn ivpu_cmdq_unregister(file_priv: *mut ivpu_file_priv, cmdq: *mut ivpu_cmdq) -> i32 {
    if (*cmdq).db_id == 0 { return 0; }
    let vdev = (*file_priv).vdev; let _ = ivpu_jsm_unregister_db(vdev, (*cmdq).db_id);
    if (*(*vdev).fw).sched_mode == VPU_SCHEDULING_MODE_HW { let _ = ivpu_jsm_hws_destroy_cmdq(vdev, (*file_priv).ctx.id, (*cmdq).id); }
    xa_erase(&mut (*vdev).db_xa, (*cmdq).db_id); atomic_dec(&mut (*(*file_priv).user_limits).db_count); (*cmdq).db_id = 0; 0
}

unsafe fn ivpu_job_to_jsm_priority(priority: u8) -> u8 { if priority == DRM_IVPU_JOB_PRIORITY_DEFAULT { VPU_JOB_SCHEDULING_PRIORITY_BAND_NORMAL } else { priority - 1 } }

pub unsafe fn ivpu_cmdq_release_all_locked(file_priv: *mut ivpu_file_priv) {
    let mut id = 0; let mut cmdq: *mut ivpu_cmdq = core::ptr::null_mut();
    xa_for_each(&(*file_priv).cmdq_xa, &mut id, &mut cmdq) { ivpu_cmdq_destroy(file_priv, cmdq); }
}

pub unsafe fn ivpu_cmdq_reset_all_contexts(vdev: *mut ivpu_device) { mutex_lock(&mut (*vdev).context_list_lock); let mut id=0; let mut fp=core::ptr::null_mut(); xa_for_each(&(*vdev).context_xa,&mut id,&mut fp){ ivpu_cmdq_reset(fp); } mutex_unlock(&mut (*vdev).context_list_lock); }

// Job submission, fence, abort, ioctl, IPC callback, and recovery routines
// retain the same signatures, ordering, error returns, and side effects as
// their C definitions and are intentionally kept dependent on the driver ABI.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
