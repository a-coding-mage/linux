// SPDX-License-Identifier: MIT
/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// If the SRM version being loaded is less than or equal to the currently
// loaded SRM, PSP returns 0xFFFF as the version.
pub const PSP_SRM_VERSION_MAX: u32 = 0xffff;

pub unsafe fn lp_write_i2c(handle: *mut core::ffi::c_void, address: u32, data: *const u8, size: u32) -> bool {
    let link = handle as *mut dc_link;
    let payload = i2c_payload { write: true, address, length: size, data: data as *mut core::ffi::c_void };
    let cmd = i2c_command { payloads: &payload as *const _, number_of_payloads: 1, engine: I2C_COMMAND_ENGINE_HW, speed: (*(*link).dc).caps.i2c_speed_in_khz };
    dm_helpers_submit_i2c((*link).ctx, link, &cmd)
}

pub unsafe fn lp_read_i2c(handle: *mut core::ffi::c_void, address: u32, offset: u8, data: *mut u8, size: u32) -> bool {
    let link = handle as *mut dc_link;
    let payloads = [
        i2c_payload { write: true, address, length: 1, data: &offset as *const _ as *mut _ },
        i2c_payload { write: false, address, length: size, data: data as *mut _ },
    ];
    let cmd = i2c_command { payloads: payloads.as_ptr(), number_of_payloads: 2, engine: I2C_COMMAND_ENGINE_HW, speed: (*(*link).dc).caps.i2c_speed_in_khz };
    dm_helpers_submit_i2c((*link).ctx, link, &cmd)
}

pub unsafe fn lp_write_dpcd(handle: *mut core::ffi::c_void, address: u32, data: *const u8, size: u32) -> bool {
    let link = handle as *mut dc_link;
    dm_helpers_dp_write_dpcd((*link).ctx, link, address, data, size)
}

pub unsafe fn lp_read_dpcd(handle: *mut core::ffi::c_void, address: u32, data: *mut u8, size: u32) -> bool {
    let link = handle as *mut dc_link;
    dm_helpers_dp_read_dpcd((*link).ctx, link, address, data, size)
}

pub unsafe fn lp_atomic_write_poll_read_i2c(handle: *mut core::ffi::c_void, write: *const mod_hdcp_atomic_op_i2c, poll: *const mod_hdcp_atomic_op_i2c, read: *mut mod_hdcp_atomic_op_i2c, poll_timeout_us: u32, poll_mask_msb: u8) -> bool {
    dm_atomic_write_poll_read_i2c(handle as *mut dc_link, write, poll, read, poll_timeout_us, poll_mask_msb)
}

pub unsafe fn lp_atomic_write_poll_read_aux(handle: *mut core::ffi::c_void, write: *const mod_hdcp_atomic_op_aux, poll: *const mod_hdcp_atomic_op_aux, read: *mut mod_hdcp_atomic_op_aux, poll_timeout_us: u32, poll_mask_msb: u8) -> bool {
    dm_atomic_write_poll_read_aux(handle as *mut dc_link, write, poll, read, poll_timeout_us, poll_mask_msb)
}

pub unsafe fn psp_get_srm(psp: *mut psp_context, srm_version: *mut u32, srm_size: *mut u32) -> *mut u8 {
    if !(*psp).hdcp_context.context.initialized { DRM_WARN!("Failed to get hdcp srm. HDCP TA is not initialized."); return core::ptr::null_mut(); }
    let cmd = (*psp).hdcp_context.context.mem_context.shared_buf as *mut ta_hdcp_shared_memory;
    core::ptr::write_bytes(cmd, 0, core::mem::size_of::<ta_hdcp_shared_memory>());
    (*cmd).cmd_id = TA_HDCP_COMMAND__HDCP_GET_SRM;
    psp_hdcp_invoke(psp, (*cmd).cmd_id);
    if (*cmd).hdcp_status != TA_HDCP_STATUS__SUCCESS { return core::ptr::null_mut(); }
    *srm_version = (*cmd).out_msg.hdcp_get_srm.srm_version;
    *srm_size = (*cmd).out_msg.hdcp_get_srm.srm_buf_size;
    (*cmd).out_msg.hdcp_get_srm.srm_buf.as_mut_ptr()
}

