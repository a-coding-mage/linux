// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Validation of package objects for predefined names.

// Dependencies supplied by the ACPICA translation unit are intentionally external.

pub unsafe fn acpi_ns_check_package(info: *mut acpi_evaluate_info, return_object_ptr: *mut *mut acpi_operand_object) -> acpi_status {
    let mut return_object = *return_object_ptr;
    let package = (*info).predefined.add(1);
    let mut status = AE_OK;
    let mut expected_count: u32;
    let mut count: u32;
    let mut elements: *mut *mut acpi_operand_object;

    acpi_ns_remove_null_elements(info, (*package).ret_info.type_, return_object);
    elements = (*return_object).package.elements;
    count = (*return_object).package.count;

    if count == 0 {
        if (*package).ret_info.type_ == ACPI_PTYPE1_VAR { return AE_OK; }
        return AE_AML_OPERAND_VALUE;
    }

    match (*package).ret_info.type_ {
        ACPI_PTYPE_CUSTOM => {
            status = acpi_ns_custom_package(info, elements, count);
        }
        ACPI_PTYPE1_FIXED => {
            expected_count = (*package).ret_info.count1 + (*package).ret_info.count2;
            if count < expected_count { return AE_AML_OPERAND_VALUE; }
            status = acpi_ns_check_package_elements(info, elements, (*package).ret_info.object_type1, (*package).ret_info.count1, (*package).ret_info.object_type2, (*package).ret_info.count2, 0);
        }
        ACPI_PTYPE1_VAR => {
            for i in 0..count {
                status = acpi_ns_check_object_type(info, elements, (*package).ret_info.object_type1, i);
                if ACPI_FAILURE(status) { return status; }
                elements = elements.add(1);
            }
        }
        ACPI_PTYPE1_OPTION => {
            expected_count = (*package).ret_info3.count;
            if count < expected_count { return AE_AML_OPERAND_VALUE; }
            for i in 0..count {
                let ty = if i < (*package).ret_info3.count { (*package).ret_info3.object_type[i as usize] } else { (*package).ret_info3.tail_object_type };
                status = acpi_ns_check_object_type(info, elements, ty, i);
                if ACPI_FAILURE(status) { return status; }
                elements = elements.add(1);
            }
        }
        ACPI_PTYPE2_REV_FIXED => {
            status = acpi_ns_check_object_type(info, elements, ACPI_RTYPE_INTEGER, 0);
            if ACPI_FAILURE(status) { return status; }
            elements = elements.add(1); count -= 1;
            status = acpi_ns_check_package_list(info, package, elements, count);
        }
        ACPI_PTYPE2_PKG_COUNT => {
            status = acpi_ns_check_object_type(info, elements, ACPI_RTYPE_INTEGER, 0);
            if ACPI_FAILURE(status) { return status; }
            expected_count = (*(*elements)).integer.value as u32;
            if expected_count >= count { return AE_AML_OPERAND_VALUE; }
            count = expected_count; elements = elements.add(1);
            status = acpi_ns_check_package_list(info, package, elements, count);
        }
        ACPI_PTYPE2 | ACPI_PTYPE2_FIXED | ACPI_PTYPE2_MIN | ACPI_PTYPE2_COUNT | ACPI_PTYPE2_FIX_VAR => {
            if !(*elements).is_null() && (*(*elements)).common.type_ != ACPI_TYPE_PACKAGE {
                status = acpi_ns_wrap_with_package(info, return_object, return_object_ptr);
                if ACPI_FAILURE(status) { return status; }
                return_object = *return_object_ptr; elements = (*return_object).package.elements; count = 1;
            }
            status = acpi_ns_check_package_list(info, package, elements, count);
        }
        ACPI_PTYPE2_VAR_VAR => {}
        ACPI_PTYPE2_UUID_PAIR => {
            if count & 1 != 0 { return AE_AML_OPERAND_VALUE; }
            while count > 0 {
                status = acpi_ns_check_object_type(info, elements, (*package).ret_info.object_type1, 0);
                if ACPI_FAILURE(status) { return status; }
                if (*(*elements)).buffer.length != 16 { return AE_AML_OPERAND_VALUE; }
                status = acpi_ns_check_object_type(info, elements.add(1), (*package).ret_info.object_type2, 0);
                if ACPI_FAILURE(status) { return status; }
                elements = elements.add(2); count -= 2;
            }
        }
        _ => return AE_AML_INTERNAL,
    }
    status
}

