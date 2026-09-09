// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: nsinit - namespace initialization
//
// Copyright (C) 2000 - 2026, Intel Corp.

// Dependencies supplied by the surrounding ACPICA translation.

pub unsafe fn acpi_ns_initialize_objects() -> acpi_status {
    let status: acpi_status;
    let mut info: acpi_init_walk_info = core::mem::zeroed();

    ACPI_FUNCTION_TRACE!(ns_initialize_objects);
    ACPI_DEBUG_PRINT!((ACPI_DB_EXEC, "[Init] Completing Initialization of ACPI Objects\n"));
    ACPI_DEBUG_PRINT!((ACPI_DB_DISPATCH, "**** Starting initialization of namespace objects ****\n"));
    ACPI_DEBUG_PRINT_RAW!((ACPI_DB_INIT, "Final data object initialization: "));

    status = acpi_walk_namespace(ACPI_TYPE_ANY, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX,
        Some(acpi_ns_init_one_object), core::ptr::null_mut(), &mut info as *mut _ as *mut _, core::ptr::null_mut());
    if ACPI_FAILURE!(status) {
        ACPI_EXCEPTION!((AE_INFO, status, "During WalkNamespace"));
    }
    ACPI_DEBUG_PRINT_RAW!((ACPI_DB_INIT, "Namespace contains %u (0x%X) objects\n", info.object_count, info.object_count));
    ACPI_DEBUG_PRINT!((ACPI_DB_DISPATCH, "%u Control Methods found\n%u Op Regions found\n", info.method_count, info.op_region_count));
    return_ACPI_STATUS!(AE_OK)
}

pub unsafe fn acpi_ns_initialize_devices(flags: u32) -> acpi_status {
    let mut status = AE_OK;
    let mut info: acpi_device_walk_info = core::mem::zeroed();
    let mut handle: acpi_handle = core::ptr::null_mut();

    ACPI_FUNCTION_TRACE!(ns_initialize_devices);
    if (flags & ACPI_NO_DEVICE_INIT) == 0 {
        ACPI_DEBUG_PRINT!((ACPI_DB_EXEC, "[Init] Initializing ACPI Devices\n"));
        info.device_count = 0; info.num_STA = 0; info.num_INI = 0;
        ACPI_DEBUG_PRINT_RAW!((ACPI_DB_INIT, "Initializing Device/Processor/Thermal objects and executing _INI/_STA methods:\n"));
        status = acpi_ns_walk_namespace(ACPI_TYPE_ANY, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, FALSE,
            Some(acpi_ns_find_ini_methods), core::ptr::null_mut(), &mut info as *mut _ as *mut _, core::ptr::null_mut());
        if ACPI_FAILURE!(status) { ACPI_EXCEPTION!((AE_INFO, status, "During device initialization")); return status; }
        info.evaluate_info = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_evaluate_info>()) as *mut acpi_evaluate_info;
        if info.evaluate_info.is_null() { status = AE_NO_MEMORY; ACPI_EXCEPTION!((AE_INFO, status, "During device initialization")); return status; }
        (*info.evaluate_info).prefix_node = acpi_gbl_root_node;
        (*info.evaluate_info).relative_pathname = METHOD_NAME__INI;
        (*info.evaluate_info).parameters = core::ptr::null_mut();
        (*info.evaluate_info).flags = ACPI_IGNORE_RETURN_VALUE;
        status = acpi_ns_evaluate(info.evaluate_info);
        if ACPI_SUCCESS!(status) { info.num_INI += 1; }
        status = acpi_get_handle(core::ptr::null_mut(), "\\_SB".as_ptr() as *const _, &mut handle);
        if ACPI_SUCCESS!(status) {
            core::ptr::write_bytes(info.evaluate_info, 0, 1);
            (*info.evaluate_info).prefix_node = handle;
            (*info.evaluate_info).relative_pathname = METHOD_NAME__INI;
            (*info.evaluate_info).parameters = core::ptr::null_mut();
            (*info.evaluate_info).flags = ACPI_IGNORE_RETURN_VALUE;
            status = acpi_ns_evaluate(info.evaluate_info);
            if ACPI_SUCCESS!(status) { info.num_INI += 1; }
        }
    }
    if (flags & ACPI_NO_ADDRESS_SPACE_INIT) == 0 {
        ACPI_DEBUG_PRINT!((ACPI_DB_EXEC, "[Init] Executing _REG OpRegion methods\n"));
        status = acpi_ev_initialize_op_regions();
        if ACPI_FAILURE!(status) { ACPI_EXCEPTION!((AE_INFO, status, "During device initialization")); return status; }
    }
    if (flags & ACPI_NO_DEVICE_INIT) == 0 {
        status = acpi_ns_walk_namespace(ACPI_TYPE_ANY, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, FALSE,
            Some(acpi_ns_init_one_device), core::ptr::null_mut(), &mut info as *mut _ as *mut _, core::ptr::null_mut());
        if acpi_gbl_osi_data >= ACPI_OSI_WIN_2000 { acpi_gbl_truncate_io_addresses = TRUE; }
        ACPI_FREE!(info.evaluate_info);
        if ACPI_FAILURE!(status) { ACPI_EXCEPTION!((AE_INFO, status, "During device initialization")); return status; }
        ACPI_DEBUG_PRINT_RAW!((ACPI_DB_INIT, "    Executed %u _INI methods requiring %u _STA executions (examined %u objects)\n", info.num_INI, info.num_STA, info.device_count));
    }
    return_ACPI_STATUS!(status)
}

