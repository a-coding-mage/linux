// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: dbtest - Various debug-related tests

// C dependencies supplied by the ACPICA translation unit are intentionally external.

const CMD_TEST_OBJECTS: u32 = 0;
const CMD_TEST_PREDEFINED: u32 = 1;
const BUFFER_FILL_VALUE: u8 = 0xff;
const ACPI_DB_READ_METHOD: &str = "\\_T98";
const ACPI_DB_WRITE_METHOD: &str = "\\_T99";

static mut READ_HANDLE: acpi_handle = core::ptr::null_mut();
static mut WRITE_HANDLE: acpi_handle = core::ptr::null_mut();

static READ_METHOD_CODE: [u8; 46] = [
    0x53,0x53,0x44,0x54,0x2e,0,0,0, 0x02,0xc9,0x49,0x6e,0x74,0x65,0x6c,0,
    0x44,0x45,0x42,0x55,0x47,0,0,0, 1,0,0,0,0x49,0x4e,0x54,0x4c,
    0x18,0x12,0x13,0x20,0x14,9,0x5f,0x54,0x39,0x38,1,0xa4,0x83,0x68,
];
static WRITE_METHOD_CODE: [u8; 46] = [
    0x53,0x53,0x44,0x54,0x2e,0,0,0, 0x02,0x15,0x49,0x6e,0x74,0x65,0x6c,0,
    0x44,0x45,0x42,0x55,0x47,0,0,0, 1,0,0,0,0x49,0x4e,0x54,0x4c,
    0x18,0x12,0x13,0x20,0x14,9,0x5f,0x54,0x39,0x39,2,0x70,0x69,0x68,
];

unsafe fn acpi_db_execute_test(type_arg: *mut i8) {
    let temp = acpi_db_match_argument(type_arg, acpi_db_test_types);
    if temp == ACPI_TYPE_NOT_FOUND { acpi_os_printf(c"Invalid or unsupported argument\n".as_ptr()); return; }
    match temp { CMD_TEST_OBJECTS => acpi_db_test_all_objects(), CMD_TEST_PREDEFINED => acpi_db_evaluate_all_predefined_names(core::ptr::null_mut()), _ => {} }
}

