// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: rsxface - Public interfaces to the resource manager
 *
 ******************************************************************************/

// #define EXPORT_ACPI_INTERFACES
// C headers supplied by the surrounding ACPICA translation.

// #define _COMPONENT ACPI_RESOURCES
// ACPI_MODULE_NAME("rsxface")

/* Local macros for 16,32-bit to 64-bit conversion */

/* Local prototypes */
unsafe fn acpi_rs_match_vendor_resource(
    resource: *mut acpi_resource,
    context: *mut core::ffi::c_void,
) -> acpi_status;

unsafe fn acpi_rs_validate_parameters(
    device_handle: acpi_handle,
    buffer: *mut acpi_buffer,
    return_node: *mut *mut acpi_namespace_node,
) -> acpi_status;

/*******************************************************************************
 *
 * FUNCTION:    acpi_rs_validate_parameters
 *
 * PARAMETERS:  device_handle   - Handle to a device
 *              buffer          - Pointer to a data buffer
 *              return_node     - Pointer to where the device node is returned
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Common parameter validation for resource interfaces
 *
 ******************************************************************************/

unsafe fn acpi_rs_validate_parameters(
    device_handle: acpi_handle,
    buffer: *mut acpi_buffer,
    return_node: *mut *mut acpi_namespace_node,
) -> acpi_status {
    let status: acpi_status;
    let node: *mut acpi_namespace_node;

    /* Must have a valid handle to an ACPI device */
    if device_handle.is_null() {
        return AE_BAD_PARAMETER;
    }

    node = acpi_ns_validate_handle(device_handle);
    if node.is_null() {
        return AE_BAD_PARAMETER;
    }

    if (*node).type_ != ACPI_TYPE_DEVICE {
        return AE_TYPE;
    }

    /* Validate the user buffer object */
    status = acpi_ut_validate_buffer(buffer);
    if ACPI_FAILURE(status) {
        return status;
    }

    *return_node = node;
    AE_OK
}

pub unsafe fn acpi_get_irq_routing_table(
    device_handle: acpi_handle,
    ret_buffer: *mut acpi_buffer,
) -> acpi_status {
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    let status = acpi_rs_validate_parameters(device_handle, ret_buffer, &mut node);
    if ACPI_FAILURE(status) { return status; }
    acpi_rs_get_prt_method_data(node, ret_buffer)
}

pub unsafe fn acpi_get_current_resources(
    device_handle: acpi_handle,
    ret_buffer: *mut acpi_buffer,
) -> acpi_status {
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    let status = acpi_rs_validate_parameters(device_handle, ret_buffer, &mut node);
    if ACPI_FAILURE(status) { return status; }
    acpi_rs_get_crs_method_data(node, ret_buffer)
}

pub unsafe fn acpi_get_possible_resources(
    device_handle: acpi_handle,
    ret_buffer: *mut acpi_buffer,
) -> acpi_status {
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    let status = acpi_rs_validate_parameters(device_handle, ret_buffer, &mut node);
    if ACPI_FAILURE(status) { return status; }
    acpi_rs_get_prs_method_data(node, ret_buffer)
}

pub unsafe fn acpi_set_current_resources(
    device_handle: acpi_handle,
    in_buffer: *mut acpi_buffer,
) -> acpi_status {
    if in_buffer.is_null() || (*in_buffer).pointer.is_null() || (*in_buffer).length == 0 {
        return AE_BAD_PARAMETER;
    }
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    let status = acpi_rs_validate_parameters(device_handle, in_buffer, &mut node);
    if ACPI_FAILURE(status) { return status; }
    acpi_rs_set_srs_method_data(node, in_buffer)
}

pub unsafe fn acpi_get_event_resources(
    device_handle: acpi_handle,
    ret_buffer: *mut acpi_buffer,
) -> acpi_status {
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    let status = acpi_rs_validate_parameters(device_handle, ret_buffer, &mut node);
    if ACPI_FAILURE(status) { return status; }
    acpi_rs_get_aei_method_data(node, ret_buffer)
}

