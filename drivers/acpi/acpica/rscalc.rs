// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Translation of acpi/acpica/rscalc.c. External ACPI types, constants, macros,
// globals, and functions are supplied by the surrounding ACPICA translation.

unsafe fn acpi_rs_count_set_bits(mut bit_field: u16) -> u8 {
    let mut bits_set: u8 = 0;
    while bit_field != 0 {
        bit_field &= bit_field.wrapping_sub(1);
        bits_set = bits_set.wrapping_add(1);
    }
    bits_set
}

unsafe fn acpi_rs_struct_option_length(
    resource_source: *const acpi_resource_source,
) -> acpi_rs_length {
    if !(*resource_source).string_ptr.is_null() {
        ((*resource_source).string_length as acpi_rs_length).wrapping_add(1)
    } else {
        0
    }
}

unsafe fn acpi_rs_stream_option_length(
    resource_length: u32,
    minimum_aml_resource_length: u32,
) -> u32 {
    let mut string_length = 0;
    if resource_length > minimum_aml_resource_length {
        string_length = resource_length - minimum_aml_resource_length - 1;
    }
    ACPI_ROUND_UP_TO_NATIVE_WORD(string_length) as u32
}

pub unsafe fn acpi_rs_get_aml_length(
    mut resource: *mut acpi_resource,
    resource_list_size: acpi_size,
    size_needed: *mut acpi_size,
) -> acpi_status {
    let mut aml_size_needed: acpi_size = 0;
    let resource_end = (resource as *mut u8).add(resource_list_size) as *mut acpi_resource;
    while resource < resource_end {
        if (*resource).type_ > ACPI_RESOURCE_TYPE_MAX {
            return AE_AML_INVALID_RESOURCE_TYPE;
        }
        if (*resource).length == 0 {
            return AE_AML_BAD_RESOURCE_LENGTH;
        }
        let mut total_size = acpi_gbl_aml_resource_sizes[(*resource).type_ as usize];
        match (*resource).type_ {
            ACPI_RESOURCE_TYPE_IRQ => {
                if (*resource).data.irq.descriptor_length == 2 { total_size -= 1; }
            }
            ACPI_RESOURCE_TYPE_START_DEPENDENT => {
                if (*resource).data.irq.descriptor_length == 0 { total_size -= 1; }
            }
            ACPI_RESOURCE_TYPE_VENDOR => {
                if (*resource).data.vendor.byte_length > 7 {
                    total_size = core::mem::size_of::<aml_resource_large_header>() as _;
                }
                total_size += (*resource).data.vendor.byte_length as _;
            }
            ACPI_RESOURCE_TYPE_END_TAG => {
                *size_needed = aml_size_needed + total_size as acpi_size;
                return AE_OK;
            }
            ACPI_RESOURCE_TYPE_ADDRESS16 => total_size += acpi_rs_struct_option_length(&(*resource).data.address16.resource_source),
            ACPI_RESOURCE_TYPE_ADDRESS32 => total_size += acpi_rs_struct_option_length(&(*resource).data.address32.resource_source),
            ACPI_RESOURCE_TYPE_ADDRESS64 => total_size += acpi_rs_struct_option_length(&(*resource).data.address64.resource_source),
            ACPI_RESOURCE_TYPE_EXTENDED_IRQ => {
                total_size += (((*resource).data.extended_irq.interrupt_count - 1) * 4) as _;
                total_size += acpi_rs_struct_option_length(&(*resource).data.extended_irq.resource_source);
            }
            ACPI_RESOURCE_TYPE_GPIO => {
                total_size += ((*resource).data.gpio.pin_table_length * 2) as _;
                total_size += (*resource).data.gpio.resource_source.string_length as _ + (*resource).data.gpio.vendor_length as _;
            }
            ACPI_RESOURCE_TYPE_PIN_FUNCTION => {
                total_size += ((*resource).data.pin_function.pin_table_length * 2) as _;
                total_size += (*resource).data.pin_function.resource_source.string_length as _ + (*resource).data.pin_function.vendor_length as _;
            }
            ACPI_RESOURCE_TYPE_CLOCK_INPUT => total_size += (*resource).data.clock_input.resource_source.string_length as _,
            ACPI_RESOURCE_TYPE_SERIAL_BUS => {
                total_size = acpi_gbl_aml_resource_serial_bus_sizes[(*resource).data.common_serial_bus.type_ as usize];
                total_size += (*resource).data.i2c_serial_bus.resource_source.string_length as _ + (*resource).data.i2c_serial_bus.vendor_length as _;
            }
            ACPI_RESOURCE_TYPE_PIN_CONFIG => {
                total_size += ((*resource).data.pin_config.pin_table_length * 2) as _;
                total_size += (*resource).data.pin_config.resource_source.string_length as _ + (*resource).data.pin_config.vendor_length as _;
            }
            ACPI_RESOURCE_TYPE_PIN_GROUP => {
                total_size += ((*resource).data.pin_group.pin_table_length * 2) as _;
                total_size += (*resource).data.pin_group.resource_label.string_length as _ + (*resource).data.pin_group.vendor_length as _;
            }
            ACPI_RESOURCE_TYPE_PIN_GROUP_FUNCTION => {
                total_size += (*resource).data.pin_group_function.resource_source.string_length as _;
                total_size += (*resource).data.pin_group_function.resource_source_label.string_length as _ + (*resource).data.pin_group_function.vendor_length as _;
            }
            ACPI_RESOURCE_TYPE_PIN_GROUP_CONFIG => {
                total_size += (*resource).data.pin_group_config.resource_source.string_length as _;
                total_size += (*resource).data.pin_group_config.resource_source_label.string_length as _ + (*resource).data.pin_group_config.vendor_length as _;
            }
            _ => {}
        }
        aml_size_needed += total_size as acpi_size;
        resource = (resource as *mut u8).add((*resource).length as usize) as *mut acpi_resource;
    }
    AE_AML_NO_RESOURCE_END_TAG
}