unsafe fn acpi_db_test_all_objects() {
    let mut status;
    if READ_HANDLE.is_null() {
        status = acpi_install_method(READ_METHOD_CODE.as_ptr() as *mut u8);
        if ACPI_FAILURE(status) { acpi_os_printf(c"%s, Could not install debugger read method\n".as_ptr(), acpi_format_exception(status)); return; }
        status = acpi_get_handle(core::ptr::null_mut(), ACPI_DB_READ_METHOD.as_ptr() as *mut i8, &mut READ_HANDLE);
        if ACPI_FAILURE(status) { acpi_os_printf(c"Could not obtain handle for debug method %s\n".as_ptr(), ACPI_DB_READ_METHOD.as_ptr()); return; }
    }
    if WRITE_HANDLE.is_null() {
        status = acpi_install_method(WRITE_METHOD_CODE.as_ptr() as *mut u8);
        if ACPI_FAILURE(status) { acpi_os_printf(c"%s, Could not install debugger write method\n".as_ptr(), acpi_format_exception(status)); return; }
        status = acpi_get_handle(core::ptr::null_mut(), ACPI_DB_WRITE_METHOD.as_ptr() as *mut i8, &mut WRITE_HANDLE);
        if ACPI_FAILURE(status) { acpi_os_printf(c"Could not obtain handle for debug method %s\n".as_ptr(), ACPI_DB_WRITE_METHOD.as_ptr()); return; }
    }
    let _ = acpi_walk_namespace(ACPI_TYPE_ANY, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, Some(acpi_db_test_one_object), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
}

unsafe extern "C" fn acpi_db_test_one_object(obj_handle: acpi_handle, _nesting_level: u32, _context: *mut core::ffi::c_void, _return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    let node = obj_handle as *mut acpi_namespace_node;
    let obj_desc = (*node).object;
    let mut local_type; let mut bit_length = 0; let mut byte_length = 0;
    match (*node).type_ {
        ACPI_TYPE_INTEGER => { local_type = ACPI_TYPE_INTEGER; bit_length = acpi_gbl_integer_bit_width; }
        ACPI_TYPE_STRING => { local_type = ACPI_TYPE_STRING; byte_length = (*obj_desc).string.length; }
        ACPI_TYPE_BUFFER => { local_type = ACPI_TYPE_BUFFER; byte_length = (*obj_desc).buffer.length; bit_length = byte_length * 8; }
        ACPI_TYPE_PACKAGE => local_type = ACPI_TYPE_PACKAGE,
        ACPI_TYPE_FIELD_UNIT | ACPI_TYPE_LOCAL_REGION_FIELD | ACPI_TYPE_LOCAL_INDEX_FIELD | ACPI_TYPE_LOCAL_BANK_FIELD => local_type = ACPI_TYPE_FIELD_UNIT,
        ACPI_TYPE_BUFFER_FIELD => { local_type = ACPI_TYPE_INTEGER; if !obj_desc.is_null() { bit_length = (*obj_desc).common_field.bit_length; byte_length = ACPI_ROUND_BITS_UP_TO_BYTES(bit_length); if bit_length > acpi_gbl_integer_bit_width { local_type = ACPI_TYPE_BUFFER; } } }
        _ => return AE_OK,
    }
    acpi_os_printf(c"%14s: %4.4s".as_ptr(), acpi_ut_get_type_name((*node).type_), (*node).name.ascii.as_ptr());
    if obj_desc.is_null() { acpi_os_printf(c" No attached sub-object, ignoring\n".as_ptr()); return AE_OK; }
    let mut status = match local_type { ACPI_TYPE_INTEGER => acpi_db_test_integer_type(node, bit_length), ACPI_TYPE_STRING => acpi_db_test_string_type(node, byte_length), ACPI_TYPE_BUFFER => acpi_db_test_buffer_type(node, bit_length), ACPI_TYPE_PACKAGE => acpi_db_test_package_type(node), ACPI_TYPE_FIELD_UNIT => acpi_db_test_field_unit_type(obj_desc), _ => { acpi_os_printf(c" Ignoring, type not implemented (%2.2X)".as_ptr(), local_type); AE_OK } };
    if ACPI_FAILURE(status) { status = AE_OK; } acpi_os_printf(c"\n".as_ptr()); status
}

unsafe fn acpi_db_test_integer_type(node: *mut acpi_namespace_node, bit_length: u32) -> acpi_status {
    if bit_length > 64 { acpi_os_printf(c" Invalid length for an Integer: %u".as_ptr(), bit_length); return AE_OK; }
    let mut a = core::ptr::null_mut(); let mut b = core::ptr::null_mut(); let mut c = core::ptr::null_mut();
    let mut status = acpi_db_read_from_object(node, ACPI_TYPE_INTEGER, &mut a); if ACPI_FAILURE(status) { return status; }
    let mut value = u64::MAX >> (64 - bit_length); if (*a).integer.value == value { value = 0; }
    let mut w: acpi_object = core::mem::zeroed(); w.type_ = ACPI_TYPE_INTEGER; w.integer.value = value;
    status = acpi_db_write_to_object(node, &mut w); if ACPI_FAILURE(status) { return AE_OK; }
    status = acpi_db_read_from_object(node, ACPI_TYPE_INTEGER, &mut b); if ACPI_FAILURE(status) { return AE_OK; }
    if (*b).integer.value != value { acpi_os_printf(c" MISMATCH 2".as_ptr()); }
    w.integer.value = (*a).integer.value; status = acpi_db_write_to_object(node, &mut w); if ACPI_FAILURE(status) { return AE_OK; }
    status = acpi_db_read_from_object(node, ACPI_TYPE_INTEGER, &mut c); if ACPI_FAILURE(status) { return AE_OK; }
    if (*c).integer.value != (*a).integer.value { acpi_os_printf(c" MISMATCH 3".as_ptr()); } acpi_os_free(a as *mut _); acpi_os_free(b as *mut _); acpi_os_free(c as *mut _); AE_OK
}

unsafe fn acpi_db_test_buffer_type(node: *mut acpi_namespace_node, bit_length: u32) -> acpi_status {
    let byte_length = ACPI_ROUND_BITS_UP_TO_BYTES(bit_length); if byte_length == 0 { acpi_os_printf(c" Ignoring zero length buffer".as_ptr()); return AE_OK; }
    let buffer = ACPI_ALLOCATE_ZEROED(byte_length) as *mut u8; if buffer.is_null() { return AE_NO_MEMORY; }
    let mut a=core::ptr::null_mut(); let mut b=core::ptr::null_mut(); let mut c=core::ptr::null_mut(); let mut status=acpi_db_read_from_object(node, ACPI_TYPE_BUFFER, &mut a); if ACPI_FAILURE(status) { ACPI_FREE(buffer as *mut _); return status; }
    core::ptr::write_bytes(buffer, BUFFER_FILL_VALUE, byte_length as usize); let extra = bit_length % 8; if extra != 0 { *buffer.add(byte_length as usize-1)=ACPI_MASK_BITS_ABOVE(extra); }
    let mut w: acpi_object=core::mem::zeroed(); w.type_=ACPI_TYPE_BUFFER; w.buffer.length=byte_length; w.buffer.pointer=buffer; status=acpi_db_write_to_object(node,&mut w); if !ACPI_FAILURE(status) { status=acpi_db_read_from_object(node,ACPI_TYPE_BUFFER,&mut b); } if !ACPI_FAILURE(status) { w.buffer.pointer=(*a).buffer.pointer; status=acpi_db_write_to_object(node,&mut w); } if !ACPI_FAILURE(status) { status=acpi_db_read_from_object(node,ACPI_TYPE_BUFFER,&mut c); }
    ACPI_FREE(buffer as *mut _); acpi_os_free(a as *mut _); if !b.is_null(){acpi_os_free(b as *mut _);} if !c.is_null(){acpi_os_free(c as *mut _);} status
}

unsafe fn acpi_db_test_string_type(node: *mut acpi_namespace_node, _byte_length: u32) -> acpi_status {
    let mut a=core::ptr::null_mut(); let mut b=core::ptr::null_mut(); let mut c=core::ptr::null_mut(); let mut status=acpi_db_read_from_object(node,ACPI_TYPE_STRING,&mut a); if ACPI_FAILURE(status){return status;}
    let s=b"Test String from AML Debugger\0"; let mut w:acpi_object=core::mem::zeroed(); w.type_=ACPI_TYPE_STRING; w.string.length=(s.len()-1) as u32; w.string.pointer=s.as_ptr() as *mut i8; status=acpi_db_write_to_object(node,&mut w); if !ACPI_FAILURE(status){status=acpi_db_read_from_object(node,ACPI_TYPE_STRING,&mut b);} if !ACPI_FAILURE(status){w.string.length=libc::strlen((*a).string.pointer) as u32;w.string.pointer=(*a).string.pointer;status=acpi_db_write_to_object(node,&mut w);} if !ACPI_FAILURE(status){status=acpi_db_read_from_object(node,ACPI_TYPE_STRING,&mut c);} acpi_os_free(a as *mut _);if !b.is_null(){acpi_os_free(b as *mut _);}if !c.is_null(){acpi_os_free(c as *mut _);} status
}

unsafe fn acpi_db_test_package_type(node:*mut acpi_namespace_node)->acpi_status{let mut a=core::ptr::null_mut();let s=acpi_db_read_from_object(node,ACPI_TYPE_PACKAGE,&mut a);if !ACPI_FAILURE(s){acpi_os_printf(c" %.2X Elements".as_ptr(),(*a).package.count);acpi_os_free(a as *mut _);}s}
unsafe fn acpi_db_test_field_unit_type(obj:*mut acpi_operand_object)->acpi_status{let r=(*obj).field.region_obj;match (*r).region.space_id{ACPI_ADR_SPACE_SYSTEM_MEMORY|ACPI_ADR_SPACE_SYSTEM_IO|ACPI_ADR_SPACE_PCI_CONFIG=>{acpi_ut_acquire_mutex(ACPI_MTX_INTERPRETER);acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);let mut ret=core::ptr::null_mut();let s=acpi_ex_read_data_from_field(core::ptr::null_mut(),obj,&mut ret);if s==AE_OK{acpi_ex_write_data_to_field(ret,obj,core::ptr::null_mut());acpi_ut_remove_reference(ret);}acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);acpi_ut_release_mutex(ACPI_MTX_INTERPRETER);s},_=>AE_OK}}

