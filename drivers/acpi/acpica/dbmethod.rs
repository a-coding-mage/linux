// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: dbmethod - Debug commands for control methods

// Dependencies are supplied by the surrounding ACPICA translation unit.

pub unsafe fn acpi_db_set_method_breakpoint(
    location: *mut i8,
    walk_state: *mut acpi_walk_state,
    op: *mut acpi_parse_object,
) {
    let address: u32;
    let aml_offset: u32;

    if op.is_null() {
        acpi_os_printf(b"There is no method currently executing\0".as_ptr() as *const i8);
        return;
    }

    address = strtoul(location, core::ptr::null_mut(), 16) as u32;
    aml_offset = acpi_ptr_diff((*op).common.aml, (*walk_state).parser_state.aml_start) as u32;
    if address <= aml_offset {
        acpi_os_printf(b"Breakpoint %X is beyond current address %X\0".as_ptr() as *const i8, address, aml_offset);
    }

    (*walk_state).user_breakpoint = address;
    acpi_os_printf(b"Breakpoint set at AML offset %X\0".as_ptr() as *const i8, address);
}

pub unsafe fn acpi_db_set_method_call_breakpoint(op: *mut acpi_parse_object) {
    if op.is_null() {
        acpi_os_printf(b"There is no method currently executing\0".as_ptr() as *const i8);
        return;
    }
    acpi_gbl_step_to_next_call = TRUE;
}

pub unsafe fn acpi_db_set_method_data(
    type_arg: *mut i8,
    index_arg: *mut i8,
    value_arg: *mut i8,
) {
    let type_: i8;
    let index: u32;
    let value: u32;
    let walk_state: *mut acpi_walk_state;
    let mut obj_desc: *mut acpi_operand_object;
    let status: acpi_status;
    let node: *mut acpi_namespace_node;

    acpi_ut_strupr(type_arg);
    type_ = *type_arg;
    if type_ != b'L' as i8 && type_ != b'A' as i8 && type_ != b'N' as i8 {
        acpi_os_printf(b"Invalid SET operand: %s\0".as_ptr() as *const i8, type_arg);
        return;
    }
    value = strtoul(value_arg, core::ptr::null_mut(), 16) as u32;

    if type_ == b'N' as i8 {
        node = acpi_db_convert_to_node(index_arg);
        if node.is_null() { return; }
        if (*node).type_ != ACPI_TYPE_INTEGER {
            acpi_os_printf(b"Can only set Integer nodes\0".as_ptr() as *const i8);
            return;
        }
        obj_desc = (*node).object;
        (*obj_desc).integer.value = value as u64;
        return;
    }

    index = strtoul(index_arg, core::ptr::null_mut(), 16) as u32;
    walk_state = acpi_ds_get_current_walk_state(acpi_gbl_current_walk_list);
    if walk_state.is_null() {
        acpi_os_printf(b"There is no method currently executing\0".as_ptr() as *const i8);
        return;
    }
    obj_desc = acpi_ut_create_integer_object(value as u64);
    if obj_desc.is_null() {
        acpi_os_printf(b"Could not create an internal object\0".as_ptr() as *const i8);
        return;
    }

    match type_ as u8 {
        b'A' => {
            if index > ACPI_METHOD_MAX_ARG { acpi_os_printf(b"Arg%u - Invalid argument name\0".as_ptr() as *const i8, index); acpi_ut_remove_reference(obj_desc); return; }
            status = acpi_ds_store_object_to_local(ACPI_REFCLASS_ARG, index, obj_desc, walk_state);
            if ACPI_FAILURE(status) { acpi_ut_remove_reference(obj_desc); return; }
            obj_desc = (*walk_state).arguments[index as usize].object;
            acpi_os_printf(b"Arg%u: \0".as_ptr() as *const i8, index);
            acpi_db_display_internal_object(obj_desc, walk_state);
        }
        b'L' => {
            if index > ACPI_METHOD_MAX_LOCAL { acpi_os_printf(b"Local%u - Invalid local variable name\0".as_ptr() as *const i8, index); acpi_ut_remove_reference(obj_desc); return; }
            status = acpi_ds_store_object_to_local(ACPI_REFCLASS_LOCAL, index, obj_desc, walk_state);
            if ACPI_FAILURE(status) { acpi_ut_remove_reference(obj_desc); return; }
            obj_desc = (*walk_state).local_variables[index as usize].object;
            acpi_os_printf(b"Local%u: \0".as_ptr() as *const i8, index);
            acpi_db_display_internal_object(obj_desc, walk_state);
        }
        _ => {}
    }
    acpi_ut_remove_reference(obj_desc);
}

