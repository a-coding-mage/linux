// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Debugger commands for the ACPI namespace.

// Types, constants, globals, and functions referenced here are supplied by the
// surrounding ACPICA translation.

static mut ACPI_DB_OBJECT_TYPES: [&'static str; 29] = [
    "ANY", "INTEGERS", "STRINGS", "BUFFERS", "PACKAGES", "FIELDS",
    "DEVICES", "EVENTS", "METHODS", "MUTEXES", "REGIONS", "POWERRESOURCES",
    "PROCESSORS", "THERMALZONES", "BUFFERFIELDS", "DDBHANDLES", "DEBUG",
    "REGIONFIELDS", "BANKFIELDS", "INDEXFIELDS", "REFERENCES", "ALIASES",
    "METHODALIASES", "NOTIFY", "ADDRESSHANDLER", "RESOURCE", "RESOURCEFIELD",
    "SCOPES", "",
];

unsafe fn acpi_db_walk_and_match_name(obj_handle: acpi_handle, nesting_level: u32, context: *mut core::ffi::c_void, _return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    let requested_name = context as *const u8;
    for i in 0..4 {
        if *requested_name.add(i) != b'?' && *requested_name.add(i) != (*(obj_handle as *mut acpi_namespace_node)).name.ascii[i] { return AE_OK; }
    }
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_LOCAL_BUFFER, pointer: core::ptr::null_mut() };
    let status = acpi_ns_handle_to_pathname(obj_handle, &mut buffer, TRUE);
    if ACPI_FAILURE(status) { acpi_os_printf(cstr!("Could Not get pathname for object %p\n"), obj_handle); }
    else {
        let mut info = acpi_walk_info { count: 0, owner_id: ACPI_OWNER_ID_MAX, debug_level: ACPI_UINT32_MAX, display_type: ACPI_DISPLAY_SUMMARY | ACPI_DISPLAY_SHORT };
        acpi_os_printf(cstr!("%32s"), buffer.pointer);
        acpi_ns_dump_one_object(obj_handle, nesting_level, &mut info, core::ptr::null_mut());
        ACPI_FREE(buffer.pointer);
    }
    AE_OK
}

pub unsafe fn acpi_db_set_scope(name: *mut i8) {
    if name.is_null() || *name == 0 { acpi_os_printf(cstr!("Current scope: %s\n"), acpi_gbl_db_scope_buf.as_ptr()); return; }
    acpi_db_prep_namestring(name);
    let mut node = core::ptr::null_mut();
    let mut status;
    if ACPI_IS_ROOT_PREFIX(*name as u8) { status = acpi_ns_get_node(acpi_gbl_root_node, name, ACPI_NS_NO_UPSEARCH, &mut node); if ACPI_FAILURE(status) { acpi_os_printf(cstr!("Could not attach scope: %s, %s\n"), name, acpi_format_exception(status)); return; } acpi_gbl_db_scope_buf[0] = 0; }
    else { status = acpi_ns_get_node(acpi_gbl_db_scope_node, name, ACPI_NS_NO_UPSEARCH, &mut node); if ACPI_FAILURE(status) { acpi_os_printf(cstr!("Could not attach scope: %s, %s\n"), name, acpi_format_exception(status)); return; } }
    if acpi_ut_safe_strcat(acpi_gbl_db_scope_buf.as_mut_ptr(), core::mem::size_of_val(&acpi_gbl_db_scope_buf), name) != 0 || acpi_ut_safe_strcat(acpi_gbl_db_scope_buf.as_mut_ptr(), core::mem::size_of_val(&acpi_gbl_db_scope_buf), cstr!("\\")) != 0 { status = AE_BUFFER_OVERFLOW; acpi_os_printf(cstr!("Could not attach scope: %s, %s\n"), name, acpi_format_exception(status)); return; }
    acpi_gbl_db_scope_node = node; acpi_os_printf(cstr!("New scope: %s\n"), acpi_gbl_db_scope_buf.as_ptr());
}

pub unsafe fn acpi_db_dump_namespace(start_arg: *mut i8, depth_arg: *mut i8) {
    let mut subtree_entry = acpi_gbl_root_node; let mut max_depth = ACPI_UINT32_MAX;
    if !start_arg.is_null() { subtree_entry = acpi_db_convert_to_node(start_arg); if subtree_entry.is_null() { return; } if !depth_arg.is_null() { max_depth = strtoul(depth_arg, core::ptr::null_mut(), 0) as u32; } }
    acpi_db_set_output_destination(ACPI_DB_DUPLICATE_OUTPUT);
    if !(*(subtree_entry as *mut acpi_namespace_node)).parent.is_null() { acpi_os_printf(cstr!("ACPI Namespace (from %4.4s (%p) subtree):\n"), (*((subtree_entry as *mut acpi_namespace_node))).name.ascii.as_ptr(), subtree_entry); } else { acpi_os_printf(cstr!("ACPI Namespace (from %s):\n"), ACPI_NAMESPACE_ROOT); }
    acpi_db_set_output_destination(ACPI_DB_REDIRECTABLE_OUTPUT); acpi_ns_dump_objects(ACPI_TYPE_ANY, ACPI_DISPLAY_SUMMARY, max_depth, ACPI_OWNER_ID_MAX, subtree_entry); acpi_db_set_output_destination(ACPI_DB_CONSOLE_OUTPUT);
}

