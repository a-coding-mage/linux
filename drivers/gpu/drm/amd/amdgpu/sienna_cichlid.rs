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
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 */

// External declarations and included definitions are supplied by the surrounding crate.

unsafe fn sienna_cichlid_is_mode2_default(
    _reset_ctl: *mut amdgpu_reset_control,
) -> bool {
    // The original build-time-disabled firmware/version check is preserved here.
    amdgpu_reset_method == AMD_RESET_METHOD_MODE2
}

unsafe fn sienna_cichlid_get_reset_handler(
    reset_ctl: *mut amdgpu_reset_control,
    reset_context: *mut amdgpu_reset_context,
) -> *mut amdgpu_reset_handler {
    let mut handler: *mut amdgpu_reset_handler = core::ptr::null_mut();
    let mut i: i32;

    if (*reset_context).method != AMD_RESET_METHOD_NONE {
        i = 0;
        while i < AMDGPU_RESET_MAX_HANDLERS {
            handler = *(*reset_ctl).reset_handlers.add(i as usize);
            if !handler.is_null() && (*handler).reset_method == (*reset_context).method {
                return handler;
            }
            i += 1;
        }
    }

    if sienna_cichlid_is_mode2_default(reset_ctl) {
        i = 0;
        while i < AMDGPU_RESET_MAX_HANDLERS {
            handler = *(*reset_ctl).reset_handlers.add(i as usize);
            if !handler.is_null() && (*handler).reset_method == AMD_RESET_METHOD_MODE2 {
                return handler;
            }
            i += 1;
        }
    }

    core::ptr::null_mut()
}

unsafe fn sienna_cichlid_mode2_suspend_ip(adev: *mut amdgpu_device) -> i32 {
    let mut r: i32;
    amdgpu_device_set_pg_state(adev, AMD_PG_STATE_UNGATE);
    amdgpu_device_set_cg_state(adev, AMD_CG_STATE_UNGATE);

    let mut i = (*adev).num_ip_blocks - 1;
    while i >= 0 {
        let ip = &mut *(*adev).ip_blocks.add(i as usize);
        if ip.version.type_ != AMD_IP_BLOCK_TYPE_GFX
            && ip.version.type_ != AMD_IP_BLOCK_TYPE_SDMA
        {
            i -= 1;
            continue;
        }
        r = amdgpu_ip_block_suspend(ip);
        if r != 0 {
            return r;
        }
        i -= 1;
    }
    0
}

unsafe fn sienna_cichlid_mode2_prepare_hwcontext(
    reset_ctl: *mut amdgpu_reset_control,
    _reset_context: *mut amdgpu_reset_context,
) -> i32 {
    let mut r = 0;
    let adev = (*reset_ctl).handle as *mut amdgpu_device;

    if !amdgpu_sriov_vf(adev) {
        if let Some(mode2_save_regs) = (*adev).gfxhub.funcs.mode2_save_regs {
            mode2_save_regs(adev);
        }
        if let Some(halt) = (*adev).gfxhub.funcs.halt {
            halt(adev);
        }
        r = sienna_cichlid_mode2_suspend_ip(adev);
    }
    r
}

unsafe fn sienna_cichlid_async_reset(work: *mut work_struct) {
    let reset_ctl = container_of!(work, amdgpu_reset_control, reset_work);
    let adev = (*reset_ctl).handle as *mut amdgpu_device;

    let mut i = 0;
    while i < AMDGPU_RESET_MAX_HANDLERS {
        let handler = *(*reset_ctl).reset_handlers.add(i as usize);
        if !handler.is_null() && (*handler).reset_method == (*reset_ctl).active_reset {
            dev_dbg!((*adev).dev, "Resetting device\n");
            ((*handler).do_reset.unwrap())(adev);
            break;
        }
        i += 1;
    }
}

unsafe fn sienna_cichlid_mode2_reset(adev: *mut amdgpu_device) -> i32 {
    // disable BM
    pci_clear_master((*adev).pdev);
    amdgpu_dpm_mode2_reset(adev)
}

unsafe fn sienna_cichlid_mode2_perform_reset(
    reset_ctl: *mut amdgpu_reset_control,
    _reset_context: *mut amdgpu_reset_context,
) -> i32 {
    let adev = (*reset_ctl).handle as *mut amdgpu_device;
    let r = sienna_cichlid_mode2_reset(adev);
    if r != 0 {
        dev_err!((*adev).dev, "ASIC reset failed with error, %d ", r);
    }
    r
}

