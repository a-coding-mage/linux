// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: hwgpe - Low level GPE enable/disable/clear functions
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the surrounding ACPICA translation.
// This module is omitted when ACPI_REDUCED_HARDWARE is enabled.

#[cfg(not(ACPI_REDUCED_HARDWARE))]
unsafe fn acpi_hw_enable_wakeup_gpe_block(
    gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
    gpe_block: *mut acpi_gpe_block_info,
    context: *mut core::ffi::c_void,
) -> acpi_status;

#[cfg(not(ACPI_REDUCED_HARDWARE))]
unsafe fn acpi_hw_gpe_enable_write(
    enable_mask: u8,
    gpe_register_info: *mut acpi_gpe_register_info,
) -> acpi_status;

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_hw_gpe_read(value: *mut u64, reg: *mut acpi_gpe_address) -> acpi_status {
    let mut status: acpi_status;
    let mut value32: u32 = 0;

    if (*reg).space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY {
        // ACPI_GPE_USE_LOGICAL_ADDRESSES selects the direct logical-address access path.
        #[cfg(ACPI_GPE_USE_LOGICAL_ADDRESSES)]
        {
            *value = ACPI_GET8((*reg).address as usize) as u64;
            return AE_OK;
        }
        #[cfg(not(ACPI_GPE_USE_LOGICAL_ADDRESSES))]
        {
            return acpi_os_read_memory(
                (*reg).address as acpi_physical_address,
                value,
                ACPI_GPE_REGISTER_WIDTH,
            );
        }
    }

    status = acpi_os_read_port(
        (*reg).address as acpi_io_address,
        &mut value32,
        ACPI_GPE_REGISTER_WIDTH,
    );
    if ACPI_FAILURE(status) {
        return status;
    }

    *value = value32 as u64;
    AE_OK
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_hw_gpe_write(value: u64, reg: *mut acpi_gpe_address) -> acpi_status {
    if (*reg).space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY {
        // ACPI_GPE_USE_LOGICAL_ADDRESSES selects the direct logical-address access path.
        #[cfg(ACPI_GPE_USE_LOGICAL_ADDRESSES)]
        {
            ACPI_SET8((*reg).address as usize, value);
            return AE_OK;
        }
        #[cfg(not(ACPI_GPE_USE_LOGICAL_ADDRESSES))]
        {
            return acpi_os_write_memory(
                (*reg).address as acpi_physical_address,
                value,
                ACPI_GPE_REGISTER_WIDTH,
            );
        }
    }

    acpi_os_write_port(
        (*reg).address as acpi_io_address,
        value as u32,
        ACPI_GPE_REGISTER_WIDTH,
    )
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_hw_get_gpe_register_bit(
    gpe_event_info: *mut acpi_gpe_event_info,
) -> u32 {
    (1u32).wrapping_shl(
        ((*gpe_event_info).gpe_number
            - (*(*gpe_event_info).register_info).base_gpe_number) as u32,
    )
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_hw_low_set_gpe(
    gpe_event_info: *mut acpi_gpe_event_info,
    action: u32,
) -> acpi_status {
    let gpe_register_info = (*gpe_event_info).register_info;
    if gpe_register_info.is_null() {
        return AE_NOT_EXIST;
    }

    let mut enable_mask: u64 = 0;
    let mut status = acpi_hw_gpe_read(&mut enable_mask, &mut (*gpe_register_info).enable_address);
    if ACPI_FAILURE(status) {
        return status;
    }

    let register_bit = acpi_hw_get_gpe_register_bit(gpe_event_info);
    match action {
        ACPI_GPE_CONDITIONAL_ENABLE => {
            if register_bit & (*gpe_register_info).enable_mask as u32 == 0 {
                return AE_BAD_PARAMETER;
            }
            enable_mask |= register_bit as u64;
        }
        ACPI_GPE_ENABLE => enable_mask |= register_bit as u64,
        ACPI_GPE_DISABLE => enable_mask &= !(register_bit as u64),
        _ => {
            ACPI_ERROR((AE_INFO, "Invalid GPE Action, %u", action));
            return AE_BAD_PARAMETER;
        }
    }

    if register_bit & (*gpe_register_info).mask_for_run as u32 == 0 {
        status = acpi_hw_gpe_write(enable_mask, &mut (*gpe_register_info).enable_address);
    }
    status
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_hw_clear_gpe(gpe_event_info: *mut acpi_gpe_event_info) -> acpi_status {
    let gpe_register_info = (*gpe_event_info).register_info;
    if gpe_register_info.is_null() {
        return AE_NOT_EXIST;
    }
    let register_bit = acpi_hw_get_gpe_register_bit(gpe_event_info);
    acpi_hw_gpe_write(register_bit as u64, &mut (*gpe_register_info).status_address)
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_hw_get_gpe_status(
    gpe_event_info: *mut acpi_gpe_event_info,
    event_status: *mut acpi_event_status,
) -> acpi_status {
    if event_status.is_null() {
        return AE_BAD_PARAMETER;
    }

    let mut local_event_status: acpi_event_status = 0;
    if ACPI_GPE_DISPATCH_TYPE((*gpe_event_info).flags) != ACPI_GPE_DISPATCH_NONE {
        local_event_status |= ACPI_EVENT_FLAG_HAS_HANDLER;
    }

    let gpe_register_info = (*gpe_event_info).register_info;
    let register_bit = acpi_hw_get_gpe_register_bit(gpe_event_info);
    if register_bit & (*gpe_register_info).enable_for_run as u32 != 0 {
        local_event_status |= ACPI_EVENT_FLAG_ENABLED;
    }
    if register_bit & (*gpe_register_info).mask_for_run as u32 != 0 {
        local_event_status |= ACPI_EVENT_FLAG_MASKED;
    }
    if register_bit & (*gpe_register_info).enable_for_wake as u32 != 0 {
        local_event_status |= ACPI_EVENT_FLAG_WAKE_ENABLED;
    }

    let mut in_byte = 0u64;
    let status = acpi_hw_gpe_read(&mut in_byte, &mut (*gpe_register_info).enable_address);
    if ACPI_FAILURE(status) { return status; }
    if register_bit as u64 & in_byte != 0 { local_event_status |= ACPI_EVENT_FLAG_ENABLE_SET; }

    let status = acpi_hw_gpe_read(&mut in_byte, &mut (*gpe_register_info).status_address);
    if ACPI_FAILURE(status) { return status; }
    if register_bit as u64 & in_byte != 0 { local_event_status |= ACPI_EVENT_FLAG_STATUS_SET; }

    *event_status = local_event_status;
    AE_OK
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
unsafe fn acpi_hw_gpe_enable_write(
    enable_mask: u8,
    gpe_register_info: *mut acpi_gpe_register_info,
) -> acpi_status {
    (*gpe_register_info).enable_mask = enable_mask;
    acpi_hw_gpe_write(enable_mask as u64, &mut (*gpe_register_info).enable_address)
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_hw_disable_gpe_block(
    _gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
    gpe_block: *mut acpi_gpe_block_info,
    _context: *mut core::ffi::c_void,
) -> acpi_status {
    for i in 0..(*gpe_block).register_count {
        let status = acpi_hw_gpe_enable_write(0, (*gpe_block).register_info.add(i as usize));
        if ACPI_FAILURE(status) { return status; }
    }
    AE_OK
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_hw_clear_gpe_block(
    _gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
    gpe_block: *mut acpi_gpe_block_info,
    _context: *mut core::ffi::c_void,
) -> acpi_status {
    for i in 0..(*gpe_block).register_count {
        let status = acpi_hw_gpe_write(
            0xff,
            &mut (*(*gpe_block).register_info.add(i as usize)).status_address,
        );
        if ACPI_FAILURE(status) { return status; }
    }
    AE_OK
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_hw_enable_runtime_gpe_block(
    _gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
    gpe_block: *mut acpi_gpe_block_info,
    _context: *mut core::ffi::c_void,
) -> acpi_status {
    for i in 0..(*gpe_block).register_count {
        let info = (*gpe_block).register_info.add(i as usize);
        if (*info).enable_for_run == 0 { continue; }
        let enable_mask = (*info).enable_for_run & !(*info).mask_for_run;
        let status = acpi_hw_gpe_enable_write(enable_mask, info);
        if ACPI_FAILURE(status) { return status; }
    }
    AE_OK
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
unsafe fn acpi_hw_enable_wakeup_gpe_block(
    _gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
    gpe_block: *mut acpi_gpe_block_info,
    _context: *mut core::ffi::c_void,
) -> acpi_status {
    for i in 0..(*gpe_block).register_count {
        let info = (*gpe_block).register_info.add(i as usize);
        let status = acpi_hw_gpe_enable_write((*info).enable_for_wake, info);
        if ACPI_FAILURE(status) { return status; }
    }
    AE_OK
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
#[repr(C)]
pub struct acpi_gpe_block_status_context {
    pub gpe_skip_register_info: *mut acpi_gpe_register_info,
    pub gpe_skip_mask: u8,
    pub retval: u8,
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
unsafe fn acpi_hw_get_gpe_block_status(
    _gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
    gpe_block: *mut acpi_gpe_block_info,
    context: *mut core::ffi::c_void,
) -> acpi_status {
    let c = context as *mut acpi_gpe_block_status_context;
    for i in 0..(*gpe_block).register_count {
        let info = (*gpe_block).register_info.add(i as usize);
        let mut in_enable = 0u64;
        let mut in_status = 0u64;
        if ACPI_FAILURE(acpi_hw_gpe_read(&mut in_enable, &mut (*info).enable_address)) { continue; }
        if ACPI_FAILURE(acpi_hw_gpe_read(&mut in_status, &mut (*info).status_address)) { continue; }
        let mut ret_mask = (in_enable & in_status) as u8;
        if ret_mask != 0 && (*c).gpe_skip_register_info == info {
            ret_mask &= !(*c).gpe_skip_mask;
        }
        (*c).retval |= ret_mask;
    }
    AE_OK
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_hw_disable_all_gpes() -> acpi_status {
    acpi_ev_walk_gpe_list(acpi_hw_disable_gpe_block, core::ptr::null_mut())
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_hw_enable_all_runtime_gpes() -> acpi_status {
    acpi_ev_walk_gpe_list(acpi_hw_enable_runtime_gpe_block, core::ptr::null_mut())
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_hw_enable_all_wakeup_gpes() -> acpi_status {
    acpi_ev_walk_gpe_list(acpi_hw_enable_wakeup_gpe_block, core::ptr::null_mut())
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_hw_check_all_gpes(
    gpe_skip_device: acpi_handle,
    gpe_skip_number: u32,
) -> u8 {
    let mut context = acpi_gpe_block_status_context {
        gpe_skip_register_info: core::ptr::null_mut(),
        gpe_skip_mask: 0,
        retval: 0,
    };
    let flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
    let gpe_event_info = acpi_ev_get_gpe_event_info(gpe_skip_device, gpe_skip_number);
    if !gpe_event_info.is_null() {
        context.gpe_skip_register_info = (*gpe_event_info).register_info;
        context.gpe_skip_mask = acpi_hw_get_gpe_register_bit(gpe_event_info) as u8;
    }
    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    let _ = acpi_ev_walk_gpe_list(
        acpi_hw_get_gpe_block_status,
        &mut context as *mut _ as *mut core::ffi::c_void,
    );
    (context.retval != 0) as u8
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