pub unsafe fn acpi_resource_to_address64(
    resource: *mut acpi_resource,
    out: *mut acpi_resource_address64,
) -> acpi_status {
    if resource.is_null() || out.is_null() { return AE_BAD_PARAMETER; }
    match (*resource).type_ {
        ACPI_RESOURCE_TYPE_ADDRESS16 => {
            let input = &(*resource).data as *const _ as *const acpi_resource_address16;
            (*out).resource_type = (*input).resource_type;
            (*out).producer_consumer = (*input).producer_consumer;
            (*out).decode = (*input).decode;
            (*out).min_address_fixed = (*input).min_address_fixed;
            (*out).max_address_fixed = (*input).max_address_fixed;
            (*out).info = (*input).info;
            (*out).address.granularity = (*input).address.granularity;
            (*out).address.minimum = (*input).address.minimum;
            (*out).address.maximum = (*input).address.maximum;
            (*out).address.translation_offset = (*input).address.translation_offset;
            (*out).address.address_length = (*input).address.address_length;
            (*out).resource_source = (*input).resource_source;
        }
        ACPI_RESOURCE_TYPE_ADDRESS32 => {
            let input = &(*resource).data as *const _ as *const acpi_resource_address32;
            core::ptr::copy_nonoverlapping(input as *const acpi_resource_address32 as *const u8, out as *mut u8, core::mem::size_of::<acpi_resource_address32>());
        }
        ACPI_RESOURCE_TYPE_ADDRESS64 => {
            core::ptr::copy_nonoverlapping(&(*resource).data as *const _ as *const u8, out as *mut u8, core::mem::size_of::<acpi_resource_address64>());
        }
        _ => return AE_BAD_PARAMETER,
    }
    AE_OK
}

pub unsafe fn acpi_get_vendor_resource(
    device_handle: acpi_handle,
    name: *mut core::ffi::c_char,
    uuid: *mut acpi_vendor_uuid,
    ret_buffer: *mut acpi_buffer,
) -> acpi_status {
    if uuid.is_null() || ret_buffer.is_null() { return AE_BAD_PARAMETER; }
    let mut info = acpi_vendor_walk_info { uuid, buffer: ret_buffer, status: AE_NOT_EXIST };
    let status = acpi_walk_resources(device_handle, name, Some(acpi_rs_match_vendor_resource), &mut info as *mut _ as *mut core::ffi::c_void);
    if ACPI_FAILURE(status) { return status; }
    info.status
}

unsafe fn acpi_rs_match_vendor_resource(resource: *mut acpi_resource, context: *mut core::ffi::c_void) -> acpi_status {
    if (*resource).type_ != ACPI_RESOURCE_TYPE_VENDOR { return AE_OK; }
    let info = &mut *(context as *mut acpi_vendor_walk_info);
    let vendor = &(*resource).data.vendor_typed;
    if vendor.byte_length < ACPI_UUID_LENGTH + 1 || vendor.uuid_subtype != (*info.uuid).subtype ||
        libc::memcmp(vendor.uuid.as_ptr() as *const _, (*info.uuid).data.as_ptr() as *const _, ACPI_UUID_LENGTH) != 0 { return AE_OK; }
    let status = acpi_ut_initialize_buffer(info.buffer, (*resource).length);
    if ACPI_FAILURE(status) { return status; }
    libc::memcpy((*info.buffer).pointer, resource as *const _, (*resource).length);
    (*info.buffer).length = (*resource).length;
    info.status = AE_OK;
    AE_CTRL_TERMINATE
}

pub unsafe fn acpi_walk_resource_buffer(buffer: *mut acpi_buffer, user_function: acpi_walk_resource_callback, context: *mut core::ffi::c_void) -> acpi_status {
    if buffer.is_null() || (*buffer).pointer.is_null() || user_function.is_none() { return AE_BAD_PARAMETER; }
    let mut resource = (*buffer).pointer as *mut acpi_resource;
    let resource_end = (*buffer).pointer.add((*buffer).length) as *mut acpi_resource;
    let mut status = AE_OK;
    while resource < resource_end {
        if (*resource).type_ > ACPI_RESOURCE_TYPE_MAX { status = AE_AML_INVALID_RESOURCE_TYPE; break; }
        if (*resource).length == 0 { return AE_AML_BAD_RESOURCE_LENGTH; }
        status = user_function.unwrap()(resource, context);
        if ACPI_FAILURE(status) {
            if status == AE_CTRL_TERMINATE { status = AE_OK; }
            break;
        }
        if (*resource).type_ == ACPI_RESOURCE_TYPE_END_TAG { break; }
        resource = (resource as *mut u8).add((*resource).length as usize) as *mut acpi_resource;
    }
    status
}

pub unsafe fn acpi_walk_resources(device_handle: acpi_handle, name: *mut core::ffi::c_char, user_function: acpi_walk_resource_callback, context: *mut core::ffi::c_void) -> acpi_status {
    if device_handle.is_null() || user_function.is_none() || name.is_null() ||
        (!ACPI_COMPARE_NAMESEG(name, METHOD_NAME__CRS) && !ACPI_COMPARE_NAMESEG(name, METHOD_NAME__PRS) && !ACPI_COMPARE_NAMESEG(name, METHOD_NAME__AEI) && !ACPI_COMPARE_NAMESEG(name, METHOD_NAME__DMA)) { return AE_BAD_PARAMETER; }
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_LOCAL_BUFFER, pointer: core::ptr::null_mut() };
    let status = acpi_rs_get_method_data(device_handle, name, &mut buffer);
    if ACPI_FAILURE(status) { return status; }
    let status = acpi_walk_resource_buffer(&mut buffer, user_function, context);
    ACPI_FREE(buffer.pointer);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
