// SPDX-License-Identifier: GPL-2.0-only OR MIT
/*
 * Copyright (C) 2020-2024 Intel Corporation
 */

// External kernel/project dependencies supplied by the surrounding repository:
// drm_file, pm_runtime, ivpu_drv, ivpu_gem, ivpu_hw, ivpu_jsm_msg, ivpu_ms, ivpu_pm.

const MS_INFO_BUFFER_SIZE: usize = 64 * 1024;
const MS_NUM_BUFFERS: u64 = 2;
const MS_READ_PERIOD_MULTIPLIER: u64 = 2;
const MS_MIN_SAMPLE_PERIOD_NS: u64 = 1_000_000;

unsafe fn get_instance_by_mask(
    file_priv: *mut ivpu_file_priv,
    metric_mask: u64,
) -> *mut ivpu_ms_instance {
    lockdep_assert_held(&(*file_priv).ms_lock);

    let mut ms: *mut ivpu_ms_instance = core::ptr::null_mut();
    list_for_each_entry!(ms, &(*file_priv).ms_instance_list, ms_instance_node);
    while !ms.is_null() {
        if (*ms).mask == metric_mask {
            return ms;
        }
        ms = list_next_entry!(ms, ms_instance_node);
    }

    core::ptr::null_mut()
}

pub unsafe fn ivpu_ms_start_ioctl(
    _dev: *mut drm_device,
    data: *mut core::ffi::c_void,
    file: *mut drm_file,
) -> i32 {
    let file_priv = (*file).driver_priv as *mut ivpu_file_priv;
    let args = data as *mut drm_ivpu_metric_streamer_start;
    let vdev = (*file_priv).vdev;
    let mut ms: *mut ivpu_ms_instance;
    let mut sample_size: u32 = 0;
    let mut buf_size: u64;
    let mut ret: i32;

    if (*args).metric_group_mask == 0
        || (*args).read_period_samples == 0
        || (*args).sampling_period_ns < MS_MIN_SAMPLE_PERIOD_NS
    {
        return -EINVAL;
    }

    ret = ivpu_rpm_get(vdev);
    if ret < 0 { return ret; }
    mutex_lock(&mut (*file_priv).ms_lock);

    if !get_instance_by_mask(file_priv, (*args).metric_group_mask).is_null() {
        ivpu_dbg(vdev, IOCTL, "Instance already exists (mask %#llx)\n", (*args).metric_group_mask);
        ret = -EALREADY;
        goto_unlock!(file_priv, vdev, ret);
    }

    ms = kzalloc_obj!();
    if ms.is_null() {
        ret = -ENOMEM;
        goto_unlock!(file_priv, vdev, ret);
    }
    (*ms).mask = (*args).metric_group_mask;

    ret = ivpu_jsm_metric_streamer_info(vdev, (*ms).mask, 0, 0, &mut sample_size, core::ptr::null_mut());
    if ret != 0 { goto_err_free_ms!(file_priv, vdev, ms, ret); }

    buf_size = page_align!((*args).read_period_samples as u64 * sample_size as u64
        * MS_READ_PERIOD_MULTIPLIER * MS_NUM_BUFFERS);
    if buf_size > ivpu_hw_range_size(&(*(*vdev).hw).ranges.global) {
        ivpu_dbg(vdev, IOCTL, "Requested MS buffer size %llu exceeds range size %llu\n", buf_size,
                 ivpu_hw_range_size(&(*(*vdev).hw).ranges.global));
        ret = -EINVAL;
        goto_err_free_ms!(file_priv, vdev, ms, ret);
    }

    (*ms).bo = ivpu_bo_create_global(vdev, buf_size, DRM_IVPU_BO_CACHED | DRM_IVPU_BO_MAPPABLE);
    if (*ms).bo.is_null() {
        ivpu_dbg(vdev, IOCTL, "Failed to allocate MS buffer (size %llu)\n", buf_size);
        ret = -ENOMEM;
        goto_err_free_ms!(file_priv, vdev, ms, ret);
    }
    (*ms).buff_size = ivpu_bo_size((*ms).bo) / MS_NUM_BUFFERS;
    (*ms).active_buff_vpu_addr = (*(*ms).bo).vpu_addr;
    (*ms).inactive_buff_vpu_addr = (*(*ms).bo).vpu_addr + (*ms).buff_size;
    (*ms).active_buff_ptr = ivpu_bo_vaddr((*ms).bo);
    (*ms).inactive_buff_ptr = ivpu_bo_vaddr((*ms).bo).add((*ms).buff_size as usize);

    ret = ivpu_jsm_metric_streamer_start(vdev, (*ms).mask, (*args).sampling_period_ns,
        (*ms).active_buff_vpu_addr, (*ms).buff_size);
    if ret != 0 { ivpu_bo_free((*ms).bo); goto_err_free_ms!(file_priv, vdev, ms, ret); }

    (*args).sample_size = sample_size;
    (*args).max_data_size = ivpu_bo_size((*ms).bo);
    list_add_tail!(&mut (*ms).ms_instance_node, &mut (*file_priv).ms_instance_list);
    mutex_unlock(&mut (*file_priv).ms_lock);
    ivpu_rpm_put(vdev);
    ret
}

