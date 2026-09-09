// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: evregion - Operation Region support

// Dependencies are supplied by the ACPICA translation unit.

extern "C" {
    static mut acpi_gbl_default_address_spaces: [u8; ACPI_NUM_DEFAULT_SPACES as usize];
}

unsafe fn acpi_ev_initialize_op_regions() -> acpi_status {
    let status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE(status) { return status; }
    for i in 0..ACPI_NUM_DEFAULT_SPACES {
        if acpi_ev_has_default_handler(acpi_gbl_root_node,
            acpi_gbl_default_address_spaces[i as usize]) {
            acpi_ev_execute_reg_methods(acpi_gbl_root_node, ACPI_UINT32_MAX,
                acpi_gbl_default_address_spaces[i as usize], ACPI_REG_CONNECT);
        }
    }
    let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    status
}

unsafe fn acpi_ev_address_space_dispatch(
    region_obj: *mut acpi_operand_object, field_obj: *mut acpi_operand_object,
    function: u32, mut region_offset: u32, mut bit_width: u32, value: *mut u64) -> acpi_status {
    let region_obj2 = acpi_ns_get_secondary_object(region_obj);
    if region_obj2.is_null() { return AE_NOT_EXIST; }
    let handler_desc = (*region_obj).region.handler;
    if handler_desc.is_null() { return AE_NOT_EXIST; }
    let context = (*handler_desc).address_space.context;
    let context_mutex = (*handler_desc).address_space.context_mutex;
    let mut context_locked = false;
    let mut region_context: *mut core::ffi::c_void = core::ptr::null_mut();

    if (*region_obj).region.flags & AOPOBJ_SETUP_COMPLETE == 0 {
        let setup = (*handler_desc).address_space.setup;
        if setup.is_none() { return AE_NOT_EXIST; }
        if !field_obj.is_null() && (*region_obj).region.space_id == ACPI_ADR_SPACE_PLATFORM_COMM {
            let ctx = context as *mut acpi_pcc_info;
            (*ctx).internal_buffer = (*field_obj).field.internal_pcc_buffer;
            (*ctx).length = (*region_obj).region.length as u16;
            (*ctx).subspace_id = (*region_obj).region.address as u8;
        }
        if (*region_obj).region.space_id == ACPI_ADR_SPACE_FIXED_HARDWARE {
            let ctx = context as *mut acpi_ffh_info;
            (*ctx).length = (*region_obj).region.length;
            (*ctx).offset = (*region_obj).region.address;
        }
        acpi_ex_exit_interpreter();
        let status = setup.unwrap()(region_obj, ACPI_REGION_ACTIVATE, context, &mut region_context);
        acpi_ex_enter_interpreter();
        if ACPI_FAILURE(status) { return status; }
        if (*region_obj).region.flags & AOPOBJ_SETUP_COMPLETE == 0 {
            (*region_obj).region.flags |= AOPOBJ_SETUP_COMPLETE;
            if (*region_obj2).extra.region_context.is_null() {
                (*region_obj2).extra.region_context = region_context;
            }
        }
    }
    let handler = (*handler_desc).address_space.handler.unwrap();
    let mut address = (*region_obj).region.address.wrapping_add(region_offset as u64);
    if (*handler_desc).address_space.handler_flags & ACPI_ADDR_HANDLER_DEFAULT_INSTALLED == 0 {
        acpi_ex_exit_interpreter();
    }
    if ((*region_obj).region.space_id == ACPI_ADR_SPACE_GSBUS ||
        (*region_obj).region.space_id == ACPI_ADR_SPACE_GPIO) && !context.is_null() && !field_obj.is_null() {
        let status = acpi_os_acquire_mutex(context_mutex, ACPI_WAIT_FOREVER);
        if ACPI_FAILURE(status) {
            if (*handler_desc).address_space.handler_flags & ACPI_ADDR_HANDLER_DEFAULT_INSTALLED == 0 { acpi_ex_enter_interpreter(); }
            return status;
        }
        context_locked = true;
        (*context).connection = (*field_obj).field.resource_buffer;
        (*context).length = (*field_obj).field.resource_length;
        (*context).access_length = (*field_obj).field.access_length;
        if (*region_obj).region.space_id == ACPI_ADR_SPACE_GPIO {
            address = (*field_obj).field.pin_number_index as u64;
            bit_width = (*field_obj).field.bit_length;
        }
    }
    let status = handler(function, address, bit_width, value, context, (*region_obj2).extra.region_context);
    if context_locked { let _ = acpi_os_release_mutex(context_mutex); }
    if (*handler_desc).address_space.handler_flags & ACPI_ADDR_HANDLER_DEFAULT_INSTALLED == 0 { acpi_ex_enter_interpreter(); }
    status
}

