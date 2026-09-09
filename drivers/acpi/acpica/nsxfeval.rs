// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Public interfaces to the ACPI subsystem: ACPI object evaluation interfaces.
// C headers and ACPICA macros are supplied by the surrounding translation unit.

unsafe fn acpi_ns_resolve_references(info: *mut acpi_evaluate_info) {
    let mut obj_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut node: *mut acpi_namespace_node;
    if (*(*info).return_object).common.type_ != ACPI_TYPE_LOCAL_REFERENCE {
        return;
    }
    match (*(*info).return_object).reference.class_ {
        ACPI_REFCLASS_INDEX => {
            obj_desc = *(*(*info).return_object).reference.where_;
        }
        ACPI_REFCLASS_REFOF => {
            node = (*(*info).return_object).reference.object;
            if !node.is_null() { obj_desc = (*node).object; }
        }
        _ => return,
    }
    if !obj_desc.is_null() {
        acpi_ut_add_reference(obj_desc);
        acpi_ut_remove_reference((*info).return_object);
        (*info).return_object = obj_desc;
    }
}

#[no_mangle]
pub unsafe extern "C" fn acpi_evaluate_object_typed(
    handle: acpi_handle, pathname: acpi_string,
    external_params: *mut acpi_object_list, return_buffer: *mut acpi_buffer,
    return_type: acpi_object_type,
) -> acpi_status {
    if return_buffer.is_null() { return AE_BAD_PARAMETER; }
    let free_buffer_on_error = (*return_buffer).length == ACPI_ALLOCATE_BUFFER;
    let mut target_handle = handle;
    if !pathname.is_null() {
        let mut status = acpi_get_handle(handle, pathname, &mut target_handle);
        if ACPI_FAILURE(status) { return status; }
    }
    let full_pathname = acpi_ns_get_external_pathname(target_handle);
    if full_pathname.is_null() { return AE_NO_MEMORY; }
    let mut status = acpi_evaluate_object(target_handle, core::ptr::null(), external_params, return_buffer);
    if ACPI_SUCCESS(status) && return_type != ACPI_TYPE_ANY {
        if (*return_buffer).length == 0 {
            status = AE_NULL_OBJECT;
        } else if (*( (*return_buffer).pointer as *mut acpi_object)).type_ != return_type {
            if free_buffer_on_error {
                acpi_os_free((*return_buffer).pointer);
                (*return_buffer).pointer = core::ptr::null_mut();
            }
            (*return_buffer).length = 0;
            status = AE_TYPE;
        }
    }
    ACPI_FREE(full_pathname);
    status
}

#[no_mangle]
pub unsafe extern "C" fn acpi_evaluate_object(
    handle: acpi_handle, pathname: acpi_string,
    external_params: *mut acpi_object_list, return_buffer: *mut acpi_buffer,
) -> acpi_status {
    let info = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_evaluate_info>()) as *mut acpi_evaluate_info;
    if info.is_null() { return AE_NO_MEMORY; }
    let mut status;
    (*info).prefix_node = acpi_ns_validate_handle(handle);
    if (*info).prefix_node.is_null() { status = AE_BAD_PARAMETER; return goto_cleanup(info, status); }
    if !pathname.is_null() && ACPI_IS_ROOT_PREFIX(*pathname as u8) {
        (*info).prefix_node = core::ptr::null_mut();
    } else if handle.is_null() {
        status = AE_BAD_PARAMETER; return goto_cleanup(info, status);
    }
    (*info).relative_pathname = pathname;
    if !external_params.is_null() && (*external_params).count != 0 {
        (*info).param_count = (*external_params).count as u16;
        if (*info).param_count > ACPI_METHOD_NUM_ARGS { (*info).param_count = ACPI_METHOD_NUM_ARGS; }
        (*info).parameters = ACPI_ALLOCATE_ZEROED(((*info).param_count as usize + 1) * core::mem::size_of::<*mut acpi_operand_object>()) as *mut *mut acpi_operand_object;
        if (*info).parameters.is_null() { status = AE_NO_MEMORY; return goto_cleanup(info, status); }
        for i in 0..(*info).param_count as usize {
            status = acpi_ut_copy_eobject_to_iobject(&(*external_params).pointer.add(i), (*info).parameters.add(i));
            if ACPI_FAILURE(status) { return goto_cleanup(info, status); }
        }
        *(*info).parameters.add((*info).param_count as usize) = core::ptr::null_mut();
    }
    status = acpi_ns_evaluate(info);
    if return_buffer.is_null() { return goto_cleanup_return(info, status); }
    if (*info).return_object.is_null() { (*return_buffer).length = 0; return goto_cleanup(info, status); }
    if ACPI_GET_DESCRIPTOR_TYPE((*info).return_object) == ACPI_DESC_TYPE_NAMED {
        status = AE_TYPE; (*info).return_object = core::ptr::null_mut(); (*return_buffer).length = 0;
    }
    if ACPI_FAILURE(status) { return goto_cleanup_return(info, status); }
    acpi_ns_resolve_references(info);
    let mut needed = 0;
    status = acpi_ut_get_object_size((*info).return_object, &mut needed);
    if ACPI_SUCCESS(status) { status = acpi_ut_initialize_buffer(return_buffer, needed); }
    if ACPI_SUCCESS(status) { status = acpi_ut_copy_iobject_to_eobject((*info).return_object, return_buffer); }
goto_cleanup_return(info, status)
}