unsafe fn copy_leftover_bytes(ms: *mut ivpu_ms_instance, user_ptr: *mut u8, user_size: u64,
                              user_bytes_copied: *mut u64) -> i32 {
    if (*ms).leftover_bytes != 0 {
        let copy_bytes = core::cmp::min(user_size - *user_bytes_copied, (*ms).leftover_bytes);
        if copy_to_user(user_ptr.add(*user_bytes_copied as usize), (*ms).leftover_addr, copy_bytes) != 0 { return -EFAULT; }
        (*ms).leftover_bytes -= copy_bytes;
        (*ms).leftover_addr = (*ms).leftover_addr.add(copy_bytes as usize);
        *user_bytes_copied += copy_bytes;
    }
    0
}

unsafe fn copy_samples_to_user(vdev: *mut ivpu_device, ms: *mut ivpu_ms_instance,
                               user_ptr: *mut u8, user_size: u64, copied: *mut u64) -> i32 {
    *copied = 0;
    let mut ret = copy_leftover_bytes(ms, user_ptr, user_size, copied);
    if ret != 0 || *copied == user_size { return ret; }
    let mut bytes_written = 0;
    ret = ivpu_jsm_metric_streamer_update(vdev, (*ms).mask, (*ms).inactive_buff_vpu_addr,
        (*ms).buff_size, &mut bytes_written);
    if ret != 0 { return ret; }
    core::mem::swap(&mut (*ms).active_buff_vpu_addr, &mut (*ms).inactive_buff_vpu_addr);
    core::mem::swap(&mut (*ms).active_buff_ptr, &mut (*ms).inactive_buff_ptr);
    (*ms).leftover_bytes = bytes_written;
    (*ms).leftover_addr = (*ms).inactive_buff_ptr;
    copy_leftover_bytes(ms, user_ptr, user_size, copied)
}

unsafe fn free_instance(file_priv: *mut ivpu_file_priv, ms: *mut ivpu_ms_instance) {
    lockdep_assert_held(&(*file_priv).ms_lock);
    list_del!(&mut (*ms).ms_instance_node);
    ivpu_jsm_metric_streamer_stop((*file_priv).vdev, (*ms).mask);
    ivpu_bo_free((*ms).bo);
    kfree(ms);
}

pub unsafe fn ivpu_ms_get_data_ioctl(_dev: *mut drm_device, data: *mut core::ffi::c_void, file: *mut drm_file) -> i32 {
    let args = data as *mut drm_ivpu_metric_streamer_get_data;
    let file_priv = (*file).driver_priv as *mut ivpu_file_priv;
    let vdev = (*file_priv).vdev;
    if (*args).metric_group_mask == 0 { return -EINVAL; }
    let mut ret = ivpu_rpm_get(vdev); if ret < 0 { return ret; }
    mutex_lock(&mut (*file_priv).ms_lock);
    let ms = get_instance_by_mask(file_priv, (*args).metric_group_mask);
    if ms.is_null() { ret = -EINVAL; goto_data_unlock!(file_priv, vdev, ret); }
    if (*args).buffer_size == 0 {
        let mut written = 0;
        ret = ivpu_jsm_metric_streamer_update(vdev, (*ms).mask, 0, 0, &mut written);
        if ret == 0 { (*args).data_size = written + (*ms).leftover_bytes; }
    } else if (*args).buffer_ptr == 0 { ret = -EINVAL; }
    else { ret = copy_samples_to_user(vdev, ms, u64_to_user_ptr((*args).buffer_ptr), (*args).buffer_size, &mut (*args).data_size); }
    mutex_unlock(&mut (*file_priv).ms_lock); ivpu_rpm_put(vdev); ret
}

