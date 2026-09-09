// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Public interfaces to the ACPI namespace subsystem.

/* External ACPI declarations, constants, macros, and types are supplied by the surrounding crate. */

unsafe fn acpi_ns_copy_device_id(
    dest: *mut acpi_pnp_device_id,
    source: *mut acpi_pnp_device_id,
    string_area: *mut c_char,
) -> *mut c_char {
    (*dest).string = string_area;
    (*dest).length = (*source).length;
    memcpy(string_area as *mut c_void, (*source).string as *const c_void, (*source).length as usize);
    string_area.add((*source).length as usize)
}

pub unsafe extern "C" fn acpi_get_handle(
    parent: acpi_handle,
    pathname: *const c_char,
    ret_handle: *mut acpi_handle,
) -> acpi_status {
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    let mut prefix_node: *mut acpi_namespace_node = core::ptr::null_mut();

    acpi_function_entry!();
    if ret_handle.is_null() || pathname.is_null() { return AE_BAD_PARAMETER; }
    if !parent.is_null() {
        prefix_node = acpi_ns_validate_handle(parent);
        if prefix_node.is_null() { return AE_BAD_PARAMETER; }
    }
    if ACPI_IS_ROOT_PREFIX!(*pathname as u8) {
        if strcmp(pathname, ACPI_NS_ROOT_PATH) == 0 {
            *ret_handle = acpi_gbl_root_node as acpi_handle;
            return AE_OK;
        }
    } else if prefix_node.is_null() {
        return AE_BAD_PARAMETER;
    }
    let status = acpi_ns_get_node(prefix_node, pathname, ACPI_NS_NO_UPSEARCH, &mut node);
    if ACPI_SUCCESS!(status) { *ret_handle = node as acpi_handle; }
    status
}

pub unsafe extern "C" fn acpi_get_name(
    handle: acpi_handle,
    name_type: u32,
    buffer: *mut acpi_buffer,
) -> acpi_status {
    if name_type > ACPI_NAME_TYPE_MAX { return AE_BAD_PARAMETER; }
    let mut status = acpi_ut_validate_buffer(buffer);
    if ACPI_FAILURE!(status) { return status; }
    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE!(status) { return status; }
    if name_type == ACPI_FULL_PATHNAME || name_type == ACPI_FULL_PATHNAME_NO_TRAILING {
        status = acpi_ns_handle_to_pathname(handle, buffer, if name_type == ACPI_FULL_PATHNAME { FALSE } else { TRUE });
    } else {
        status = acpi_ns_handle_to_name(handle, buffer);
    }
    acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    status
}

pub unsafe extern "C" fn acpi_get_object_info(
    handle: acpi_handle,
    return_buffer: *mut *mut acpi_device_info,
) -> acpi_status {
    let mut node: *mut acpi_namespace_node;
    let mut info: *mut acpi_device_info;
    let mut cid_list: *mut acpi_pnp_device_id_list = core::ptr::null_mut();
    let mut hid: *mut acpi_pnp_device_id = core::ptr::null_mut();
    let mut uid: *mut acpi_pnp_device_id = core::ptr::null_mut();
    let mut cls: *mut acpi_pnp_device_id = core::ptr::null_mut();
    let mut next_id_string: *mut c_char;
    let mut param_count: u8 = 0;
    let mut valid: u16 = 0;
    let mut info_size: u32;
    let mut i: u32;
    let mut status: acpi_status;
    if handle.is_null() || return_buffer.is_null() { return AE_BAD_PARAMETER; }
    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE!(status) { return status; }
    node = acpi_ns_validate_handle(handle);
    if node.is_null() { acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); return AE_BAD_PARAMETER; }
    info_size = core::mem::size_of::<acpi_device_info>() as u32;
    let obj_type = (*node).type_;
    let name = (*node).name.integer;
    if obj_type == ACPI_TYPE_METHOD { param_count = (*(*node).object).method.param_count; }
    status = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE!(status) { return status; }
    if obj_type == ACPI_TYPE_DEVICE || obj_type == ACPI_TYPE_PROCESSOR {
        status = acpi_ut_execute_HID(node, &mut hid); if ACPI_SUCCESS!(status) { info_size += (*hid).length; valid |= ACPI_VALID_HID; }
        status = acpi_ut_execute_UID(node, &mut uid); if ACPI_SUCCESS!(status) { info_size += (*uid).length; valid |= ACPI_VALID_UID; }
        status = acpi_ut_execute_CID(node, &mut cid_list); if ACPI_SUCCESS!(status) { info_size += (*cid_list).list_size - core::mem::size_of::<acpi_pnp_device_id_list>() as u32; valid |= ACPI_VALID_CID; }
        status = acpi_ut_execute_CLS(node, &mut cls); if ACPI_SUCCESS!(status) { info_size += (*cls).length; valid |= ACPI_VALID_CLS; }
    }
    info = ACPI_ALLOCATE_ZEROED!(info_size) as *mut acpi_device_info;
    if info.is_null() { status = AE_NO_MEMORY; goto_cleanup!(); }
    if obj_type == ACPI_TYPE_DEVICE || obj_type == ACPI_TYPE_PROCESSOR {
        status = acpi_ut_evaluate_numeric_object(METHOD_NAME__ADR, node, &mut (*info).address); if ACPI_SUCCESS!(status) { valid |= ACPI_VALID_ADR; }
        status = acpi_ut_execute_power_methods(node, acpi_gbl_lowest_dstate_names, ACPI_NUM_sx_w_METHODS, (*info).lowest_dstates.as_mut_ptr()); if ACPI_SUCCESS!(status) { valid |= ACPI_VALID_SXWS; }
        status = acpi_ut_execute_power_methods(node, acpi_gbl_highest_dstate_names, ACPI_NUM_sx_d_METHODS, (*info).highest_dstates.as_mut_ptr()); if ACPI_SUCCESS!(status) { valid |= ACPI_VALID_SXDS; }
    }
    next_id_string = (*info).compatible_id_list.ids.as_mut_ptr() as *mut c_char;
    if !cid_list.is_null() { next_id_string = next_id_string.add((*cid_list).count as usize * core::mem::size_of::<acpi_pnp_device_id>()); }
    if !hid.is_null() { next_id_string = acpi_ns_copy_device_id(&mut (*info).hardware_id, hid, next_id_string); if acpi_ut_is_pci_root_bridge((*hid).string) { (*info).flags |= ACPI_PCI_ROOT_BRIDGE; } }
    if !uid.is_null() { next_id_string = acpi_ns_copy_device_id(&mut (*info).unique_id, uid, next_id_string); }
    if !cid_list.is_null() { (*info).compatible_id_list.count = (*cid_list).count; (*info).compatible_id_list.list_size = (*cid_list).list_size; for i in 0..(*cid_list).count { next_id_string = acpi_ns_copy_device_id(&mut (*info).compatible_id_list.ids.add(i as usize).read(), &mut (*cid_list).ids.add(i as usize).read(), next_id_string); if acpi_ut_is_pci_root_bridge((*cid_list).ids.add(i as usize).read().string) { (*info).flags |= ACPI_PCI_ROOT_BRIDGE; } } }
    if !cls.is_null() { acpi_ns_copy_device_id(&mut (*info).class_code, cls, next_id_string); }
    (*info).info_size = info_size; (*info).type_ = obj_type; (*info).name = name; (*info).param_count = param_count; (*info).valid = valid;
    *return_buffer = info; status = AE_OK;
