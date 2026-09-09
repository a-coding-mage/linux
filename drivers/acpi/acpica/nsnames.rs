// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Module Name: nsnames - Name manipulation and search
 */

use core::ffi::{c_char, c_void};

pub unsafe fn acpi_ns_get_external_pathname(
    node: *mut acpi_namespace_node,
) -> *mut c_char {
    acpi_ns_get_normalized_pathname(node, FALSE)
}

pub unsafe fn acpi_ns_get_pathname_length(node: *mut acpi_namespace_node) -> acpi_size {
    if ACPI_GET_DESCRIPTOR_TYPE(node) != ACPI_DESC_TYPE_NAMED {
        ACPI_ERROR((AE_INFO, "Invalid/cached reference target node: %p, descriptor type %d", node, ACPI_GET_DESCRIPTOR_TYPE(node)));
        return 0;
    }

    acpi_ns_build_normalized_path(node, core::ptr::null_mut(), 0, FALSE) as acpi_size
}

pub unsafe fn acpi_ns_handle_to_name(
    target_handle: acpi_handle,
    buffer: *mut acpi_buffer,
) -> acpi_status {
    let node = acpi_ns_validate_handle(target_handle);
    if node.is_null() {
        return AE_BAD_PARAMETER;
    }

    let status = acpi_ut_initialize_buffer(buffer, ACPI_PATH_SEGMENT_LENGTH);
    if ACPI_FAILURE(status) {
        return status;
    }

    let node_name = acpi_ut_get_node_name(node);
    ACPI_COPY_NAMESEG((*buffer).pointer, node_name);
    *((*buffer).pointer as *mut c_char).add(ACPI_NAMESEG_SIZE as usize) = 0;

    ACPI_DEBUG_PRINT((ACPI_DB_EXEC, "%4.4s\n", (*buffer).pointer as *mut c_char));
    AE_OK
}

pub unsafe fn acpi_ns_handle_to_pathname(
    target_handle: acpi_handle,
    buffer: *mut acpi_buffer,
    no_trailing: u8,
) -> acpi_status {
    let node = acpi_ns_validate_handle(target_handle);
    if node.is_null() {
        return AE_BAD_PARAMETER;
    }

    let required_size = acpi_ns_build_normalized_path(
        node,
        core::ptr::null_mut(),
        0,
        no_trailing,
    );
    if required_size == 0 {
        return AE_BAD_PARAMETER;
    }

    let status = acpi_ut_initialize_buffer(buffer, required_size as acpi_size);
    if ACPI_FAILURE(status) {
        return status;
    }

    acpi_ns_build_normalized_path(
        node,
        (*buffer).pointer as *mut c_char,
        required_size,
        no_trailing,
    );

    ACPI_DEBUG_PRINT((ACPI_DB_EXEC, "%s [%X]\n", (*buffer).pointer as *mut c_char, required_size));
    AE_OK
}

pub unsafe fn acpi_ns_build_normalized_path(
    node: *mut acpi_namespace_node,
    full_path: *mut c_char,
    mut path_size: u32,
    no_trailing: u8,
) -> u32 {
    let mut length: u32 = 0;
    let mut name = [0 as c_char; ACPI_NAMESEG_SIZE as usize];
    let mut next_node: *mut acpi_namespace_node;

    macro_rules! path_put8 {
        ($path:expr, $size:expr, $byte:expr, $len:expr) => {{
            if $len < $size {
                *$path.add($len as usize) = $byte;
            }
            $len += 1;
        }};
    }

    if full_path.is_null() {
        path_size = 0;
    }
    if node.is_null() {
        path_put8!(full_path, path_size, 0 as c_char, length);
        return length;
    }
    if ACPI_GET_DESCRIPTOR_TYPE(node) != ACPI_DESC_TYPE_NAMED {
        path_put8!(full_path, path_size, 0 as c_char, length);
        return length;
    }

    next_node = node;
    while !next_node.is_null() && next_node != acpi_gbl_root_node {
        if next_node != node {
            path_put8!(full_path, path_size, AML_DUAL_NAME_PREFIX as c_char, length);
        }

        ACPI_MOVE_32_TO_32(name.as_mut_ptr() as *mut c_void, &(*next_node).name as *const _ as *const c_void);
        let mut do_no_trailing = no_trailing;
        for i in 0..4 {
            let c = name[4 - i - 1];
            if do_no_trailing != 0 && c != '_' as c_char {
                do_no_trailing = FALSE;
            }
            if do_no_trailing == 0 {
                path_put8!(full_path, path_size, c, length);
            }
        }
        next_node = (*next_node).parent;
    }

    path_put8!(full_path, path_size, AML_ROOT_PREFIX as c_char, length);

    if length <= path_size {
        let mut left = full_path;
        let mut right = full_path.add(length as usize - 1);
        while left < right {
            let c = *left;
            *left = *right;
            *right = c;
            left = left.add(1);
            right = right.sub(1);
        }
    }

    path_put8!(full_path, path_size, 0 as c_char, length);
    length
}

