// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: rsdump - AML debugger support for resource structures.
//
// External ACPI declarations, constants, macros, and types are supplied by
// the surrounding translation unit.

// All functions in this module are used by the AML Debugger only.

unsafe fn acpi_rs_out_string(title: *const i8, value: *const i8) {
    acpi_os_printf(b"%27s : %s\0".as_ptr() as *const i8, title, value);
    if *value == 0 {
        acpi_os_printf(b"[NULL NAMESTRING]\0".as_ptr() as *const i8);
    }
    acpi_os_printf(b"\n\0".as_ptr() as *const i8);
}

unsafe fn acpi_rs_out_integer8(title: *const i8, value: u8) {
    acpi_os_printf(b"%27s : %2.2X\n\0".as_ptr() as *const i8, title, value);
}

unsafe fn acpi_rs_out_integer16(title: *const i8, value: u16) {
    acpi_os_printf(b"%27s : %4.4X\n\0".as_ptr() as *const i8, title, value);
}

unsafe fn acpi_rs_out_integer32(title: *const i8, value: u32) {
    acpi_os_printf(b"%27s : %8.8X\n\0".as_ptr() as *const i8, title, value);
}

unsafe fn acpi_rs_out_integer64(title: *const i8, value: u64) {
    acpi_os_printf(
        b"%27s : %8.8X%8.8X\n\0".as_ptr() as *const i8,
        title,
        (value >> 32) as u32,
        value as u32,
    );
}

unsafe fn acpi_rs_out_title(title: *const i8) {
    acpi_os_printf(b"%27s : \0".as_ptr() as *const i8, title);
}

unsafe fn acpi_rs_dump_byte_list(length: u16, data: *mut u8) {
    let mut i: u16 = 0;
    while i < length {
        acpi_os_printf(b"%25s%2.2X : %2.2X\n\0".as_ptr() as *const i8, b"Byte\0".as_ptr() as *const i8, i, *data.add(i as usize));
        i = i.wrapping_add(1);
    }
}

unsafe fn acpi_rs_dump_word_list(length: u16, data: *mut u16) {
    let mut i: u16 = 0;
    while i < length {
        acpi_os_printf(b"%25s%2.2X : %4.4X\n\0".as_ptr() as *const i8, b"Word\0".as_ptr() as *const i8, i, *data.add(i as usize));
        i = i.wrapping_add(1);
    }
}

unsafe fn acpi_rs_dump_dword_list(length: u8, data: *mut u32) {
    let mut i: u8 = 0;
    while i < length {
        acpi_os_printf(b"%25s%2.2X : %8.8X\n\0".as_ptr() as *const i8, b"Dword\0".as_ptr() as *const i8, i, *data.add(i as usize));
        i = i.wrapping_add(1);
    }
}

unsafe fn acpi_rs_dump_short_byte_list(length: u8, data: *mut u8) {
    let mut i: u8 = 0;
    while i < length {
        acpi_os_printf(b"%X \0".as_ptr() as *const i8, *data.add(i as usize));
        i = i.wrapping_add(1);
    }
    acpi_os_printf(b"\n\0".as_ptr() as *const i8);
}

unsafe fn acpi_rs_dump_resource_source(resource_source: *mut acpi_resource_source) {
    ACPI_FUNCTION_ENTRY!();
    if (*resource_source).index == 0xFF {
        return;
    }
    acpi_rs_out_integer8(b"Resource Source Index\0".as_ptr() as *const i8, (*resource_source).index);
    acpi_rs_out_string(b"Resource Source\0".as_ptr() as *const i8, if !(*resource_source).string_ptr.is_null() { (*resource_source).string_ptr } else { b"[Not Specified]\0".as_ptr() as *const i8 });
}

unsafe fn acpi_rs_dump_resource_label(title: *mut i8, resource_label: *mut acpi_resource_label) {
    ACPI_FUNCTION_ENTRY!();
    acpi_rs_out_string(title, if !(*resource_label).string_ptr.is_null() { (*resource_label).string_ptr } else { b"[Not Specified]\0".as_ptr() as *const i8 });
}

unsafe fn acpi_rs_dump_address_common(resource: *mut acpi_resource_data) {
    ACPI_FUNCTION_ENTRY!();
    match (*resource).address.resource_type {
        ACPI_MEMORY_RANGE => acpi_rs_dump_descriptor(resource as *mut _, acpi_rs_dump_memory_flags),
        ACPI_IO_RANGE => acpi_rs_dump_descriptor(resource as *mut _, acpi_rs_dump_io_flags),
        ACPI_BUS_NUMBER_RANGE => acpi_rs_out_string(b"Resource Type\0".as_ptr() as *const i8, b"Bus Number Range\0".as_ptr() as *const i8),
        value => acpi_rs_out_integer8(b"Resource Type\0".as_ptr() as *const i8, value as u8),
    }
    acpi_rs_dump_descriptor(resource as *mut _, acpi_rs_dump_general_flags);
}

