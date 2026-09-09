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
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

unsafe fn aldebaran_is_mode2_default(reset_ctl: *mut amdgpu_reset_control) -> bool {
    let adev = (*reset_ctl).handle as *mut amdgpu_device;
    amdgpu_ip_version(adev, MP1_HWIP, 0) == IP_VERSION(13, 0, 2)
        && (*adev).gmc.xgmi.connected_to_cpu
}

unsafe fn aldebaran_get_reset_handler(
    reset_ctl: *mut amdgpu_reset_control,
    reset_context: *mut amdgpu_reset_context,
) -> *mut amdgpu_reset_handler {
    let adev = (*reset_ctl).handle as *mut amdgpu_device;
    if (*reset_context).method == AMD_RESET_METHOD_NONE {
        if aldebaran_is_mode2_default(reset_ctl) {
            (*reset_context).method = AMD_RESET_METHOD_MODE2;
        } else {
            (*reset_context).method = amdgpu_asic_reset_method(adev);
        }
    }
    if (*reset_context).method != AMD_RESET_METHOD_NONE {
        dev_dbg((*adev).dev, "Getting reset handler for method %d\n", (*reset_context).method);
        let mut i = 0;
        let mut handler: *mut amdgpu_reset_handler = core::ptr::null_mut();
        for_each_handler!(i, handler, reset_ctl);
        while !handler.is_null() {
            if (*handler).reset_method == (*reset_context).method { return handler; }
            for_each_handler!(i, handler, reset_ctl);
        }
    }
    dev_dbg((*adev).dev, "Reset handler not found!\n");
    core::ptr::null_mut()
}

unsafe fn aldebaran_get_ip_block_mask(adev: *mut amdgpu_device) -> u32 {
    let mut mask = BIT(AMD_IP_BLOCK_TYPE_GFX) | BIT(AMD_IP_BLOCK_TYPE_SDMA);
    if (*adev).aid_mask != 0 { mask |= BIT(AMD_IP_BLOCK_TYPE_IH); }
    mask
}

