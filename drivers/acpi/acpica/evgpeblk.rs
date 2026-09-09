// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: evgpeblk - GPE block creation and initialization.
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// Dependencies: <acpi/acpi.h>, "accommon.h", "acevents.h", "acnamesp.h"
// #define _COMPONENT ACPI_EVENTS
// ACPI_MODULE_NAME("evgpeblk")
// Entire module is excluded when ACPI_REDUCED_HARDWARE is enabled.

unsafe fn acpi_ev_install_gpe_block(
    gpe_block: *mut acpi_gpe_block_info,
    interrupt_number: u32,
) -> acpi_status {
    let mut next_gpe_block: *mut acpi_gpe_block_info;
    let mut gpe_xrupt_block: *mut acpi_gpe_xrupt_info = core::ptr::null_mut();
    let mut status: acpi_status;
    let flags: acpi_cpu_flags;

    status = acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);
    if (ACPI_FAILURE(status)) {
        return status;
    }

    status = acpi_ev_get_gpe_xrupt_block(interrupt_number, &mut gpe_xrupt_block);
    if (ACPI_FAILURE(status)) {
        acpi_ut_release_mutex(ACPI_MTX_EVENTS);
        return status;
    }

    flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
    if !(*gpe_xrupt_block).gpe_block_list_head.is_null() {
        next_gpe_block = (*gpe_xrupt_block).gpe_block_list_head;
        while !(*next_gpe_block).next.is_null() {
            next_gpe_block = (*next_gpe_block).next;
        }
        (*next_gpe_block).next = gpe_block;
        (*gpe_block).previous = next_gpe_block;
    } else {
        (*gpe_xrupt_block).gpe_block_list_head = gpe_block;
    }

    (*gpe_block).xrupt_block = gpe_xrupt_block;
    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    acpi_ut_release_mutex(ACPI_MTX_EVENTS);
    status
}

pub unsafe fn acpi_ev_delete_gpe_block(gpe_block: *mut acpi_gpe_block_info) -> acpi_status {
    let mut status = acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);
    if (ACPI_FAILURE(status)) {
        return status;
    }

    status = acpi_hw_disable_gpe_block((*gpe_block).xrupt_block, gpe_block, core::ptr::null_mut());
    if (ACPI_FAILURE(status)) {
        return status;
    }

    if (*gpe_block).previous.is_null() && (*gpe_block).next.is_null() {
        status = acpi_ev_delete_gpe_xrupt((*gpe_block).xrupt_block);
        if (ACPI_FAILURE(status)) {
            acpi_ut_release_mutex(ACPI_MTX_EVENTS);
            return status;
        }
    } else {
        let flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
        if !(*gpe_block).previous.is_null() {
            (*(*gpe_block).previous).next = (*gpe_block).next;
        } else {
            (*(*gpe_block).xrupt_block).gpe_block_list_head = (*gpe_block).next;
        }
        if !(*gpe_block).next.is_null() {
            (*(*gpe_block).next).previous = (*gpe_block).previous;
        }
        acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    }

    acpi_current_gpe_count -= (*gpe_block).gpe_count;
    ACPI_FREE((*gpe_block).register_info);
    ACPI_FREE((*gpe_block).event_info);
    ACPI_FREE(gpe_block);
    acpi_ut_release_mutex(ACPI_MTX_EVENTS)
}

unsafe fn acpi_ev_create_gpe_info_blocks(gpe_block: *mut acpi_gpe_block_info) -> acpi_status {
    let mut gpe_register_info: *mut acpi_gpe_register_info = core::ptr::null_mut();
    let mut gpe_event_info: *mut acpi_gpe_event_info = core::ptr::null_mut();
    let mut this_event: *mut acpi_gpe_event_info;
    let mut this_register: *mut acpi_gpe_register_info;
    let mut status: acpi_status;

    gpe_register_info = ACPI_ALLOCATE_ZEROED((*gpe_block).register_count as acpi_size * core::mem::size_of::<acpi_gpe_register_info>());
    if gpe_register_info.is_null() {
        return AE_NO_MEMORY;
    }
    gpe_event_info = ACPI_ALLOCATE_ZEROED((*gpe_block).gpe_count as acpi_size * core::mem::size_of::<acpi_gpe_event_info>());
    if gpe_event_info.is_null() {
        ACPI_FREE(gpe_register_info);
        return AE_NO_MEMORY;
    }

    (*gpe_block).register_info = gpe_register_info;
    (*gpe_block).event_info = gpe_event_info;
    this_register = gpe_register_info;
    this_event = gpe_event_info;

    for i in 0..(*gpe_block).register_count {
        (*this_register).base_gpe_number = ((*gpe_block).block_base_number + i * ACPI_GPE_REGISTER_WIDTH) as u16;
        (*this_register).status_address.address = (*gpe_block).address + i as u64;
        (*this_register).enable_address.address = (*gpe_block).address + i as u64 + (*gpe_block).register_count as u64;
        (*this_register).status_address.space_id = (*gpe_block).space_id;
        (*this_register).enable_address.space_id = (*gpe_block).space_id;
        for j in 0..ACPI_GPE_REGISTER_WIDTH {
            (*this_event).gpe_number = ((*this_register).base_gpe_number + j) as u8;
            (*this_event).register_info = this_register;
            this_event = this_event.add(1);
        }
        status = acpi_hw_gpe_write(0x00, &mut (*this_register).enable_address);
        if (ACPI_FAILURE(status)) { ACPI_FREE(gpe_register_info); ACPI_FREE(gpe_event_info); return status; }
        status = acpi_hw_gpe_write(0xFF, &mut (*this_register).status_address);
        if (ACPI_FAILURE(status)) { ACPI_FREE(gpe_register_info); ACPI_FREE(gpe_event_info); return status; }
        this_register = this_register.add(1);
    }
    AE_OK
}