unsafe fn goto_cleanup_return(info: *mut acpi_evaluate_info, mut status: acpi_status) -> acpi_status {
    if !(*info).return_object.is_null() { acpi_ex_enter_interpreter(); acpi_ut_remove_reference((*info).return_object); acpi_ex_exit_interpreter(); }
    goto_cleanup(info, status)
}
unsafe fn goto_cleanup(info: *mut acpi_evaluate_info, status: acpi_status) -> acpi_status {
    if !(*info).parameters.is_null() { acpi_ut_delete_internal_object_list((*info).parameters); }
    ACPI_FREE(info); status
}

#[no_mangle]
pub unsafe extern "C" fn acpi_walk_namespace(type_: acpi_object_type, start_object: acpi_handle, max_depth: u32, descending_callback: acpi_walk_callback, ascending_callback: acpi_walk_callback, context: *mut core::ffi::c_void, return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    if type_ > ACPI_TYPE_LOCAL_MAX || max_depth == 0 || (descending_callback.is_none() && ascending_callback.is_none()) { return AE_BAD_PARAMETER; }
    let mut status = acpi_ut_acquire_read_lock(&mut acpi_gbl_namespace_rw_lock);
    if ACPI_FAILURE(status) { return status; }
    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_SUCCESS(status) {
        if acpi_ns_validate_handle(start_object).is_null() { status = AE_BAD_PARAMETER; }
        else { status = acpi_ns_walk_namespace(type_, start_object, max_depth, ACPI_NS_WALK_UNLOCK, descending_callback, ascending_callback, context, return_value); }
        let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    }
    let _ = acpi_ut_release_read_lock(&mut acpi_gbl_namespace_rw_lock); status
}

#[no_mangle]
pub unsafe extern "C" fn acpi_get_devices(HID: *const i8, user_function: acpi_walk_callback, context: *mut core::ffi::c_void, return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    if user_function.is_none() { return AE_BAD_PARAMETER; }
    let mut info = acpi_get_devices_info { hid: HID, context, user_function };
    let mut status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_SUCCESS(status) { status = acpi_ns_walk_namespace(ACPI_TYPE_DEVICE, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, ACPI_NS_WALK_UNLOCK, Some(acpi_ns_get_device_callback), core::ptr::null_mut(), &mut info as *mut _ as *mut _, return_value); let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); }
    status
}

unsafe extern "C" fn acpi_ns_get_device_callback(obj_handle: acpi_handle, nesting_level: u32, _context: *mut core::ffi::c_void, return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    let info = _context as *mut acpi_get_devices_info;
    let mut status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); if ACPI_FAILURE(status) { return status; }
    let node = acpi_ns_validate_handle(obj_handle); let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    if node.is_null() { return AE_BAD_PARAMETER; }
    if !(*info).hid.is_null() {
        let mut hid = core::ptr::null_mut(); status = acpi_ut_execute_HID(node, &mut hid); if status == AE_NOT_FOUND { return AE_OK; } if ACPI_FAILURE(status) { return AE_CTRL_DEPTH; }
        let matched = libc::strcmp((*hid).string, (*info).hid) == 0; ACPI_FREE(hid);
        if !matched { return AE_OK; }
    }
    let mut flags = 0; status = acpi_ut_execute_STA(node, &mut flags); if ACPI_FAILURE(status) { return AE_CTRL_DEPTH; }
    if flags & ACPI_STA_DEVICE_PRESENT == 0 && flags & ACPI_STA_DEVICE_FUNCTIONING == 0 { return AE_CTRL_DEPTH; }
    (*info).user_function.unwrap()(obj_handle, nesting_level, (*info).context, return_value)
}

#[no_mangle]
pub unsafe extern "C" fn acpi_attach_data(obj_handle: acpi_handle, handler: acpi_object_handler, data: *mut core::ffi::c_void) -> acpi_status {
    if obj_handle.is_null() || handler.is_none() || data.is_null() { return AE_BAD_PARAMETER; }
    let mut s = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); if ACPI_FAILURE(s) { return s; }
    let node = acpi_ns_validate_handle(obj_handle); if node.is_null() { s = AE_BAD_PARAMETER; } else { s = acpi_ns_attach_data(node, handler, data); } let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); s
}
#[no_mangle]
pub unsafe extern "C" fn acpi_detach_data(obj_handle: acpi_handle, handler: acpi_object_handler) -> acpi_status {
    if obj_handle.is_null() || handler.is_none() { return AE_BAD_PARAMETER; }
    let mut s = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); if ACPI_FAILURE(s) { return s; }
    let node = acpi_ns_validate_handle(obj_handle); if node.is_null() { s = AE_BAD_PARAMETER; } else { s = acpi_ns_detach_data(node, handler); } let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); s
}
#[no_mangle]
pub unsafe extern "C" fn acpi_get_data_full(obj_handle: acpi_handle, handler: acpi_object_handler, data: *mut *mut core::ffi::c_void, callback: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>) -> acpi_status {
    if obj_handle.is_null() || handler.is_none() || data.is_null() { return AE_BAD_PARAMETER; }
    let mut s = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); if ACPI_FAILURE(s) { return s; }
    let node = acpi_ns_validate_handle(obj_handle); if node.is_null() { s = AE_BAD_PARAMETER; } else { s = acpi_ns_get_attached_data(node, handler, data); if ACPI_SUCCESS(s) { if let Some(cb) = callback { cb(*data); } } } let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); s
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