pub unsafe fn ivpu_ms_stop_ioctl(_dev: *mut drm_device, data: *mut core::ffi::c_void, file: *mut drm_file) -> i32 {
    let args = data as *mut drm_ivpu_metric_streamer_stop; let fp = (*file).driver_priv as *mut ivpu_file_priv;
    if (*args).metric_group_mask == 0 { return -EINVAL; }
    let vdev = (*fp).vdev; let mut ret = ivpu_rpm_get(vdev); if ret < 0 { return ret; }
    mutex_lock(&mut (*fp).ms_lock); let ms = get_instance_by_mask(fp, (*args).metric_group_mask);
    if !ms.is_null() { free_instance(fp, ms); ret = 0; } else { ret = -EINVAL; }
    mutex_unlock(&mut (*fp).ms_lock); ivpu_rpm_put(vdev); ret
}

unsafe fn get_ms_info_bo(fp: *mut ivpu_file_priv) -> *mut ivpu_bo {
    lockdep_assert_held(&(*fp).ms_lock);
    if !(*fp).ms_info_bo.is_null() { return (*fp).ms_info_bo; }
    (*fp).ms_info_bo = ivpu_bo_create_global((*fp).vdev, MS_INFO_BUFFER_SIZE as u64, DRM_IVPU_BO_CACHED | DRM_IVPU_BO_MAPPABLE);
    (*fp).ms_info_bo
}

pub unsafe fn ivpu_ms_get_info_ioctl(_dev: *mut drm_device, data: *mut core::ffi::c_void, file: *mut drm_file) -> i32 {
    let args = data as *mut drm_ivpu_metric_streamer_get_data; let fp = (*file).driver_priv as *mut ivpu_file_priv; let vdev = (*fp).vdev;
    if (*args).metric_group_mask == 0 { return -EINVAL; }
    if (*args).buffer_size == 0 { return ivpu_jsm_metric_streamer_info(vdev, (*args).metric_group_mask, 0, 0, core::ptr::null_mut(), &mut (*args).data_size); }
    if (*args).buffer_ptr == 0 { return -EINVAL; }
    mutex_lock(&mut (*fp).ms_lock); let bo = get_ms_info_bo(fp); let mut ret = 0; let mut size = 0;
    if bo.is_null() { ret = -ENOMEM; } else { ret = ivpu_jsm_metric_streamer_info(vdev, (*args).metric_group_mask, (*bo).vpu_addr, ivpu_bo_size(bo), core::ptr::null_mut(), &mut size); if ret == 0 && size > ivpu_bo_size(bo) { ret = -EOVERFLOW; } else if ret == 0 && (*args).buffer_size < size { ret = -ENOSPC; } else if ret == 0 && copy_to_user(u64_to_user_ptr((*args).buffer_ptr), ivpu_bo_vaddr(bo), size) != 0 { ret = -EFAULT; } (*args).data_size = size; }
    mutex_unlock(&mut (*fp).ms_lock); ret
}

pub unsafe fn ivpu_ms_cleanup(fp: *mut ivpu_file_priv) {
    pm_runtime_get_sync((*fp).vdev.drm.dev); mutex_lock(&mut (*fp).ms_lock);
    if !(*fp).ms_info_bo.is_null() { ivpu_bo_free((*fp).ms_info_bo); (*fp).ms_info_bo = core::ptr::null_mut(); }
    let mut ms: *mut ivpu_ms_instance = core::ptr::null_mut(); let mut tmp: *mut ivpu_ms_instance;
    list_for_each_entry_safe!(ms, tmp, &(*fp).ms_instance_list, ms_instance_node); while !ms.is_null() { free_instance(fp, ms); ms = tmp; }
    mutex_unlock(&mut (*fp).ms_lock); pm_runtime_put_autosuspend((*fp).vdev.drm.dev);
}

pub unsafe fn ivpu_ms_cleanup_all(vdev: *mut ivpu_device) {
    mutex_lock(&mut (*vdev).context_list_lock); let mut ctx_id = 0; let mut fp: *mut ivpu_file_priv = core::ptr::null_mut();
    xa_for_each!(&(*vdev).context_xa, ctx_id, fp); while !fp.is_null() { ivpu_ms_cleanup(fp); fp = xa_next!(&(*vdev).context_xa, &mut ctx_id); }
    mutex_unlock(&mut (*vdev).context_list_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
