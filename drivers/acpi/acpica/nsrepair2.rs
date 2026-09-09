// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Module Name: nsrepair2 - Repair for objects returned by specific
 *                          predefined methods
 * Copyright (C) 2000 - 2026, Intel Corp.
 */

// External ACPI declarations and constants are supplied by the surrounding translation.

type AcpiRepairFunction = unsafe fn(
    info: *mut acpi_evaluate_info,
    return_object_ptr: *mut *mut acpi_operand_object,
) -> acpi_status;

#[repr(C)]
struct acpi_repair_info {
    name: [i8; ACPI_NAMESEG_SIZE as usize],
    repair_function: Option<AcpiRepairFunction>,
}

const ACPI_SORT_ASCENDING: u8 = 0;
const ACPI_SORT_DESCENDING: u8 = 1;
const ACPI_FDE_FIELD_COUNT: u32 = 5;
const ACPI_FDE_BYTE_BUFFER_SIZE: u32 = 5;
const ACPI_FDE_DWORD_BUFFER_SIZE: u32 = ACPI_FDE_FIELD_COUNT * core::mem::size_of::<u32>() as u32;

static ACPI_NS_REPAIRABLE_NAMES: [acpi_repair_info; 10] = [
    acpi_repair_info { name: *b"_ALR", repair_function: Some(acpi_ns_repair_ALR) },
    acpi_repair_info { name: *b"_CID", repair_function: Some(acpi_ns_repair_CID) },
    acpi_repair_info { name: *b"_CST", repair_function: Some(acpi_ns_repair_CST) },
    acpi_repair_info { name: *b"_FDE", repair_function: Some(acpi_ns_repair_FDE) },
    acpi_repair_info { name: *b"_GTM", repair_function: Some(acpi_ns_repair_FDE) },
    acpi_repair_info { name: *b"_HID", repair_function: Some(acpi_ns_repair_HID) },
    acpi_repair_info { name: *b"_PRT", repair_function: Some(acpi_ns_repair_PRT) },
    acpi_repair_info { name: *b"_PSS", repair_function: Some(acpi_ns_repair_PSS) },
    acpi_repair_info { name: *b"_TSS", repair_function: Some(acpi_ns_repair_TSS) },
    acpi_repair_info { name: [0; ACPI_NAMESEG_SIZE as usize], repair_function: None },
];

unsafe fn acpi_ns_match_complex_repair(
    node: *mut acpi_namespace_node,
) -> *const acpi_repair_info {
    let mut this_name = ACPI_NS_REPAIRABLE_NAMES.as_ptr();
    while (*this_name).repair_function.is_some() {
        if ACPI_COMPARE_NAMESEG((*node).name.ascii.as_ptr(), (*this_name).name.as_ptr()) {
            return this_name;
        }
        this_name = this_name.add(1);
    }
    core::ptr::null()
}

pub unsafe fn acpi_ns_complex_repairs(
    info: *mut acpi_evaluate_info,
    node: *mut acpi_namespace_node,
    validate_status: acpi_status,
    return_object_ptr: *mut *mut acpi_operand_object,
) -> acpi_status {
    let predefined = acpi_ns_match_complex_repair(node);
    if predefined.is_null() {
        return validate_status;
    }
    ((*predefined).repair_function.unwrap())(info, return_object_ptr)
}

unsafe fn acpi_ns_repair_ALR(info: *mut acpi_evaluate_info, p: *mut *mut acpi_operand_object) -> acpi_status {
    acpi_ns_check_sorted_list(info, *p, 0, 2, 1, ACPI_SORT_ASCENDING, b"AmbientIlluminance\0".as_ptr() as *mut i8)
}

