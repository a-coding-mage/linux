// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: nsutils - Utilities for accessing ACPI namespace.

// C dependencies are supplied by the surrounding ACPI translation unit.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe fn acpi_ns_print_node_pathname(
    node: *mut acpi_namespace_node,
    message: *const ::std::ffi::c_char,
) {
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_LOCAL_BUFFER, pointer: core::ptr::null_mut() };
    if node.is_null() {
        acpi_os_printf(b"[NULL NAME]\0".as_ptr() as *const _);
        return;
    }
    let status = acpi_ns_handle_to_pathname(node, &mut buffer, TRUE);
    if ACPI_SUCCESS(status) {
        if !message.is_null() { acpi_os_printf_2(b"%s \0".as_ptr() as *const _, message); }
        acpi_os_printf_2(b"%s\0".as_ptr() as *const _, buffer.pointer as *const _);
        ACPI_FREE(buffer.pointer);
    }
}

pub unsafe fn acpi_ns_get_type(node: *mut acpi_namespace_node) -> acpi_object_type {
    if node.is_null() { ACPI_WARNING_NULL_NODE(); return ACPI_TYPE_ANY; }
    (*node).type_
}

pub unsafe fn acpi_ns_local(type_: acpi_object_type) -> u32 {
    if !acpi_ut_valid_object_type(type_) { ACPI_WARNING_INVALID_TYPE(type_); return ACPI_NS_NORMAL; }
    acpi_gbl_ns_properties[type_ as usize] & ACPI_NS_LOCAL
}

pub unsafe fn acpi_ns_get_internal_name_length(info: *mut acpi_namestring_info) {
    let mut next = (*info).external_name;
    (*info).num_carats = 0;
    (*info).num_segments = 0;
    (*info).fully_qualified = FALSE;
    if ACPI_IS_ROOT_PREFIX(*next) {
        (*info).fully_qualified = TRUE; next = next.add(1);
        while ACPI_IS_ROOT_PREFIX(*next) { next = next.add(1); }
    } else {
        while ACPI_IS_PARENT_PREFIX(*next) { (*info).num_carats += 1; next = next.add(1); }
    }
    if *next != 0 {
        (*info).num_segments = 1;
        let mut i = 0;
        while *next.add(i) != 0 { if ACPI_IS_PATH_SEPARATOR(*next.add(i)) { (*info).num_segments += 1; } i += 1; }
    }
    (*info).length = (ACPI_NAMESEG_SIZE * (*info).num_segments) + 4 + (*info).num_carats;
    (*info).next_external_char = next;
}

pub unsafe fn acpi_ns_build_internal_name(info: *mut acpi_namestring_info) -> acpi_status {
    let mut n = (*info).num_segments;
    let internal = (*info).internal_name;
    let mut external = (*info).next_external_char;
    let mut result: *mut i8;
    if (*info).fully_qualified {
        *internal = AML_ROOT_PREFIX;
        if n <= 1 { result = internal.add(1); }
        else if n == 2 { *internal.add(1) = AML_DUAL_NAME_PREFIX; result = internal.add(2); }
        else { *internal.add(1) = AML_MULTI_NAME_PREFIX; *internal.add(2) = n as i8; result = internal.add(3); }
    } else {
        let mut i = 0;
        while i < (*info).num_carats { *internal.add(i as usize) = AML_PARENT_PREFIX; i += 1; }
        if n <= 1 { result = internal.add(i as usize); }
        else if n == 2 { *internal.add(i as usize) = AML_DUAL_NAME_PREFIX; result = internal.add(i as usize + 1); }
        else { *internal.add(i as usize) = AML_MULTI_NAME_PREFIX; *internal.add(i as usize + 1) = n as i8; result = internal.add(i as usize + 2); }
    }
    while n != 0 {
        let mut i = 0;
        while i < ACPI_NAMESEG_SIZE as usize {
            if ACPI_IS_PATH_SEPARATOR(*external) || *external == 0 { *result.add(i) = b'_' as i8; }
            else { *result.add(i) = toupper(*external as i32) as i8; external = external.add(1); }
            i += 1;
        }
        if !ACPI_IS_PATH_SEPARATOR(*external) && *external != 0 { return AE_BAD_PATHNAME; }
        external = external.add(1); result = result.add(ACPI_NAMESEG_SIZE as usize); n -= 1;
    }
    *result = 0; AE_OK
}

