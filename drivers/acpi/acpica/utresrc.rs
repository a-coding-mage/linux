// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: utresrc - Resource management utilities
 *
 ******************************************************************************/

// External ACPI definitions supplied by the surrounding translation unit.

/*
 * Base sizes of the raw AML resource descriptors, indexed by resource type.
 * Zero indicates a reserved (and therefore invalid) resource type.
 */
pub static ACPI_GBL_RESOURCE_AML_SIZES: [u8; 37] = [
    /* Small descriptors */
    0, 0, 0, 0,
    ACPI_AML_SIZE_SMALL!(aml_resource_irq),
    ACPI_AML_SIZE_SMALL!(aml_resource_dma),
    ACPI_AML_SIZE_SMALL!(aml_resource_start_dependent),
    ACPI_AML_SIZE_SMALL!(aml_resource_end_dependent),
    ACPI_AML_SIZE_SMALL!(aml_resource_io),
    ACPI_AML_SIZE_SMALL!(aml_resource_fixed_io),
    ACPI_AML_SIZE_SMALL!(aml_resource_fixed_dma),
    0, 0, 0,
    ACPI_AML_SIZE_SMALL!(aml_resource_vendor_small),
    ACPI_AML_SIZE_SMALL!(aml_resource_end_tag),

    /* Large descriptors */
    0,
    ACPI_AML_SIZE_LARGE!(aml_resource_memory24),
    ACPI_AML_SIZE_LARGE!(aml_resource_generic_register),
    0,
    ACPI_AML_SIZE_LARGE!(aml_resource_vendor_large),
    ACPI_AML_SIZE_LARGE!(aml_resource_memory32),
    ACPI_AML_SIZE_LARGE!(aml_resource_fixed_memory32),
    ACPI_AML_SIZE_LARGE!(aml_resource_address32),
    ACPI_AML_SIZE_LARGE!(aml_resource_address16),
    ACPI_AML_SIZE_LARGE!(aml_resource_extended_irq),
    ACPI_AML_SIZE_LARGE!(aml_resource_address64),
    ACPI_AML_SIZE_LARGE!(aml_resource_extended_address64),
    ACPI_AML_SIZE_LARGE!(aml_resource_gpio),
    ACPI_AML_SIZE_LARGE!(aml_resource_pin_function),
    ACPI_AML_SIZE_LARGE!(aml_resource_common_serialbus),
    ACPI_AML_SIZE_LARGE!(aml_resource_pin_config),
    ACPI_AML_SIZE_LARGE!(aml_resource_pin_group),
    ACPI_AML_SIZE_LARGE!(aml_resource_pin_group_function),
    ACPI_AML_SIZE_LARGE!(aml_resource_pin_group_config),
    ACPI_AML_SIZE_LARGE!(aml_resource_clock_input),
];

pub static ACPI_GBL_RESOURCE_AML_SERIAL_BUS_SIZES: [u8; 5] = [
    0,
    ACPI_AML_SIZE_LARGE!(aml_resource_i2c_serialbus),
    ACPI_AML_SIZE_LARGE!(aml_resource_spi_serialbus),
    ACPI_AML_SIZE_LARGE!(aml_resource_uart_serialbus),
    ACPI_AML_SIZE_LARGE!(aml_resource_csi2_serialbus),
];