unsafe fn acpi_ns_repair_FDE(info: *mut acpi_evaluate_info, p: *mut *mut acpi_operand_object) -> acpi_status {
    let return_object = *p;
    if (*return_object).common.type_ == ACPI_TYPE_BUFFER {
        if (*return_object).buffer.length >= ACPI_FDE_DWORD_BUFFER_SIZE { return AE_OK; }
        if (*return_object).buffer.length != ACPI_FDE_BYTE_BUFFER_SIZE { return AE_AML_OPERAND_TYPE; }
        let buffer_object = acpi_ut_create_buffer_object(ACPI_FDE_DWORD_BUFFER_SIZE);
        if buffer_object.is_null() { return AE_NO_MEMORY; }
        let mut byte_buffer = (*return_object).buffer.pointer;
        let mut dword_buffer = (*buffer_object).buffer.pointer as *mut u32;
        for _ in 0..ACPI_FDE_FIELD_COUNT {
            *dword_buffer = *byte_buffer as u32;
            dword_buffer = dword_buffer.add(1);
            byte_buffer = byte_buffer.add(1);
        }
        acpi_ut_remove_reference(return_object);
        *p = buffer_object;
        (*info).return_flags |= ACPI_OBJECT_REPAIRED;
        return AE_OK;
    }
    AE_AML_OPERAND_TYPE
}

unsafe fn acpi_ns_repair_CID(info: *mut acpi_evaluate_info, p: *mut *mut acpi_operand_object) -> acpi_status {
    let obj = *p;
    if (*obj).common.type_ == ACPI_TYPE_STRING { return acpi_ns_repair_HID(info, p); }
    if (*obj).common.type_ != ACPI_TYPE_PACKAGE { return AE_OK; }
    let mut elements = (*obj).package.elements;
    for _ in 0..(*obj).package.count {
        let original = *elements;
        let refs = (*original).common.reference_count;
        let status = acpi_ns_repair_HID(info, elements);
        if ACPI_FAILURE(status) { return status; }
        if original != *elements { (**elements).common.reference_count = refs; }
        elements = elements.add(1);
    }
    AE_OK
}

unsafe fn acpi_ns_repair_CST(info: *mut acpi_evaluate_info, p: *mut *mut acpi_operand_object) -> acpi_status {
    let obj = *p;
    let mut count = (*obj).package.count - 1;
    let mut i = 0;
    while i < count {
        let outer = *(*obj).package.elements.add((i + 1) as usize);
        let remove = (*outer).package.count == 0 || (*(*outer).package.elements.add(1)).integer.value == 0;
        if remove { acpi_ns_remove_element(obj, i + 1); count -= 1; } else { i += 1; }
    }
    (*(*obj).package.elements).integer.value = count as u64;
    acpi_ns_check_sorted_list(info, obj, 1, 4, 1, ACPI_SORT_ASCENDING, b"C-State Type\0".as_ptr() as *mut i8)
}

unsafe fn acpi_ns_repair_HID(info: *mut acpi_evaluate_info, p: *mut *mut acpi_operand_object) -> acpi_status {
    let obj = *p;
    if (*obj).common.type_ != ACPI_TYPE_STRING { return AE_OK; }
    if (*obj).string.length == 0 { (*info).return_flags |= ACPI_OBJECT_REPAIRED; return AE_OK; }
    let new_string = acpi_ut_create_string_object((*obj).string.length);
    if new_string.is_null() { return AE_NO_MEMORY; }
    let mut source = (*obj).string.pointer;
    if *source == b'*' as i8 { source = source.add(1); (*new_string).string.length -= 1; }
    let mut dest = (*new_string).string.pointer;
    while *source != 0 { *dest = (*source as u8).to_ascii_uppercase() as i8; source = source.add(1); dest = dest.add(1); }
    acpi_ut_remove_reference(obj);
    *p = new_string;
    AE_OK
}

unsafe fn acpi_ns_repair_PRT(info: *mut acpi_evaluate_info, p: *mut *mut acpi_operand_object) -> acpi_status {
    let obj = *p;
    let mut top = (*obj).package.elements;
    for index in 0..(*obj).package.count {
        let sub = *top;
        if (*sub).package.count >= 4 {
            let list = (*sub).package.elements;
            if (*list.add(3)).is_null() || (**list.add(3)).common.type_ != ACPI_TYPE_INTEGER {
                let temp = *list.add(3); *list.add(3) = *list.add(2); *list.add(2) = temp;
                (*info).return_flags |= ACPI_OBJECT_REPAIRED;
            }
        }
        top = top.add(1);
    }
    AE_OK
}