unsafe fn acpi_ev_detach_region(region_obj: *mut acpi_operand_object, acpi_ns_is_locked: u8) {
    let region_obj2 = acpi_ns_get_secondary_object(region_obj); if region_obj2.is_null() { return; }
    let handler_obj = (*region_obj).region.handler; if handler_obj.is_null() { return; }
    let mut obj_desc = (*handler_obj).address_space.region_list;
    let start_desc = obj_desc;
    let mut last = &mut (*handler_obj).address_space.region_list as *mut *mut acpi_operand_object;
    while !obj_desc.is_null() {
        if obj_desc == region_obj {
            *last = (*obj_desc).region.next; (*obj_desc).region.next = core::ptr::null_mut();
            if acpi_ns_is_locked != 0 { let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); }
            let _ = acpi_ev_execute_reg_method(region_obj, ACPI_REG_DISCONNECT);
            if acpi_ns_is_locked != 0 { let _ = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); }
            if (*region_obj).region.flags & AOPOBJ_SETUP_COMPLETE != 0 {
                let setup = (*handler_obj).address_space.setup;
                if let Some(f) = setup { let _ = f(region_obj, ACPI_REGION_DEACTIVATE, (*handler_obj).address_space.context, &mut (*region_obj2).extra.region_context); }
                (*region_obj2).extra.region_context = core::ptr::null_mut();
                (*region_obj).region.flags &= !AOPOBJ_SETUP_COMPLETE;
            }
            (*region_obj).region.handler = core::ptr::null_mut(); acpi_ut_remove_reference(handler_obj); return;
        }
        last = &mut (*obj_desc).region.next; obj_desc = (*obj_desc).region.next;
        if obj_desc == start_desc { return; }
    }
}

unsafe fn acpi_ev_attach_region(handler_obj: *mut acpi_operand_object, region_obj: *mut acpi_operand_object, _locked: u8) -> acpi_status {
    if !(*region_obj).region.handler.is_null() { return AE_ALREADY_EXISTS; }
    (*region_obj).region.next = (*handler_obj).address_space.region_list;
    (*handler_obj).address_space.region_list = region_obj; (*region_obj).region.handler = handler_obj;
    acpi_ut_add_reference(handler_obj); AE_OK
}
unsafe fn acpi_ev_execute_reg_method(region_obj: *mut acpi_operand_object, function: u32) -> acpi_status {
    if !acpi_gbl_namespace_initialized || (*region_obj).region.handler.is_null() { return AE_OK; }
    let region_obj2 = acpi_ns_get_secondary_object(region_obj); if region_obj2.is_null() { return AE_NOT_EXIST; }
    let node = (*region_obj).region.node; let mut method_node = core::ptr::null_mut();
    let reg_name_ptr = ACPI_CAST_PTR(acpi_name, METHOD_NAME__REG);
    if ACPI_SUCCESS(acpi_ns_search_one_scope(*reg_name_ptr, (*node).parent, ACPI_TYPE_METHOD, &mut method_node)) {
        (*region_obj2).extra.method_REG = method_node;
    }
    if (*region_obj2).extra.method_REG.is_null() { return AE_OK; }
    if (function == ACPI_REG_CONNECT && (*region_obj).common.flags & AOPOBJ_REG_CONNECTED != 0) ||
       (function == ACPI_REG_DISCONNECT && (*region_obj).common.flags & AOPOBJ_REG_CONNECTED == 0) { return AE_OK; }
    let info = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_evaluate_info>()) as *mut acpi_evaluate_info;
    if info.is_null() { return AE_NO_MEMORY; }
    let mut args: [*mut acpi_operand_object; 3] = [core::ptr::null_mut(); 3];
    (*info).prefix_node = (*region_obj2).extra.method_REG; (*info).relative_pathname = core::ptr::null();
    (*info).parameters = args.as_mut_ptr(); (*info).flags = ACPI_IGNORE_RETURN_VALUE;
    args[0] = acpi_ut_create_integer_object((*region_obj).region.space_id as u64);
    if args[0].is_null() { ACPI_FREE(info as *mut core::ffi::c_void); return AE_NO_MEMORY; }
    args[1] = acpi_ut_create_integer_object(function as u64);
    if args[1].is_null() { acpi_ut_remove_reference(args[0]); ACPI_FREE(info as *mut core::ffi::c_void); return AE_NO_MEMORY; }
    let mut status = acpi_ns_evaluate(info); acpi_ut_remove_reference(args[1]);
    if ACPI_SUCCESS(status) {
        if function == ACPI_REG_CONNECT { (*region_obj).common.flags |= AOPOBJ_REG_CONNECTED; }
        else { (*region_obj).common.flags &= !AOPOBJ_REG_CONNECTED; }
    }
    acpi_ut_remove_reference(args[0]); ACPI_FREE(info as *mut core::ffi::c_void); status
}