pub unsafe fn acpi_db_dump_namespace_paths() { acpi_db_set_output_destination(ACPI_DB_DUPLICATE_OUTPUT); acpi_os_printf(cstr!("ACPI Namespace (from root):\n")); acpi_db_set_output_destination(ACPI_DB_REDIRECTABLE_OUTPUT); acpi_ns_dump_object_paths(ACPI_TYPE_ANY, ACPI_DISPLAY_SUMMARY, ACPI_UINT32_MAX, ACPI_OWNER_ID_MAX, acpi_gbl_root_node); acpi_db_set_output_destination(ACPI_DB_CONSOLE_OUTPUT); }

pub unsafe fn acpi_db_dump_namespace_by_owner(owner_arg: *mut i8, depth_arg: *mut i8) { let owner_id = strtoul(owner_arg, core::ptr::null_mut(), 0) as acpi_owner_id; let max_depth = if depth_arg.is_null() { ACPI_UINT32_MAX } else { strtoul(depth_arg, core::ptr::null_mut(), 0) as u32 }; acpi_db_set_output_destination(ACPI_DB_DUPLICATE_OUTPUT); acpi_os_printf(cstr!("ACPI Namespace by owner %X:\n"), owner_id); acpi_db_set_output_destination(ACPI_DB_REDIRECTABLE_OUTPUT); acpi_ns_dump_objects(ACPI_TYPE_ANY, ACPI_DISPLAY_SUMMARY, max_depth, owner_id, acpi_gbl_root_node); acpi_db_set_output_destination(ACPI_DB_CONSOLE_OUTPUT); }

pub unsafe fn acpi_db_find_name_in_namespace(name_arg: *mut i8) -> acpi_status { let mut name = *b"____\0"; if strlen(name_arg) > ACPI_NAMESEG_SIZE { acpi_os_printf(cstr!("Name must be no longer than 4 characters\n")); return AE_OK; } acpi_ut_strupr(name_arg); let mut i=0; while *name_arg != 0 { name[i]=*name_arg as u8; i+=1; name_arg=name_arg.add(1); } acpi_walk_namespace(ACPI_TYPE_ANY, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, Some(acpi_db_walk_and_match_name), core::ptr::null_mut(), name.as_mut_ptr() as *mut _, core::ptr::null_mut()); acpi_db_set_output_destination(ACPI_DB_CONSOLE_OUTPUT); AE_OK }

pub unsafe fn acpi_db_check_predefined_names() { let mut count=0u32; acpi_walk_namespace(ACPI_TYPE_ANY, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, Some(acpi_db_walk_for_predefined_names), core::ptr::null_mut(), &mut count as *mut _ as *mut _, core::ptr::null_mut()); acpi_os_printf(cstr!("Found %u predefined names in the namespace\n"), count); }

unsafe fn acpi_db_walk_for_predefined_names(_obj_handle: acpi_handle, _nesting_level:u32, _context:*mut core::ffi::c_void, _return_value:*mut *mut core::ffi::c_void)->acpi_status { AE_OK }

unsafe fn acpi_db_walk_for_object_counts(_obj_handle: acpi_handle, _nesting_level:u32, context:*mut core::ffi::c_void, _return_value:*mut *mut core::ffi::c_void)->acpi_status { let info=&mut *(context as *mut acpi_object_info); let node=&*(_obj_handle as *mut acpi_namespace_node); if node.type_ > ACPI_TYPE_NS_NODE_MAX { acpi_os_printf(cstr!("[%4.4s]: Unknown object type %X\n"), node.name.ascii.as_ptr(), node.type_); } else { info.types[node.type_ as usize]+=1; } AE_OK }

unsafe fn acpi_db_walk_for_specific_objects(obj_handle:acpi_handle, nesting_level:u32, context:*mut core::ffi::c_void, _return_value:*mut *mut core::ffi::c_void)->acpi_status { let info=&mut *(context as *mut acpi_walk_info); info.count+=1; let mut buffer=acpi_buffer{length:ACPI_ALLOCATE_LOCAL_BUFFER,pointer:core::ptr::null_mut()}; if ACPI_FAILURE(acpi_ns_handle_to_pathname(obj_handle,&mut buffer,TRUE)) { acpi_os_printf(cstr!("Could Not get pathname for object %p\n"),obj_handle); return AE_OK; } acpi_os_printf(cstr!("%32s"),buffer.pointer); ACPI_FREE(buffer.pointer); acpi_ns_dump_one_object(obj_handle,nesting_level,info,core::ptr::null_mut()); AE_OK }

