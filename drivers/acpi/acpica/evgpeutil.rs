// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: evgpeutil - GPE utilities
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Includes: acpi/acpi.h, accommon.h, acevents.h
// _COMPONENT: ACPI_EVENTS
// ACPI_MODULE_NAME("evgpeutil")

// Entire module is excluded when ACPI_REDUCED_HARDWARE is enabled.

/** Walk the GPE lists. */
pub unsafe fn acpi_ev_walk_gpe_list(
    gpe_walk_callback: acpi_gpe_callback,
    context: *mut core::ffi::c_void,
) -> acpi_status {
    let mut gpe_xrupt_info: *mut acpi_gpe_xrupt_info;
    let mut gpe_block: *mut acpi_gpe_block_info;
    let mut status: acpi_status = AE_OK;
    let flags: acpi_cpu_flags;

    // ACPI_FUNCTION_TRACE(ev_walk_gpe_list)
    flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);

    /* Walk the interrupt level descriptor list */
    gpe_xrupt_info = acpi_gbl_gpe_xrupt_list_head;
    while !gpe_xrupt_info.is_null() {
        /* Walk all Gpe Blocks attached to this interrupt level */
        gpe_block = (*gpe_xrupt_info).gpe_block_list_head;
        while !gpe_block.is_null() {
            /* One callback per GPE block */
            status = gpe_walk_callback(gpe_xrupt_info, gpe_block, context);
            if ACPI_FAILURE(status) {
                if status == AE_CTRL_END {
                    status = AE_OK;
                }
                break;
            }
            gpe_block = (*gpe_block).next;
        }
        if ACPI_FAILURE(status) {
            break;
        }
        gpe_xrupt_info = (*gpe_xrupt_info).next;
    }

    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    status
}

/** Match a GPE index with a GPE block device. */
pub unsafe fn acpi_ev_get_gpe_device(
    _gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
    gpe_block: *mut acpi_gpe_block_info,
    context: *mut core::ffi::c_void,
) -> acpi_status {
    let info = context as *mut acpi_gpe_device_info;

    (*info).next_block_base_index += (*gpe_block).gpe_count;
    if (*info).index < (*info).next_block_base_index {
        if (*(*gpe_block).node).type_ == ACPI_TYPE_DEVICE {
            (*info).gpe_device = (*gpe_block).node;
        }
        (*info).status = AE_OK;
        return AE_CTRL_END;
    }
    AE_OK
}

/** Get or create a GPE interrupt block. */
pub unsafe fn acpi_ev_get_gpe_xrupt_block(
    interrupt_number: u32,
    gpe_xrupt_block: *mut *mut acpi_gpe_xrupt_info,
) -> acpi_status {
    let mut next_gpe_xrupt = acpi_gbl_gpe_xrupt_list_head;
    let gpe_xrupt: *mut acpi_gpe_xrupt_info;
    let status: acpi_status;
    let flags: acpi_cpu_flags;

    while !next_gpe_xrupt.is_null() {
        if (*next_gpe_xrupt).interrupt_number == interrupt_number {
            *gpe_xrupt_block = next_gpe_xrupt;
            return AE_OK;
        }
        next_gpe_xrupt = (*next_gpe_xrupt).next;
    }

    gpe_xrupt = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_gpe_xrupt_info>()) as *mut acpi_gpe_xrupt_info;
    if gpe_xrupt.is_null() {
        return AE_NO_MEMORY;
    }
    (*gpe_xrupt).interrupt_number = interrupt_number;

    flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
    if !acpi_gbl_gpe_xrupt_list_head.is_null() {
        next_gpe_xrupt = acpi_gbl_gpe_xrupt_list_head;
        while !(*next_gpe_xrupt).next.is_null() {
            next_gpe_xrupt = (*next_gpe_xrupt).next;
        }
        (*next_gpe_xrupt).next = gpe_xrupt;
        (*gpe_xrupt).previous = next_gpe_xrupt;
    } else {
        acpi_gbl_gpe_xrupt_list_head = gpe_xrupt;
    }
    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);

    if interrupt_number != acpi_gbl_FADT.sci_interrupt {
        status = acpi_os_install_interrupt_handler(
            interrupt_number, acpi_ev_gpe_xrupt_handler, gpe_xrupt);
        if ACPI_FAILURE(status) {
            ACPI_EXCEPTION((AE_INFO, status,
                "Could not install GPE interrupt handler at level 0x%X",
                interrupt_number));
            return status;
        }
    }
    *gpe_xrupt_block = gpe_xrupt;
    AE_OK
}

/** Remove and free a GPE interrupt block. */
pub unsafe fn acpi_ev_delete_gpe_xrupt(gpe_xrupt: *mut acpi_gpe_xrupt_info) -> acpi_status {
    let status: acpi_status;
    let flags: acpi_cpu_flags;

    if (*gpe_xrupt).interrupt_number == acpi_gbl_FADT.sci_interrupt {
        (*gpe_xrupt).gpe_block_list_head = core::ptr::null_mut();
        return AE_OK;
    }
    status = acpi_os_remove_interrupt_handler(
        (*gpe_xrupt).interrupt_number, acpi_ev_gpe_xrupt_handler);
    if ACPI_FAILURE(status) {
        return status;
    }

    flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
    if !(*gpe_xrupt).previous.is_null() {
        (*(*gpe_xrupt).previous).next = (*gpe_xrupt).next;
    } else {
        acpi_gbl_gpe_xrupt_list_head = (*gpe_xrupt).next;
    }
    if !(*gpe_xrupt).next.is_null() {
        (*(*gpe_xrupt).next).previous = (*gpe_xrupt).previous;
    }
    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    ACPI_FREE(gpe_xrupt as *mut core::ffi::c_void);
    AE_OK
}

/** Delete all handler objects found in the GPE data structures. */
pub unsafe fn acpi_ev_delete_gpe_handlers(
    _gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
    gpe_block: *mut acpi_gpe_block_info,
    _context: *mut core::ffi::c_void,
) -> acpi_status {
    for i in 0..(*gpe_block).register_count {
        for j in 0..ACPI_GPE_REGISTER_WIDTH {
            let gpe_event_info = &mut *(*gpe_block).event_info.add(
                (i as usize) * (ACPI_GPE_REGISTER_WIDTH as usize) + (j as usize));
            if ACPI_GPE_DISPATCH_TYPE(gpe_event_info.flags) == ACPI_GPE_DISPATCH_HANDLER
                || ACPI_GPE_DISPATCH_TYPE(gpe_event_info.flags) == ACPI_GPE_DISPATCH_RAW_HANDLER {
                ACPI_FREE(gpe_event_info.dispatch.handler as *mut core::ffi::c_void);
                gpe_event_info.dispatch.handler = core::ptr::null_mut();
                gpe_event_info.flags &= !ACPI_GPE_DISPATCH_MASK;
            } else if ACPI_GPE_DISPATCH_TYPE(gpe_event_info.flags) == ACPI_GPE_DISPATCH_NOTIFY {
                let mut notify = gpe_event_info.dispatch.notify_list;
                while !notify.is_null() {
                    let next = (*notify).next;
                    ACPI_FREE(notify as *mut core::ffi::c_void);
                    notify = next;
                }
                gpe_event_info.dispatch.notify_list = core::ptr::null_mut();
                gpe_event_info.flags &= !ACPI_GPE_DISPATCH_MASK;
            }
        }
    }
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
