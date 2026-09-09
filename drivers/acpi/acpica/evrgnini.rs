// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 * Module Name: evrgnini - ACPI address_space (op_region) init
 * Copyright (C) 2000 - 2026, Intel Corp.
 ******************************************************************************/

// C dependencies supplied by the ACPI implementation are intentionally left
// as external Rust items.

pub unsafe fn acpi_ev_system_memory_region_setup(
    handle: acpi_handle,
    function: u32,
    _handler_context: *mut core::ffi::c_void,
    region_context: *mut *mut core::ffi::c_void,
) -> acpi_status {
    let region_desc = handle as *mut acpi_operand_object;
    let mut local_region_context: *mut acpi_mem_space_context;
    let mut mm: *mut acpi_mem_mapping;

    if function == ACPI_REGION_DEACTIVATE {
        if !(*region_context).is_null() {
            local_region_context = *region_context as *mut acpi_mem_space_context;
            while !(*local_region_context).first_mm.is_null() {
                mm = (*local_region_context).first_mm;
                (*local_region_context).first_mm = (*mm).next_mm;
                acpi_os_unmap_memory((*mm).logical_address, (*mm).length);
                ACPI_FREE(mm as *mut core::ffi::c_void);
            }
            ACPI_FREE(local_region_context as *mut core::ffi::c_void);
            *region_context = core::ptr::null_mut();
        }
        return AE_OK;
    }

    local_region_context = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_mem_space_context>())
        as *mut acpi_mem_space_context;
    if local_region_context.is_null() {
        return AE_NO_MEMORY;
    }
    (*local_region_context).length = (*region_desc).region.length;
    (*local_region_context).address = (*region_desc).region.address;
    *region_context = local_region_context as *mut core::ffi::c_void;
    AE_OK
}

pub unsafe fn acpi_ev_io_space_region_setup(
    _handle: acpi_handle,
    function: u32,
    handler_context: *mut core::ffi::c_void,
    region_context: *mut *mut core::ffi::c_void,
) -> acpi_status {
    if function == ACPI_REGION_DEACTIVATE {
        *region_context = core::ptr::null_mut();
    } else {
        *region_context = handler_context;
    }
    AE_OK
}

pub unsafe fn acpi_ev_pci_config_region_setup(
    handle: acpi_handle,
    function: u32,
    _handler_context: *mut core::ffi::c_void,
    region_context: *mut *mut core::ffi::c_void,
) -> acpi_status {
    let mut status = AE_OK;
    let mut pci_value: u64 = 0;
    let mut pci_id = *region_context as *mut acpi_pci_id;
    let region_obj = handle as *mut acpi_operand_object;
    let handler_obj = (*region_obj).region.handler;
    let parent_node: *mut acpi_namespace_node;
    let pci_root_node: *mut acpi_namespace_node;
    let mut pci_device_node: *mut acpi_namespace_node;

    if handler_obj.is_null() {
        return AE_NOT_EXIST;
    }
    *region_context = core::ptr::null_mut();
    if function == ACPI_REGION_DEACTIVATE {
        if !pci_id.is_null() { ACPI_FREE(pci_id as *mut core::ffi::c_void); }
        return status;
    }

    parent_node = (*region_obj).region.node.parent;
    let mut root = if (*handler_obj).address_space.node == acpi_gbl_root_node {
        let mut p = parent_node;
        while p != acpi_gbl_root_node {
            if acpi_ev_is_pci_root_bridge(p) != 0 {
                status = acpi_install_address_space_handler(
                    p as acpi_handle, ACPI_ADR_SPACE_PCI_CONFIG,
                    ACPI_DEFAULT_HANDLER, core::ptr::null_mut(), core::ptr::null_mut());
                if ACPI_FAILURE(status) && status != AE_SAME_HANDLER {
                    ACPI_EXCEPTION(status);
                }
                break;
            }
            p = (*p).parent;
        }
        p
    } else { (*handler_obj).address_space.node };
    pci_root_node = root;

    if (*region_obj).region.flags & AOPOBJ_SETUP_COMPLETE != 0 { return AE_OK; }
    pci_id = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_pci_id>()) as *mut acpi_pci_id;
    if pci_id.is_null() { return AE_NO_MEMORY; }

    pci_device_node = (*region_obj).region.node;
    while !pci_device_node.is_null() && (*pci_device_node).type_ != ACPI_TYPE_DEVICE {
        pci_device_node = (*pci_device_node).parent;
    }
    if pci_device_node.is_null() {
        ACPI_FREE(pci_id as *mut core::ffi::c_void);
        return AE_AML_OPERAND_TYPE;
    }
    status = acpi_ut_evaluate_numeric_object(METHOD_NAME__ADR, pci_device_node, &mut pci_value);
    if ACPI_SUCCESS(status) {
        (*pci_id).device = ACPI_HIWORD(ACPI_LODWORD(pci_value));
        (*pci_id).function = ACPI_LOWORD(ACPI_LODWORD(pci_value));
    }
    status = acpi_ut_evaluate_numeric_object(METHOD_NAME__SEG, pci_root_node, &mut pci_value);
    if ACPI_SUCCESS(status) { (*pci_id).segment = ACPI_LOWORD(pci_value); }
    status = acpi_ut_evaluate_numeric_object(METHOD_NAME__BBN, pci_root_node, &mut pci_value);
    if ACPI_SUCCESS(status) { (*pci_id).bus = ACPI_LOWORD(pci_value); }
    status = acpi_hw_derive_pci_id(pci_id, pci_root_node, (*region_obj).region.node);
    if ACPI_FAILURE(status) {
        ACPI_FREE(pci_id as *mut core::ffi::c_void);
        return status;
    }
    *region_context = pci_id as *mut core::ffi::c_void;
    AE_OK
}