#[cfg(feature = "ACPI_DISASSEMBLER")]
pub unsafe fn acpi_db_disassemble_aml(statements: *mut i8, op: *mut acpi_parse_object) {
    let mut num_statements: u32 = 8;
    if op.is_null() { acpi_os_printf(b"There is no method currently executing\0".as_ptr() as *const i8); return; }
    if !statements.is_null() { num_statements = strtoul(statements, core::ptr::null_mut(), 0) as u32; }
    acpi_dm_disassemble(core::ptr::null_mut(), op, num_statements);
}

#[cfg(feature = "ACPI_DISASSEMBLER")]
pub unsafe fn acpi_db_disassemble_method(name: *mut i8) -> acpi_status {
    let mut status: acpi_status;
    let method = acpi_db_convert_to_node(name);
    if method.is_null() { return AE_BAD_PARAMETER; }
    if (*method).type_ != ACPI_TYPE_METHOD { return AE_BAD_PARAMETER; }
    let obj_desc = (*method).object;
    let op = acpi_ps_create_scope_op((*obj_desc).method.aml_start);
    if op.is_null() { return AE_NO_MEMORY; }
    let walk_state = acpi_ds_create_walk_state(0, op, core::ptr::null_mut(), core::ptr::null_mut());
    if walk_state.is_null() { return AE_NO_MEMORY; }
    status = acpi_ds_init_aml_walk(walk_state, op, core::ptr::null_mut(), (*obj_desc).method.aml_start, (*obj_desc).method.aml_length, core::ptr::null_mut(), ACPI_IMODE_LOAD_PASS1);
    if ACPI_FAILURE(status) { return status; }
    status = acpi_ut_allocate_owner_id(&mut (*obj_desc).method.owner_id);
    if ACPI_FAILURE(status) { return status; }
    (*walk_state).owner_id = (*obj_desc).method.owner_id;
    status = acpi_ds_scope_stack_push(method, (*method).type_, walk_state);
    if ACPI_FAILURE(status) { return status; }
    (*walk_state).parse_flags &= !ACPI_PARSE_DELETE_TREE;
    (*walk_state).parse_flags |= ACPI_PARSE_DISASSEMBLE;
    status = acpi_ps_parse_aml(walk_state);
    if ACPI_FAILURE(status) { return status; }
    acpi_dm_parse_deferred_ops(op);
    acpi_gbl_dm_opt_verbose = FALSE;
    acpi_dm_disassemble(core::ptr::null_mut(), op, 0);
    acpi_gbl_dm_opt_verbose = TRUE;
    acpi_ps_delete_parse_tree(op);
    acpi_ns_delete_namespace_subtree(method);
    acpi_ns_delete_namespace_by_owner((*obj_desc).method.owner_id);
    acpi_ut_release_owner_id(&mut (*obj_desc).method.owner_id);
    AE_OK
}

