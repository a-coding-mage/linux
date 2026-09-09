// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: nsaccess - Top-level functions for accessing ACPI namespace

// C dependencies are supplied by the surrounding translation unit.

pub unsafe fn acpi_ns_root_initialize() -> acpi_status {
    let mut status: acpi_status;
    let mut init_val: *const acpi_predefined_names = core::ptr::null();
    let mut new_node: *mut acpi_namespace_node;
    let mut prev_node: *mut acpi_namespace_node = core::ptr::null_mut();
    let mut obj_desc: *mut acpi_operand_object;
    let mut val: acpi_string = core::ptr::null_mut();

    ACPI_FUNCTION_TRACE!(ns_root_initialize);
    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE(status) { return_ACPI_STATUS!(status); }
    if !acpi_gbl_root_node.is_null() { status = AE_OK; goto_unlock_and_exit!(); }
    acpi_gbl_root_node = &mut acpi_gbl_root_node_struct;

    ACPI_DEBUG_PRINT!((ACPI_DB_INFO, "Entering predefined entries into namespace\n"));
    init_val = acpi_gbl_pre_defined_names;
    while !(*init_val).name.is_null() {
        status = AE_OK;
        if strcmp((*init_val).name, b"_OSI\0".as_ptr() as *const i8) == 0 && !acpi_gbl_create_osi_method { init_val = init_val.add(1); continue; }
        new_node = acpi_ns_create_node(*ACPI_CAST_PTR!(u32, (*init_val).name));
        if new_node.is_null() { status = AE_NO_MEMORY; goto_unlock_and_exit!(); }
        (*new_node).descriptor_type = ACPI_DESC_TYPE_NAMED;
        (*new_node).type = (*init_val).type;
        if prev_node.is_null() { acpi_gbl_root_node_struct.child = new_node; } else { (*prev_node).peer = new_node; }
        (*new_node).parent = &mut acpi_gbl_root_node_struct;
        prev_node = new_node;
        if !(*init_val).val.is_null() {
            status = acpi_os_predefined_override(init_val, &mut val);
            if ACPI_FAILURE(status) { ACPI_ERROR!((AE_INFO, "Could not override predefined %s", (*init_val).name)); }
            if val.is_null() { val = (*init_val).val; }
            obj_desc = acpi_ut_create_internal_object((*init_val).type);
            if obj_desc.is_null() { status = AE_NO_MEMORY; goto_unlock_and_exit!(); }
            match (*init_val).type {
                ACPI_TYPE_METHOD => {
                    (*obj_desc).method.param_count = ACPI_TO_INTEGER!(val) as u8;
                    (*obj_desc).common.flags |= AOPOBJ_DATA_VALID;
                    #[cfg(feature = "acpi_asl_compiler")]
                    { (*new_node).value = (*obj_desc).method.param_count; }
                    #[cfg(not(feature = "acpi_asl_compiler"))]
                    { (*obj_desc).method.info_flags = ACPI_METHOD_INTERNAL_ONLY; (*obj_desc).method.dispatch.implementation = Some(acpi_ut_osi_implementation); }
                }
                ACPI_TYPE_INTEGER => { (*obj_desc).integer.value = ACPI_TO_INTEGER!(val); }
                ACPI_TYPE_STRING => { (*obj_desc).string.length = strlen(val) as u32; (*obj_desc).string.pointer = val; (*obj_desc).common.flags |= AOPOBJ_STATIC_POINTER; }
                ACPI_TYPE_MUTEX => {
                    (*obj_desc).mutex.node = new_node;
                    (*obj_desc).mutex.sync_level = (ACPI_TO_INTEGER!(val) - 1) as u8;
                    status = acpi_os_create_mutex(&mut (*obj_desc).mutex.os_mutex);
                    if ACPI_FAILURE(status) { acpi_ut_remove_reference(obj_desc); goto_unlock_and_exit!(); }
                    if strcmp((*init_val).name, b"_GL_\0".as_ptr() as *const i8) == 0 {
                        acpi_gbl_global_lock_mutex = obj_desc;
                        status = acpi_os_create_semaphore(1, 0, &mut acpi_gbl_global_lock_semaphore);
                        if ACPI_FAILURE(status) { acpi_ut_remove_reference(obj_desc); goto_unlock_and_exit!(); }
                    }
                }
                _ => { ACPI_ERROR!((AE_INFO, "Unsupported initial type value 0x%X", (*init_val).type)); acpi_ut_remove_reference(obj_desc); obj_desc = core::ptr::null_mut(); init_val = init_val.add(1); continue; }
            }
            status = acpi_ns_attach_object(new_node, obj_desc, (*obj_desc).common.type);
            acpi_ut_remove_reference(obj_desc);
        }
        init_val = init_val.add(1);
    }
    goto_unlock_and_exit!();
    #[allow(unreachable_code)]
    { acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); if ACPI_SUCCESS(status) { status = acpi_ns_get_node(core::ptr::null_mut(), b"\\_GPE\0".as_ptr() as *mut i8, ACPI_NS_NO_UPSEARCH, &mut acpi_gbl_fadt_gpe_device); } return_ACPI_STATUS!(status) }
}

