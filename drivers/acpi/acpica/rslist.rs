// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: rslist - Linked list utilities
 *
 ******************************************************************************/

// Dependencies supplied by the ACPI implementation.

/* Resource conversion callback: convert an AML resource to an internal resource. */
pub unsafe fn acpi_rs_convert_aml_to_resources(
    aml: *mut u8,
    length: u32,
    offset: u32,
    resource_index: u8,
    context: *mut *mut core::ffi::c_void,
) -> acpi_status {
    let resource_ptr = context as *mut *mut acpi_resource;
    let resource: *mut acpi_resource = *resource_ptr;
    let aml_resource = aml as *mut aml_resource;
    let conversion_table: *mut acpi_rsconvert_info;
    let status: acpi_status;

    acpi_function_trace!(rs_convert_aml_to_resources);

    // Check that the input buffer and all subsequent pointers into it are aligned.
    if acpi_is_misaligned!(resource) {
        acpi_warning!(AE_INFO, "Misaligned resource pointer %p", resource);
    }

    if acpi_ut_get_resource_type(aml) == ACPI_RESOURCE_NAME_SERIAL_BUS {
        if (*aml_resource).common_serial_bus.type_ > AML_RESOURCE_MAX_SERIALBUSTYPE {
            conversion_table = core::ptr::null_mut();
        } else {
            conversion_table = acpi_gbl_convert_resource_serial_bus_dispatch
                [(*aml_resource).common_serial_bus.type_ as usize];
        }
    } else {
        conversion_table = acpi_gbl_get_resource_dispatch[resource_index as usize];
    }

    if conversion_table.is_null() {
        acpi_error!(
            AE_INFO,
            "Invalid/unsupported resource descriptor: Type 0x%2.2X",
            resource_index
        );
        return AE_AML_INVALID_RESOURCE_TYPE;
    }

    status = acpi_rs_convert_aml_to_resource(resource, aml_resource, conversion_table);
    if acpi_failure(status) {
        acpi_exception!(
            AE_INFO,
            status,
            "Could not convert AML resource (Type 0x%X)",
            *aml
        );
        return status;
    }

    if (*resource).length == 0 {
        acpi_exception!(
            AE_INFO,
            status,
            "Zero-length resource returned from RsConvertAmlToResource"
        );
    }

    acpi_debug_print!(
        ACPI_DB_RESOURCES,
        "Type %.2X, AmlLength %.2X InternalLength %.2X\n",
        acpi_ut_get_resource_type(aml),
        length,
        (*resource).length
    );

    *resource_ptr = acpi_next_resource!(resource);
    AE_OK
}

/* Convert the internal resource linked list to an AML byte stream. */
pub unsafe fn acpi_rs_convert_resources_to_aml(
    mut resource: *mut acpi_resource,
    aml_size_needed: acpi_size,
    output_buffer: *mut u8,
) -> acpi_status {
    let mut aml = output_buffer;
    let end_aml = output_buffer.add(aml_size_needed as usize);
    let conversion_table: *mut acpi_rsconvert_info;
    let status: acpi_status;

    acpi_function_trace!(rs_convert_resources_to_aml);

    while aml < end_aml {
        if (*resource).type_ > ACPI_RESOURCE_TYPE_MAX {
            acpi_error!(
                AE_INFO,
                "Invalid descriptor type (0x%X) in resource list",
                (*resource).type_
            );
            return AE_BAD_DATA;
        }

        if (*resource).length == 0 {
            acpi_error!(AE_INFO, "Invalid zero length descriptor in resource list\n");
            return AE_AML_BAD_RESOURCE_LENGTH;
        }

        if (*resource).type_ == ACPI_RESOURCE_TYPE_SERIAL_BUS {
            if (*resource).data.common_serial_bus.type_ > AML_RESOURCE_MAX_SERIALBUSTYPE {
                conversion_table = core::ptr::null_mut();
            } else {
                conversion_table = acpi_gbl_convert_resource_serial_bus_dispatch
                    [(*resource).data.common_serial_bus.type_ as usize];
            }
        } else {
            conversion_table = acpi_gbl_set_resource_dispatch[(*resource).type_ as usize];
        }

        if conversion_table.is_null() {
            acpi_error!(
                AE_INFO,
                "Invalid/unsupported resource descriptor: Type 0x%2.2X",
                (*resource).type_
            );
            return AE_AML_INVALID_RESOURCE_TYPE;
        }

        status = acpi_rs_convert_resource_to_aml(resource, aml as *mut aml_resource, conversion_table);
        if acpi_failure(status) {
            acpi_exception!(
                AE_INFO,
                status,
                "Could not convert resource (type 0x%X) to AML",
                (*resource).type_
            );
            return status;
        }

        status = acpi_ut_validate_resource(core::ptr::null_mut(), aml as *mut aml_resource, core::ptr::null_mut());
        if acpi_failure(status) {
            return status;
        }

        if (*resource).type_ == ACPI_RESOURCE_TYPE_END_TAG {
            return AE_OK;
        }

        aml = aml.add(acpi_ut_get_descriptor_length(aml) as usize);
        resource = acpi_next_resource!(resource);
    }

    AE_AML_NO_RESOURCE_END_TAG
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