/* Resource types, used to validate the resource length field. */
static ACPI_GBL_RESOURCE_TYPES: [u8; 37] = [
    0, 0, 0, 0,
    ACPI_SMALL_VARIABLE_LENGTH, ACPI_FIXED_LENGTH,
    ACPI_SMALL_VARIABLE_LENGTH, ACPI_FIXED_LENGTH,
    ACPI_FIXED_LENGTH, ACPI_FIXED_LENGTH, ACPI_FIXED_LENGTH,
    0, 0, 0,
    ACPI_VARIABLE_LENGTH, ACPI_FIXED_LENGTH,
    0, ACPI_FIXED_LENGTH, ACPI_FIXED_LENGTH, 0,
    ACPI_VARIABLE_LENGTH, ACPI_FIXED_LENGTH, ACPI_FIXED_LENGTH,
    ACPI_VARIABLE_LENGTH, ACPI_VARIABLE_LENGTH, ACPI_VARIABLE_LENGTH,
    ACPI_VARIABLE_LENGTH, ACPI_FIXED_LENGTH, ACPI_VARIABLE_LENGTH,
    ACPI_VARIABLE_LENGTH, ACPI_VARIABLE_LENGTH, ACPI_VARIABLE_LENGTH,
    ACPI_VARIABLE_LENGTH, ACPI_VARIABLE_LENGTH, ACPI_VARIABLE_LENGTH,
    ACPI_VARIABLE_LENGTH, ACPI_VARIABLE_LENGTH, ACPI_VARIABLE_LENGTH,
];

pub unsafe fn acpi_ut_walk_aml_resources(
    walk_state: *mut acpi_walk_state,
    mut aml: *mut u8,
    aml_length: acpi_size,
    user_function: acpi_walk_aml_callback,
    context: *mut *mut core::ffi::c_void,
) -> acpi_status {
    let mut status: acpi_status;
    let end_aml = aml.add(aml_length as usize);
    let mut resource_index: u8 = 0;
    let mut length: u32;
    let mut offset: u32 = 0;
    let mut end_tag = [0x79u8, 0x00];

    if aml_length < core::mem::size_of::<aml_resource_end_tag>() {
        return AE_AML_NO_RESOURCE_END_TAG;
    }
    while aml < end_aml {
        if end_aml.offset_from(aml) < core::mem::size_of::<aml_resource_end_tag>() as isize {
            return AE_AML_BUFFER_LENGTH;
        }
        if (*aml & ACPI_RESOURCE_NAME_LARGE) != 0
            && end_aml.offset_from(aml)
                < (ACPI_OFFSET!(aml_resource_common_serialbus, type_) + 1) as isize
        {
            return AE_AML_BUFFER_LENGTH;
        }
        status = acpi_ut_validate_resource(walk_state, aml as *mut _, &mut resource_index);
        if ACPI_FAILURE(status) { return status; }
        length = acpi_ut_get_descriptor_length(aml as *mut _);
        if length as isize > end_aml.offset_from(aml) { return AE_AML_BUFFER_LENGTH; }
        if let Some(function) = user_function {
            status = function(aml, length, offset, resource_index, context);
            if ACPI_FAILURE(status) { return status; }
        }
        if acpi_ut_get_resource_type(aml as *mut _) == ACPI_RESOURCE_NAME_END_TAG {
            if aml.add(1) >= end_aml { return AE_AML_NO_RESOURCE_END_TAG; }
            if user_function.is_none() { *context = aml as *mut _; }
            return AE_OK;
        }
        aml = aml.add(length as usize);
        offset = offset.wrapping_add(length);
    }
    if user_function.is_some() {
        let _ = acpi_ut_validate_resource(walk_state, end_tag.as_mut_ptr() as *mut _, &mut resource_index);
        status = user_function.unwrap()(end_tag.as_mut_ptr(), 2, offset, resource_index, context);
        if ACPI_FAILURE(status) { return status; }
    }
    AE_AML_NO_RESOURCE_END_TAG
}

