// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Support for device IDs: HID, UID, CID, SUB, CLS.

use crate::*;
use core::ffi::c_char;
use core::ptr;

pub unsafe fn acpi_ut_execute_HID(
    device_node: *mut acpi_namespace_node,
    return_id: *mut *mut acpi_pnp_device_id,
) -> acpi_status {
    let mut obj_desc: *mut acpi_operand_object = ptr::null_mut();
    let mut hid: *mut acpi_pnp_device_id;
    let length: u32;
    let mut status = acpi_ut_evaluate_object(
        device_node, METHOD_NAME__HID, ACPI_BTYPE_INTEGER | ACPI_BTYPE_STRING,
        &mut obj_desc,
    );
    if ACPI_FAILURE(status) { return status; }

    if (*obj_desc).common.type_ == ACPI_TYPE_INTEGER {
        length = ACPI_EISAID_STRING_SIZE;
    } else {
        length = (*obj_desc).string.length + 1;
    }
    hid = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_pnp_device_id>() + length as usize)
        as *mut acpi_pnp_device_id;
    if hid.is_null() { status = AE_NO_MEMORY; }
    else {
        (*hid).string = (hid as *mut u8).add(core::mem::size_of::<acpi_pnp_device_id>()) as *mut c_char;
        if (*obj_desc).common.type_ == ACPI_TYPE_INTEGER {
            acpi_ex_eisa_id_to_string((*hid).string, (*obj_desc).integer.value);
        } else {
            strcpy((*hid).string, (*obj_desc).string.pointer);
        }
        (*hid).length = length;
        *return_id = hid;
    }
    acpi_ut_remove_reference(obj_desc);
    status
}

pub unsafe fn acpi_ut_execute_UID(
    device_node: *mut acpi_namespace_node,
    return_id: *mut *mut acpi_pnp_device_id,
) -> acpi_status {
    let mut obj_desc: *mut acpi_operand_object = ptr::null_mut();
    let mut uid: *mut acpi_pnp_device_id;
    let length: u32;
    let mut status = acpi_ut_evaluate_object(
        device_node, METHOD_NAME__UID, ACPI_BTYPE_INTEGER | ACPI_BTYPE_STRING,
        &mut obj_desc,
    );
    if ACPI_FAILURE(status) { return status; }
    if (*obj_desc).common.type_ == ACPI_TYPE_INTEGER { length = ACPI_MAX64_DECIMAL_DIGITS + 1; }
    else { length = (*obj_desc).string.length + 1; }
    uid = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_pnp_device_id>() + length as usize)
        as *mut acpi_pnp_device_id;
    if uid.is_null() { status = AE_NO_MEMORY; }
    else {
        (*uid).string = (uid as *mut u8).add(core::mem::size_of::<acpi_pnp_device_id>()) as *mut c_char;
        if (*obj_desc).common.type_ == ACPI_TYPE_INTEGER {
            acpi_ex_integer_to_string((*uid).string, (*obj_desc).integer.value);
        } else { strcpy((*uid).string, (*obj_desc).string.pointer); }
        (*uid).length = length;
        *return_id = uid;
    }
    acpi_ut_remove_reference(obj_desc);
    status
}

pub unsafe fn acpi_ut_execute_CID(
    device_node: *mut acpi_namespace_node,
    return_cid_list: *mut *mut acpi_pnp_device_id_list,
) -> acpi_status {
    let mut obj_desc: *mut acpi_operand_object = ptr::null_mut();
    let mut status = acpi_ut_evaluate_object(device_node, METHOD_NAME__CID,
        ACPI_BTYPE_INTEGER | ACPI_BTYPE_STRING | ACPI_BTYPE_PACKAGE, &mut obj_desc);
    if ACPI_FAILURE(status) { return status; }
    let (count, cid_objects) = if (*obj_desc).common.type_ == ACPI_TYPE_PACKAGE {
        ((*obj_desc).package.count, (*obj_desc).package.elements)
    } else { (1, &mut obj_desc as *mut *mut acpi_operand_object) };
    let mut string_area_size = 0u32;
    for i in 0..count {
        match (*(*cid_objects.add(i as usize))).common.type_ {
            ACPI_TYPE_INTEGER => string_area_size += ACPI_EISAID_STRING_SIZE,
            ACPI_TYPE_STRING => string_area_size += (*(*cid_objects.add(i as usize))).string.length + 1,
            _ => { status = AE_TYPE; acpi_ut_remove_reference(obj_desc); return status; }
        }
    }
    let cid_list_size = core::mem::size_of::<acpi_pnp_device_id_list>() as u32
        + count * core::mem::size_of::<acpi_pnp_device_id>() as u32 + string_area_size;
    let cid_list = ACPI_ALLOCATE_ZEROED(cid_list_size as usize) as *mut acpi_pnp_device_id_list;
    if cid_list.is_null() { status = AE_NO_MEMORY; acpi_ut_remove_reference(obj_desc); return status; }
    let ids = (*cid_list).ids as *mut acpi_pnp_device_id;
    let mut next = ids.add(count as usize) as *mut c_char;
    for i in 0..count {
        let object = *cid_objects.add(i as usize);
        let length;
        if (*object).common.type_ == ACPI_TYPE_INTEGER {
            acpi_ex_eisa_id_to_string(next, (*object).integer.value); length = ACPI_EISAID_STRING_SIZE;
        } else { strcpy(next, (*object).string.pointer); length = (*object).string.length + 1; }
        (*ids.add(i as usize)).string = next; (*ids.add(i as usize)).length = length; next = next.add(length as usize);
    }
    (*cid_list).count = count; (*cid_list).list_size = cid_list_size; *return_cid_list = cid_list;
    acpi_ut_remove_reference(obj_desc); status
}

pub unsafe fn acpi_ut_execute_CLS(
    device_node: *mut acpi_namespace_node,
    return_id: *mut *mut acpi_pnp_device_id,
) -> acpi_status {
    let mut obj_desc: *mut acpi_operand_object = ptr::null_mut();
    let mut status = acpi_ut_evaluate_object(device_node, METHOD_NAME__CLS, ACPI_BTYPE_PACKAGE, &mut obj_desc);
    if ACPI_FAILURE(status) { return status; }
    let objects = (*obj_desc).package.elements; let count = (*obj_desc).package.count;
    let mut class_code = [0u8; 3];
    for i in 0..3 { if i < count && (*(*objects.add(i as usize))).common.type_ == ACPI_TYPE_INTEGER { class_code[i as usize] = (*(*objects.add(i as usize))).integer.value as u8; } }
    let length = ACPI_PCICLS_STRING_SIZE;
    let cls = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_pnp_device_id>() + length as usize) as *mut acpi_pnp_device_id;
    if cls.is_null() { status = AE_NO_MEMORY; } else {
        (*cls).string = (cls as *mut u8).add(core::mem::size_of::<acpi_pnp_device_id>()) as *mut c_char;
        acpi_ex_pci_cls_to_string((*cls).string, class_code.as_mut_ptr()); (*cls).length = length; *return_id = cls;
    }
    acpi_ut_remove_reference(obj_desc); status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