unsafe fn acpi_rs_dump_descriptor(resource: *mut core::ffi::c_void, table: *mut acpi_rsdump_info) {
    let mut target: *mut u8 = core::ptr::null_mut();
    let mut count = (*table).offset;
    while count != 0 {
        let previous_target = target;
        target = (resource as *mut u8).add((*table).offset as usize);
        let name = (*table).name;
        match (*table).opcode {
            ACPI_RSD_TITLE => if !name.is_null() { acpi_os_printf(b"%s Resource\n\0".as_ptr() as *const i8, name); },
            ACPI_RSD_LITERAL => acpi_rs_out_string(name, (*table).pointer as *const i8),
            ACPI_RSD_STRING => acpi_rs_out_string(name, target as *const i8),
            ACPI_RSD_UINT8 => if !(*table).pointer.is_null() { acpi_rs_out_string(name, *((*table).pointer as *const *const i8).add(*target as usize)); } else { acpi_rs_out_integer8(name, *target); },
            ACPI_RSD_UINT16 => acpi_rs_out_integer16(name, *(target as *const u16)),
            ACPI_RSD_UINT32 => acpi_rs_out_integer32(name, *(target as *const u32)),
            ACPI_RSD_UINT64 => acpi_rs_out_integer64(name, *(target as *const u64)),
            ACPI_RSD_1BITFLAG => acpi_rs_out_string(name, *((*table).pointer as *const *const i8).add((*target & 1) as usize)),
            ACPI_RSD_2BITFLAG => acpi_rs_out_string(name, *((*table).pointer as *const *const i8).add((*target & 3) as usize)),
            ACPI_RSD_3BITFLAG => acpi_rs_out_string(name, *((*table).pointer as *const *const i8).add((*target & 7) as usize)),
            ACPI_RSD_6BITFLAG => acpi_rs_out_integer8(name, *target & 0x3F),
            ACPI_RSD_SHORTLIST => if !previous_target.is_null() { acpi_rs_out_title(name); acpi_rs_dump_short_byte_list(*previous_target, target); },
            ACPI_RSD_SHORTLISTX => if !previous_target.is_null() { acpi_rs_out_title(name); acpi_rs_dump_short_byte_list(*previous_target, *(target as *mut *mut u8)); },
            ACPI_RSD_LONGLIST => if !previous_target.is_null() { acpi_rs_dump_byte_list(*(previous_target as *const u16), target); },
            ACPI_RSD_DWORDLIST => if !previous_target.is_null() { acpi_rs_dump_dword_list(*previous_target, target as *mut u32); },
            ACPI_RSD_WORDLIST => if !previous_target.is_null() { acpi_rs_dump_word_list(*previous_target as u16, *(target as *mut *mut u16)); },
            ACPI_RSD_ADDRESS => acpi_rs_dump_address_common(target as *mut acpi_resource_data),
            ACPI_RSD_SOURCE => acpi_rs_dump_resource_source(target as *mut acpi_resource_source),
            ACPI_RSD_LABEL => acpi_rs_dump_resource_label(b"Resource Label\0".as_ptr() as *mut i8, target as *mut acpi_resource_label),
            ACPI_RSD_SOURCE_LABEL => acpi_rs_dump_resource_label(b"Resource Source Label\0".as_ptr() as *mut i8, target as *mut acpi_resource_label),
            _ => { acpi_os_printf(b"**** Invalid table opcode [%X] ****\n\0".as_ptr() as *const i8, (*table).opcode); return; }
        }
        table = table.add(1);
        count -= 1;
    }
}

#[cfg(ACPI_DEBUGGER)]
pub unsafe fn acpi_rs_dump_resource_list(mut resource_list: *mut acpi_resource) {
    let mut count: u32 = 0;
    ACPI_FUNCTION_ENTRY!();
    if !ACPI_IS_DEBUG_ENABLED!(ACPI_LV_RESOURCES, ACPI_RESOURCES) { return; }
    loop {
        acpi_os_printf(b"\n[%02X] \0".as_ptr() as *const i8, count); count += 1;
        let typ = (*resource_list).type_;
        if typ > ACPI_RESOURCE_TYPE_MAX { acpi_os_printf(b"Invalid descriptor type (%X) in resource list\n\0".as_ptr() as *const i8, typ); return; }
        if typ == 0 { ACPI_ERROR!(AE_INFO, b"Invalid Zero Resource Type\0".as_ptr()); return; }
        if (*resource_list).length == 0 { acpi_os_printf(b"Invalid zero length descriptor in resource list\n\0".as_ptr() as *const i8); return; }
        if typ == ACPI_RESOURCE_TYPE_SERIAL_BUS { acpi_rs_dump_descriptor(&mut (*resource_list).data as *mut _ as *mut _, acpi_gbl_dump_serial_bus_dispatch[(*resource_list).data.common_serial_bus.type_ as usize]); } else { acpi_rs_dump_descriptor(&mut (*resource_list).data as *mut _ as *mut _, acpi_gbl_dump_resource_dispatch[typ as usize]); }
        resource_list = ACPI_NEXT_RESOURCE!(resource_list);
        if typ == ACPI_RESOURCE_TYPE_END_TAG { break; }
    }
}

#[cfg(ACPI_DEBUGGER)]
pub unsafe fn acpi_rs_dump_irq_list(route_table: *mut u8) {
    let mut prt_element = route_table as *mut acpi_pci_routing_table;
    let mut count: u8 = 0;
    ACPI_FUNCTION_ENTRY!();
    if !ACPI_IS_DEBUG_ENABLED!(ACPI_LV_RESOURCES, ACPI_RESOURCES) { return; }
    while (*prt_element).length != 0 {
        acpi_os_printf(b"\n[%02X] PCI IRQ Routing Table Package\n\0".as_ptr() as *const i8, count);
        acpi_rs_dump_descriptor(prt_element as *mut _, acpi_rs_dump_prt);
        prt_element = (prt_element as *mut u8).add((*prt_element).length as usize) as *mut acpi_pci_routing_table;
        count = count.wrapping_add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