pub unsafe fn acpi_ut_validate_resource(
    walk_state: *mut acpi_walk_state, aml: *mut core::ffi::c_void, return_index: *mut u8,
) -> acpi_status {
    let resource_type = ACPI_GET8!(aml);
    let resource_index: u8 = if (resource_type & ACPI_RESOURCE_NAME_LARGE) != 0 {
        if resource_type > ACPI_RESOURCE_NAME_LARGE_MAX { return invalid_resource(walk_state, resource_type); }
        resource_type.wrapping_sub(0x70)
    } else { (resource_type & ACPI_RESOURCE_NAME_SMALL_MASK) >> 3 };
    if ACPI_GBL_RESOURCE_TYPES[resource_index as usize] == 0 { return invalid_resource(walk_state, resource_type); }
    let resource_length = acpi_ut_get_resource_length(aml);
    let minimum_resource_length = ACPI_GBL_RESOURCE_AML_SIZES[resource_index as usize] as u16;
    match ACPI_GBL_RESOURCE_TYPES[resource_index as usize] {
        ACPI_FIXED_LENGTH if resource_length != minimum_resource_length => return bad_resource_length(walk_state, resource_type, resource_length, minimum_resource_length),
        ACPI_VARIABLE_LENGTH if resource_length < minimum_resource_length => return bad_resource_length(walk_state, resource_type, resource_length, minimum_resource_length),
        ACPI_SMALL_VARIABLE_LENGTH if resource_length > minimum_resource_length || resource_length < minimum_resource_length - 1 => return bad_resource_length(walk_state, resource_type, resource_length, minimum_resource_length),
        ACPI_FIXED_LENGTH | ACPI_VARIABLE_LENGTH | ACPI_SMALL_VARIABLE_LENGTH => {}
        _ => return invalid_resource(walk_state, resource_type),
    }
    if resource_type == ACPI_RESOURCE_NAME_SERIAL_BUS {
        let bus_type = (*(aml as *mut aml_resource)).common_serial_bus.type_;
        if bus_type == 0 || bus_type > AML_RESOURCE_MAX_SERIALBUSTYPE { return AE_AML_INVALID_RESOURCE_TYPE; }
    }
    if !return_index.is_null() { *return_index = resource_index; }
    AE_OK
}

unsafe fn invalid_resource(_walk_state: *mut acpi_walk_state, _resource_type: u8) -> acpi_status { AE_AML_INVALID_RESOURCE_TYPE }
unsafe fn bad_resource_length(_walk_state: *mut acpi_walk_state, _resource_type: u8, _length: u16, _minimum: u16) -> acpi_status { AE_AML_BAD_RESOURCE_LENGTH }

pub unsafe fn acpi_ut_get_resource_type(aml: *mut core::ffi::c_void) -> u8 {
    let value = ACPI_GET8!(aml);
    if value & ACPI_RESOURCE_NAME_LARGE != 0 { value } else { value & ACPI_RESOURCE_NAME_SMALL_MASK }
}

pub unsafe fn acpi_ut_get_resource_length(aml: *mut core::ffi::c_void) -> u16 {
    if ACPI_GET8!(aml) & ACPI_RESOURCE_NAME_LARGE != 0 {
        u16::from_le_bytes([ACPI_GET8!(ACPI_ADD_PTR!(u8, aml, 1)), ACPI_GET8!(ACPI_ADD_PTR!(u8, aml, 2))])
    } else { (ACPI_GET8!(aml) & ACPI_RESOURCE_NAME_SMALL_LENGTH_MASK) as u16 }
}

pub unsafe fn acpi_ut_get_resource_header_length(aml: *mut core::ffi::c_void) -> u8 {
    if ACPI_GET8!(aml) & ACPI_RESOURCE_NAME_LARGE != 0 { core::mem::size_of::<aml_resource_large_header>() as u8 } else { core::mem::size_of::<aml_resource_small_header>() as u8 }
}

pub unsafe fn acpi_ut_get_descriptor_length(aml: *mut core::ffi::c_void) -> u32 {
    acpi_ut_get_resource_length(aml) as u32 + acpi_ut_get_resource_header_length(aml) as u32
}

pub unsafe fn acpi_ut_get_resource_end_tag(obj_desc: *mut acpi_operand_object, end_tag: *mut *mut u8) -> acpi_status {
    if (*obj_desc).buffer.length == 0 {
        *end_tag = (*obj_desc).buffer.pointer;
        return AE_OK;
    }
    acpi_ut_walk_aml_resources(core::ptr::null_mut(), (*obj_desc).buffer.pointer, (*obj_desc).buffer.length, None, end_tag as *mut _)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
