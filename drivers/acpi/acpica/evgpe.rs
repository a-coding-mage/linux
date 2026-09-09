// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: evgpe - General Purpose Event handling and dispatch
// Copyright (C) 2000 - 2026, Intel Corp.

// Dependencies supplied by the ACPI implementation.
// This module is omitted when ACPI_REDUCED_HARDWARE is enabled.

pub unsafe fn acpi_ev_update_gpe_enable_mask(
    gpe_event_info: *mut acpi_gpe_event_info,
) -> acpi_status {
    let gpe_register_info = (*gpe_event_info).register_info;
    if gpe_register_info.is_null() { return AE_NOT_EXIST; }
    let register_bit = acpi_hw_get_gpe_register_bit(gpe_event_info);
    (*gpe_register_info).enable_for_run &= !register_bit;
    if (*gpe_event_info).runtime_count != 0 {
        (*gpe_register_info).enable_for_run |= register_bit as u8;
    }
    (*gpe_register_info).enable_mask = (*gpe_register_info).enable_for_run;
    AE_OK
}

pub unsafe fn acpi_ev_enable_gpe(gpe_event_info: *mut acpi_gpe_event_info) -> acpi_status {
    acpi_hw_low_set_gpe(gpe_event_info, ACPI_GPE_ENABLE)
}

pub unsafe fn acpi_ev_mask_gpe(gpe_event_info: *mut acpi_gpe_event_info, is_masked: u8) -> acpi_status {
    let gpe_register_info = (*gpe_event_info).register_info;
    if gpe_register_info.is_null() { return AE_NOT_EXIST; }
    let register_bit = acpi_hw_get_gpe_register_bit(gpe_event_info);
    if is_masked != 0 {
        if register_bit & (*gpe_register_info).mask_for_run as u32 != 0 { return AE_BAD_PARAMETER; }
        let _ = acpi_hw_low_set_gpe(gpe_event_info, ACPI_GPE_DISABLE);
        (*gpe_register_info).mask_for_run |= register_bit as u8;
    } else {
        if register_bit & (*gpe_register_info).mask_for_run as u32 == 0 { return AE_BAD_PARAMETER; }
        (*gpe_register_info).mask_for_run &= !(register_bit as u8);
        if (*gpe_event_info).runtime_count != 0 && !(*gpe_event_info).disable_for_dispatch {
            let _ = acpi_hw_low_set_gpe(gpe_event_info, ACPI_GPE_ENABLE);
        }
    }
    AE_OK
}

pub unsafe fn acpi_ev_add_gpe_reference(gpe_event_info: *mut acpi_gpe_event_info, clear_on_enable: u8) -> acpi_status {
    let mut status = AE_OK;
    if (*gpe_event_info).runtime_count == ACPI_UINT8_MAX { return AE_LIMIT; }
    (*gpe_event_info).runtime_count += 1;
    if (*gpe_event_info).runtime_count == 1 {
        if clear_on_enable != 0 { let _ = acpi_hw_clear_gpe(gpe_event_info); }
        status = acpi_ev_update_gpe_enable_mask(gpe_event_info);
        if ACPI_SUCCESS(status) { status = acpi_ev_enable_gpe(gpe_event_info); }
        if ACPI_FAILURE(status) { (*gpe_event_info).runtime_count -= 1; }
    }
    status
}

pub unsafe fn acpi_ev_remove_gpe_reference(gpe_event_info: *mut acpi_gpe_event_info) -> acpi_status {
    let mut status = AE_OK;
    if (*gpe_event_info).runtime_count == 0 { return AE_LIMIT; }
    (*gpe_event_info).runtime_count -= 1;
    if (*gpe_event_info).runtime_count == 0 {
        status = acpi_ev_update_gpe_enable_mask(gpe_event_info);
        if ACPI_SUCCESS(status) { status = acpi_hw_low_set_gpe(gpe_event_info, ACPI_GPE_DISABLE); }
        if ACPI_FAILURE(status) { (*gpe_event_info).runtime_count += 1; }
    }
    status
}