pub unsafe fn acpi_ns_internalize_name(external_name: *const i8, converted_name: *mut *mut i8) -> acpi_status {
    if external_name.is_null() || *external_name == 0 || converted_name.is_null() { return AE_BAD_PARAMETER; }
    let mut info = core::mem::zeroed::<acpi_namestring_info>(); info.external_name = external_name;
    acpi_ns_get_internal_name_length(&mut info);
    let internal = ACPI_ALLOCATE_ZEROED(info.length as usize);
    if internal.is_null() { return AE_NO_MEMORY; }
    info.internal_name = internal;
    let status = acpi_ns_build_internal_name(&mut info);
    if ACPI_FAILURE(status) { ACPI_FREE(internal as *mut _); return status; }
    *converted_name = internal; AE_OK
}

pub unsafe fn acpi_ns_externalize_name(internal_name_length: u32, internal_name: *const i8, converted_name_length: *mut u32, converted_name: *mut *mut i8) -> acpi_status {
    if internal_name_length == 0 || internal_name.is_null() || converted_name.is_null() { return AE_BAD_PARAMETER; }
    let mut prefix = 0u32; let mut i = 0u32; let mut names = 0u32;
    match *internal_name { AML_ROOT_PREFIX => prefix = 1, AML_PARENT_PREFIX => { while i < internal_name_length && ACPI_IS_PARENT_PREFIX(*internal_name.add(i as usize)) { i += 1; prefix = i; } }, _ => {} }
    if prefix < internal_name_length { match *internal_name.add(prefix as usize) { AML_MULTI_NAME_PREFIX => { names = *internal_name.add(prefix as usize + 1) as u8 as u32; i = prefix + 2; }, AML_DUAL_NAME_PREFIX => { names = 2; i = prefix + 1; }, 0 => { i = 0; }, _ => { names = 1; i = prefix; } } }
    let required = prefix + 4 * names + if names > 0 { names - 1 } else { 0 } + 1;
    if required > internal_name_length { return AE_BAD_PATHNAME; }
    let out = ACPI_ALLOCATE_ZEROED(required as usize); if out.is_null() { return AE_NO_MEMORY; }
    for j in 0..prefix { *out.add(j as usize) = *internal_name.add(j as usize); }
    let mut j = prefix;
    for seg in 0..names { if seg > 0 { *out.add(j as usize) = b'.' as i8; j += 1; } ACPI_COPY_NAMESEG(out.add(j as usize), internal_name.add(i as usize)); acpi_ut_repair_name(out.add(j as usize)); j += 4; i += 4; }
    if !converted_name_length.is_null() { *converted_name_length = required; } *converted_name = out; AE_OK
}

pub unsafe fn acpi_ns_validate_handle(handle: acpi_handle) -> *mut acpi_namespace_node {
    if handle.is_null() || handle == ACPI_ROOT_OBJECT { return acpi_gbl_root_node; }
    if ACPI_GET_DESCRIPTOR_TYPE(handle) != ACPI_DESC_TYPE_NAMED { return core::ptr::null_mut(); }
    handle as *mut acpi_namespace_node
}

pub unsafe fn acpi_ns_terminate() { acpi_ns_delete_namespace_subtree(acpi_gbl_root_node); let status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); if ACPI_FAILURE(status) { return; } acpi_ns_delete_node(acpi_gbl_root_node); let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); }

pub unsafe fn acpi_ns_opens_scope(type_: acpi_object_type) -> u32 { if type_ > ACPI_TYPE_LOCAL_MAX { return ACPI_NS_NORMAL; } (acpi_gbl_ns_properties[type_ as usize] as u32) & ACPI_NS_NEWSCOPE }

pub unsafe fn acpi_ns_get_node_unlocked(prefix_node: *mut acpi_namespace_node, pathname: *const i8, flags: u32, return_node: *mut *mut acpi_namespace_node) -> acpi_status {
    if pathname.is_null() { *return_node = if prefix_node.is_null() { acpi_gbl_root_node } else { prefix_node }; return AE_OK; }
    if ACPI_IS_ROOT_PREFIX(*pathname) && *pathname.add(1) == 0 { *return_node = acpi_gbl_root_node; return AE_OK; }
    let mut internal = core::ptr::null_mut(); let status = acpi_ns_internalize_name(pathname, &mut internal); if ACPI_FAILURE(status) { return status; }
    let mut scope: acpi_generic_state = core::mem::zeroed(); scope.scope.node = prefix_node;
    let status = acpi_ns_lookup(&mut scope, internal, ACPI_TYPE_ANY, ACPI_IMODE_EXECUTE, flags | ACPI_NS_DONT_OPEN_SCOPE, core::ptr::null_mut(), return_node); ACPI_FREE(internal as *mut _); status
}

pub unsafe fn acpi_ns_get_node(prefix_node: *mut acpi_namespace_node, pathname: *const i8, flags: u32, return_node: *mut *mut acpi_namespace_node) -> acpi_status { let status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); if ACPI_FAILURE(status) { return status; } let result = acpi_ns_get_node_unlocked(prefix_node, pathname, flags, return_node); let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); result }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
