// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: evgpeinit - System GPE initialization and update
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies are supplied by the surrounding ACPICA translation.

// Entire module is excluded when ACPI_REDUCED_HARDWARE is enabled.

/*
 * Note: History of _PRW support in ACPICA
 *
 * Originally (2000 - 2010), the GPE initialization code performed a walk of
 * the entire namespace to execute the _PRW methods and detect all GPEs
 * capable of waking the system.
 *
 * As of 10/2010, the _PRW method execution has been removed since it is
 * actually unnecessary. The host OS must in fact execute all _PRW methods
 * in order to identify the device/power-resource dependencies. We now put
 * the onus on the host OS to identify the wake GPEs as part of this process
 * and to inform ACPICA of these GPEs via the acpi_setup_gpe_for_wake interface. This
 * not only reduces the complexity of the ACPICA initialization code, but in
 * some cases (on systems with very large namespaces) it should reduce the
 * kernel boot time as well.
 */

// ACPI_FADT_GPE_BLOCK_ADDRESS(N) is represented by the corresponding FADT
// address expression at each call site; logical-address builds retain the
// conditional intent from the original macro.

/// Initialize the GPE data structures and the FADT GPE 0/1 blocks.
pub unsafe fn acpi_ev_gpe_initialize() -> acpi_status {
    let mut register_count0: u32 = 0;
    let mut register_count1: u32 = 0;
    let mut gpe_number_max: u32 = 0;
    let status: acpi_status;
    let mut address: u64;

    ACPI_FUNCTION_TRACE!(ev_gpe_initialize);
    ACPI_DEBUG_PRINT_RAW!((ACPI_DB_INIT,
        "Initializing General Purpose Events (GPEs):\n"));

    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE(status) {
        return status;
    }

    address = acpi_gbl_FADT.xgpe0_block.address;
    if acpi_gbl_FADT.gpe0_block_length != 0 && address != 0 {
        register_count0 = (acpi_gbl_FADT.gpe0_block_length / 2) as u32;
        gpe_number_max = register_count0 * ACPI_GPE_REGISTER_WIDTH - 1;
        let status = acpi_ev_create_gpe_block(
            acpi_gbl_fadt_gpe_device, address, acpi_gbl_FADT.xgpe0_block.space_id,
            register_count0, 0, acpi_gbl_FADT.sci_interrupt,
            &mut acpi_gbl_gpe_fadt_blocks[0]);
        if ACPI_FAILURE(status) {
            ACPI_EXCEPTION!((AE_INFO, status, "Could not create GPE Block 0"));
        }
    }

    address = acpi_gbl_FADT.xgpe1_block.address;
    if acpi_gbl_FADT.gpe1_block_length != 0 && address != 0 {
        register_count1 = (acpi_gbl_FADT.gpe1_block_length / 2) as u32;
        if register_count0 != 0 && gpe_number_max >= acpi_gbl_FADT.gpe1_base {
            ACPI_ERROR!((AE_INFO,
                "GPE0 block (GPE 0 to %u) overlaps the GPE1 block (GPE %u to %u) - Ignoring GPE1",
                gpe_number_max, acpi_gbl_FADT.gpe1_base,
                acpi_gbl_FADT.gpe1_base + register_count1 * ACPI_GPE_REGISTER_WIDTH - 1));
            register_count1 = 0;
        } else {
            let status = acpi_ev_create_gpe_block(
                acpi_gbl_fadt_gpe_device, address, acpi_gbl_FADT.xgpe1_block.space_id,
                register_count1, acpi_gbl_FADT.gpe1_base, acpi_gbl_FADT.sci_interrupt,
                &mut acpi_gbl_gpe_fadt_blocks[1]);
            if ACPI_FAILURE(status) {
                ACPI_EXCEPTION!((AE_INFO, status, "Could not create GPE Block 1"));
            }
        }
    }

    if register_count0 + register_count1 == 0 {
        ACPI_DEBUG_PRINT!((ACPI_DB_INIT,
            "There are no GPE blocks defined in the FADT\n"));
    }
    let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    AE_OK
}