unsafe fn acpi_db_read_from_object(node:*mut acpi_namespace_node, expected:u32, value:*mut *mut acpi_object)->acpi_status{let mut p:acpi_object=core::mem::zeroed();p.type_=ACPI_TYPE_LOCAL_REFERENCE;p.reference.actual_type=(*node).type_;p.reference.handle=node as acpi_handle;let mut list=acpi_object_list{count:1,pointer:&mut p};let mut ret=acpi_buffer{length:ACPI_ALLOCATE_BUFFER,pointer:core::ptr::null_mut()};acpi_gbl_method_executing=true;let s=acpi_evaluate_object(READ_HANDLE,core::ptr::null_mut(),&mut list,&mut ret);acpi_gbl_method_executing=false;if ACPI_FAILURE(s){return s;}let out=ret.pointer as *mut acpi_object;if (*out).type_!=expected{acpi_os_free(ret.pointer);return AE_TYPE;}*value=out;s}
unsafe fn acpi_db_write_to_object(node:*mut acpi_namespace_node,value:*mut acpi_object)->acpi_status{let mut p:acpi_object=core::mem::zeroed();p.type_=ACPI_TYPE_LOCAL_REFERENCE;p.reference.actual_type=(*node).type_;p.reference.handle=node as acpi_handle;let mut params=[p,*value];let mut list=acpi_object_list{count:2,pointer:params.as_mut_ptr()};acpi_gbl_method_executing=true;let s=acpi_evaluate_object(WRITE_HANDLE,core::ptr::null_mut(),&mut list,core::ptr::null_mut());acpi_gbl_method_executing=false;s}