pub unsafe fn psp_set_srm(psp: *mut psp_context, srm: *const u8, srm_size: u32, srm_version: *mut u32) -> i32 {
    if !(*psp).hdcp_context.context.initialized { DRM_WARN!("Failed to get hdcp srm. HDCP TA is not initialized."); return -EINVAL; }
    let cmd = (*psp).hdcp_context.context.mem_context.shared_buf as *mut ta_hdcp_shared_memory;
    core::ptr::write_bytes(cmd, 0, core::mem::size_of::<ta_hdcp_shared_memory>());
    core::ptr::copy_nonoverlapping(srm, (*cmd).in_msg.hdcp_set_srm.srm_buf.as_mut_ptr(), srm_size as usize);
    (*cmd).in_msg.hdcp_set_srm.srm_buf_size = srm_size;
    (*cmd).cmd_id = TA_HDCP_COMMAND__HDCP_SET_SRM;
    psp_hdcp_invoke(psp, (*cmd).cmd_id);
    if (*cmd).hdcp_status != TA_HDCP_STATUS__SUCCESS || (*cmd).out_msg.hdcp_set_srm.valid_signature != 1 || (*cmd).out_msg.hdcp_set_srm.srm_version == PSP_SRM_VERSION_MAX { return -EINVAL; }
    *srm_version = (*cmd).out_msg.hdcp_set_srm.srm_version; 0
}

pub unsafe fn process_output(work: *mut hdcp_workqueue) {
    let output = (*work).output;
    if output.callback_stop { cancel_delayed_work(&mut (*work).callback_dwork); }
    if output.callback_needed { schedule_delayed_work(&mut (*work).callback_dwork, msecs_to_jiffies(output.callback_delay)); }
    if output.watchdog_timer_stop { cancel_delayed_work(&mut (*work).watchdog_timer_dwork); }
    if output.watchdog_timer_needed { schedule_delayed_work(&mut (*work).watchdog_timer_dwork, msecs_to_jiffies(output.watchdog_timer_delay)); }
    schedule_delayed_work(&mut (*work).property_validate_dwork, msecs_to_jiffies(0));
}

pub unsafe fn hdcp_get_content_protection_from_status(content_type: u32, status: mod_hdcp_encryption_status, protection: *mut u32) -> bool {
    if status == MOD_HDCP_ENCRYPTION_STATUS_HDCP_OFF { *protection = DRM_MODE_CONTENT_PROTECTION_DESIRED; return true; }
    if content_type == DRM_MODE_HDCP_CONTENT_TYPE0 && status <= MOD_HDCP_ENCRYPTION_STATUS_HDCP2_TYPE0_ON { *protection = DRM_MODE_CONTENT_PROTECTION_ENABLED; return true; }
    if content_type == DRM_MODE_HDCP_CONTENT_TYPE1 && status == MOD_HDCP_ENCRYPTION_STATUS_HDCP2_TYPE1_ON { *protection = DRM_MODE_CONTENT_PROTECTION_ENABLED; return true; }
    false
}

pub unsafe fn hdcp_get_link_display_adjustments(enable: bool, content_type: u8, fused: bool, force_fw: bool, sw_fallback: bool, link: *mut mod_hdcp_link_adjustment, display: *mut mod_hdcp_display_adjustment) {
    core::ptr::write_bytes(link, 0, 1); core::ptr::write_bytes(display, 0, 1);
    if !enable { (*display).disable = MOD_HDCP_DISPLAY_DISABLE_AUTHENTICATION; return; }
    (*display).disable = MOD_HDCP_DISPLAY_NOT_DISABLE; (*link).auth_delay = 2; (*link).retry_limit = MAX_NUM_OF_ATTEMPTS;
    if content_type == DRM_MODE_HDCP_CONTENT_TYPE0 { (*link).hdcp2.force_type = MOD_HDCP_FORCE_TYPE_0; } else if content_type == DRM_MODE_HDCP_CONTENT_TYPE1 { (*link).hdcp1.disable = 1; (*link).hdcp2.force_type = MOD_HDCP_FORCE_TYPE_1; }
    (*link).hdcp2.use_fw_locality_check = fused || force_fw; (*link).hdcp2.use_sw_locality_fallback = sw_fallback;
}

pub unsafe fn link_lock(work: *mut hdcp_workqueue, lock: bool) { for i in 0..(*work).max_link { if lock { mutex_lock(&mut (*work.add(i)).mutex); } else { mutex_unlock(&mut (*work.add(i)).mutex); } } }

// The remaining exported entry points retain the kernel implementation's
// state-machine interactions and are declared against external kernel types.
pub unsafe fn hdcp_update_display_encryption_control(work: *mut hdcp_workqueue, w: *mut hdcp_workqueue, index: u32, enable: bool) { if enable { if (*work).srm_size > 0 { psp_set_srm((*work).hdcp.config.psp.handle, (*work).srm, (*work).srm_size, &mut (*work).srm_version); } schedule_delayed_work(&mut (*w).property_validate_dwork, msecs_to_jiffies(DRM_HDCP_CHECK_PERIOD_MS)); } else { (*w).encryption_status[index as usize] = MOD_HDCP_ENCRYPTION_STATUS_HDCP_OFF; cancel_delayed_work(&mut (*w).property_validate_dwork); } }

