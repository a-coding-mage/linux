/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies are supplied by the surrounding kernel translation.

unsafe fn smu_v13_0_10_is_mode2_default(reset_ctl: *mut amdgpu_reset_control) -> bool {
    let adev = (*reset_ctl).handle as *mut amdgpu_device;
    if (*adev).pm.fw_version >= 0x00502005 && !amdgpu_sriov_vf(adev) { true } else { false }
}

unsafe fn smu_v13_0_10_get_reset_handler(reset_ctl: *mut amdgpu_reset_control, reset_context: *mut amdgpu_reset_context) -> *mut amdgpu_reset_handler {
    let adev = (*reset_ctl).handle as *mut amdgpu_device;
    if (*reset_context).method != AMD_RESET_METHOD_NONE {
        for_each_handler!(i, handler, reset_ctl, {
            if (*handler).reset_method == (*reset_context).method { return handler; }
        });
    }
    if smu_v13_0_10_is_mode2_default(reset_ctl) && amdgpu_asic_reset_method(adev) == AMD_RESET_METHOD_MODE2 {
        for_each_handler!(i, handler, reset_ctl, {
            if (*handler).reset_method == AMD_RESET_METHOD_MODE2 { return handler; }
        });
    }
    core::ptr::null_mut()
}

unsafe fn smu_v13_0_10_mode2_suspend_ip(adev: *mut amdgpu_device) -> i32 {
    amdgpu_device_set_pg_state(adev, AMD_PG_STATE_UNGATE);
    amdgpu_device_set_cg_state(adev, AMD_CG_STATE_UNGATE);
    let mut i = (*adev).num_ip_blocks - 1;
    while i >= 0 {
        let ty = (*(*adev).ip_blocks.add(i as usize)).version_type();
        if ty == AMD_IP_BLOCK_TYPE_GFX || ty == AMD_IP_BLOCK_TYPE_SDMA || ty == AMD_IP_BLOCK_TYPE_MES {
            let r = amdgpu_ip_block_suspend((*adev).ip_blocks.add(i as usize));
            if r != 0 { return r; }
        }
        i -= 1;
    }
    0
}

unsafe fn smu_v13_0_10_mode2_prepare_hwcontext(reset_ctl: *mut amdgpu_reset_control, _reset_context: *mut amdgpu_reset_context) -> i32 {
    let adev = (*reset_ctl).handle as *mut amdgpu_device;
    if !amdgpu_sriov_vf(adev) { smu_v13_0_10_mode2_suspend_ip(adev) } else { 0 }
}

unsafe fn smu_v13_0_10_mode2_reset(adev: *mut amdgpu_device) -> i32 { amdgpu_dpm_mode2_reset(adev) }

unsafe fn smu_v13_0_10_async_reset(work: *mut work_struct) {
    let reset_ctl = container_of!(work, amdgpu_reset_control, reset_work);
    let adev = (*reset_ctl).handle as *mut amdgpu_device;
    for_each_handler!(i, handler, reset_ctl, {
        if (*handler).reset_method == (*reset_ctl).active_reset {
            dev_dbg!((*adev).dev, "Resetting device\n");
            ((*handler).do_reset)(adev);
            break;
        }
    });
}

unsafe fn smu_v13_0_10_mode2_perform_reset(reset_ctl: *mut amdgpu_reset_control, _reset_context: *mut amdgpu_reset_context) -> i32 {
    let adev = (*reset_ctl).handle as *mut amdgpu_device;
    let r = smu_v13_0_10_mode2_reset(adev);
    if r != 0 { dev_err!((*adev).dev, "ASIC reset failed with error, %d ", r); }
    r
}