unsafe fn acpi_ev_execute_reg_methods(node: *mut acpi_namespace_node, max_depth: u32, space_id: acpi_adr_space_type, function: u32) {
    if space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY || space_id == ACPI_ADR_SPACE_SYSTEM_IO || space_id == ACPI_ADR_SPACE_DATA_TABLE { return; }
    let mut info = acpi_reg_walk_info { space_id, function, reg_run_count: 0 };
    let _ = acpi_ns_walk_namespace(ACPI_TYPE_ANY, node, max_depth, ACPI_NS_WALK_UNLOCK, Some(acpi_ev_reg_run), core::ptr::null_mut(), &mut info as *mut _ as *mut _, core::ptr::null_mut());
    if space_id == ACPI_ADR_SPACE_EC || space_id == ACPI_ADR_SPACE_GPIO { acpi_ev_execute_orphan_reg_method(node, space_id); }
}

unsafe extern "C" fn acpi_ev_reg_run(obj_handle: acpi_handle, _level: u32, context: *mut core::ffi::c_void, _return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    let node = acpi_ns_validate_handle(obj_handle); if node.is_null() { return AE_BAD_PARAMETER; }
    if (*node).type_ != ACPI_TYPE_REGION && node != acpi_gbl_root_node { return AE_OK; }
    let obj_desc = acpi_ns_get_attached_object(node); if obj_desc.is_null() { return AE_OK; }
    let info = &mut *(context as *mut acpi_reg_walk_info); if (*obj_desc).region.space_id != info.space_id { return AE_OK; }
    info.reg_run_count += 1; acpi_ev_execute_reg_method(obj_desc, info.function)
}

unsafe fn acpi_ev_execute_orphan_reg_method(device_node: *mut acpi_namespace_node, space_id: acpi_adr_space_type) {
    if device_node.is_null() { return; }
    let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    let mut reg_method = core::ptr::null_mut();
    if ACPI_FAILURE(acpi_get_handle(device_node, METHOD_NAME__REG, &mut reg_method)) { let _ = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); return; }
    let mut next_node = acpi_ns_get_next_node(device_node, core::ptr::null_mut());
    while !next_node.is_null() {
        if (*next_node).type_ == ACPI_TYPE_REGION && !(*next_node).object.is_null() && (*(*next_node).object).region.space_id == space_id { let _ = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); return; }
        next_node = acpi_ns_get_next_node(device_node, next_node);
    }
    let mut objects = [acpi_object { type_: ACPI_TYPE_INTEGER, integer: acpi_integer { value: space_id as u64 } }, acpi_object { type_: ACPI_TYPE_INTEGER, integer: acpi_integer { value: ACPI_REG_CONNECT as u64 } }];
    let args = acpi_object_list { count: 2, pointer: objects.as_mut_ptr() };
    let _ = acpi_evaluate_object(reg_method, core::ptr::null(), &args, core::ptr::null_mut());
    let _ = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