unsafe fn acpi_ns_repair_PSS(info: *mut acpi_evaluate_info, p: *mut *mut acpi_operand_object) -> acpi_status {
    let obj = *p;
    let status = acpi_ns_check_sorted_list(info, obj, 0, 6, 0, ACPI_SORT_DESCENDING, b"CpuFrequency\0".as_ptr() as *mut i8);
    if ACPI_FAILURE(status) { return status; }
    let mut previous = ACPI_UINT32_MAX;
    for i in 0..(*obj).package.count {
        let sub = *(*obj).package.elements.add(i as usize);
        let value = (*(*sub).package.elements.add(1)).integer.value as u32;
        if value > previous { /* warning: suspicious power dissipation values */ }
        previous = value;
    }
    AE_OK
}

unsafe fn acpi_ns_repair_TSS(info: *mut acpi_evaluate_info, p: *mut *mut acpi_operand_object) -> acpi_status {
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    if ACPI_SUCCESS(acpi_ns_get_node((*info).node, b"^_PSS\0".as_ptr() as *const i8, ACPI_NS_NO_UPSEARCH, &mut node)) { return AE_OK; }
    acpi_ns_check_sorted_list(info, *p, 0, 5, 1, ACPI_SORT_DESCENDING, b"PowerDissipation\0".as_ptr() as *mut i8)
}

unsafe fn acpi_ns_check_sorted_list(info: *mut acpi_evaluate_info, obj: *mut acpi_operand_object, start: u32, expected: u32, index: u32, direction: u8, _key: *mut i8) -> acpi_status {
    if (*obj).common.type_ != ACPI_TYPE_PACKAGE { return AE_AML_OPERAND_TYPE; }
    let total = (*obj).package.count;
    if total == 0 || start >= total { return AE_AML_PACKAGE_LIMIT; }
    let mut previous = if direction == ACPI_SORT_DESCENDING { ACPI_UINT32_MAX } else { 0 };
    for i in 0..(total - start) {
        let outer = *(*obj).package.elements.add((start + i) as usize);
        if (*outer).common.type_ != ACPI_TYPE_PACKAGE || (*outer).package.count < expected { return AE_AML_OPERAND_TYPE; }
        let value = (*(*outer).package.elements.add(index as usize)).integer.value as u32;
        if (direction == ACPI_SORT_ASCENDING && value < previous) || (direction == ACPI_SORT_DESCENDING && value > previous) {
            acpi_ns_sort_list((*obj).package.elements.add(start as usize), total - start, index, direction);
            (*info).return_flags |= ACPI_OBJECT_REPAIRED;
            return AE_OK;
        }
        previous = value;
    }
    AE_OK
}

unsafe fn acpi_ns_sort_list(elements: *mut *mut acpi_operand_object, count: u32, index: u32, direction: u8) {
    for i in 1..count { let mut j = count - 1; while j >= i {
        let a = *(*elements.add((j - 1) as usize)).package.elements.add(index as usize);
        let b = *(*elements.add(j as usize)).package.elements.add(index as usize);
        if (direction == ACPI_SORT_ASCENDING && (*a).integer.value > (*b).integer.value) || (direction == ACPI_SORT_DESCENDING && (*a).integer.value < (*b).integer.value) {
            let t = *elements.add((j - 1) as usize); *elements.add((j - 1) as usize) = *elements.add(j as usize); *elements.add(j as usize) = t;
        }
        j -= 1;
    }}
}

unsafe fn acpi_ns_remove_element(obj: *mut acpi_operand_object, index: u32) {
    let count = (*obj).package.count;
    let source = (*obj).package.elements;
    let mut dest = source;
    for i in 0..count {
        if i == index { acpi_ut_remove_reference(*source.add(i as usize)); acpi_ut_remove_reference(*source.add(i as usize)); }
        else { *dest = *source.add(i as usize); dest = dest.add(1); }
    }
    *dest = core::ptr::null_mut();
    (*obj).package.count = count - 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