unsafe fn acpi_db_evaluate_object(node: *mut acpi_namespace_node) -> acpi_status {
    let pathname = acpi_ns_get_external_pathname(node);
    if pathname.is_null() { return AE_OK; }
    let mut obj_info: *mut acpi_device_info = core::ptr::null_mut();
    let status = acpi_get_object_info(node, &mut obj_info);
    if ACPI_FAILURE(status) { ACPI_FREE(pathname); return status; }
    let mut params: [acpi_object; ACPI_METHOD_NUM_ARGS as usize] = core::mem::zeroed();
    let mut param_objects = acpi_object_list { pointer: core::ptr::null_mut(), count: 0 };
    if (*obj_info).type_ == ACPI_TYPE_METHOD {
        for i in 0..(*obj_info).param_count { params[i as usize].type_ = ACPI_TYPE_INTEGER; params[i as usize].integer.value = 1; }
        param_objects.pointer = params.as_mut_ptr(); param_objects.count = (*obj_info).param_count;
    }
    ACPI_FREE(obj_info);
    let mut return_obj = acpi_buffer { pointer: core::ptr::null_mut(), length: ACPI_ALLOCATE_BUFFER };
    acpi_gbl_method_executing = TRUE;
    let status = acpi_evaluate_object(node, core::ptr::null_mut(), &mut param_objects, &mut return_obj);
    acpi_gbl_method_executing = FALSE;
    acpi_os_printf(b"%-32s returned %s\0".as_ptr() as *const i8, pathname, acpi_format_exception(status));
    if return_obj.length != 0 { acpi_db_dump_external_object(return_obj.pointer, 1); acpi_os_printf(b"\n\0".as_ptr() as *const i8); }
    ACPI_FREE(pathname);
    AE_OK
}

unsafe fn acpi_db_walk_for_execute(obj_handle: acpi_handle, _nesting_level: u32, context: *mut core::ffi::c_void, _return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    let node = obj_handle as *mut acpi_namespace_node;
    let info = context as *mut acpi_db_execute_walk;
    let predefined = acpi_ut_match_predefined_method((*node).name.ascii.as_ptr());
    if predefined.is_null() || (*node).type_ == ACPI_TYPE_LOCAL_SCOPE { return AE_OK; }
    acpi_db_evaluate_object(node);
    (*info).count += 1;
    if (*info).count >= (*info).max_count { AE_CTRL_TERMINATE } else { AE_OK }
}

unsafe fn acpi_db_walk_for_execute_all(obj_handle: acpi_handle, _nesting_level: u32, context: *mut core::ffi::c_void, _return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    let node = obj_handle as *mut acpi_namespace_node;
    let info = context as *mut acpi_db_execute_walk;
    if !acpi_compare_nameseg((*node).name.ascii.as_ptr(), (*info).name_seg.as_ptr()) || (*node).type_ == ACPI_TYPE_LOCAL_SCOPE { return AE_OK; }
    acpi_db_evaluate_object(node);
    (*info).count += 1;
    AE_OK
}

pub unsafe fn acpi_db_evaluate_predefined_names() {
    let mut info = acpi_db_execute_walk { count: 0, max_count: ACPI_UINT32_MAX };
    acpi_walk_namespace(ACPI_TYPE_ANY, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, acpi_db_walk_for_execute, core::ptr::null_mut(), &mut info as *mut _ as *mut core::ffi::c_void, core::ptr::null_mut());
    acpi_os_printf(b"Evaluated %u predefined names in the namespace\n\0".as_ptr() as *const i8, info.count);
}

pub unsafe fn acpi_db_evaluate_all(name_seg: *mut i8) {
    let mut info = acpi_db_execute_walk { count: 0, max_count: ACPI_UINT32_MAX, name_seg: [0; ACPI_NAMESEG_SIZE as usize + 1] };
    acpi_copy_nameseg(info.name_seg.as_mut_ptr(), name_seg);
    info.name_seg[ACPI_NAMESEG_SIZE as usize] = 0;
    acpi_walk_namespace(ACPI_TYPE_ANY, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, acpi_db_walk_for_execute_all, core::ptr::null_mut(), &mut info as *mut _ as *mut core::ffi::c_void, core::ptr::null_mut());
    acpi_os_printf(b"Evaluated %u names in the namespace\n\0".as_ptr() as *const i8, info.count);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