cleanup:
    if !hid.is_null() { ACPI_FREE!(hid); } if !uid.is_null() { ACPI_FREE!(uid); } if !cid_list.is_null() { ACPI_FREE!(cid_list); } if !cls.is_null() { ACPI_FREE!(cls); }
    status
}

pub unsafe extern "C" fn acpi_install_method(buffer: *mut u8) -> acpi_status {
    if buffer.is_null() { return AE_BAD_PARAMETER; }
    let table = buffer as *mut acpi_table_header;
    if !ACPI_COMPARE_NAMESEG!((*table).signature, ACPI_SIG_DSDT) && !ACPI_COMPARE_NAMESEG!((*table).signature, ACPI_SIG_SSDT) { return AE_BAD_HEADER; }
    let mut parser_state: acpi_parse_state = core::mem::zeroed();
    parser_state.aml = buffer.add(core::mem::size_of::<acpi_table_header>());
    let opcode = acpi_ps_peek_opcode(&parser_state); if opcode != AML_METHOD_OP { return AE_BAD_PARAMETER; }
    parser_state.aml = parser_state.aml.add(acpi_ps_get_opcode_size(opcode) as usize); parser_state.pkg_end = acpi_ps_get_next_package_end(&parser_state);
    if parser_state.pkg_end > parser_state.aml_end || parser_state.pkg_end < parser_state.aml { return AE_AML_PACKAGE_LIMIT; }
    let path = acpi_ps_get_next_namestring(&mut parser_state); let method_flags = *parser_state.aml; parser_state.aml = parser_state.aml.add(1); let aml_start = parser_state.aml; let aml_length = parser_state.pkg_end.offset_from(aml_start) as u32;
    let aml_buffer = ACPI_ALLOCATE!(aml_length); if aml_buffer.is_null() { return AE_NO_MEMORY; }
    let method_obj = acpi_ut_create_internal_object(ACPI_TYPE_METHOD); if method_obj.is_null() { ACPI_FREE!(aml_buffer); return AE_NO_MEMORY; }
    let mut status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); if ACPI_FAILURE!(status) { ACPI_FREE!(aml_buffer); acpi_ut_delete_object_desc(method_obj); return status; }
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut(); status = acpi_ns_lookup(core::ptr::null_mut(), path, ACPI_TYPE_METHOD, ACPI_IMODE_LOAD_PASS1, ACPI_NS_DONT_OPEN_SCOPE | ACPI_NS_ERROR_IF_FOUND, core::ptr::null_mut(), &mut node); acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE!(status) && status != AE_ALREADY_EXISTS { ACPI_FREE!(aml_buffer); acpi_ut_delete_object_desc(method_obj); return status; } if status == AE_ALREADY_EXISTS && (*node).type_ != ACPI_TYPE_METHOD { ACPI_FREE!(aml_buffer); acpi_ut_delete_object_desc(method_obj); return AE_TYPE; }
    memcpy(aml_buffer, aml_start as *const c_void, aml_length as usize); (*method_obj).method.aml_start = aml_buffer; (*method_obj).method.aml_length = aml_length; (*method_obj).method.param_count = method_flags & AML_METHOD_ARG_COUNT; if method_flags & AML_METHOD_SERIALIZED != 0 { (*method_obj).method.info_flags = ACPI_METHOD_SERIALIZED; (*method_obj).method.sync_level = (method_flags & AML_METHOD_SYNC_LEVEL) >> 4; }
    status = acpi_ns_attach_object(node, method_obj, ACPI_TYPE_METHOD); (*node).flags |= ANOBJ_ALLOCATED_BUFFER; acpi_ut_remove_reference(method_obj); status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