pub unsafe fn acpi_db_display_objects(obj_type_arg:*mut i8, _display_count_arg:*mut i8)->acpi_status { if obj_type_arg.is_null() { let object_info=ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_object_info>()) as *mut acpi_object_info; if object_info.is_null(){return AE_NO_MEMORY;} acpi_walk_namespace(ACPI_TYPE_ANY,ACPI_ROOT_OBJECT,ACPI_UINT32_MAX,Some(acpi_db_walk_for_object_counts),core::ptr::null_mut(),object_info as *mut _,core::ptr::null_mut()); let mut total=0; for i in 0..ACPI_TOTAL_TYPES { acpi_os_printf(cstr!("%8u %s\n"),(*object_info).types[i],acpi_ut_get_type_name(i as u32)); total+=(*object_info).types[i]; } acpi_os_printf(cstr!("\n%8u Total namespace objects\n\n"),total); ACPI_FREE(object_info as *mut _); return AE_OK; } let typ=acpi_db_match_argument(obj_type_arg,ACPI_DB_OBJECT_TYPES.as_mut_ptr()); if typ==ACPI_TYPE_NOT_FOUND { acpi_os_printf(cstr!("Invalid or unsupported argument\n")); return AE_OK; } let mut info=acpi_walk_info{count:0,owner_id:ACPI_OWNER_ID_MAX,debug_level:ACPI_UINT32_MAX,display_type:ACPI_DISPLAY_SUMMARY|ACPI_DISPLAY_SHORT}; acpi_walk_namespace(typ,ACPI_ROOT_OBJECT,ACPI_UINT32_MAX,Some(acpi_db_walk_for_specific_objects),core::ptr::null_mut(),&mut info as *mut _ as *mut _,core::ptr::null_mut()); acpi_os_printf(cstr!("\nFound %u objects of type [%s] in the current ACPI Namespace\n"),info.count,acpi_ut_get_type_name(typ)); AE_OK }

pub unsafe fn acpi_db_display_fields(address_space_id:u32)->acpi_status { let mut info=acpi_region_walk_info{count:0,owner_id:ACPI_OWNER_ID_MAX,debug_level:ACPI_UINT32_MAX,display_type:ACPI_DISPLAY_SUMMARY|ACPI_DISPLAY_SHORT,address_space_id}; acpi_walk_namespace(ACPI_TYPE_LOCAL_REGION_FIELD,ACPI_ROOT_OBJECT,ACPI_UINT32_MAX,Some(acpi_db_walk_for_fields),core::ptr::null_mut(),&mut info as *mut _ as *mut _,core::ptr::null_mut()); AE_OK }
unsafe fn acpi_db_walk_for_fields(_a:acpi_handle,_b:u32,_c:*mut core::ffi::c_void,_d:*mut *mut core::ffi::c_void)->acpi_status { AE_OK }

pub unsafe fn acpi_db_check_integrity() { let mut info=acpi_integrity_info { nodes:0, objects:0 }; acpi_walk_namespace(ACPI_TYPE_ANY, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, Some(acpi_db_integrity_walk), core::ptr::null_mut(), &mut info as *mut _ as *mut _, core::ptr::null_mut()); acpi_os_printf(cstr!("Verified %u namespace nodes with %u Objects\n"), info.nodes, info.objects); }
unsafe fn acpi_db_integrity_walk(_a:acpi_handle,_b:u32,_c:*mut core::ffi::c_void,_d:*mut *mut core::ffi::c_void)->acpi_status { AE_OK }

pub unsafe fn acpi_db_find_references(object_arg:*mut i8) { let obj_desc=ACPI_TO_POINTER(strtoul(object_arg,core::ptr::null_mut(),16)); acpi_walk_namespace(ACPI_TYPE_ANY, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, Some(acpi_db_walk_for_references), core::ptr::null_mut(), obj_desc, core::ptr::null_mut()); }
unsafe fn acpi_db_walk_for_references(_a:acpi_handle,_b:u32,_c:*mut core::ffi::c_void,_d:*mut *mut core::ffi::c_void)->acpi_status { AE_OK }

pub unsafe fn acpi_db_get_bus_info() { acpi_walk_namespace(ACPI_TYPE_ANY, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, Some(acpi_db_bus_walk), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()); }
unsafe fn acpi_db_bus_walk(_a:acpi_handle,_b:u32,_c:*mut core::ffi::c_void,_d:*mut *mut core::ffi::c_void)->acpi_status { AE_OK }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