unsafe fn aldebaran_mode2_suspend_ip(adev: *mut amdgpu_device) -> i32 {
    let mut mask = aldebaran_get_ip_block_mask(adev);
    if (*adev).aid_mask != 0 { mask &= !BIT(AMD_IP_BLOCK_TYPE_SDMA); }
    amdgpu_device_set_pg_state(adev, AMD_PG_STATE_UNGATE);
    amdgpu_device_set_cg_state(adev, AMD_CG_STATE_UNGATE);
    let mut i = (*adev).num_ip_blocks as isize - 1;
    while i >= 0 {
        let ip_block = BIT((*adev).ip_blocks[i as usize].version.r#type);
        if mask & ip_block != 0 {
            let r = amdgpu_ip_block_suspend(&mut (*adev).ip_blocks[i as usize]);
            if r != 0 { return r; }
        }
        i -= 1;
    }
    0
}

unsafe fn aldebaran_mode2_prepare_hwcontext(reset_ctl: *mut amdgpu_reset_control, _reset_context: *mut amdgpu_reset_context) -> i32 {
    let adev = (*reset_ctl).handle as *mut amdgpu_device;
    dev_dbg((*adev).dev, "Aldebaran prepare hw context\n");
    if !amdgpu_sriov_vf(adev) { aldebaran_mode2_suspend_ip(adev) } else { 0 }
}

unsafe fn aldebaran_async_reset(work: *mut work_struct) {
    let reset_ctl = container_of!(work, amdgpu_reset_control, reset_work);
    let adev = (*reset_ctl).handle as *mut amdgpu_device;
    let mut i = 0;
    let mut handler: *mut amdgpu_reset_handler = core::ptr::null_mut();
    for_each_handler!(i, handler, reset_ctl);
    while !handler.is_null() {
        if (*handler).reset_method == (*reset_ctl).active_reset {
            dev_dbg((*adev).dev, "Resetting device\n");
            ((*handler).do_reset.unwrap())(adev);
            break;
        }
        for_each_handler!(i, handler, reset_ctl);
    }
}

unsafe fn aldebaran_mode2_reset(adev: *mut amdgpu_device) -> i32 {
    pci_clear_master((*adev).pdev);
    (*adev).asic_reset_res = amdgpu_dpm_mode2_reset(adev);
    (*adev).asic_reset_res
}

unsafe fn aldebaran_mode2_perform_reset(reset_ctl: *mut amdgpu_reset_control, reset_context: *mut amdgpu_reset_context) -> i32 {
    let adev = (*reset_ctl).handle as *mut amdgpu_device;
    let list = (*reset_context).reset_device_list;
    if list.is_null() { return -EINVAL; }
    if amdgpu_ip_version(adev, MP1_HWIP, 0) == IP_VERSION(13, 0, 2) && (*reset_context).hive.is_null() { return -EINVAL; }
    let mut tmp_adev: *mut amdgpu_device = core::ptr::null_mut();
    list_for_each_entry!(tmp_adev, list, reset_list) {
        mutex_lock(&mut (*(*tmp_adev).reset_cntl).reset_lock);
        (*(*tmp_adev).reset_cntl).active_reset = AMD_RESET_METHOD_MODE2;
    }
    let mut r = 0;
    list_for_each_entry!(tmp_adev, list, reset_list) {
        if (*tmp_adev).gmc.xgmi.num_physical_nodes > 1 {
            if !queue_work(system_dfl_wq, &mut (*(*tmp_adev).reset_cntl).reset_work) { r = -EALREADY; }
        } else { r = aldebaran_mode2_reset(tmp_adev); }
        if r != 0 { break; }
    }
    if r == 0 {
        list_for_each_entry!(tmp_adev, list, reset_list) {
            if (*tmp_adev).gmc.xgmi.num_physical_nodes > 1 { flush_work(&mut (*(*tmp_adev).reset_cntl).reset_work); r = (*tmp_adev).asic_reset_res; if r != 0 { break; } }
        }
    }
    list_for_each_entry!(tmp_adev, list, reset_list) { mutex_unlock(&mut (*(*tmp_adev).reset_cntl).reset_lock); (*(*tmp_adev).reset_cntl).active_reset = AMD_RESET_METHOD_NONE; }
    r
}

/* The remaining mode2 restore path is a direct unsafe translation of the C implementation. */
unsafe fn aldebaran_mode2_restore_ip(adev: *mut amdgpu_device) -> i32 {
    let mut ucode_list: [*mut amdgpu_firmware_info; AMDGPU_UCODE_ID_MAXIMUM] = [core::ptr::null_mut(); AMDGPU_UCODE_ID_MAXIMUM];
    let mut count = 0usize;
    for i in 0..(*adev).firmware.max_ucodes as usize {
        let u = &mut (*adev).firmware.ucode[i];
        if u.fw.is_null() { continue; }
        match u.ucode_id { AMDGPU_UCODE_ID_SDMA0..=AMDGPU_UCODE_ID_SDMA7 | AMDGPU_UCODE_ID_CP_MEC1 | AMDGPU_UCODE_ID_CP_MEC1_JT | AMDGPU_UCODE_ID_RLC_RESTORE_LIST_CNTL | AMDGPU_UCODE_ID_RLC_RESTORE_LIST_GPM_MEM | AMDGPU_UCODE_ID_RLC_RESTORE_LIST_SRM_MEM | AMDGPU_UCODE_ID_RLC_G => { ucode_list[count] = u; count += 1; }, _ => {} }
    }
    let cmn = amdgpu_device_ip_get_ip_block(adev, AMD_IP_BLOCK_TYPE_COMMON);
    if cmn.is_null() { return -EINVAL; }
    let mut r = amdgpu_ip_block_resume(cmn); if r != 0 { return r; }
    if (*adev).aid_mask != 0 { let ih = amdgpu_device_ip_get_ip_block(adev, AMD_IP_BLOCK_TYPE_IH); if ih.is_null() { return -EINVAL; } r = amdgpu_ip_block_resume(ih); if r != 0 { return r; } }
    ((*adev).gfxhub.funcs.init.unwrap())(adev); r = ((*adev).gfxhub.funcs.gart_enable.unwrap())(adev); if r != 0 { return r; }
    r = psp_load_fw_list(&mut (*adev).psp, ucode_list.as_mut_ptr(), count as i32); if r != 0 { return r; }
    ((*adev).gfx.rlc.funcs.resume.unwrap())(adev);
    r = amdgpu_dpm_wait_for_event(adev, SMU_EVENT_RESET_COMPLETE, 0); if r != 0 { return r; }
    for i in 0..(*adev).num_ip_blocks as usize { let t = (*adev).ip_blocks[i].version.r#type; if t == AMD_IP_BLOCK_TYPE_GFX || t == AMD_IP_BLOCK_TYPE_SDMA { r = amdgpu_ip_block_resume(&mut (*adev).ip_blocks[i]); if r != 0 { return r; } } }
    for i in 0..(*adev).num_ip_blocks as usize { let t = (*adev).ip_blocks[i].version.r#type; if (t == AMD_IP_BLOCK_TYPE_GFX || t == AMD_IP_BLOCK_TYPE_SDMA || t == AMD_IP_BLOCK_TYPE_COMMON) && !(*adev).ip_blocks[i].version.funcs.late_init.is_none() { r = ((*adev).ip_blocks[i].version.funcs.late_init.unwrap())(&mut (*adev).ip_blocks[i]); if r != 0 { return r; } (*adev).ip_blocks[i].status.late_initialized = true; } }
    amdgpu_device_set_cg_state(adev, AMD_CG_STATE_GATE); amdgpu_device_set_pg_state(adev, AMD_PG_STATE_GATE); r
}

unsafe fn aldebaran_mode2_restore_hwcontext(reset_ctl: *mut amdgpu_reset_control, reset_context: *mut amdgpu_reset_context) -> i32 {
    let list = (*reset_context).reset_device_list; if list.is_null() { return -EINVAL; }
    let mut tmp: *mut amdgpu_device = core::ptr::null_mut(); let mut r = 0;
    list_for_each_entry!(tmp, list, reset_list) { amdgpu_set_init_level(tmp, AMDGPU_INIT_LEVEL_RESET_RECOVERY); amdgpu_ras_clear_err_state(tmp); r = aldebaran_mode2_restore_ip(tmp); if r != 0 { break; } amdgpu_register_gpu_instance(tmp); amdgpu_ras_resume(tmp); amdgpu_set_init_level(tmp, AMDGPU_INIT_LEVEL_DEFAULT); amdgpu_irq_gpu_reset_resume_helper(tmp); r = amdgpu_ib_ring_tests(tmp); if r != 0 { r = -EAGAIN; (*tmp).asic_reset_res = r; break; } }
    r
}

static mut aldebaran_mode2_handler: amdgpu_reset_handler = amdgpu_reset_handler { reset_method: AMD_RESET_METHOD_MODE2, prepare_env: None, prepare_hwcontext: Some(aldebaran_mode2_prepare_hwcontext), perform_reset: Some(aldebaran_mode2_perform_reset), restore_hwcontext: Some(aldebaran_mode2_restore_hwcontext), restore_env: None, do_reset: Some(aldebaran_mode2_reset) };
static mut aldebaran_rst_handlers: [*mut amdgpu_reset_handler; AMDGPU_RESET_MAX_HANDLERS] = [unsafe { &mut aldebaran_mode2_handler }, unsafe { &mut xgmi_reset_on_init_handler }];

pub unsafe fn aldebaran_reset_init(adev: *mut amdgpu_device) -> i32 { let reset_ctl = kzalloc_obj::<amdgpu_reset_control>(); if reset_ctl.is_null() { return -ENOMEM; } (*reset_ctl).handle = adev; (*reset_ctl).async_reset = Some(aldebaran_async_reset); (*reset_ctl).active_reset = AMD_RESET_METHOD_NONE; (*reset_ctl).get_reset_handler = Some(aldebaran_get_reset_handler); INIT_WORK!(&mut (*reset_ctl).reset_work, aldebaran_async_reset); (*reset_ctl).reset_handlers = &mut aldebaran_rst_handlers; (*adev).reset_cntl = reset_ctl; 0 }
pub unsafe fn aldebaran_reset_fini(adev: *mut amdgpu_device) -> i32 { kfree((*adev).reset_cntl); (*adev).reset_cntl = core::ptr::null_mut(); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
