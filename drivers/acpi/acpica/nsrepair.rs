// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: nsrepair - Repair for objects returned by predefined methods
//
// Copyright (C) 2000 - 2026, Intel Corp.

// C dependencies supplied by the ACPICA translation unit are intentionally
// left as external Rust symbols.

/*
 * This module attempts to repair or convert objects returned by the
 * predefined methods to an object type that is expected, as per the ACPI
 * specification. The need for this code is dictated by the many machines that
 * return incorrect types for the standard predefined methods.
 */

extern "C" {
    fn acpi_ns_convert_to_resource(node: *mut acpi_namespace_node, obj: *mut acpi_operand_object, new_obj: *mut *mut acpi_operand_object) -> acpi_status;
    fn acpi_ns_convert_to_reference(node: *mut acpi_namespace_node, obj: *mut acpi_operand_object, new_obj: *mut *mut acpi_operand_object) -> acpi_status;
    fn acpi_ns_convert_to_unicode(node: *mut acpi_namespace_node, obj: *mut acpi_operand_object, new_obj: *mut *mut acpi_operand_object) -> acpi_status;
    fn acpi_ns_convert_to_integer(obj: *mut acpi_operand_object, new_obj: *mut *mut acpi_operand_object) -> acpi_status;
    fn acpi_ns_convert_to_string(obj: *mut acpi_operand_object, new_obj: *mut *mut acpi_operand_object) -> acpi_status;
    fn acpi_ns_convert_to_buffer(obj: *mut acpi_operand_object, new_obj: *mut *mut acpi_operand_object) -> acpi_status;
    fn acpi_ut_create_integer_object(value: u64) -> *mut acpi_operand_object;
    fn acpi_ut_create_string_object(length: u32) -> *mut acpi_operand_object;
    fn acpi_ut_create_buffer_object(length: u32) -> *mut acpi_operand_object;
    fn acpi_ut_create_package_object(count: u32) -> *mut acpi_operand_object;
    fn acpi_ut_remove_reference(obj: *mut acpi_operand_object);
    fn acpi_ut_get_object_type_name(obj: *mut acpi_operand_object) -> *const core::ffi::c_char;
}

// Types, constants, and diagnostic macros are supplied by ACPICA headers.
// The following declarations describe the file-local interfaces used here.
#[repr(C)]
pub struct acpi_namespace_node { pub name: acpi_name_union }
#[repr(C)] pub union acpi_name_union { pub ascii: [u8; 4], pub integer: u32 }
#[repr(C)] pub struct acpi_evaluate_info {
    pub node: *mut acpi_namespace_node, pub return_btype: u32,
    pub full_pathname: *const core::ffi::c_char, pub return_flags: u32,
    pub parent_package: *mut acpi_operand_object,
}
#[repr(C)] pub struct acpi_operand_common { pub reference_count: u32 }
#[repr(C)] pub struct acpi_package { pub count: u32, pub elements: *mut *mut acpi_operand_object }
#[repr(C)] pub union acpi_operand_object { pub common: acpi_operand_common, pub package: acpi_package }
pub type acpi_status = i32;

#[repr(C)] pub struct acpi_simple_repair_info {
    pub name: [u8; 4], pub unexpected_btypes: u32, pub package_index: u32,
    pub object_converter: Option<unsafe extern "C" fn(*mut acpi_namespace_node, *mut acpi_operand_object, *mut *mut acpi_operand_object) -> acpi_status>,
}