pub unsafe fn acpi_rs_get_list_length(mut aml_buffer: *mut u8, aml_buffer_length: u32, size_needed: *mut acpi_size) -> acpi_status {
    let end_aml = aml_buffer.add(aml_buffer_length as usize);
    *size_needed = ACPI_RS_SIZE_MIN;
    while aml_buffer < end_aml {
        let mut resource_index = 0u8;
        let status = acpi_ut_validate_resource(core::ptr::null_mut(), aml_buffer, &mut resource_index);
        if ACPI_FAILURE(status) { return status; }
        let aml_resource = aml_buffer as *mut aml_resource;
        let resource_length = acpi_ut_get_resource_length(aml_buffer);
        let mut minimum = acpi_gbl_resource_aml_sizes[resource_index as usize];
        let mut buffer = aml_buffer.add(acpi_ut_get_resource_header_length(aml_buffer) as usize);
        let mut extra: u32 = 0;
        match acpi_ut_get_resource_type(aml_buffer) {
            ACPI_RESOURCE_NAME_IRQ => { let mut t = 0u16; ACPI_MOVE_16_TO_16(&mut t, buffer); extra = acpi_rs_count_set_bits(t) as _; }
            ACPI_RESOURCE_NAME_DMA => extra = acpi_rs_count_set_bits(*buffer as u16) as _,
            ACPI_RESOURCE_NAME_VENDOR_SMALL | ACPI_RESOURCE_NAME_VENDOR_LARGE => { extra = resource_length; if extra != 0 { extra -= 1; } }
            ACPI_RESOURCE_NAME_END_TAG => return AE_OK,
            ACPI_RESOURCE_NAME_ADDRESS32 | ACPI_RESOURCE_NAME_ADDRESS16 | ACPI_RESOURCE_NAME_ADDRESS64 | ACPI_RESOURCE_NAME_CLOCK_INPUT => extra = acpi_rs_stream_option_length(resource_length, minimum),
            ACPI_RESOURCE_NAME_EXTENDED_IRQ => { extra = ((*buffer.add(1) as u32) - 1) * core::mem::size_of::<u32>() as u32; extra += acpi_rs_stream_option_length(resource_length - extra, minimum); }
            ACPI_RESOURCE_NAME_SERIAL_BUS => { minimum = acpi_gbl_resource_aml_serial_bus_sizes[(*aml_resource).common_serial_bus.type_ as usize]; extra += (*aml_resource).common_serial_bus.resource_length - minimum; }
            ACPI_RESOURCE_NAME_GPIO => { extra += if (*aml_resource).gpio.vendor_length != 0 { (*aml_resource).gpio.vendor_offset - (*aml_resource).gpio.pin_table_offset + (*aml_resource).gpio.vendor_length } else { (*aml_resource).large_header.resource_length + core::mem::size_of::<aml_resource_large_header>() as u32 - (*aml_resource).gpio.pin_table_offset }; }
            ACPI_RESOURCE_NAME_PIN_FUNCTION => { extra += if (*aml_resource).pin_function.vendor_length != 0 { (*aml_resource).pin_function.vendor_offset - (*aml_resource).pin_function.pin_table_offset + (*aml_resource).pin_function.vendor_length } else { (*aml_resource).large_header.resource_length + core::mem::size_of::<aml_resource_large_header>() as u32 - (*aml_resource).pin_function.pin_table_offset }; }
            ACPI_RESOURCE_NAME_PIN_CONFIG => { extra += if (*aml_resource).pin_config.vendor_length != 0 { (*aml_resource).pin_config.vendor_offset - (*aml_resource).pin_config.pin_table_offset + (*aml_resource).pin_config.vendor_length } else { (*aml_resource).large_header.resource_length + core::mem::size_of::<aml_resource_large_header>() as u32 - (*aml_resource).pin_config.pin_table_offset }; }
            ACPI_RESOURCE_NAME_PIN_GROUP => extra += (*aml_resource).pin_group.vendor_offset - (*aml_resource).pin_group.pin_table_offset + (*aml_resource).pin_group.vendor_length,
            ACPI_RESOURCE_NAME_PIN_GROUP_FUNCTION => extra += (*aml_resource).pin_group_function.vendor_offset - (*aml_resource).pin_group_function.res_source_offset + (*aml_resource).pin_group_function.vendor_length,
            ACPI_RESOURCE_NAME_PIN_GROUP_CONFIG => extra += (*aml_resource).pin_group_config.vendor_offset - (*aml_resource).pin_group_config.res_source_offset + (*aml_resource).pin_group_config.vendor_length,
            _ => {}
        }
        let mut buffer_size = if acpi_ut_get_resource_type(aml_buffer) == ACPI_RESOURCE_NAME_SERIAL_BUS { acpi_gbl_resource_struct_serial_bus_sizes[(*aml_resource).common_serial_bus.type_ as usize] } else { acpi_gbl_resource_struct_sizes[resource_index as usize] };
        buffer_size += extra;
        *size_needed += ACPI_ROUND_UP_TO_NATIVE_WORD(buffer_size) as acpi_size;
        aml_buffer = aml_buffer.add(acpi_ut_get_descriptor_length(aml_buffer) as usize);
    }
    AE_AML_NO_RESOURCE_END_TAG
}