pub unsafe fn acpi_ev_create_gpe_block(
    gpe_device: *mut acpi_namespace_node, address: u64, space_id: u8,
    register_count: u32, gpe_block_base_number: u16, interrupt_number: u32,
    return_gpe_block: *mut *mut acpi_gpe_block_info,
) -> acpi_status {
    if register_count == 0 { return AE_OK; }
    if space_id != ACPI_ADR_SPACE_SYSTEM_MEMORY && space_id != ACPI_ADR_SPACE_SYSTEM_IO { return AE_SUPPORT; }
    if space_id == ACPI_ADR_SPACE_SYSTEM_IO {
        let status = acpi_hw_validate_io_block(address, ACPI_GPE_REGISTER_WIDTH, register_count);
        if ACPI_FAILURE(status) { return status; }
    }
    let gpe_block: *mut acpi_gpe_block_info = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_gpe_block_info>());
    if gpe_block.is_null() { return AE_NO_MEMORY; }
    (*gpe_block).address = address;
    (*gpe_block).space_id = space_id;
    (*gpe_block).node = gpe_device;
    (*gpe_block).gpe_count = (register_count * ACPI_GPE_REGISTER_WIDTH) as u16;
    (*gpe_block).initialized = FALSE;
    (*gpe_block).register_count = register_count;
    (*gpe_block).block_base_number = gpe_block_base_number;
    let mut status = acpi_ev_create_gpe_info_blocks(gpe_block);
    if ACPI_FAILURE(status) { ACPI_FREE(gpe_block); return status; }
    status = acpi_ev_install_gpe_block(gpe_block, interrupt_number);
    if ACPI_FAILURE(status) { ACPI_FREE((*gpe_block).register_info); ACPI_FREE((*gpe_block).event_info); ACPI_FREE(gpe_block); return status; }
    acpi_gbl_all_gpes_initialized = FALSE;
    let mut walk_info: acpi_gpe_walk_info = core::mem::zeroed();
    walk_info.gpe_block = gpe_block;
    walk_info.gpe_device = gpe_device;
    walk_info.execute_by_owner_id = FALSE;
    acpi_ns_walk_namespace(ACPI_TYPE_METHOD, gpe_device, ACPI_UINT32_MAX, ACPI_NS_WALK_NO_UNLOCK, acpi_ev_match_gpe_method, core::ptr::null_mut(), &mut walk_info, core::ptr::null_mut());
    if !return_gpe_block.is_null() { *return_gpe_block = gpe_block; }
    acpi_current_gpe_count += (*gpe_block).gpe_count;
    AE_OK
}

pub unsafe fn acpi_ev_initialize_gpe_block(_gpe_xrupt_info: *mut acpi_gpe_xrupt_info, gpe_block: *mut acpi_gpe_block_info, context: *mut core::ffi::c_void) -> acpi_status {
    if gpe_block.is_null() || (*gpe_block).initialized { return AE_OK; }
    let is_polling_needed = context as *mut u8;
    let mut gpe_enabled_count = 0u32;
    for i in 0..(*gpe_block).register_count {
        for j in 0..ACPI_GPE_REGISTER_WIDTH {
            let gpe_index = i * ACPI_GPE_REGISTER_WIDTH + j;
            let gpe_event_info = &mut *(*gpe_block).event_info.add(gpe_index as usize);
            gpe_event_info.flags |= ACPI_GPE_INITIALIZED;
            if ACPI_GPE_DISPATCH_TYPE(gpe_event_info.flags) != ACPI_GPE_DISPATCH_METHOD || (gpe_event_info.flags & ACPI_GPE_CAN_WAKE) != 0 { continue; }
            let status = acpi_ev_add_gpe_reference(gpe_event_info, FALSE);
            if ACPI_FAILURE(status) { continue; }
            gpe_event_info.flags |= ACPI_GPE_AUTO_ENABLED;
            if !is_polling_needed.is_null() && ACPI_GPE_IS_POLLING_NEEDED(gpe_event_info) { *is_polling_needed = TRUE; }
            gpe_enabled_count += 1;
        }
    }
    (*gpe_block).initialized = TRUE;
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