pub unsafe fn acpi_ns_init_one_package(obj_handle: acpi_handle, _level: u32, _context: *mut core::ffi::c_void, _return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    let node = obj_handle as *mut acpi_namespace_node;
    let obj_desc = acpi_ns_get_attached_object(node);
    if obj_desc.is_null() || ((*obj_desc).package.flags & AOPOBJ_DATA_VALID) != 0 { return AE_OK; }
    if ACPI_FAILURE!(acpi_ds_get_package_arguments(obj_desc)) { return AE_OK; }
    if ACPI_FAILURE!(acpi_ut_walk_package_tree(obj_desc, core::ptr::null_mut(), Some(acpi_ds_init_package_element), core::ptr::null_mut())) { return AE_OK; }
    (*obj_desc).package.flags |= AOPOBJ_DATA_VALID;
    AE_OK
}

unsafe fn acpi_ns_init_one_object(obj_handle: acpi_handle, level: u32, context: *mut core::ffi::c_void, _return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    let info = &mut *(context as *mut acpi_init_walk_info);
    let node = obj_handle as *mut acpi_namespace_node;
    info.object_count += 1;
    let object_type = acpi_ns_get_type(obj_handle);
    let obj_desc = acpi_ns_get_attached_object(node);
    if obj_desc.is_null() { return AE_OK; }
    match object_type {
        ACPI_TYPE_REGION => info.op_region_count += 1,
        ACPI_TYPE_BUFFER_FIELD | ACPI_TYPE_LOCAL_BANK_FIELD => info.field_count += 1,
        ACPI_TYPE_BUFFER => info.buffer_count += 1,
        ACPI_TYPE_PACKAGE => info.package_count += 1,
        _ => return AE_OK,
    }
    if ((*obj_desc).common.flags & AOPOBJ_DATA_VALID) != 0 { return AE_OK; }
    acpi_ex_enter_interpreter();
    let status = match object_type {
        ACPI_TYPE_LOCAL_BANK_FIELD => { info.field_init += 1; acpi_ds_get_bank_field_arguments(obj_desc) }
        ACPI_TYPE_PACKAGE => { info.package_init += 1; acpi_ns_init_one_package(obj_handle, level, core::ptr::null_mut(), core::ptr::null_mut()) }
        _ => AE_TYPE,
    };
    if ACPI_FAILURE!(status) { ACPI_EXCEPTION!((AE_INFO, status, "Could not execute arguments for [%4.4s] (%)", acpi_ut_get_node_name(node), acpi_ut_get_type_name(object_type))); }
    acpi_ex_exit_interpreter();
    AE_OK
}

unsafe fn acpi_ns_find_ini_methods(obj_handle: acpi_handle, _nesting_level: u32, context: *mut core::ffi::c_void, _return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    let info = &mut *(context as *mut acpi_device_walk_info);
    let node = obj_handle as *mut acpi_namespace_node;
    if (*node).type_ == ACPI_TYPE_DEVICE || (*node).type_ == ACPI_TYPE_PROCESSOR || (*node).type_ == ACPI_TYPE_THERMAL { info.device_count += 1; return AE_OK; }
    if !ACPI_COMPARE_NAMESEG!((*node).name.ascii, METHOD_NAME__INI) { return AE_OK; }
    let mut parent = (*node).parent;
    if parent.is_null() { return AE_OK; }
    if (*parent).type_ == ACPI_TYPE_DEVICE || (*parent).type_ == ACPI_TYPE_PROCESSOR || (*parent).type_ == ACPI_TYPE_THERMAL {
        while !parent.is_null() { (*parent).flags |= ANOBJ_SUBTREE_HAS_INI; parent = (*parent).parent; }
    }
    AE_OK
}

unsafe fn acpi_ns_init_one_device(obj_handle: acpi_handle, _nesting_level: u32, context: *mut core::ffi::c_void, _return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    let walk_info = &mut *(context as *mut acpi_device_walk_info);
    let info = walk_info.evaluate_info;
    let device_node = obj_handle as *mut acpi_namespace_node;
    if (*device_node).type_ != ACPI_TYPE_DEVICE && (*device_node).type_ != ACPI_TYPE_PROCESSOR && (*device_node).type_ != ACPI_TYPE_THERMAL { return AE_OK; }
    if ((*device_node).flags & ANOBJ_SUBTREE_HAS_INI) == 0 { return AE_CTRL_DEPTH; }
    let mut flags = 0u32;
    if ACPI_FAILURE!(acpi_ut_execute_STA(device_node, &mut flags)) { return AE_OK; }
    if flags != ACPI_UINT32_MAX { walk_info.num_STA += 1; }
    if (flags & ACPI_STA_DEVICE_PRESENT) == 0 {
        if (flags & ACPI_STA_DEVICE_FUNCTIONING) != 0 { return AE_OK; } else { return AE_CTRL_DEPTH; }
    }
    if !ACPI_COMPARE_NAMESEG!((*device_node).name.ascii, "_SB_".as_bytes()) || (*device_node).parent != acpi_gbl_root_node {
        core::ptr::write_bytes(info, 0, 1);
        (*info).prefix_node = device_node;
        (*info).relative_pathname = METHOD_NAME__INI;
        (*info).parameters = core::ptr::null_mut();
        (*info).flags = ACPI_IGNORE_RETURN_VALUE;
        if ACPI_SUCCESS!(acpi_ns_evaluate(info)) { walk_info.num_INI += 1; }
    }
    let mut status = AE_OK;
    if !acpi_gbl_init_handler.is_none() { status = acpi_gbl_init_handler.unwrap()(device_node, ACPI_INIT_DEVICE_INI); }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