pub unsafe fn acpi_ev_low_get_gpe_info(gpe_number: u32, gpe_block: *mut acpi_gpe_block_info) -> *mut acpi_gpe_event_info {
    if gpe_block.is_null() || gpe_number < (*gpe_block).block_base_number { return core::ptr::null_mut(); }
    let gpe_index = gpe_number - (*gpe_block).block_base_number;
    if gpe_index >= (*gpe_block).gpe_count { return core::ptr::null_mut(); }
    (*gpe_block).event_info.add(gpe_index as usize)
}

pub unsafe fn acpi_ev_get_gpe_event_info(gpe_device: acpi_handle, gpe_number: u32) -> *mut acpi_gpe_event_info {
    if gpe_device.is_null() {
        for i in 0..ACPI_MAX_GPE_BLOCKS as usize {
            let info = acpi_ev_low_get_gpe_info(gpe_number, acpi_gbl_gpe_fadt_blocks[i]);
            if !info.is_null() { return info; }
        }
        return core::ptr::null_mut();
    }
    let obj_desc = acpi_ns_get_attached_object(gpe_device as *mut acpi_namespace_node);
    if obj_desc.is_null() || (*obj_desc).device.gpe_block.is_null() { return core::ptr::null_mut(); }
    acpi_ev_low_get_gpe_info(gpe_number, (*obj_desc).device.gpe_block)
}

pub unsafe fn acpi_ev_gpe_detect(gpe_xrupt_list: *mut acpi_gpe_xrupt_info) -> u32 {
    let mut int_status = ACPI_INTERRUPT_NOT_HANDLED;
    if gpe_xrupt_list.is_null() { return int_status; }
    let mut flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
    let mut gpe_block = (*gpe_xrupt_list).gpe_block_list_head;
    while !gpe_block.is_null() {
        let gpe_device = (*gpe_block).node;
        for i in 0..(*gpe_block).register_count {
            let reg = (*gpe_block).register_info.add(i as usize);
            if (*reg).enable_for_run | (*reg).enable_for_wake == 0 { continue; }
            for j in 0..ACPI_GPE_REGISTER_WIDTH {
                let event = (*gpe_block).event_info.add((i * ACPI_GPE_REGISTER_WIDTH + j) as usize);
                let number = j + (*reg).base_gpe_number;
                acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
                int_status |= acpi_ev_detect_gpe(gpe_device, event, number);
                flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
            }
        }
        gpe_block = (*gpe_block).next;
    }
    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    int_status
}

unsafe fn acpi_ev_asynch_execute_gpe_method(context: *mut core::ffi::c_void) {
    let gpe_event_info = context as *mut acpi_gpe_event_info;
    let mut status = AE_OK;
    match ACPI_GPE_DISPATCH_TYPE((*gpe_event_info).flags) {
        ACPI_GPE_DISPATCH_NOTIFY => {
            let mut notify = (*gpe_event_info).dispatch.notify_list;
            while ACPI_SUCCESS(status) && !notify.is_null() {
                status = acpi_ev_queue_notify_request((*notify).device_node, ACPI_NOTIFY_DEVICE_WAKE);
                notify = (*notify).next;
            }
        }
        ACPI_GPE_DISPATCH_METHOD => {
            let info = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_evaluate_info>()) as *mut acpi_evaluate_info;
            if info.is_null() { status = AE_NO_MEMORY; } else {
                (*info).prefix_node = (*gpe_event_info).dispatch.method_node;
                (*info).flags = ACPI_IGNORE_RETURN_VALUE;
                status = acpi_ns_evaluate(info);
                ACPI_FREE(info as *mut core::ffi::c_void);
            }
        }
        _ => { acpi_ev_asynch_enable_gpe(gpe_event_info as *mut core::ffi::c_void); return; }
    }
    if ACPI_SUCCESS(acpi_os_execute(OSL_NOTIFY_HANDLER, acpi_ev_asynch_enable_gpe, gpe_event_info as *mut core::ffi::c_void)) { return; }
    acpi_ev_asynch_enable_gpe(gpe_event_info as *mut core::ffi::c_void);
}

unsafe fn acpi_ev_asynch_enable_gpe(context: *mut core::ffi::c_void) {
    let flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
    let _ = acpi_ev_finish_gpe(context as *mut acpi_gpe_event_info);
    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
}