pub unsafe fn acpi_ns_get_normalized_pathname(
    node: *mut acpi_namespace_node,
    no_trailing: u8,
) -> *mut c_char {
    let size = acpi_ns_build_normalized_path(node, core::ptr::null_mut(), 0, no_trailing);
    if size == 0 {
        return core::ptr::null_mut();
    }

    let name_buffer = ACPI_ALLOCATE_ZEROED(size as acpi_size) as *mut c_char;
    if name_buffer.is_null() {
        ACPI_ERROR((AE_INFO, "Could not allocate %u bytes", size));
        return core::ptr::null_mut();
    }

    acpi_ns_build_normalized_path(node, name_buffer, size, no_trailing);
    ACPI_DEBUG_PRINT_RAW((ACPI_DB_NAMES, "%s: Path \"%s\"\n", ACPI_GET_FUNCTION_NAME, name_buffer));
    name_buffer
}

pub unsafe fn acpi_ns_build_prefixed_pathname(
    prefix_scope: *mut acpi_generic_state,
    internal_path: *const c_char,
) -> *mut c_char {
    let mut full_path: *mut c_char = core::ptr::null_mut();
    let mut external_path: *mut c_char = core::ptr::null_mut();
    let mut prefix_path: *mut c_char = core::ptr::null_mut();
    let mut prefix_path_length: acpi_size = 0;

    if !prefix_scope.is_null() && !(*prefix_scope).scope.node.is_null() {
        prefix_path = acpi_ns_get_normalized_pathname((*prefix_scope).scope.node, TRUE);
        if !prefix_path.is_null() {
            prefix_path_length = strlen(prefix_path) as acpi_size;
        }
    }

    let status = acpi_ns_externalize_name(ACPI_UINT32_MAX, internal_path, core::ptr::null_mut(), &mut external_path);
    if ACPI_FAILURE(status) {
        if !prefix_path.is_null() { ACPI_FREE(prefix_path as *mut c_void); }
        if !external_path.is_null() { ACPI_FREE(external_path as *mut c_void); }
        return full_path;
    }

    full_path = ACPI_ALLOCATE_ZEROED(prefix_path_length + strlen(external_path) as acpi_size + 2) as *mut c_char;
    if full_path.is_null() {
        if !prefix_path.is_null() { ACPI_FREE(prefix_path as *mut c_void); }
        if !external_path.is_null() { ACPI_FREE(external_path as *mut c_void); }
        return full_path;
    }

    if !prefix_path.is_null() && *external_path != b'\\' as c_char && *external_path != b'^' as c_char {
        strcat(full_path, prefix_path);
        if *prefix_path.add(1) != 0 {
            strcat(full_path, b".".as_ptr() as *const c_char);
        }
    }

    acpi_ns_normalize_pathname(external_path);
    strcat(full_path, external_path);

    if !prefix_path.is_null() { ACPI_FREE(prefix_path as *mut c_void); }
    if !external_path.is_null() { ACPI_FREE(external_path as *mut c_void); }
    full_path
}

pub unsafe fn acpi_ns_normalize_pathname(original_path: *mut c_char) {
    let mut input_path = original_path;
    let new_path_buffer = ACPI_ALLOCATE_ZEROED(strlen(input_path) as acpi_size + 1) as *mut c_char;
    if new_path_buffer.is_null() { return; }
    let mut new_path = new_path_buffer;

    if *input_path == b'\\' as c_char {
        *new_path = *input_path; new_path = new_path.add(1); input_path = input_path.add(1);
    }
    while *input_path == b'^' as c_char {
        *new_path = *input_path; new_path = new_path.add(1); input_path = input_path.add(1);
    }
    while *input_path != 0 {
        for i in 0..ACPI_NAMESEG_SIZE {
            if *input_path == 0 { break; }
            if i == 0 || *input_path != b'_' as c_char { *new_path = *input_path; new_path = new_path.add(1); }
            input_path = input_path.add(1);
        }
        if *input_path == b'.' as c_char { *new_path = *input_path; new_path = new_path.add(1); input_path = input_path.add(1); }
    }
    *new_path = 0;
    strcpy(original_path, new_path_buffer);
    ACPI_FREE(new_path_buffer as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