pub unsafe fn acpi_ev_update_gpes(table_owner_id: acpi_owner_id) {
    let mut status = AE_OK;
    let mut walk_info: acpi_gpe_walk_info = core::mem::zeroed();
    status = acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);
    if ACPI_FAILURE(status) { return; }
    walk_info.count = 0;
    walk_info.owner_id = table_owner_id;
    walk_info.execute_by_owner_id = TRUE;

    let mut gpe_xrupt_info = acpi_gbl_gpe_xrupt_list_head;
    while !gpe_xrupt_info.is_null() {
        let mut gpe_block = (*gpe_xrupt_info).gpe_block_list_head;
        while !gpe_block.is_null() {
            walk_info.gpe_block = gpe_block;
            walk_info.gpe_device = (*gpe_block).node;
            status = acpi_ns_walk_namespace(ACPI_TYPE_METHOD, walk_info.gpe_device,
                ACPI_UINT32_MAX, ACPI_NS_WALK_NO_UNLOCK, acpi_ev_match_gpe_method,
                core::ptr::null_mut(), &mut walk_info, core::ptr::null_mut());
            if ACPI_FAILURE(status) {
                ACPI_EXCEPTION!((AE_INFO, status, "While decoding _Lxx/_Exx methods"));
            }
            gpe_block = (*gpe_block).next;
        }
        gpe_xrupt_info = (*gpe_xrupt_info).next;
    }
    if walk_info.count != 0 { ACPI_INFO!(("Enabled %u new GPEs", walk_info.count)); }
    let _ = acpi_ut_release_mutex(ACPI_MTX_EVENTS);
}

pub unsafe fn acpi_ev_match_gpe_method(
    obj_handle: acpi_handle, _level: u32, context: *mut core::ffi::c_void,
    _return_value: *mut *mut core::ffi::c_void,
) -> acpi_status {
    let method_node = obj_handle as *mut acpi_namespace_node;
    let walk_info = context as *mut acpi_gpe_walk_info;
    if (*walk_info).execute_by_owner_id && (*method_node).owner_id != (*walk_info).owner_id {
        return AE_OK;
    }
    let name = (*method_node).name.integer.to_ne_bytes();
    if name[0] != b'_' { return AE_OK; }
    let type_ = match name[1] { b'L' => ACPI_GPE_LEVEL_TRIGGERED, b'E' => ACPI_GPE_EDGE_TRIGGERED,
        _ => return AE_OK };
    let mut temp_gpe_number: u8 = 0;
    let mut name_text = [0i8; ACPI_NAMESEG_SIZE as usize + 1];
    for (i, byte) in name.iter().enumerate() { name_text[i] = *byte as i8; }
    if ACPI_FAILURE(acpi_ut_ascii_to_hex_byte(name_text.as_mut_ptr().add(2), &mut temp_gpe_number)) {
        return AE_OK;
    }
    let gpe_event_info = acpi_ev_low_get_gpe_info(temp_gpe_number as u32, (*walk_info).gpe_block);
    if gpe_event_info.is_null() { return AE_OK; }
    let dispatch = ACPI_GPE_DISPATCH_TYPE((*gpe_event_info).flags);
    if dispatch == ACPI_GPE_DISPATCH_HANDLER || dispatch == ACPI_GPE_DISPATCH_RAW_HANDLER { return AE_OK; }
    if dispatch == ACPI_GPE_DISPATCH_METHOD {
        if type_ != ((*gpe_event_info).flags & ACPI_GPE_XRUPT_TYPE_MASK) {
            ACPI_ERROR!((AE_INFO, "For GPE 0x%.2X, found both _L%2.2X and _E%2.2X methods",
                temp_gpe_number, temp_gpe_number, temp_gpe_number));
        }
        return AE_OK;
    }
    let _ = acpi_hw_low_set_gpe(gpe_event_info, ACPI_GPE_DISABLE);
    (*gpe_event_info).flags &= !ACPI_GPE_DISPATCH_MASK;
    (*gpe_event_info).flags |= type_ as u8 | ACPI_GPE_DISPATCH_METHOD;
    (*gpe_event_info).dispatch.method_node = method_node;
    (*walk_info).count += 1;
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
