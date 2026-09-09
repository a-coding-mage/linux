// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// utmisc - common utility procedures

// Dependencies supplied by the surrounding ACPICA translation.

pub unsafe fn acpi_ut_is_pci_root_bridge(id: *mut i8) -> u8 {
    if libc::strcmp(id, PCI_ROOT_HID_STRING) == 0 ||
       libc::strcmp(id, PCI_EXPRESS_ROOT_HID_STRING) == 0 {
        return TRUE;
    }
    FALSE
}

#[cfg(any(feature = "acpi_asl_compiler", feature = "acpi_exec_app", feature = "acpi_names_app"))]
pub unsafe fn acpi_ut_is_aml_table(table: *mut acpi_table_header) -> u8 {
    if acpi_compare_nameseg((*table).signature.as_ptr(), ACPI_SIG_DSDT) ||
       acpi_compare_nameseg((*table).signature.as_ptr(), ACPI_SIG_PSDT) ||
       acpi_compare_nameseg((*table).signature.as_ptr(), ACPI_SIG_SSDT) ||
       acpi_compare_nameseg((*table).signature.as_ptr(), ACPI_SIG_OSDT) ||
       acpi_is_oem_sig((*table).signature.as_ptr()) {
        return TRUE;
    }
    FALSE
}

pub unsafe fn acpi_ut_dword_byte_swap(value: u32) -> u32 {
    value.swap_bytes()
}

pub unsafe fn acpi_ut_set_integer_width(revision: u8) {
    if revision < 2 {
        acpi_gbl_integer_bit_width = 32;
        acpi_gbl_integer_nybble_width = 8;
        acpi_gbl_integer_byte_width = 4;
    } else {
        acpi_gbl_integer_bit_width = 64;
        acpi_gbl_integer_nybble_width = 16;
        acpi_gbl_integer_byte_width = 8;
    }
}

pub unsafe fn acpi_ut_create_update_state_and_push(
    object: *mut acpi_operand_object,
    action: u16,
    state_list: *mut *mut acpi_generic_state,
) -> acpi_status {
    if object.is_null() { return AE_OK; }
    let state = acpi_ut_create_update_state(object, action);
    if state.is_null() { return AE_NO_MEMORY; }
    acpi_ut_push_generic_state(state_list, state);
    AE_OK
}

pub unsafe fn acpi_ut_walk_package_tree(
    source_object: *mut acpi_operand_object,
    target_object: *mut core::ffi::c_void,
    walk_callback: acpi_pkg_callback,
    context: *mut core::ffi::c_void,
) -> acpi_status {
    let mut status = AE_OK;
    let mut state_list: *mut acpi_generic_state = core::ptr::null_mut();
    let mut state = acpi_ut_create_pkg_state(source_object, target_object, 0);
    if state.is_null() { return AE_NO_MEMORY; }

    while !state.is_null() {
        let this_index = (*state).pkg.index;
        let this_source_obj = (*(*state).pkg.source_object).package.elements[this_index as usize];
        (*state).pkg.this_target_obj = &mut (*(*state).pkg.source_object).package.elements[this_index as usize];
        if this_source_obj.is_null() ||
           acpi_get_descriptor_type(this_source_obj) != ACPI_DESC_TYPE_OPERAND ||
           (*this_source_obj).common.type_ != ACPI_TYPE_PACKAGE {
            status = walk_callback(ACPI_COPY_TYPE_SIMPLE, this_source_obj, state, context);
            if acpi_failure(status) { return status; }
            (*state).pkg.index += 1;
            while (*state).pkg.index >= (*(*state).pkg.source_object).package.count {
                acpi_ut_delete_generic_state(state);
                state = acpi_ut_pop_generic_state(&mut state_list);
                if state.is_null() { return AE_OK; }
                (*state).pkg.index += 1;
            }
        } else {
            status = walk_callback(ACPI_COPY_TYPE_PACKAGE, this_source_obj, state, context);
            if acpi_failure(status) { return status; }
            acpi_ut_push_generic_state(&mut state_list, state);
            state = acpi_ut_create_pkg_state(this_source_obj, (*state).pkg.this_target_obj, 0);
            if state.is_null() {
                while !state_list.is_null() {
                    state = acpi_ut_pop_generic_state(&mut state_list);
                    acpi_ut_delete_generic_state(state);
                }
                return AE_NO_MEMORY;
            }
        }
    }
    acpi_error(AE_INFO, "State list did not terminate correctly");
    AE_AML_INTERNAL
}

#[cfg(feature = "acpi_debug_output")]
pub unsafe fn acpi_ut_display_init_pathname(
    type_: u8, obj_handle: *mut acpi_namespace_node, path: *const i8,
) {
    if (acpi_dbg_level & ACPI_LV_INIT_NAMES) == 0 { return; }
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_LOCAL_BUFFER, pointer: core::ptr::null_mut() };
    let status = acpi_ns_handle_to_pathname(obj_handle, &mut buffer, TRUE);
    if acpi_failure(status) { return; }
    if type_ == ACPI_TYPE_METHOD { acpi_os_printf(b"Executing  \0".as_ptr() as *const i8); }
    else { acpi_os_printf(b"Initializing \0".as_ptr() as *const i8); }
    acpi_os_printf(b"%-12s %s\0".as_ptr() as *const i8, acpi_ut_get_type_name(type_), buffer.pointer as *const i8);
    if !path.is_null() { acpi_os_printf(b".%s\0".as_ptr() as *const i8, path); }
    acpi_os_printf(b"\n\0".as_ptr() as *const i8);
    acpi_free(buffer.pointer);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
