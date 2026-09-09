// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Translation of rsutils.c. ACPI types, constants, and external routines are
// supplied by the surrounding ACPICA Rust translation.

pub unsafe fn acpi_rs_decode_bitmask(mut mask: u16, list: *mut u8) -> u8 {
    let mut i: u8 = 0;
    let mut bit_count: u8 = 0;
    while mask != 0 {
        if mask & 1 != 0 {
            *list.add(bit_count as usize) = i;
            bit_count = bit_count.wrapping_add(1);
        }
        mask >>= 1;
        i = i.wrapping_add(1);
    }
    bit_count
}

pub unsafe fn acpi_rs_encode_bitmask(list: *mut u8, count: u8) -> u16 {
    let mut mask: u16 = 0;
    let mut i: u32 = 0;
    while i < count as u32 {
        mask |= 0x1u16 << *list.add(i as usize);
        i += 1;
    }
    mask
}

pub unsafe fn acpi_rs_move_data(
    destination: *mut core::ffi::c_void,
    source: *mut core::ffi::c_void,
    item_count: u16,
    move_type: u8,
) {
    for i in 0..item_count as usize {
        match move_type {
            ACPI_RSC_MOVE8 | ACPI_RSC_MOVE_GPIO_RES | ACPI_RSC_MOVE_SERIAL_VEN
            | ACPI_RSC_MOVE_SERIAL_RES => {
                core::ptr::copy_nonoverlapping(source as *const u8, destination as *mut u8, item_count as usize);
                return;
            }
            ACPI_RSC_MOVE16 | ACPI_RSC_MOVE_GPIO_PIN => {
                let v = (source as *const u16).add(i).read_unaligned();
                (destination as *mut u16).add(i).write_unaligned(v);
            }
            ACPI_RSC_MOVE32 => {
                let v = (source as *const u32).add(i).read_unaligned();
                (destination as *mut u32).add(i).write_unaligned(v);
            }
            ACPI_RSC_MOVE64 => {
                let v = (source as *const u64).add(i).read_unaligned();
                (destination as *mut u64).add(i).write_unaligned(v);
            }
            _ => return,
        }
    }
}

pub unsafe fn acpi_rs_set_resource_length(
    total_length: acpi_rsdesc_size,
    aml: *mut aml_resource,
) {
    let resource_length: acpi_rs_length =
        (total_length - acpi_ut_get_resource_header_length(aml)) as acpi_rs_length;
    if (*aml).small_header.descriptor_type & ACPI_RESOURCE_NAME_LARGE != 0 {
        (*aml).large_header.resource_length = resource_length;
    } else {
        (*aml).small_header.descriptor_type =
            (((*aml).small_header.descriptor_type & !ACPI_RESOURCE_NAME_SMALL_LENGTH_MASK)
                | resource_length as u8) as u8;
    }
}

pub unsafe fn acpi_rs_set_resource_header(
    descriptor_type: u8,
    total_length: acpi_rsdesc_size,
    aml: *mut aml_resource,
) {
    (*aml).small_header.descriptor_type = descriptor_type;
    acpi_rs_set_resource_length(total_length, aml);
}

unsafe fn acpi_rs_strcpy(destination: *mut i8, source: *mut i8) -> u16 {
    let mut i: u16 = 0;
    while *source.add(i as usize) != 0 {
        *destination.add(i as usize) = *source.add(i as usize);
        i += 1;
    }
    *destination.add(i as usize) = 0;
    i.wrapping_add(1)
}

pub unsafe fn acpi_rs_get_resource_source(
    resource_length: acpi_rs_length,
    minimum_length: acpi_rs_length,
    resource_source: *mut acpi_resource_source,
    aml: *mut aml_resource,
    mut string_ptr: *mut i8,
) -> acpi_rs_length {
    let mut total_length = resource_length as acpi_rsdesc_size
        + core::mem::size_of::<aml_resource_large_header>() as acpi_rsdesc_size;
    let aml_resource_source = (aml as *mut u8).add(minimum_length as usize);
    if total_length > (minimum_length as acpi_rsdesc_size + 1) {
        (*resource_source).index = *aml_resource_source;
        (*resource_source).string_ptr = string_ptr;
        if string_ptr.is_null() {
            string_ptr = (resource_source as *mut u8)
                .add(core::mem::size_of::<acpi_resource_source>()) as *mut i8;
            (*resource_source).string_ptr = string_ptr;
        }
        let mut len = 0usize;
        while *aml_resource_source.add(1 + len) != 0 { len += 1; }
        total_length = ((len + 1 + core::mem::size_of::<usize>() - 1)
            / core::mem::size_of::<usize>() * core::mem::size_of::<usize>()) as acpi_rsdesc_size;
        core::ptr::write_bytes(string_ptr, 0, total_length as usize);
        (*resource_source).string_length =
            acpi_rs_strcpy(string_ptr, aml_resource_source.add(1) as *mut i8);
        return total_length as acpi_rs_length;
    }
    (*resource_source).index = 0;
    (*resource_source).string_length = 0;
    (*resource_source).string_ptr = core::ptr::null_mut();
    0
}

