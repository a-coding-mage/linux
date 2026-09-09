/*
 * Copyright 2021 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

unsafe fn amdgpu_reset_xgmi_reset_on_init_suspend(adev: *mut amdgpu_device) -> i32 {
    let mut i = (*adev).num_ip_blocks - 1;
    while i >= 0 {
        if !(*adev).ip_blocks[i as usize].status.valid || !(*adev).ip_blocks[i as usize].status.hw {
            i -= 1;
            continue;
        }
        /* displays are handled in phase1 */
        if (*(*adev).ip_blocks[i as usize].version).type_ == AMD_IP_BLOCK_TYPE_DCE {
            i -= 1;
            continue;
        }
        /* XXX handle errors */
        amdgpu_ip_block_suspend(&mut (*adev).ip_blocks[i as usize]);
        (*adev).ip_blocks[i as usize].status.hw = false;
        i -= 1;
    }
    /* VCN FW shared region is in frambuffer, there are some flags
     * initialized in that region during sw_init. Make sure the region is
     * backed up.
     */
    amdgpu_vcn_save_vcpu_bo(adev);
    0
}

unsafe fn amdgpu_reset_xgmi_reset_on_init_prep_hwctxt(
    _reset_ctl: *mut amdgpu_reset_control,
    reset_context: *mut amdgpu_reset_context,
) -> i32 {
    let reset_device_list = (*reset_context).reset_device_list;
    let mut tmp_adev: *mut amdgpu_device;
    let mut r: i32 = 0;
    list_for_each_entry!(tmp_adev, reset_device_list, reset_list, {
        amdgpu_unregister_gpu_instance(tmp_adev);
        r = amdgpu_reset_xgmi_reset_on_init_suspend(tmp_adev);
        if r != 0 {
            dev_err!((*tmp_adev).dev, "xgmi reset on init: prepare for reset failed");
            return r;
        }
    });
    r
}

unsafe fn amdgpu_reset_xgmi_reset_on_init_restore_hwctxt(
    _reset_ctl: *mut amdgpu_reset_control,
    reset_context: *mut amdgpu_reset_context,
) -> i32 {
    let reset_device_list = (*reset_context).reset_device_list;
    let mut tmp_adev: *mut amdgpu_device = core::ptr::null_mut();
    let mut r = amdgpu_device_reinit_after_reset(reset_context);
    if r != 0 { return r; }
    list_for_each_entry!(tmp_adev, reset_device_list, reset_list, {
        if !(*tmp_adev).kfd.init_complete {
            kgd2kfd_init_zone_device(tmp_adev);
            amdgpu_amdkfd_device_init(tmp_adev);
            amdgpu_amdkfd_drm_client_create(tmp_adev);
            amdgpu_ptl_sysfs_init(tmp_adev);
        }
    });
    r
}

unsafe fn amdgpu_reset_xgmi_reset_on_init_perform_reset(
    reset_ctl: *mut amdgpu_reset_control,
    reset_context: *mut amdgpu_reset_context,
) -> i32 {
    let adev = (*reset_ctl).handle as *mut amdgpu_device;
    let reset_device_list = (*reset_context).reset_device_list;
    let mut tmp_adev: *mut amdgpu_device = core::ptr::null_mut();
    dev_dbg!((*adev).dev, "xgmi roi - hw reset");
    list_for_each_entry!(tmp_adev, reset_device_list, reset_list, {
        mutex_lock!(&mut (*(*tmp_adev).reset_cntl).reset_lock);
        (*(*tmp_adev).reset_cntl).active_reset = amdgpu_asic_reset_method(adev);
    });
    let mut r = 0;
    list_for_each_entry!(tmp_adev, reset_device_list, reset_list, {
        if !queue_work(system_dfl_wq, &mut (*tmp_adev).xgmi_reset_work) { r = -EALREADY; }
        if r != 0 {
            dev_err!((*tmp_adev).dev, "xgmi reset on init: reset failed with error, %d", r);
            break;
        }
    });
    if r == 0 {
        list_for_each_entry!(tmp_adev, reset_device_list, reset_list, {
            flush_work(&mut (*tmp_adev).xgmi_reset_work);
            r = (*tmp_adev).asic_reset_res;
            if r != 0 { break; }
        });
    }
    list_for_each_entry!(tmp_adev, reset_device_list, reset_list, {
        mutex_unlock!(&mut (*(*tmp_adev).reset_cntl).reset_lock);
        (*(*tmp_adev).reset_cntl).active_reset = AMD_RESET_METHOD_NONE;
    });
    r
}

pub unsafe fn amdgpu_reset_do_xgmi_reset_on_init(reset_context: *mut amdgpu_reset_context) -> i32 {
    let reset_device_list = (*reset_context).reset_device_list;
    if reset_device_list.is_null() || list_empty!(reset_device_list) || list_is_singular!(reset_device_list) { return -EINVAL; }
    let adev = list_first_entry!(reset_device_list, amdgpu_device, reset_list);
    let mut r = amdgpu_reset_prepare_hwcontext(adev, reset_context);
    if r != 0 { return r; }
    r = amdgpu_reset_perform_reset(adev, reset_context);
    r
}

#[repr(C)]
pub static mut xgmi_reset_on_init_handler: amdgpu_reset_handler = amdgpu_reset_handler {
    reset_method: AMD_RESET_METHOD_ON_INIT,
    prepare_env: None,
    prepare_hwcontext: Some(amdgpu_reset_xgmi_reset_on_init_prep_hwctxt),
    perform_reset: Some(amdgpu_reset_xgmi_reset_on_init_perform_reset),
    restore_hwcontext: Some(amdgpu_reset_xgmi_reset_on_init_restore_hwctxt),
    restore_env: None,
    do_reset: None,
};