pub unsafe fn acpi_ev_is_pci_root_bridge(node: *mut acpi_namespace_node) -> u8 {
    let mut hid: *mut acpi_pnp_device_id;
    let mut cid: *mut acpi_pnp_device_id_list;
    let status = acpi_ut_execute_HID(node, &mut hid);
    if ACPI_FAILURE(status) { return FALSE; }
    let matched = acpi_ut_is_pci_root_bridge((*hid).string);
    ACPI_FREE(hid as *mut core::ffi::c_void);
    if matched != 0 { return TRUE; }
    let status = acpi_ut_execute_CID(node, &mut cid);
    if ACPI_FAILURE(status) { return FALSE; }
    for i in 0..(*cid).count {
        if acpi_ut_is_pci_root_bridge((*cid).ids[i as usize].string) != 0 {
            ACPI_FREE(cid as *mut core::ffi::c_void);
            return TRUE;
        }
    }
    ACPI_FREE(cid as *mut core::ffi::c_void);
    FALSE
}

pub unsafe fn acpi_ev_pci_bar_region_setup(_: acpi_handle, _: u32, _: *mut core::ffi::c_void, _: *mut *mut core::ffi::c_void) -> acpi_status { AE_OK }
pub unsafe fn acpi_ev_cmos_region_setup(_: acpi_handle, _: u32, _: *mut core::ffi::c_void, _: *mut *mut core::ffi::c_void) -> acpi_status { AE_OK }

pub unsafe fn acpi_ev_data_table_region_setup(
    handle: acpi_handle, function: u32, _: *mut core::ffi::c_void,
    region_context: *mut *mut core::ffi::c_void,
) -> acpi_status {
    let region_desc = handle as *mut acpi_operand_object;
    if function == ACPI_REGION_DEACTIVATE {
        if !(*region_context).is_null() { ACPI_FREE(*region_context); *region_context = core::ptr::null_mut(); }
        return AE_OK;
    }
    let context = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_data_table_mapping>()) as *mut acpi_data_table_mapping;
    if context.is_null() { return AE_NO_MEMORY; }
    (*context).pointer = (*region_desc).region.pointer;
    *region_context = context as *mut core::ffi::c_void;
    AE_OK
}

pub unsafe fn acpi_ev_default_region_setup(_: acpi_handle, function: u32, handler_context: *mut core::ffi::c_void, region_context: *mut *mut core::ffi::c_void) -> acpi_status {
    if function == ACPI_REGION_DEACTIVATE { *region_context = core::ptr::null_mut(); } else { *region_context = handler_context; }
    AE_OK
}

pub unsafe fn acpi_ev_initialize_region(region_obj: *mut acpi_operand_object) -> acpi_status {
    if region_obj.is_null() { return AE_BAD_PARAMETER; }
    if (*region_obj).common.flags & AOPOBJ_OBJECT_INITIALIZED != 0 { return AE_OK; }
    (*region_obj).common.flags |= AOPOBJ_OBJECT_INITIALIZED;
    let space_id = (*region_obj).region.space_id;
    let mut node = (*region_obj).region.node.parent;
    while !node.is_null() {
        let mut handler_obj = core::ptr::null_mut();
        let obj_desc = acpi_ns_get_attached_object(node);
        if !obj_desc.is_null() {
            match (*node).type_ {
                ACPI_TYPE_DEVICE | ACPI_TYPE_PROCESSOR | ACPI_TYPE_THERMAL => handler_obj = (*obj_desc).common_notify.handler,
                _ => {}
            }
            handler_obj = acpi_ev_find_region_handler(space_id, handler_obj);
            if !handler_obj.is_null() {
                acpi_ev_attach_region(handler_obj, region_obj, FALSE);
                acpi_ex_exit_interpreter();
                acpi_ev_execute_reg_method(region_obj, ACPI_REG_CONNECT);
                acpi_ex_enter_interpreter();
                return AE_OK;
            }
        }
        node = (*node).parent;
    }
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