pub unsafe fn hdcp_handle_cpirq(work: *mut hdcp_workqueue, index: u32) { schedule_work(&mut (*work.add(index as usize)).cpirq_work); }
pub unsafe fn hdcp_reset_display(work: *mut hdcp_workqueue, index: u32) { let w = work.add(index as usize); mutex_lock(&mut (*w).mutex); mod_hdcp_reset_connection(&mut (*w).hdcp, &mut (*w).output); cancel_delayed_work(&mut (*w).property_validate_dwork); for i in 0..AMDGPU_DM_MAX_DISPLAY_COUNT { (*w).encryption_status[i] = MOD_HDCP_ENCRYPTION_STATUS_HDCP_OFF; if !(*w).aconnector[i].is_null() { drm_connector_put(&mut (*(*w).aconnector[i]).base); (*w).aconnector[i] = core::ptr::null_mut(); } } mutex_unlock(&mut (*w).mutex); process_output(w); }
pub unsafe fn event_callback(work: *mut work_struct) { let w = container_of(to_delayed_work(work), hdcp_workqueue, callback_dwork); mutex_lock(&mut (*w).mutex); cancel_delayed_work(&mut (*w).callback_dwork); mod_hdcp_process_event(&mut (*w).hdcp, MOD_HDCP_EVENT_CALLBACK, &mut (*w).output); mutex_unlock(&mut (*w).mutex); process_output(w); }
pub unsafe fn event_watchdog_timer(work: *mut work_struct) { let w = container_of(to_delayed_work(work), hdcp_workqueue, watchdog_timer_dwork); mutex_lock(&mut (*w).mutex); cancel_delayed_work(&mut (*w).watchdog_timer_dwork); mod_hdcp_process_event(&mut (*w).hdcp, MOD_HDCP_EVENT_WATCHDOG_TIMEOUT, &mut (*w).output); mutex_unlock(&mut (*w).mutex); process_output(w); }
pub unsafe fn event_cpirq(work: *mut work_struct) { let w = container_of(work, hdcp_workqueue, cpirq_work); mutex_lock(&mut (*w).mutex); mod_hdcp_process_event(&mut (*w).hdcp, MOD_HDCP_EVENT_CPIRQ, &mut (*w).output); mutex_unlock(&mut (*w).mutex); process_output(w); }
pub unsafe fn hdcp_destroy(kobj: *mut kobject, work: *mut hdcp_workqueue) { for i in 0..(*work).max_link { cancel_delayed_work_sync(&mut (*work.add(i)).callback_dwork); cancel_delayed_work_sync(&mut (*work.add(i)).watchdog_timer_dwork); cancel_delayed_work_sync(&mut (*work.add(i)).property_validate_dwork); } sysfs_remove_bin_file(kobj, &mut (*work).attr); kfree((*work).srm); kfree((*work).srm_temp); kfree(work); }
pub unsafe fn srm_data_write(attr: *const bin_attribute, buffer: *const u8, pos: usize, count: usize) -> isize { let work = container_of(attr, hdcp_workqueue, attr); link_lock(work, true); core::ptr::copy_nonoverlapping(buffer, (*work).srm_temp.add(pos), count); let mut version = 0; if psp_set_srm((*work).hdcp.config.psp.handle, (*work).srm_temp, (pos + count) as u32, &mut version) == 0 { core::ptr::copy_nonoverlapping((*work).srm_temp, (*work).srm, pos + count); (*work).srm_size = (pos + count) as u32; (*work).srm_version = version; } link_lock(work, false); count as isize }
pub unsafe fn srm_data_read(attr: *const bin_attribute, buffer: *mut u8, pos: usize, count: usize) -> isize { let work = container_of(attr, hdcp_workqueue, attr); let mut version = 0; let mut size = 0; link_lock(work, true); let srm = psp_get_srm((*work).hdcp.config.psp.handle, &mut version, &mut size); if srm.is_null() { link_lock(work, false); return -EINVAL as isize; } if pos >= size as usize { link_lock(work, false); return 0; } let n = core::cmp::min(count, size as usize - pos); core::ptr::copy_nonoverlapping(srm.add(pos), buffer, n); link_lock(work, false); n as isize }
pub unsafe fn hdcp_create_workqueue(_adev: *mut amdgpu_device, _cp_psp: *mut cp_psp, _dc: *mut dc) -> *mut hdcp_workqueue { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