pub unsafe fn acpi_ev_finish_gpe(gpe_event_info: *mut acpi_gpe_event_info) -> acpi_status {
    if (*gpe_event_info).flags & ACPI_GPE_XRUPT_TYPE_MASK == ACPI_GPE_LEVEL_TRIGGERED {
        let status = acpi_hw_clear_gpe(gpe_event_info);
        if ACPI_FAILURE(status) { return status; }
    }
    let _ = acpi_hw_low_set_gpe(gpe_event_info, ACPI_GPE_CONDITIONAL_ENABLE);
    (*gpe_event_info).disable_for_dispatch = FALSE;
    AE_OK
}

pub unsafe fn acpi_ev_detect_gpe(gpe_device: *mut acpi_namespace_node, mut gpe_event_info: *mut acpi_gpe_event_info, gpe_number: u32) -> u32 {
    let mut int_status = ACPI_INTERRUPT_NOT_HANDLED;
    let flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
    if gpe_event_info.is_null() { gpe_event_info = acpi_ev_get_gpe_event_info(gpe_device as acpi_handle, gpe_number); if gpe_event_info.is_null() { acpi_os_release_lock(acpi_gbl_gpe_lock, flags); return int_status; } }
    let reg = (*gpe_event_info).register_info;
    let bit = acpi_hw_get_gpe_register_bit(gpe_event_info);
    let mut enable = 0u64; let mut active = 0u64;
    if ACPI_FAILURE(acpi_hw_gpe_read(&mut enable, &(*reg).enable_address)) || ACPI_FAILURE(acpi_hw_gpe_read(&mut active, &(*reg).status_address)) { acpi_os_release_lock(acpi_gbl_gpe_lock, flags); return int_status; }
    if ((active & enable) as u8 & bit as u8) == 0 { acpi_os_release_lock(acpi_gbl_gpe_lock, flags); return int_status; }
    acpi_gpe_count += 1;
    if !acpi_gbl_global_event_handler.is_none() { acpi_gbl_global_event_handler.unwrap()(ACPI_EVENT_TYPE_GPE, gpe_device as acpi_handle, gpe_number, acpi_gbl_global_event_handler_context); }
    if ACPI_GPE_DISPATCH_TYPE((*gpe_event_info).flags) == ACPI_GPE_DISPATCH_RAW_HANDLER {
        let handler = (*gpe_event_info).dispatch.handler;
        acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
        int_status |= ((*handler).address)(gpe_device, gpe_number, (*handler).context);
        return int_status;
    }
    int_status |= acpi_ev_gpe_dispatch(gpe_device, gpe_event_info, gpe_number);
    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    int_status
}

pub unsafe fn acpi_ev_gpe_dispatch(gpe_device: *mut acpi_namespace_node, gpe_event_info: *mut acpi_gpe_event_info, gpe_number: u32) -> u32 {
    let mut status = acpi_hw_low_set_gpe(gpe_event_info, ACPI_GPE_DISABLE);
    if ACPI_FAILURE(status) { return ACPI_INTERRUPT_NOT_HANDLED; }
    if (*gpe_event_info).flags & ACPI_GPE_XRUPT_TYPE_MASK == ACPI_GPE_EDGE_TRIGGERED {
        status = acpi_hw_clear_gpe(gpe_event_info);
        if ACPI_FAILURE(status) { let _ = acpi_hw_low_set_gpe(gpe_event_info, ACPI_GPE_CONDITIONAL_ENABLE); return ACPI_INTERRUPT_NOT_HANDLED; }
    }
    (*gpe_event_info).disable_for_dispatch = TRUE;
    match ACPI_GPE_DISPATCH_TYPE((*gpe_event_info).flags) {
        ACPI_GPE_DISPATCH_HANDLER => {
            let handler = (*gpe_event_info).dispatch.handler;
            let value = ((*handler).address)(gpe_device, gpe_number, (*handler).context);
            if value & ACPI_REENABLE_GPE != 0 { let _ = acpi_ev_finish_gpe(gpe_event_info); }
        }
        ACPI_GPE_DISPATCH_METHOD | ACPI_GPE_DISPATCH_NOTIFY => { let _ = acpi_os_execute(OSL_GPE_HANDLER, acpi_ev_asynch_execute_gpe_method, gpe_event_info as *mut core::ffi::c_void); }
        _ => {}
    }
    ACPI_INTERRUPT_HANDLED
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