unsafe fn sienna_cichlid_mode2_restore_ip(adev: *mut amdgpu_device) -> i32 {
    let psp = &mut (*adev).psp;
    let mut r = psp_rlc_autoload_start(psp);
    if r != 0 {
        dev_err!((*adev).dev, "Failed to start rlc autoload\n");
        return r;
    }

    if let Some(restore_regs) = (*adev).gfxhub.funcs.mode2_restore_regs {
        restore_regs(adev);
    }
    ((*adev).gfxhub.funcs.init.unwrap())(adev);
    r = ((*adev).gfxhub.funcs.gart_enable.unwrap())(adev);
    if r != 0 {
        dev_err!((*adev).dev, "GFXHUB gart reenable failed after reset\n");
        return r;
    }

    let mut i = 0;
    while i < (*adev).num_ip_blocks {
        let ip = &mut *(*adev).ip_blocks.add(i as usize);
        if ip.version.type_ == AMD_IP_BLOCK_TYPE_IH {
            r = amdgpu_ip_block_resume(ip);
            if r != 0 { return r; }
        }
        i += 1;
    }
    i = 0;
    while i < (*adev).num_ip_blocks {
        let ip = &mut *(*adev).ip_blocks.add(i as usize);
        if ip.version.type_ == AMD_IP_BLOCK_TYPE_GFX || ip.version.type_ == AMD_IP_BLOCK_TYPE_SDMA {
            r = amdgpu_ip_block_resume(ip);
            if r != 0 { return r; }
        }
        i += 1;
    }
    i = 0;
    while i < (*adev).num_ip_blocks {
        let ip = &mut *(*adev).ip_blocks.add(i as usize);
        if ip.version.type_ == AMD_IP_BLOCK_TYPE_GFX || ip.version.type_ == AMD_IP_BLOCK_TYPE_SDMA {
            if let Some(late_init) = ip.version.funcs.late_init {
                r = late_init(ip);
                if r != 0 {
                    dev_err!((*adev).dev, "late_init of IP block <%s> failed %d after reset\n", ip.version.funcs.name, r);
                    return r;
                }
            }
            ip.status.late_initialized = true;
        }
        i += 1;
    }
    amdgpu_device_set_cg_state(adev, AMD_CG_STATE_GATE);
    amdgpu_device_set_pg_state(adev, AMD_PG_STATE_GATE);
    r
}

unsafe fn sienna_cichlid_mode2_restore_hwcontext(
    reset_ctl: *mut amdgpu_reset_control,
    _reset_context: *mut amdgpu_reset_context,
) -> i32 {
    let tmp_adev = (*reset_ctl).handle as *mut amdgpu_device;
    amdgpu_set_init_level(tmp_adev, AMDGPU_INIT_LEVEL_RESET_RECOVERY);
    dev_info!((*tmp_adev).dev, "GPU reset succeeded, trying to resume\n");
    let mut r = sienna_cichlid_mode2_restore_ip(tmp_adev);
    if r != 0 { return -EAGAIN; }
    amdgpu_register_gpu_instance(tmp_adev);
    amdgpu_ras_resume(tmp_adev);
    amdgpu_irq_gpu_reset_resume_helper(tmp_adev);
    amdgpu_set_init_level(tmp_adev, AMDGPU_INIT_LEVEL_DEFAULT);
    r = amdgpu_ib_ring_tests(tmp_adev);
    if r != 0 {
        dev_err!((*tmp_adev).dev, "ib ring test failed (%d).\n", r);
        r = -EAGAIN;
    }
    if r != 0 { -EAGAIN } else { r }
}

static mut sienna_cichlid_mode2_handler: amdgpu_reset_handler = amdgpu_reset_handler {
    reset_method: AMD_RESET_METHOD_MODE2,
    prepare_env: None,
    prepare_hwcontext: Some(sienna_cichlid_mode2_prepare_hwcontext),
    perform_reset: Some(sienna_cichlid_mode2_perform_reset),
    restore_hwcontext: Some(sienna_cichlid_mode2_restore_hwcontext),
    restore_env: None,
    do_reset: Some(sienna_cichlid_mode2_reset),
};

static mut sienna_cichlid_rst_handlers: [*mut amdgpu_reset_handler; AMDGPU_RESET_MAX_HANDLERS as usize] = [
    unsafe { &raw mut sienna_cichlid_mode2_handler },
];

pub unsafe fn sienna_cichlid_reset_init(adev: *mut amdgpu_device) -> i32 {
    let reset_ctl = kzalloc_obj::<amdgpu_reset_control>();
    if reset_ctl.is_null() { return -ENOMEM; }
    (*reset_ctl).handle = adev as *mut core::ffi::c_void;
    (*reset_ctl).async_reset = Some(sienna_cichlid_async_reset);
    (*reset_ctl).active_reset = AMD_RESET_METHOD_NONE;
    (*reset_ctl).get_reset_handler = Some(sienna_cichlid_get_reset_handler);
    INIT_WORK!(&mut (*reset_ctl).reset_work, sienna_cichlid_async_reset);
    // Only mode2 is handled through reset control now
    (*reset_ctl).reset_handlers = sienna_cichlid_rst_handlers.as_mut_ptr();
    (*adev).reset_cntl = reset_ctl;
    0
}

pub unsafe fn sienna_cichlid_reset_fini(adev: *mut amdgpu_device) -> i32 {
    kfree((*adev).reset_cntl);
    (*adev).reset_cntl = core::ptr::null_mut();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