pub unsafe fn acpi_ns_lookup(scope_info: *mut acpi_generic_state, pathname: *mut i8, mut type_: acpi_object_type, interpreter_mode: acpi_interpreter_mode, flags: u32, walk_state: *mut acpi_walk_state, return_node: *mut *mut acpi_namespace_node) -> acpi_status {
    if return_node.is_null() { return_ACPI_STATUS!(AE_BAD_PARAMETER); }
    let mut path = pathname; let mut prefix_node: *mut acpi_namespace_node; let mut current_node = core::ptr::null_mut(); let mut this_node = core::ptr::null_mut();
    let mut num_segments: u32; let mut num_carats: u32; let mut simple_name: acpi_name = 0; let mut type_to_check_for = type_; let mut this_search_type = ACPI_TYPE_ANY; let mut search_parent_flag = ACPI_NS_SEARCH_PARENT; let mut local_flags = flags & !(ACPI_NS_ERROR_IF_FOUND | ACPI_NS_OVERRIDE_IF_FOUND | ACPI_NS_SEARCH_PARENT); let mut local_interpreter_mode;
    *return_node = ACPI_ENTRY_NOT_FOUND; acpi_gbl_ns_lookup_count += 1;
    if acpi_gbl_root_node.is_null() { return_ACPI_STATUS!(AE_NO_NAMESPACE); }
    if scope_info.is_null() || (*scope_info).scope.node.is_null() { prefix_node = acpi_gbl_root_node; } else { prefix_node = (*scope_info).scope.node; if ACPI_GET_DESCRIPTOR_TYPE!(prefix_node) != ACPI_DESC_TYPE_NAMED { return_ACPI_STATUS!(AE_AML_INTERNAL); } if flags & ACPI_NS_PREFIX_IS_SCOPE == 0 { while !acpi_ns_opens_scope((*prefix_node).type) && (*prefix_node).type != ACPI_TYPE_ANY { prefix_node = (*prefix_node).parent; } } }
    if pathname.is_null() { num_segments = 0; this_node = acpi_gbl_root_node; path = b"\0".as_ptr() as *mut i8; } else { if *path as u8 == AML_ROOT_PREFIX { this_node = acpi_gbl_root_node; search_parent_flag = ACPI_NS_NO_UPSEARCH; path = path.add(1); } else { this_node = prefix_node; num_carats = 0; while *path as u8 == AML_PARENT_PREFIX { search_parent_flag = ACPI_NS_NO_UPSEARCH; path = path.add(1); num_carats += 1; this_node = (*this_node).parent; if this_node.is_null() { return_ACPI_STATUS!(AE_NOT_FOUND); } } } match *path as u8 { 0 => { num_segments = 0; type_ = (*this_node).type; }, AML_DUAL_NAME_PREFIX => { search_parent_flag = ACPI_NS_NO_UPSEARCH; num_segments = 2; path = path.add(1); }, AML_MULTI_NAME_PREFIX => { search_parent_flag = ACPI_NS_NO_UPSEARCH; path = path.add(1); num_segments = *path as u8 as u32; path = path.add(1); }, _ => { num_segments = 1; } } }
    this_search_type = ACPI_TYPE_ANY; current_node = this_node;
    while num_segments != 0 && !current_node.is_null() { num_segments -= 1; if num_segments == 0 { this_search_type = type_; if search_parent_flag != ACPI_NS_NO_UPSEARCH && flags & ACPI_NS_SEARCH_PARENT != 0 { local_flags |= ACPI_NS_SEARCH_PARENT; } if flags & ACPI_NS_ERROR_IF_FOUND != 0 { local_flags |= ACPI_NS_ERROR_IF_FOUND; } if flags & ACPI_NS_OVERRIDE_IF_FOUND != 0 { local_flags |= ACPI_NS_OVERRIDE_IF_FOUND; } } local_interpreter_mode = interpreter_mode; if flags & ACPI_NS_PREFIX_MUST_EXIST != 0 && num_segments > 0 { local_interpreter_mode = ACPI_IMODE_EXECUTE; } ACPI_MOVE_32_TO_32!(&mut simple_name, path); let status = acpi_ns_search_and_enter(simple_name, walk_state, current_node, local_interpreter_mode, this_search_type, local_flags, &mut this_node); if ACPI_FAILURE(status) { *return_node = this_node; return_ACPI_STATUS!(status); } if num_segments > 0 && (*this_node).type == ACPI_TYPE_LOCAL_ALIAS { if (*this_node).object.is_null() { return_ACPI_STATUS!(AE_NOT_EXIST); } if acpi_ns_opens_scope((*( (*this_node).object as *mut acpi_namespace_node)).type) { this_node = (*this_node).object as *mut acpi_namespace_node; } } else if num_segments == 0 { if type_to_check_for != ACPI_TYPE_ANY && type_to_check_for != ACPI_TYPE_LOCAL_ALIAS && type_to_check_for != ACPI_TYPE_LOCAL_METHOD_ALIAS && type_to_check_for != ACPI_TYPE_LOCAL_SCOPE && (*this_node).type != ACPI_TYPE_ANY && (*this_node).type != type_to_check_for { ACPI_WARNING!((AE_INFO, "NsLookup: Type mismatch")); } if type_ == ACPI_TYPE_ANY { type_ = (*this_node).type; } } path = path.add(ACPI_NAMESEG_SIZE as usize); current_node = this_node; }
    if flags & ACPI_NS_DONT_OPEN_SCOPE == 0 && !walk_state.is_null() && acpi_ns_opens_scope(type_) { let status = acpi_ds_scope_stack_push(this_node, type_, walk_state); if ACPI_FAILURE(status) { return_ACPI_STATUS!(status); } }
    *return_node = this_node; return_ACPI_STATUS!(AE_OK)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
