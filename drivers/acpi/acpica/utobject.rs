// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: utobject - ACPI object create/delete/size/cache routines

// Dependencies are supplied by the surrounding ACPI translation.

unsafe extern "C" {
    fn acpi_ut_get_type_name(ty: acpi_object_type) -> *const core::ffi::c_char;
    fn acpi_ut_allocate_object_desc_dbg(module: *const core::ffi::c_char, line: u32, component: u32) -> *mut acpi_operand_object;
    fn acpi_ut_delete_object_desc(object: *mut acpi_operand_object);
    fn acpi_ut_create_internal_object(ty: acpi_object_type) -> *mut acpi_operand_object;
    fn acpi_ut_remove_reference(object: *mut acpi_operand_object);
    fn acpi_os_acquire_object(cache: *mut core::ffi::c_void) -> *mut acpi_operand_object;
    fn acpi_os_release_object(cache: *mut core::ffi::c_void, object: *mut acpi_operand_object) -> acpi_status;
    fn acpi_ns_get_pathname_length(node: *mut core::ffi::c_void) -> acpi_size;
    fn acpi_ut_get_reference_name(object: *mut acpi_operand_object) -> *const core::ffi::c_char;
    fn acpi_ut_get_object_type_name(object: *mut acpi_operand_object) -> *const core::ffi::c_char;
    fn acpi_ut_get_descriptor_name(object: *mut core::ffi::c_void) -> *const core::ffi::c_char;
    fn acpi_ut_walk_package_tree(object: *mut acpi_operand_object, state: *mut core::ffi::c_void,
        callback: unsafe extern "C" fn(u8, *mut acpi_operand_object, *mut acpi_generic_state, *mut core::ffi::c_void) -> acpi_status,
        context: *mut core::ffi::c_void) -> acpi_status;
    static mut acpi_gbl_operand_cache: *mut core::ffi::c_void;
}

type acpi_size = usize;
type acpi_status = i32;
type acpi_object_type = u32;

const AE_OK: acpi_status = 0;
const AE_BAD_PARAMETER: acpi_status = 1;
const AE_AML_INTERNAL: acpi_status = 2;
const AE_TYPE: acpi_status = 3;
const ACPI_DESC_TYPE_OPERAND: u8 = 1;
const ACPI_DESC_TYPE_NAMED: u8 = 2;
const ACPI_TYPE_REGION: acpi_object_type = 0x06;
const ACPI_TYPE_BUFFER_FIELD: acpi_object_type = 0x0d;
const ACPI_TYPE_LOCAL_BANK_FIELD: acpi_object_type = 0x13;
const ACPI_TYPE_LOCAL_EXTRA: u8 = 0x1b;
const ACPI_TYPE_PACKAGE: acpi_object_type = 0x12;
const ACPI_TYPE_INTEGER: acpi_object_type = 0x01;
const ACPI_TYPE_BUFFER: acpi_object_type = 0x03;
const ACPI_TYPE_STRING: acpi_object_type = 0x02;
const ACPI_TYPE_PROCESSOR: acpi_object_type = 0x05;
const ACPI_TYPE_POWER: acpi_object_type = 0x0a;
const ACPI_TYPE_LOCAL_REFERENCE: acpi_object_type = 0x14;
const ACPI_REFCLASS_NAME: u8 = 0x0a;
const ACPI_COPY_TYPE_SIMPLE: u8 = 1;
const ACPI_COPY_TYPE_PACKAGE: u8 = 2;
const AOPOBJ_DATA_VALID: u8 = 1;

