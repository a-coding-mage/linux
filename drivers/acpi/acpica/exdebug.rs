// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: exdebug - Support for stores to the AML Debug Object
//
// This is the Rust translation of exdebug.c. The ACPI types and helper
// functions referenced below are supplied by the surrounding ACPI bindings.

#[cfg(not(feature = "acpi_no_error_messages"))]
extern "C" {
    static mut acpi_gbl_enable_aml_debug_object: u8;
    static mut acpi_dbg_level: u32;
    static mut acpi_gbl_display_debug_timer: u8;
    static mut acpi_gbl_integer_byte_width: u32;

    fn acpi_os_printf(format: *const i8, ...);
    fn acpi_os_get_timer() -> u64;
    fn acpi_ut_get_object_type_name(object: *mut acpi_operand_object) -> *const i8;
    fn acpi_ut_valid_internal_object(object: *mut acpi_operand_object) -> u8;
    fn acpi_ut_get_type_name(object_type: u32) -> *const i8;
    fn acpi_ut_dump_buffer(buffer: *const u8, length: u32, display: u32, component: u32);
    fn acpi_ut_get_reference_name(object: *mut acpi_operand_object) -> *const i8;
}

#[cfg(not(feature = "acpi_no_error_messages"))]
#[repr(C)]
pub struct acpi_operand_object {
    pub common: acpi_object_common,
    pub integer: acpi_object_integer,
    pub buffer: acpi_object_buffer,
    pub string: acpi_object_string,
    pub package: acpi_object_package,
    pub reference: acpi_object_reference,
}

#[cfg(not(feature = "acpi_no_error_messages"))]
#[repr(C)] pub struct acpi_object_common { pub descriptor_type: u8, pub type_: u32 }
#[cfg(not(feature = "acpi_no_error_messages"))]
#[repr(C)] pub struct acpi_object_integer { pub value: u64 }
#[cfg(not(feature = "acpi_no_error_messages"))]
#[repr(C)] pub struct acpi_object_buffer { pub length: u32, pub pointer: *mut u8 }
#[cfg(not(feature = "acpi_no_error_messages"))]
#[repr(C)] pub struct acpi_object_string { pub length: u32, pub pointer: *mut i8 }
#[cfg(not(feature = "acpi_no_error_messages"))]
#[repr(C)] pub struct acpi_object_package { pub count: u32, pub elements: *mut *mut acpi_operand_object }
#[cfg(not(feature = "acpi_no_error_messages"))]
#[repr(C)] pub struct acpi_object_reference {
    pub class: u32, pub value: u32, pub node: *mut acpi_namespace_node,
    pub object: *mut acpi_operand_object, pub index_pointer: *mut u8,
    pub where_: *mut *mut acpi_operand_object,
}
#[cfg(not(feature = "acpi_no_error_messages"))]
#[repr(C)] pub struct acpi_namespace_node { pub descriptor_type: u8, pub type_: u32, pub name: [i8; 4], pub object: *mut acpi_operand_object }

#[cfg(not(feature = "acpi_no_error_messages"))]
pub const ACPI_DESC_TYPE_OPERAND: u8 = 1;
#[cfg(not(feature = "acpi_no_error_messages"))]
pub const ACPI_DESC_TYPE_NAMED: u8 = 2;
#[cfg(not(feature = "acpi_no_error_messages"))]
pub const ACPI_TYPE_INTEGER: u32 = 1;
#[cfg(not(feature = "acpi_no_error_messages"))]
pub const ACPI_TYPE_BUFFER: u32 = 3;
#[cfg(not(feature = "acpi_no_error_messages"))]
pub const ACPI_TYPE_STRING: u32 = 4;
#[cfg(not(feature = "acpi_no_error_messages"))]
pub const ACPI_TYPE_PACKAGE: u32 = 5;
#[cfg(not(feature = "acpi_no_error_messages"))]
pub const ACPI_TYPE_DEVICE: u32 = 6;
#[cfg(not(feature = "acpi_no_error_messages"))]
pub const ACPI_TYPE_THERMAL: u32 = 0x0c;
#[cfg(not(feature = "acpi_no_error_messages"))]
pub const ACPI_TYPE_LOCAL_REFERENCE: u32 = 0x14;
#[cfg(not(feature = "acpi_no_error_messages"))]
pub const ACPI_REFCLASS_INDEX: u32 = 0;
#[cfg(not(feature = "acpi_no_error_messages"))]
pub const ACPI_REFCLASS_TABLE: u32 = 1;
#[cfg(not(feature = "acpi_no_error_messages"))]
pub const ACPI_LV_DEBUG_OBJECT: u32 = 0x0000_0800;
#[cfg(not(feature = "acpi_no_error_messages"))]
pub const DB_BYTE_DISPLAY: u32 = 0;