unsafe fn acpi_db_evaluate_all_predefined_names(count_arg:*mut i8){let mut info=acpi_db_execute_walk{count:0,max_count:ACPI_UINT32_MAX};if !count_arg.is_null(){info.max_count=libc::strtoul(count_arg,core::ptr::null_mut(),0) as u32;}let _=acpi_walk_namespace(ACPI_TYPE_ANY,ACPI_ROOT_OBJECT,ACPI_UINT32_MAX,Some(acpi_db_evaluate_one_predefined_name),core::ptr::null_mut(),&mut info as *mut _ as *mut _,core::ptr::null_mut());acpi_os_printf(c"Evaluated %u predefined names in the namespace\n".as_ptr(),info.count);}
unsafe extern "C" fn acpi_db_evaluate_one_predefined_name(obj_handle:acpi_handle,_nesting:u32,context:*mut core::ffi::c_void,_ret:*mut *mut core::ffi::c_void)->acpi_status{let node=obj_handle as *mut acpi_namespace_node;let info=context as *mut acpi_db_execute_walk;let pre=acpi_ut_match_predefined_method((*node).name.ascii.as_ptr());if pre.is_null()||(*node).type_==ACPI_TYPE_LOCAL_SCOPE{return AE_OK;}let mut oi=core::ptr::null_mut();let s=acpi_get_object_info(obj_handle,&mut oi);if ACPI_FAILURE(s){return s;}let mut params:[acpi_object;ACPI_METHOD_NUM_ARGS as usize]=core::mem::zeroed();let mut list=acpi_object_list{count:0,pointer:core::ptr::null_mut()};if (*oi).type_==ACPI_TYPE_METHOD{let mut types=(*pre).info.argument_list;let n=METHOD_GET_ARG_COUNT(types);for i in 0..n as usize{let t=METHOD_GET_NEXT_TYPE(types);params[i].type_=t; if t==ACPI_TYPE_INTEGER{params[i].integer.value=1;}}list.count=n;list.pointer=params.as_mut_ptr();}ACPI_FREE(oi);let mut out=acpi_buffer{length:ACPI_ALLOCATE_BUFFER,pointer:core::ptr::null_mut()};acpi_gbl_method_executing=true;let _=acpi_evaluate_object(node,core::ptr::null_mut(),&mut list,&mut out);acpi_gbl_method_executing=false;ACPI_FREE(out.pointer);(*info).count+=1;if (*info).count>=(*info).max_count{AE_CTRL_TERMINATE}else{AE_OK}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