#[repr(C)] pub struct acpi_operand_object {
    pub common: acpi_object_common,
    pub integer: acpi_object_integer,
    pub buffer: acpi_object_buffer,
    pub string: acpi_object_string,
    pub package: acpi_object_package,
    pub reference: acpi_object_reference,
}
#[repr(C)] pub struct acpi_object_common { pub descriptor_type: u8, pub type_: u8, pub reference_count: u16, pub next_object: *mut acpi_operand_object }
#[repr(C)] pub struct acpi_object_integer { pub value: u64 }
#[repr(C)] pub struct acpi_object_buffer { pub flags: u8, pub pointer: *mut u8, pub length: u32 }
#[repr(C)] pub struct acpi_object_string { pub pointer: *mut core::ffi::c_char, pub length: u32 }
#[repr(C)] pub struct acpi_object_package { pub count: u32, pub elements: *mut *mut acpi_operand_object }
#[repr(C)] pub struct acpi_object_reference { pub class: u8, pub node: *mut core::ffi::c_void }
#[repr(C)] pub struct acpi_generic_state { pub pkg: acpi_pkg_state }
#[repr(C)] pub struct acpi_pkg_state { pub this_target_obj: *mut acpi_operand_object }
#[repr(C)] pub struct acpi_pkg_info { pub length: acpi_size, pub object_space: acpi_size, pub num_packages: u32 }
#[repr(C)] pub struct acpi_object;

#[inline] unsafe fn round_word(x: acpi_size) -> acpi_size { (x + core::mem::size_of::<usize>() - 1) & !(core::mem::size_of::<usize>() - 1) }

pub unsafe fn acpi_ut_create_internal_object_dbg(module_name: *const core::ffi::c_char, line_number: u32, component_id: u32, ty: acpi_object_type) -> *mut acpi_operand_object {
    let object = acpi_ut_allocate_object_desc_dbg(module_name, line_number, component_id);
    if object.is_null() { return core::ptr::null_mut(); }
    match ty {
        ACPI_TYPE_REGION | ACPI_TYPE_BUFFER_FIELD | ACPI_TYPE_LOCAL_BANK_FIELD => {
            let second = acpi_ut_allocate_object_desc_dbg(module_name, line_number, component_id);
            if second.is_null() { acpi_ut_delete_object_desc(object); return core::ptr::null_mut(); }
            (*second).common.type_ = ACPI_TYPE_LOCAL_EXTRA;
            (*second).common.reference_count = 1;
            (*object).common.next_object = second;
        }
        _ => {}
    }
    (*object).common.type_ = ty as u8;
    (*object).common.reference_count = 1;
    object
}

pub unsafe fn acpi_ut_create_package_object(count: u32) -> *mut acpi_operand_object {
    let package = acpi_ut_create_internal_object(ACPI_TYPE_PACKAGE);
    if package.is_null() { return core::ptr::null_mut(); }
    let elements = libc::calloc((count as usize) + 1, core::mem::size_of::<*mut acpi_operand_object>()) as *mut *mut acpi_operand_object;
    if elements.is_null() { acpi_ut_delete_object_desc(package); return core::ptr::null_mut(); }
    (*package).package.count = count; (*package).package.elements = elements; package
}

pub unsafe fn acpi_ut_create_integer_object(value: u64) -> *mut acpi_operand_object {
    let object = acpi_ut_create_internal_object(ACPI_TYPE_INTEGER); if object.is_null() { return object; }
    (*object).integer.value = value; object
}

pub unsafe fn acpi_ut_create_buffer_object(size: acpi_size) -> *mut acpi_operand_object {
    let object = acpi_ut_create_internal_object(ACPI_TYPE_BUFFER); if object.is_null() { return object; }
    let buffer = if size != 0 { libc::calloc(size, 1) as *mut u8 } else { core::ptr::null_mut() };
    if size != 0 && buffer.is_null() { acpi_ut_remove_reference(object); return core::ptr::null_mut(); }
    (*object).buffer.flags |= AOPOBJ_DATA_VALID; (*object).buffer.pointer = buffer; (*object).buffer.length = size as u32; object
}

pub unsafe fn acpi_ut_create_string_object(size: acpi_size) -> *mut acpi_operand_object {
    let object = acpi_ut_create_internal_object(ACPI_TYPE_STRING); if object.is_null() { return object; }
    let string = libc::calloc(size + 1, 1) as *mut core::ffi::c_char;
    if string.is_null() { acpi_ut_remove_reference(object); return core::ptr::null_mut(); }
    (*object).string.pointer = string; (*object).string.length = size as u32; object
}