#[cfg(not(feature = "acpi_no_error_messages"))]
#[inline]
pub unsafe fn acpi_ex_do_debug_object(source_desc: *mut acpi_operand_object, level: u32, index: u32) {
    if acpi_gbl_enable_aml_debug_object == 0 && (acpi_dbg_level & ACPI_LV_DEBUG_OBJECT) == 0 { return; }
    if !source_desc.is_null() && (*source_desc).common.descriptor_type == ACPI_DESC_TYPE_OPERAND
        && (*source_desc).common.type_ == ACPI_TYPE_STRING && (*source_desc).string.length == 1
        && *(*source_desc).string.pointer == b'\n' as i8 {
        acpi_os_printf(b"\n\0".as_ptr() as *const i8); return;
    }
    if !(level > 0 && index == 0) {
        if acpi_gbl_display_debug_timer != 0 {
            let timer = ((acpi_os_get_timer() / 10) as u32) & 0x03ff_ffff;
            acpi_os_printf(b"ACPI Debug: T=0x%8.8X %*s\0".as_ptr() as *const i8, timer, level, b" \0".as_ptr());
        } else { acpi_os_printf(b"ACPI Debug: %*s\0".as_ptr() as *const i8, level, b" \0".as_ptr()); }
    }
    if index > 0 { acpi_os_printf(b"(%.2u) \0".as_ptr() as *const i8, index - 1); }
    if source_desc.is_null() { acpi_os_printf(b"[Null Object]\n\0".as_ptr() as *const i8); return; }
    if (*source_desc).common.descriptor_type == ACPI_DESC_TYPE_NAMED {
        acpi_os_printf(b"%s (Node %p)\n\0".as_ptr() as *const i8, acpi_ut_get_type_name((*source_desc).common.type_), source_desc); return;
    }
    if (*source_desc).common.descriptor_type != ACPI_DESC_TYPE_OPERAND { return; }
    let ty = (*source_desc).common.type_;
    if ty != ACPI_TYPE_INTEGER && ty != ACPI_TYPE_STRING { acpi_os_printf(b"%s \0".as_ptr() as *const i8, acpi_ut_get_object_type_name(source_desc)); }
    if acpi_ut_valid_internal_object(source_desc) == 0 { acpi_os_printf(b"%p, Invalid Internal Object!\n\0".as_ptr() as *const i8, source_desc); return; }
    match ty {
        ACPI_TYPE_INTEGER => { if acpi_gbl_integer_byte_width == 4 { acpi_os_printf(b"0x%8.8X\n\0".as_ptr() as *const i8, (*source_desc).integer.value as u32); } else { acpi_os_printf(b"0x%8.8X%8.8X\n\0".as_ptr() as *const i8, ((*source_desc).integer.value >> 32) as u32, (*source_desc).integer.value as u32); } }
        ACPI_TYPE_BUFFER => { let n = (*source_desc).buffer.length.min(256); acpi_os_printf(b"[0x%.2X]\n\0".as_ptr() as *const i8, (*source_desc).buffer.length); acpi_ut_dump_buffer((*source_desc).buffer.pointer, n, DB_BYTE_DISPLAY, 0); }
        ACPI_TYPE_STRING => acpi_os_printf(b"\"%s\"\n\0".as_ptr() as *const i8, (*source_desc).string.pointer),
        ACPI_TYPE_PACKAGE => { acpi_os_printf(b"(Contains 0x%.2X Elements):\n\0".as_ptr() as *const i8, (*source_desc).package.count); for i in 0..(*source_desc).package.count { acpi_ex_do_debug_object(*(*source_desc).package.elements.add(i as usize), level + 4, i + 1); } }
        ACPI_TYPE_LOCAL_REFERENCE => {
            acpi_os_printf(b"[%s] \0".as_ptr() as *const i8, acpi_ut_get_reference_name(source_desc));
            match (*source_desc).reference.class {
                ACPI_REFCLASS_INDEX => acpi_os_printf(b"0x%X\n\0".as_ptr() as *const i8, (*source_desc).reference.value),
                ACPI_REFCLASS_TABLE => { acpi_os_printf(b"Table Index 0x%X\n\0".as_ptr() as *const i8, (*source_desc).reference.value); return; }, _ => {}
            }
            acpi_os_printf(b" \0".as_ptr() as *const i8);
            let r = &(*source_desc).reference;
            if !r.node.is_null() {
                if (*r.node).descriptor_type != ACPI_DESC_TYPE_NAMED { acpi_os_printf(b" %p - Not a valid namespace node\n\0".as_ptr() as *const i8, r.node); }
                else { acpi_os_printf(b"Node %p [%4.4s] \0".as_ptr() as *const i8, r.node, (*r.node).name.as_ptr()); match (*r.node).type_ { ACPI_TYPE_DEVICE => acpi_os_printf(b"Device\n\0".as_ptr() as *const i8), ACPI_TYPE_THERMAL => acpi_os_printf(b"Thermal Zone\n\0".as_ptr() as *const i8), _ => acpi_ex_do_debug_object((*r.node).object, level + 4, 0) } }
            } else if !r.object.is_null() {
                if (*r.object).common.descriptor_type == ACPI_DESC_TYPE_NAMED { acpi_ex_do_debug_object(r.object, level + 4, 0); }
                else { match (*r.object).common.type_ { ACPI_TYPE_BUFFER => acpi_os_printf(b"Buffer[%u] = 0x%2.2X\n\0".as_ptr() as *const i8, r.value, *r.index_pointer), ACPI_TYPE_STRING => acpi_os_printf(b"String[%u] = \"%c\" (0x%2.2X)\n\0".as_ptr() as *const i8, r.value, *r.index_pointer, *r.index_pointer), ACPI_TYPE_PACKAGE => { acpi_os_printf(b"Package[%u] = \0".as_ptr() as *const i8, r.value); if (*r.where_).is_null() { acpi_os_printf(b"[Uninitialized Package Element]\n\0".as_ptr() as *const i8); } else { acpi_ex_do_debug_object(*r.where_, level + 4, 0); } }, _ => acpi_os_printf(b"Unknown Reference object type %X\n\0".as_ptr() as *const i8, (*r.object).common.type_) } }
            }
        }
        _ => acpi_os_printf(b"(Descriptor %p)\n\0".as_ptr() as *const i8, source_desc),
    }
    acpi_os_printf(b"\n\0".as_ptr() as *const i8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