static mut ACPI_OBJECT_REPAIR_INFO: [acpi_simple_repair_info; 7] = [
    acpi_simple_repair_info { name: *b"_CRS", unexpected_btypes: ACPI_RTYPE_INTEGER | ACPI_RTYPE_STRING | ACPI_RTYPE_BUFFER | ACPI_RTYPE_NONE, package_index: ACPI_NOT_PACKAGE_ELEMENT, object_converter: Some(acpi_ns_convert_to_resource) },
    acpi_simple_repair_info { name: *b"_DMA", unexpected_btypes: ACPI_RTYPE_INTEGER | ACPI_RTYPE_STRING | ACPI_RTYPE_BUFFER | ACPI_RTYPE_NONE, package_index: ACPI_NOT_PACKAGE_ELEMENT, object_converter: Some(acpi_ns_convert_to_resource) },
    acpi_simple_repair_info { name: *b"_PRS", unexpected_btypes: ACPI_RTYPE_INTEGER | ACPI_RTYPE_STRING | ACPI_RTYPE_BUFFER | ACPI_RTYPE_NONE, package_index: ACPI_NOT_PACKAGE_ELEMENT, object_converter: Some(acpi_ns_convert_to_resource) },
    acpi_simple_repair_info { name: *b"_DEP", unexpected_btypes: ACPI_RTYPE_STRING, package_index: ACPI_ALL_PACKAGE_ELEMENTS, object_converter: Some(acpi_ns_convert_to_reference) },
    acpi_simple_repair_info { name: *b"_MLS", unexpected_btypes: ACPI_RTYPE_STRING, package_index: 1, object_converter: Some(acpi_ns_convert_to_unicode) },
    acpi_simple_repair_info { name: *b"_STR", unexpected_btypes: ACPI_RTYPE_STRING | ACPI_RTYPE_BUFFER, package_index: ACPI_NOT_PACKAGE_ELEMENT, object_converter: Some(acpi_ns_convert_to_unicode) },
    acpi_simple_repair_info { name: [0; 4], unexpected_btypes: 0, package_index: 0, object_converter: None },
];

// Constants and diagnostic helpers below are provided by the ACPICA environment.
pub unsafe fn acpi_ns_simple_repair(info: *mut acpi_evaluate_info, expected_btypes: u32, package_index: u32, return_object_ptr: *mut *mut acpi_operand_object) -> acpi_status {
    let return_object = *return_object_ptr;
    let mut new_object: *mut acpi_operand_object = core::ptr::null_mut();
    let predefined = acpi_ns_match_simple_repair((*info).node, (*info).return_btype, package_index);
    if !predefined.is_null() {
        let status = ((*predefined).object_converter.unwrap())((*info).node, return_object, &mut new_object);
        if status != AE_OK { return status; }
        if !new_object.is_null() { return acpi_ns_install_repaired(info, package_index, return_object, new_object, return_object_ptr); }
    }
    if (*info).return_btype & expected_btypes != 0 { return AE_OK; }
    if return_object.is_null() && expected_btypes != 0 {
        if expected_btypes & ACPI_RTYPE_NONE == 0 && package_index != ACPI_NOT_PACKAGE_ELEMENT {
            let status = acpi_ns_repair_null_element(info, expected_btypes, package_index, return_object_ptr);
            if status == AE_OK { return AE_OK; }
        }
        if expected_btypes != ACPI_RTYPE_NONE { return AE_AML_NO_RETURN_VALUE; }
    }
    let converters: &[(u32, unsafe extern "C" fn(*mut acpi_operand_object, *mut *mut acpi_operand_object) -> acpi_status)] = &[(ACPI_RTYPE_INTEGER, acpi_ns_convert_to_integer), (ACPI_RTYPE_STRING, acpi_ns_convert_to_string), (ACPI_RTYPE_BUFFER, acpi_ns_convert_to_buffer)];
    for (bit, converter) in converters { if expected_btypes & bit != 0 && converter(return_object, &mut new_object) == AE_OK { return acpi_ns_install_repaired(info, package_index, return_object, new_object, return_object_ptr); } }
    if expected_btypes & ACPI_RTYPE_PACKAGE != 0 && acpi_ns_wrap_with_package(info, return_object, &mut new_object) == AE_OK { *return_object_ptr = new_object; (*info).return_flags |= ACPI_OBJECT_REPAIRED; return AE_OK; }
    AE_AML_OPERAND_TYPE
}