pub unsafe fn acpi_ut_valid_internal_object(object: *mut core::ffi::c_void) -> u8 {
    if object.is_null() { return 0; }
    if (*(object as *mut acpi_operand_object)).common.descriptor_type == ACPI_DESC_TYPE_OPERAND { 1 } else { 0 }
}

pub unsafe fn acpi_ut_allocate_object_desc_dbg(module: *const core::ffi::c_char, line: u32, component: u32) -> *mut acpi_operand_object {
    let object = acpi_os_acquire_object(acpi_gbl_operand_cache); if object.is_null() { return core::ptr::null_mut(); }
    (*object).common.descriptor_type = ACPI_DESC_TYPE_OPERAND; object
}

pub unsafe fn acpi_ut_delete_object_desc(object: *mut acpi_operand_object) {
    if (*object).common.descriptor_type != ACPI_DESC_TYPE_OPERAND { return; }
    let _ = acpi_os_release_object(acpi_gbl_operand_cache, object);
}

unsafe fn acpi_ut_get_simple_object_size(object: *mut acpi_operand_object, length_out: *mut acpi_size) -> acpi_status {
    let mut length = core::mem::size_of::<acpi_object>();
    if object.is_null() { *length_out = round_word(length); return AE_OK; }
    match (*object).common.type_ as acpi_object_type {
        ACPI_TYPE_STRING => length += (*object).string.length as usize + 1,
        ACPI_TYPE_BUFFER => length += (*object).buffer.length as usize,
        ACPI_TYPE_INTEGER | ACPI_TYPE_PROCESSOR | ACPI_TYPE_POWER => {}
        ACPI_TYPE_LOCAL_REFERENCE => if (*object).reference.class == ACPI_REFCLASS_NAME {
            let size = acpi_ns_get_pathname_length((*object).reference.node); if size == 0 { return AE_BAD_PARAMETER; } length += round_word(size);
        } else { return AE_TYPE },
        _ => return AE_TYPE,
    }
    *length_out = round_word(length); AE_OK
}

unsafe extern "C" fn acpi_ut_get_element_length(ty: u8, source: *mut acpi_operand_object, state: *mut acpi_generic_state, context: *mut core::ffi::c_void) -> acpi_status {
    let info = &mut *(context as *mut acpi_pkg_info);
    match ty { ACPI_COPY_TYPE_SIMPLE => { let mut space = 0; let status = acpi_ut_get_simple_object_size(source, &mut space); if status != AE_OK { return status; } info.length += space; }
        ACPI_COPY_TYPE_PACKAGE => { info.num_packages += 1; (*state).pkg.this_target_obj = core::ptr::null_mut(); }
        _ => return AE_BAD_PARAMETER }
    AE_OK
}

unsafe fn acpi_ut_get_package_object_size(object: *mut acpi_operand_object, out: *mut acpi_size) -> acpi_status {
    let mut info = acpi_pkg_info { length: 0, object_space: 0, num_packages: 1 };
    let status = acpi_ut_walk_package_tree(object, core::ptr::null_mut(), acpi_ut_get_element_length, &mut info as *mut _ as *mut _);
    if status != AE_OK { return status; }
    info.length += round_word(core::mem::size_of::<acpi_object>()) * info.num_packages as usize; *out = info.length; status
}

pub unsafe fn acpi_ut_get_object_size(object: *mut acpi_operand_object, out: *mut acpi_size) -> acpi_status {
    if !object.is_null() && (*object).common.descriptor_type == ACPI_DESC_TYPE_OPERAND && (*object).common.type_ as acpi_object_type == ACPI_TYPE_PACKAGE { acpi_ut_get_package_object_size(object, out) } else { acpi_ut_get_simple_object_size(object, out) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