unsafe fn smu_v13_0_10_mode2_restore_ip(adev: *mut amdgpu_device) -> i32 {
    let psp = &mut (*adev).psp;
    let mut ucode_list: [*mut amdgpu_firmware_info; 2] = [core::ptr::null_mut(); 2];
    let mut ucode_count = 0usize;
    for i in 0..(*adev).firmware.max_ucodes {
        let ucode = (*adev).firmware.ucode.add(i as usize);
        match (*ucode).ucode_id {
            AMDGPU_UCODE_ID_IMU_I | AMDGPU_UCODE_ID_IMU_D => { ucode_list[ucode_count] = ucode; ucode_count += 1; }
            _ => {}
        }
    }
    let mut r = psp_load_fw_list(psp, ucode_list.as_mut_ptr(), ucode_count as i32);
    if r != 0 { dev_err!((*adev).dev, "IMU ucode load failed after mode2 reset\n"); return r; }
    r = psp_rlc_autoload_start(psp);
    if r != 0 { drm_error!("Failed to start rlc autoload after mode2 reset\n"); return r; }
    amdgpu_dpm_enable_gfx_features(adev);
    for i in 0..(*adev).num_ip_blocks {
        let block = (*adev).ip_blocks.add(i as usize);
        let ty = (*block).version_type();
        if ty != AMD_IP_BLOCK_TYPE_GFX && ty != AMD_IP_BLOCK_TYPE_MES && ty != AMD_IP_BLOCK_TYPE_SDMA { continue; }
        r = amdgpu_ip_block_resume(block); if r != 0 { return r; }
    }
    for i in 0..(*adev).num_ip_blocks {
        let block = (*adev).ip_blocks.add(i as usize); let ty = (*block).version_type();
        if ty != AMD_IP_BLOCK_TYPE_GFX && ty != AMD_IP_BLOCK_TYPE_MES && ty != AMD_IP_BLOCK_TYPE_SDMA { continue; }
        if (*block).has_late_init() { r = (*block).late_init(); if r != 0 { dev_err!((*adev).dev, "late_init of IP block failed %d after reset\n", r); return r; } }
        (*block).status.late_initialized = true;
    }
    amdgpu_device_set_cg_state(adev, AMD_CG_STATE_GATE); amdgpu_device_set_pg_state(adev, AMD_PG_STATE_GATE); r
}

unsafe fn smu_v13_0_10_mode2_restore_hwcontext(reset_ctl: *mut amdgpu_reset_control, _reset_context: *mut amdgpu_reset_context) -> i32 {
    let adev = (*reset_ctl).handle as *mut amdgpu_device;
    amdgpu_set_init_level(adev, AMDGPU_INIT_LEVEL_RESET_RECOVERY);
    dev_info!((*adev).dev, "GPU reset succeeded, trying to resume\n");
    let mut r = smu_v13_0_10_mode2_restore_ip(adev);
    if r != 0 { return -EAGAIN; }
    amdgpu_register_gpu_instance(adev); amdgpu_ras_resume(adev); amdgpu_irq_gpu_reset_resume_helper(adev);
    amdgpu_set_init_level(adev, AMDGPU_INIT_LEVEL_DEFAULT); r = amdgpu_ib_ring_tests(adev);
    if r != 0 { dev_err!((*adev).dev, "ib ring test failed (%d).\n", r); r = -EAGAIN; }
    if r != 0 { -EAGAIN } else { r }
}

static mut SMU_V13_0_10_MODE2_HANDLER: amdgpu_reset_handler = amdgpu_reset_handler {
    reset_method: AMD_RESET_METHOD_MODE2,
    prepare_env: None,
    prepare_hwcontext: Some(smu_v13_0_10_mode2_prepare_hwcontext),
    perform_reset: Some(smu_v13_0_10_mode2_perform_reset),
    restore_hwcontext: Some(smu_v13_0_10_mode2_restore_hwcontext),
    restore_env: None,
    do_reset: smu_v13_0_10_mode2_reset,
};

static mut SMU_V13_0_10_RST_HANDLERS: [*mut amdgpu_reset_handler; AMDGPU_RESET_MAX_HANDLERS] = [
    unsafe { &mut SMU_V13_0_10_MODE2_HANDLER },
];

// The reset-handler objects and reset init/fini entry points retain the C ABI and
// are defined against the surrounding kernel translation's struct definitions.
pub unsafe fn smu_v13_0_10_reset_init(adev: *mut amdgpu_device) -> i32 {
    let reset_ctl = kzalloc_obj::<amdgpu_reset_control>();
    if reset_ctl.is_null() { return -ENOMEM; }
    (*reset_ctl).handle = adev as *mut _; (*reset_ctl).async_reset = smu_v13_0_10_async_reset;
    (*reset_ctl).active_reset = AMD_RESET_METHOD_NONE; (*reset_ctl).get_reset_handler = smu_v13_0_10_get_reset_handler;
    init_work!(&mut (*reset_ctl).reset_work, (*reset_ctl).async_reset);
    (*reset_ctl).reset_handlers = &SMU_V13_0_10_RST_HANDLERS; (*adev).reset_cntl = reset_ctl; 0
}

pub unsafe fn smu_v13_0_10_reset_fini(adev: *mut amdgpu_device) -> i32 {
    kfree((*adev).reset_cntl); (*adev).reset_cntl = core::ptr::null_mut(); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