pub unsafe fn acpi_rs_set_resource_source(
    aml: *mut aml_resource,
    minimum_length: acpi_rs_length,
    resource_source: *mut acpi_resource_source,
) -> acpi_rsdesc_size {
    let mut descriptor_length = minimum_length as acpi_rsdesc_size;
    if (*resource_source).string_length != 0 {
        let p = (aml as *mut u8).add(minimum_length as usize);
        *p = (*resource_source).index as u8;
        let mut i = 0usize;
        while *(*resource_source).string_ptr.add(i) != 0 {
            *(p.add(1) as *mut i8).add(i) = *(*resource_source).string_ptr.add(i);
            i += 1;
        }
        *(p.add(1) as *mut i8).add(i) = 0;
        descriptor_length += (*resource_source).string_length as acpi_rsdesc_size + 1;
    }
    descriptor_length
}

pub unsafe fn acpi_rs_get_prt_method_data(node: *mut acpi_namespace_node, ret_buffer: *mut acpi_buffer) -> acpi_status {
    let mut obj_desc = core::ptr::null_mut();
    let status = acpi_ut_evaluate_object(node, METHOD_NAME__PRT, ACPI_BTYPE_PACKAGE, &mut obj_desc);
    if ACPI_FAILURE(status) { return status; }
    let result = acpi_rs_create_pci_routing_table(obj_desc, ret_buffer);
    acpi_ut_remove_reference(obj_desc);
    result
}

pub unsafe fn acpi_rs_get_crs_method_data(node: *mut acpi_namespace_node, ret_buffer: *mut acpi_buffer) -> acpi_status {
    let mut obj_desc = core::ptr::null_mut();
    let status = acpi_ut_evaluate_object(node, METHOD_NAME__CRS, ACPI_BTYPE_BUFFER, &mut obj_desc);
    if ACPI_FAILURE(status) { return status; }
    let result = acpi_rs_create_resource_list(obj_desc, ret_buffer);
    acpi_ut_remove_reference(obj_desc);
    result
}

pub unsafe fn acpi_rs_get_prs_method_data(node: *mut acpi_namespace_node, ret_buffer: *mut acpi_buffer) -> acpi_status {
    acpi_rs_get_method_data(node as acpi_handle, METHOD_NAME__PRS, ret_buffer)
}

pub unsafe fn acpi_rs_get_aei_method_data(node: *mut acpi_namespace_node, ret_buffer: *mut acpi_buffer) -> acpi_status {
    acpi_rs_get_method_data(node as acpi_handle, METHOD_NAME__AEI, ret_buffer)
}

pub unsafe fn acpi_rs_get_method_data(handle: acpi_handle, path: *const i8, ret_buffer: *mut acpi_buffer) -> acpi_status {
    let mut obj_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let status = acpi_ut_evaluate_object(handle as *mut acpi_namespace_node, path, ACPI_BTYPE_BUFFER, &mut obj_desc);
    if ACPI_FAILURE(status) { return status; }
    let result = acpi_rs_create_resource_list(obj_desc, ret_buffer);
    acpi_ut_remove_reference(obj_desc);
    result
}

pub unsafe fn acpi_rs_set_srs_method_data(node: *mut acpi_namespace_node, in_buffer: *mut acpi_buffer) -> acpi_status {
    let info = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_evaluate_info>()) as *mut acpi_evaluate_info;
    if info.is_null() { return AE_NO_MEMORY; }
    let mut args: [*mut acpi_operand_object; 2] = [core::ptr::null_mut(); 2];
    (*info).prefix_node = node;
    (*info).relative_pathname = METHOD_NAME__SRS;
    (*info).parameters = args.as_mut_ptr();
    (*info).flags = ACPI_IGNORE_RETURN_VALUE;
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_LOCAL_BUFFER, pointer: core::ptr::null_mut() };
    let mut status = acpi_rs_create_aml_resources(in_buffer, &mut buffer);
    if ACPI_FAILURE(status) { ACPI_FREE(info as *mut core::ffi::c_void); return status; }
    args[0] = acpi_ut_create_internal_object(ACPI_TYPE_BUFFER);
    if args[0].is_null() {
        ACPI_FREE(buffer.pointer);
        status = AE_NO_MEMORY;
    } else {
        (*args[0]).buffer.length = buffer.length as u32;
        (*args[0]).buffer.pointer = buffer.pointer;
        (*args[0]).common.flags = AOPOBJ_DATA_VALID;
        status = acpi_ns_evaluate(info);
        acpi_ut_remove_reference(args[0]);
    }
    ACPI_FREE(info as *mut core::ffi::c_void);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