unsafe fn acpi_ns_install_repaired(info: *mut acpi_evaluate_info, package_index: u32, old: *mut acpi_operand_object, new: *mut acpi_operand_object, out: *mut *mut acpi_operand_object) -> acpi_status {
    if package_index != ACPI_NOT_PACKAGE_ELEMENT && (*info).return_flags & ACPI_OBJECT_WRAPPED == 0 { (*new).common.reference_count = (*old).common.reference_count; }
    acpi_ut_remove_reference(old); *out = new; (*info).return_flags |= ACPI_OBJECT_REPAIRED; AE_OK
}

unsafe fn acpi_ns_match_simple_repair(node: *mut acpi_namespace_node, return_btype: u32, package_index: u32) -> *const acpi_simple_repair_info {
    let mut i = 0; while i < ACPI_OBJECT_REPAIR_INFO.len() && ACPI_OBJECT_REPAIR_INFO[i].object_converter.is_some() { if (*node).name.ascii == ACPI_OBJECT_REPAIR_INFO[i].name && return_btype & ACPI_OBJECT_REPAIR_INFO[i].unexpected_btypes != 0 && (ACPI_OBJECT_REPAIR_INFO[i].package_index == ACPI_ALL_PACKAGE_ELEMENTS || package_index == ACPI_OBJECT_REPAIR_INFO[i].package_index) { return &ACPI_OBJECT_REPAIR_INFO[i]; } i += 1; } core::ptr::null()
}

pub unsafe fn acpi_ns_remove_null_elements(info: *mut acpi_evaluate_info, package_type: u8, obj_desc: *mut acpi_operand_object) {
    match package_type { ACPI_PTYPE1_VAR | ACPI_PTYPE2 | ACPI_PTYPE2_COUNT | ACPI_PTYPE2_PKG_COUNT | ACPI_PTYPE2_FIXED | ACPI_PTYPE2_MIN | ACPI_PTYPE2_REV_FIXED | ACPI_PTYPE2_FIX_VAR => {}, _ => return }
    let count = (*obj_desc).package.count; let mut dest = (*obj_desc).package.elements; let mut new_count = count;
    for _ in 0..count { let source = dest; if (*source).is_null() { new_count -= 1; } else { *dest = *source; dest = dest.add(1); } }
    if new_count < count { *dest = core::ptr::null_mut(); (*obj_desc).package.count = new_count; }
}

pub unsafe fn acpi_ns_repair_null_element(info: *mut acpi_evaluate_info, expected_btypes: u32, package_index: u32, return_object_ptr: *mut *mut acpi_operand_object) -> acpi_status {
    if !(*return_object_ptr).is_null() { return AE_OK; }
    let new_object = if expected_btypes & ACPI_RTYPE_INTEGER != 0 { acpi_ut_create_integer_object(0) }
        else if expected_btypes & ACPI_RTYPE_STRING != 0 { acpi_ut_create_string_object(0) }
        else if expected_btypes & ACPI_RTYPE_BUFFER != 0 { acpi_ut_create_buffer_object(0) }
        else { return AE_AML_OPERAND_TYPE };
    if new_object.is_null() { return AE_NO_MEMORY; }
    (*new_object).common.reference_count = (*(*info).parent_package).common.reference_count;
    *return_object_ptr = new_object; (*info).return_flags |= ACPI_OBJECT_REPAIRED; AE_OK
}

pub unsafe fn acpi_ns_wrap_with_package(info: *mut acpi_evaluate_info, original_object: *mut acpi_operand_object, obj_desc_ptr: *mut *mut acpi_operand_object) -> acpi_status {
    let pkg_obj_desc = acpi_ut_create_package_object(1);
    if pkg_obj_desc.is_null() { return AE_NO_MEMORY; }
    (*pkg_obj_desc).package.elements.add(0).write(original_object);
    *obj_desc_ptr = pkg_obj_desc;
    (*info).return_flags |= ACPI_OBJECT_REPAIRED | ACPI_OBJECT_WRAPPED;
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