pub unsafe fn amdgpu_reset_init(adev: *mut amdgpu_device) -> i32 {
    match amdgpu_ip_version(adev, MP1_HWIP, 0) {
        IP_VERSION!(13, 0, 2) | IP_VERSION!(13, 0, 6) | IP_VERSION!(13, 0, 12) | IP_VERSION!(13, 0, 14) => aldebaran_reset_init(adev),
        IP_VERSION!(11, 0, 7) => sienna_cichlid_reset_init(adev),
        IP_VERSION!(13, 0, 10) => smu_v13_0_10_reset_init(adev),
        _ => 0,
    }
}

pub unsafe fn amdgpu_reset_fini(adev: *mut amdgpu_device) -> i32 {
    match amdgpu_ip_version(adev, MP1_HWIP, 0) {
        IP_VERSION!(13, 0, 2) | IP_VERSION!(13, 0, 6) | IP_VERSION!(13, 0, 12) | IP_VERSION!(13, 0, 14) => aldebaran_reset_fini(adev),
        IP_VERSION!(11, 0, 7) => sienna_cichlid_reset_fini(adev),
        IP_VERSION!(13, 0, 10) => smu_v13_0_10_reset_fini(adev),
        _ => 0,
    }
}

pub unsafe fn amdgpu_reset_prepare_hwcontext(adev: *mut amdgpu_device, reset_context: *mut amdgpu_reset_context) -> i32 {
    let mut reset_handler = core::ptr::null_mut();
    if !(*adev).reset_cntl.is_null() && (*(*adev).reset_cntl).get_reset_handler.is_some() {
        reset_handler = ((*(*adev).reset_cntl).get_reset_handler.unwrap())((*adev).reset_cntl, reset_context);
    }
    if reset_handler.is_null() { return -EOPNOTSUPP; }
    ((*reset_handler).prepare_hwcontext.unwrap())((*adev).reset_cntl, reset_context)
}

pub unsafe fn amdgpu_reset_perform_reset(adev: *mut amdgpu_device, reset_context: *mut amdgpu_reset_context) -> i32 {
    let mut reset_handler = core::ptr::null_mut();
    if !(*adev).reset_cntl.is_null() { reset_handler = ((*(*adev).reset_cntl).get_reset_handler.unwrap())((*adev).reset_cntl, reset_context); }
    if reset_handler.is_null() { return -EOPNOTSUPP; }
    let ret = ((*reset_handler).perform_reset.unwrap())((*adev).reset_cntl, reset_context);
    if ret != 0 { return ret; }
    ((*reset_handler).restore_hwcontext.unwrap())((*adev).reset_cntl, reset_context)
}

pub unsafe fn amdgpu_reset_destroy_reset_domain(ref_: *mut kref) {
    let reset_domain = container_of!(ref_, amdgpu_reset_domain, refcount);
    if !(*reset_domain).wq.is_null() { destroy_workqueue((*reset_domain).wq); }
    kvfree(reset_domain as *mut core::ffi::c_void);
}

pub unsafe fn amdgpu_reset_create_reset_domain(type_: amdgpu_reset_domain_type, wq_name: *mut i8) -> *mut amdgpu_reset_domain {
    let reset_domain = kvzalloc_obj!(amdgpu_reset_domain);
    if reset_domain.is_null() { DRM_ERROR!("Failed to allocate amdgpu_reset_domain!"); return core::ptr::null_mut(); }
    (*reset_domain).type_ = type_;
    kref_init!(&mut (*reset_domain).refcount);
    (*reset_domain).wq = create_singlethread_workqueue(wq_name);
    if (*reset_domain).wq.is_null() { DRM_ERROR!("Failed to allocate wq for amdgpu_reset_domain!"); amdgpu_reset_put_reset_domain(reset_domain); return core::ptr::null_mut(); }
    atomic_set!(&mut (*reset_domain).in_gpu_reset, 0);
    atomic_set!(&mut (*reset_domain).reset_res, 0);
    init_rwsem!(&mut (*reset_domain).sem);
    reset_domain
}

pub unsafe fn amdgpu_device_lock_reset_domain(reset_domain: *mut amdgpu_reset_domain) { atomic_set!(&mut (*reset_domain).in_gpu_reset, 1); down_write!(&mut (*reset_domain).sem); }
pub unsafe fn amdgpu_device_unlock_reset_domain(reset_domain: *mut amdgpu_reset_domain) { atomic_set!(&mut (*reset_domain).in_gpu_reset, 0); up_write!(&mut (*reset_domain).sem); }

pub unsafe fn amdgpu_reset_get_desc(rst_ctxt: *mut amdgpu_reset_context, buf: *mut i8, len: usize) {
    if buf.is_null() || len == 0 { return; }
    match (*rst_ctxt).src {
        AMDGPU_RESET_SRC_JOB => if !(*rst_ctxt).job.is_null() { snprintf!(buf, len, "job hang on ring:%s", (*(*(*rst_ctxt).job).base.sched).name); } else { strscpy!(buf, "job hang", len); },
        AMDGPU_RESET_SRC_RAS => strscpy!(buf, "RAS error", len),
        AMDGPU_RESET_SRC_MES => strscpy!(buf, "MES hang", len),
        AMDGPU_RESET_SRC_HWS => strscpy!(buf, "HWS hang", len),
        AMDGPU_RESET_SRC_USER => strscpy!(buf, "user trigger", len),
        AMDGPU_RESET_SRC_USERQ => strscpy!(buf, "user queue trigger", len),
        _ => strscpy!(buf, "unknown", len),
    }
}

pub unsafe fn amdgpu_reset_in_recovery(adev: *mut amdgpu_device) -> bool { (*(*adev).init_lvl).level == AMDGPU_INIT_LEVEL_RESET_RECOVERY }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