unsafe fn acpi_ns_check_package_list(info: *mut acpi_evaluate_info, package: *const acpi_predefined_info, mut elements: *mut *mut acpi_operand_object, count: u32) -> acpi_status {
    for i in 0..count {
        let sub_package = *elements;
        let sub_elements = (*sub_package).package.elements;
        (*info).parent_package = sub_package;
        let mut status = acpi_ns_check_object_type(info, &mut (sub_package as *mut acpi_operand_object), ACPI_RTYPE_PACKAGE, i);
        if ACPI_FAILURE(status) { return status; }
        (*info).parent_package = sub_package;
        match (*package).ret_info.type_ {
            ACPI_PTYPE2 | ACPI_PTYPE2_PKG_COUNT | ACPI_PTYPE2_REV_FIXED => {
                let expected = (*package).ret_info.count1 + (*package).ret_info.count2;
                if (*sub_package).package.count < expected { return AE_AML_OPERAND_VALUE; }
                status = acpi_ns_check_package_elements(info, sub_elements, (*package).ret_info.object_type1, (*package).ret_info.count1, (*package).ret_info.object_type2, (*package).ret_info.count2, 0);
            }
            ACPI_PTYPE2_FIX_VAR => {
                let expected = (*package).ret_info.count1 + (*package).ret_info.count2;
                if (*sub_package).package.count < expected { return AE_AML_OPERAND_VALUE; }
                status = acpi_ns_check_package_elements(info, sub_elements, (*package).ret_info.object_type1, (*package).ret_info.count1, (*package).ret_info.object_type2, (*sub_package).package.count - (*package).ret_info.count1, 0);
            }
            ACPI_PTYPE2_VAR_VAR => { status = AE_OK; }
            ACPI_PTYPE2_FIXED => {
                let expected = (*package).ret_info2.count;
                if (*sub_package).package.count < expected { return AE_AML_OPERAND_VALUE; }
                for j in 0..expected { status = acpi_ns_check_object_type(info, sub_elements.add(j as usize), (*package).ret_info2.object_type[j as usize], j); if ACPI_FAILURE(status) { return status; } }
            }
            ACPI_PTYPE2_MIN => {
                let expected = (*package).ret_info.count1;
                if (*sub_package).package.count < expected { return AE_AML_OPERAND_VALUE; }
                status = acpi_ns_check_package_elements(info, sub_elements, (*package).ret_info.object_type1, (*sub_package).package.count, 0, 0, 0);
            }
            ACPI_PTYPE2_COUNT => {
                status = acpi_ns_check_object_type(info, sub_elements, ACPI_RTYPE_INTEGER, 0);
                if ACPI_FAILURE(status) { return status; }
                let mut expected = (*(*sub_elements)).integer.value as u32;
                if (*sub_package).package.count < expected || (*sub_package).package.count < (*package).ret_info.count1 { return AE_AML_OPERAND_VALUE; }
                if expected == 0 { expected = (*sub_package).package.count; (*(*sub_elements)).integer.value = expected as _; }
                status = acpi_ns_check_package_elements(info, sub_elements.add(1), (*package).ret_info.object_type1, expected - 1, 0, 0, 1);
            }
            _ => return AE_AML_INTERNAL,
        }
        if ACPI_FAILURE(status) { return status; }
        elements = elements.add(1);
    }
    AE_OK
}

unsafe fn acpi_ns_custom_package(info: *mut acpi_evaluate_info, elements: *mut *mut acpi_operand_object, count: u32) -> acpi_status {
    if (*elements).is_null() || (*(*elements)).common.type_ != ACPI_TYPE_INTEGER { return AE_AML_OPERAND_TYPE; }
    let version = (*(*elements)).integer.value as u32;
    let expected = if version == 0 { 20 } else { 21 };
    if count < expected { return AE_AML_OPERAND_VALUE; }
    let mut status = acpi_ns_check_package_elements(info, elements, ACPI_RTYPE_INTEGER, 16, ACPI_RTYPE_STRING, 4, 0);
    if ACPI_FAILURE(status) { return status; }
    if version > 0 { status = acpi_ns_check_package_elements(info, elements.add(20), ACPI_RTYPE_INTEGER, 1, 0, 0, 20); }
    status
}

unsafe fn acpi_ns_check_package_elements(info: *mut acpi_evaluate_info, mut elements: *mut *mut acpi_operand_object, type1: u8, count1: u32, type2: u8, count2: u32, start_index: u32) -> acpi_status {
    for i in 0..count1 { let status = acpi_ns_check_object_type(info, elements, type1, i + start_index); if ACPI_FAILURE(status) { return status; } elements = elements.add(1); }
    for i in 0..count2 { let status = acpi_ns_check_object_type(info, elements, type2, i + count1 + start_index); if ACPI_FAILURE(status) { return status; } elements = elements.add(1); }
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
