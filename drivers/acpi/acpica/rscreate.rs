// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: rscreate - Create resource lists/tables

// C dependencies are supplied by the surrounding ACPICA translation.

#[allow(non_camel_case_types)]
pub unsafe fn acpi_buffer_to_resource(
    aml_buffer: *mut u8,
    aml_buffer_length: u16,
    resource_ptr: *mut *mut acpi_resource,
) -> acpi_status {
    let mut status: acpi_status;
    let mut list_size_needed: acpi_size = 0;
    let resource: *mut core::ffi::c_void;
    let mut current_resource_ptr: *mut core::ffi::c_void;

    // Note: we allow AE_AML_NO_RESOURCE_END_TAG, since an end tag is not required here.
    status = acpi_rs_get_list_length(aml_buffer, aml_buffer_length, &mut list_size_needed);
    if status == AE_AML_NO_RESOURCE_END_TAG {
        status = AE_OK;
    }
    if ACPI_FAILURE(status) {
        return status;
    }

    resource = ACPI_ALLOCATE_ZEROED(list_size_needed);
    current_resource_ptr = resource;
    if resource.is_null() {
        return AE_NO_MEMORY;
    }

    status = acpi_ut_walk_aml_resources(
        core::ptr::null_mut(),
        aml_buffer,
        aml_buffer_length,
        Some(acpi_rs_convert_aml_to_resources),
        &mut current_resource_ptr as *mut *mut core::ffi::c_void,
    );
    if status == AE_AML_NO_RESOURCE_END_TAG {
        status = AE_OK;
    }
    if ACPI_FAILURE(status) {
        ACPI_FREE(resource);
    } else {
        *resource_ptr = resource as *mut acpi_resource;
    }
    status
}

pub unsafe fn acpi_rs_create_resource_list(
    aml_buffer: *mut acpi_operand_object,
    output_buffer: *mut acpi_buffer,
) -> acpi_status {
    let mut status: acpi_status;
    let aml_start: *mut u8;
    let mut list_size_needed: acpi_size = 0;
    let aml_buffer_length: u32;
    let mut resource: *mut core::ffi::c_void;

    aml_buffer_length = (*aml_buffer).buffer.length;
    aml_start = (*aml_buffer).buffer.pointer;

    status = acpi_rs_get_list_length(aml_start, aml_buffer_length, &mut list_size_needed);
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_ut_initialize_buffer(output_buffer, list_size_needed);
    if ACPI_FAILURE(status) {
        return status;
    }

    resource = (*output_buffer).pointer as *mut core::ffi::c_void;
    status = acpi_ut_walk_aml_resources(
        core::ptr::null_mut(),
        aml_start,
        aml_buffer_length,
        Some(acpi_rs_convert_aml_to_resources),
        &mut resource as *mut *mut core::ffi::c_void,
    );
    if ACPI_FAILURE(status) {
        return status;
    }
    AE_OK
}

pub unsafe fn acpi_rs_create_pci_routing_table(
    package_object: *mut acpi_operand_object,
    output_buffer: *mut acpi_buffer,
) -> acpi_status {
    let mut buffer: *mut u8;
    let mut top_object_list: *mut *mut acpi_operand_object;
    let mut sub_object_list: *mut *mut acpi_operand_object;
    let mut obj_desc: *mut acpi_operand_object;
    let mut buffer_size_needed: acpi_size = 0;
    let number_of_elements: u32;
    let mut index: u32;
    let mut user_prt: *mut acpi_pci_routing_table;
    let mut node: *mut acpi_namespace_node;
    let mut status: acpi_status;
    let mut path_buffer: acpi_buffer = core::mem::zeroed();

    status = acpi_rs_get_pci_routing_table_length(package_object, &mut buffer_size_needed);
    if ACPI_FAILURE(status) {
        return status;
    }
    status = acpi_ut_initialize_buffer(output_buffer, buffer_size_needed);
    if ACPI_FAILURE(status) {
        return status;
    }

    top_object_list = (*package_object).package.elements;
    number_of_elements = (*package_object).package.count;
    buffer = (*output_buffer).pointer;
    user_prt = buffer as *mut acpi_pci_routing_table;
    index = 0;
    while index < number_of_elements {
        buffer = buffer.add((*user_prt).length as usize);
        user_prt = buffer as *mut acpi_pci_routing_table;
        (*user_prt).length = (core::mem::size_of::<acpi_pci_routing_table>() - 4) as u32;

        if (**top_object_list).package.count != 4 {
            return AE_AML_PACKAGE_LIMIT;
        }
        sub_object_list = (**top_object_list).package.elements;

        obj_desc = *sub_object_list.add(0);
        if obj_desc.is_null() || (*obj_desc).common.type_ != ACPI_TYPE_INTEGER {
            return AE_BAD_DATA;
        }
        (*user_prt).address = (*obj_desc).integer.value;

        obj_desc = *sub_object_list.add(1);
        if obj_desc.is_null() || (*obj_desc).common.type_ != ACPI_TYPE_INTEGER {
            return AE_BAD_DATA;
        }
        (*user_prt).pin = (*obj_desc).integer.value as u32;

        obj_desc = *sub_object_list.add(2);
        if !obj_desc.is_null() {
            match (*obj_desc).common.type_ {
                ACPI_TYPE_LOCAL_REFERENCE => {
                    if (*obj_desc).reference.class != ACPI_REFCLASS_NAME {
                        return AE_BAD_DATA;
                    }
                    node = (*obj_desc).reference.node;
                    path_buffer.length = (*output_buffer).length -
                        ((*user_prt).source as usize - (*output_buffer).pointer as usize) as u32;
                    path_buffer.pointer = (*user_prt).source;
                    status = acpi_ns_handle_to_pathname(node as acpi_handle, &mut path_buffer, FALSE);
                    if ACPI_FAILURE(status) {
                        return status;
                    }
                    (*user_prt).length += libc_strlen((*user_prt).source) as u32 + 1;
                }
                ACPI_TYPE_STRING => {
                    libc_strcpy((*user_prt).source, (*obj_desc).string.pointer);
                    (*user_prt).length += (*obj_desc).string.length + 1;
                }
                ACPI_TYPE_INTEGER => {
                    (*user_prt).length += core::mem::size_of::<u32>() as u32;
                }
                _ => return AE_BAD_DATA,
            }
        }

        (*user_prt).length = ACPI_ROUND_UP_TO_64BIT((*user_prt).length) as u32;
        obj_desc = *sub_object_list.add(3);
        if obj_desc.is_null() || (*obj_desc).common.type_ != ACPI_TYPE_INTEGER {
            return AE_BAD_DATA;
        }
        (*user_prt).source_index = (*obj_desc).integer.value as u32;
        top_object_list = top_object_list.add(1);
        index += 1;
    }
    AE_OK
}

pub unsafe fn acpi_rs_create_aml_resources(
    resource_list: *mut acpi_buffer,
    output_buffer: *mut acpi_buffer,
) -> acpi_status {
    let mut status: acpi_status;
    let mut aml_size_needed: acpi_size = 0;

    status = acpi_rs_get_aml_length(
        (*resource_list).pointer,
        (*resource_list).length,
        &mut aml_size_needed,
    );
    if ACPI_FAILURE(status) {
        return status;
    }
    status = acpi_ut_initialize_buffer(output_buffer, aml_size_needed);
    if ACPI_FAILURE(status) {
        return status;
    }
    status = acpi_rs_convert_resources_to_aml(
        (*resource_list).pointer,
        aml_size_needed,
        (*output_buffer).pointer,
    );
    if ACPI_FAILURE(status) {
        return status;
    }
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