pub unsafe fn acpi_rs_get_pci_routing_table_length(package_object: *mut acpi_operand_object, buffer_size_needed: *mut acpi_size) -> acpi_status {
    let count = (*package_object).package.count;
    let mut total: acpi_size = 0;
    let mut top = (*package_object).package.elements;
    for _ in 0..count {
        let package_element = *top;
        if package_element.is_null() || (*package_element).common.type_ != ACPI_TYPE_PACKAGE { return AE_AML_OPERAND_TYPE; }
        let mut sub = (*package_element).package.elements;
        let mut found = false;
        for _ in 0..(*package_element).package.count {
            if !sub.is_null() && ((*sub).common.type_ == ACPI_TYPE_STRING || ((*sub).common.type_ == ACPI_TYPE_LOCAL_REFERENCE && (*sub).reference.class == ACPI_REFCLASS_NAME)) { found = true; break; }
            sub = sub.add(1);
        }
        total += (core::mem::size_of::<acpi_pci_routing_table>() - 4) as acpi_size;
        if found { total += if (*sub).common.type_ == ACPI_TYPE_STRING { ((*sub).string.length + 1) as acpi_size } else { acpi_ns_get_pathname_length((*sub).reference.node) as acpi_size }; } else { total += 4; }
        total = ACPI_ROUND_UP_TO_64BIT(total) as acpi_size;
        top = top.add(1);
    }
    *buffer_size_needed = total + core::mem::size_of::<acpi_pci_routing_table>() as acpi_size;
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
