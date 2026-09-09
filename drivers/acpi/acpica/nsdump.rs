// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Translation of nsdump.c. External ACPI types, constants, and routines are
// supplied by the surrounding ACPI implementation.

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_debugger"))]
pub unsafe fn acpi_ns_print_pathname(mut num_segments: u32, mut pathname: *const i8) {
    if !ACPI_IS_DEBUG_ENABLED(ACPI_LV_NAMES, ACPI_NAMESPACE) { return; }
    ACPI_DEBUG_PRINT((ACPI_DB_NAMES, "["));
    while num_segments != 0 {
        for i in 0..4 {
            let c = *pathname.add(i);
            if libc::isprint(c as i32) != 0 { acpi_os_printf!("%c", c); }
            else { acpi_os_printf!("?"); }
        }
        pathname = pathname.add(ACPI_NAMESEG_SIZE as usize);
        num_segments -= 1;
        if num_segments != 0 { acpi_os_printf!("."); }
    }
    acpi_os_printf!("]\n");
}

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_debugger"))]
pub unsafe extern "C" fn acpi_ns_dump_one_object(
    obj_handle: acpi_handle, level: u32, context: *mut core::ffi::c_void,
    _return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    let info = &mut *(context as *mut acpi_walk_info);
    if (acpi_dbg_level & info.debug_level) == 0 { return AE_OK; }
    if obj_handle.is_null() { ACPI_DEBUG_PRINT((ACPI_DB_INFO, "Null object handle\n")); return AE_OK; }
    let this_node = acpi_ns_validate_handle(obj_handle);
    if this_node.is_null() { ACPI_DEBUG_PRINT((ACPI_DB_INFO, "Invalid object handle %p\n", obj_handle)); return AE_OK; }
    let typ = (*this_node).type_;
    info.count += 1;
    if info.owner_id != ACPI_OWNER_ID_MAX && info.owner_id != (*this_node).owner_id { return AE_OK; }
    if (info.display_type & ACPI_DISPLAY_SHORT) == 0 {
        acpi_os_printf!("%2d%*s", level - 1, (level * 2) as i32, " ");
        if typ > ACPI_TYPE_LOCAL_MAX { ACPI_WARNING((AE_INFO, "Invalid ACPI Object Type 0x%08X", typ)); }
        acpi_os_printf!("%4.4s", acpi_ut_get_node_name(this_node));
    }
    acpi_os_printf!(" %-12s %p %3.3X ", acpi_ut_get_type_name(typ), this_node, (*this_node).owner_id);
    let saved = acpi_dbg_level; acpi_dbg_level = 0;
    let mut obj_desc = acpi_ns_get_attached_object(this_node); acpi_dbg_level = saved;
    if ((*this_node).flags & ANOBJ_TEMPORARY) != 0 { acpi_os_printf!("(T) "); }
    match info.display_type & ACPI_DISPLAY_MASK {
        ACPI_DISPLAY_SUMMARY => {
            if obj_desc.is_null() {
                match typ { ACPI_TYPE_INTEGER | ACPI_TYPE_PACKAGE | ACPI_TYPE_BUFFER | ACPI_TYPE_STRING | ACPI_TYPE_METHOD => acpi_os_printf!("<No attached object>"), _ => {} }
                acpi_os_printf!("\n"); return AE_OK;
            }
            match typ {
                ACPI_TYPE_PROCESSOR => acpi_os_printf!("ID %02X Len %02X Addr %8.8X%8.8X\n", (*obj_desc).processor.proc_id, (*obj_desc).processor.length, ACPI_FORMAT_UINT64((*obj_desc).processor.address)),
                ACPI_TYPE_DEVICE => acpi_os_printf!("Notify Object: %p\n", obj_desc),
                ACPI_TYPE_METHOD => acpi_os_printf!("Args %X Len %.4X Aml %p\n", (*obj_desc).method.param_count, (*obj_desc).method.aml_length, (*obj_desc).method.aml_start),
                ACPI_TYPE_INTEGER => acpi_os_printf!("= %8.8X%8.8X\n", ACPI_FORMAT_UINT64((*obj_desc).integer.value)),
                ACPI_TYPE_PACKAGE => if ((*obj_desc).common.flags & AOPOBJ_DATA_VALID) != 0 { acpi_os_printf!("Elements %.2X\n", (*obj_desc).package.count) } else { acpi_os_printf!("[Length not yet evaluated]\n") },
                ACPI_TYPE_BUFFER => { if ((*obj_desc).common.flags & AOPOBJ_DATA_VALID) != 0 { acpi_os_printf!("Len %.2X", (*obj_desc).buffer.length); if (*obj_desc).buffer.length > 0 { acpi_os_printf!(" ="); for i in 0..core::cmp::min((*obj_desc).buffer.length, 12) { acpi_os_printf!(" %2.2X", *(*obj_desc).buffer.pointer.add(i as usize)); } } acpi_os_printf!("\n"); } else { acpi_os_printf!("[Length not yet evaluated]\n"); } },
                ACPI_TYPE_STRING => { acpi_os_printf!("Len %.2X ", (*obj_desc).string.length); acpi_ut_print_string((*obj_desc).string.pointer, 80); acpi_os_printf!("\n"); },
                ACPI_TYPE_REGION => { acpi_os_printf!("[%s]", acpi_ut_get_region_name((*obj_desc).region.space_id)); if ((*obj_desc).region.flags & AOPOBJ_DATA_VALID) != 0 { acpi_os_printf!(" Addr %8.8X%8.8X Len %.4X\n", ACPI_FORMAT_UINT64((*obj_desc).region.address), (*obj_desc).region.length); } else { acpi_os_printf!(" [Address/Length not yet evaluated]\n"); } },
                ACPI_TYPE_LOCAL_REFERENCE => acpi_os_printf!("[%s]\n", acpi_ut_get_reference_name(obj_desc)),
                ACPI_TYPE_LOCAL_ALIAS | ACPI_TYPE_LOCAL_METHOD_ALIAS => acpi_os_printf!("Target %4.4s (%p)\n", acpi_ut_get_node_name(obj_desc), obj_desc),
                _ => acpi_os_printf!("Object %p\n", obj_desc),
            }
            match typ { ACPI_TYPE_BUFFER_FIELD | ACPI_TYPE_LOCAL_REGION_FIELD | ACPI_TYPE_LOCAL_BANK_FIELD | ACPI_TYPE_LOCAL_INDEX_FIELD => acpi_os_printf!(" Off %.3X Len %.2X Acc %.2X\n", (*obj_desc).common_field.base_byte_offset * 8 + (*obj_desc).common_field.start_field_bit_offset, (*obj_desc).common_field.bit_length, (*obj_desc).common_field.access_byte_width), _ => {} }
        }
        ACPI_DISPLAY_OBJECTS => { acpi_os_printf!("O:%p", obj_desc); if obj_desc.is_null() { acpi_os_printf!("\n"); return AE_OK; } acpi_os_printf!("(R%u)", (*obj_desc).common.reference_count); match typ { ACPI_TYPE_METHOD => acpi_os_printf!(" M:%p-%X\n", (*obj_desc).method.aml_start, (*obj_desc).method.aml_length), ACPI_TYPE_INTEGER => acpi_os_printf!(" I:%8.8X8.8%X\n", ACPI_FORMAT_UINT64((*obj_desc).integer.value)), ACPI_TYPE_STRING => acpi_os_printf!(" S:%p-%X\n", (*obj_desc).string.pointer, (*obj_desc).string.length), ACPI_TYPE_BUFFER => acpi_os_printf!(" B:%p-%X\n", (*obj_desc).buffer.pointer, (*obj_desc).buffer.length), _ => acpi_os_printf!("\n") } }
        _ => acpi_os_printf!("\n"),
    }
    if (acpi_dbg_level & ACPI_LV_VALUES) == 0 { return AE_OK; }
    let saved = acpi_dbg_level; acpi_dbg_level = 0; obj_desc = acpi_ns_get_attached_object(this_node); acpi_dbg_level = saved;
    while !obj_desc.is_null() {
        let mut obj_type = ACPI_TYPE_INVALID; acpi_os_printf!("Attached Object %p: ", obj_desc);
        match ACPI_GET_DESCRIPTOR_TYPE(obj_desc) { ACPI_DESC_TYPE_NAMED => { acpi_os_printf!("(Ptr to Node)\n"); ACPI_DUMP_BUFFER(obj_desc, core::mem::size_of::<acpi_namespace_node>() as u32); }, ACPI_DESC_TYPE_OPERAND => { obj_type = (*obj_desc).common.type_; acpi_os_printf!("(Pointer to ACPI Object type %.2X [%s])\n", obj_type, acpi_ut_get_type_name(obj_type)); ACPI_DUMP_BUFFER(obj_desc, core::mem::size_of::<acpi_operand_object>() as u32); }, _ => {} }
        if ACPI_GET_DESCRIPTOR_TYPE(obj_desc) != ACPI_DESC_TYPE_OPERAND { break; }
        match obj_type { ACPI_TYPE_BUFFER | ACPI_TYPE_STRING => { let n = (*obj_desc).string.length; obj_desc = (*obj_desc).string.pointer as *mut acpi_operand_object; acpi_os_printf!("(Buffer/String pointer %p length %X)\n", obj_desc, n); ACPI_DUMP_BUFFER(obj_desc, n); break; }, ACPI_TYPE_BUFFER_FIELD => obj_desc = (*obj_desc).buffer_field.buffer_obj, ACPI_TYPE_PACKAGE => obj_desc = (*obj_desc).package.elements as *mut acpi_operand_object, ACPI_TYPE_METHOD => obj_desc = (*obj_desc).method.aml_start as *mut acpi_operand_object, ACPI_TYPE_LOCAL_REGION_FIELD => obj_desc = (*obj_desc).field.region_obj, ACPI_TYPE_LOCAL_BANK_FIELD => obj_desc = (*obj_desc).bank_field.region_obj, ACPI_TYPE_LOCAL_INDEX_FIELD => obj_desc = (*obj_desc).index_field.index_obj, _ => break }
    }
    acpi_os_printf!("\n"); AE_OK
}

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_debugger"))]
pub unsafe fn acpi_ns_dump_objects(typ: acpi_object_type, display_type: u8, max_depth: u32, owner_id: acpi_owner_id, start_handle: acpi_handle) { let mut info = acpi_walk_info { count: 0, debug_level: ACPI_LV_TABLES, owner_id, display_type, ..core::mem::zeroed() }; let status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); if ACPI_FAILURE(status) { acpi_os_printf!("Could not acquire namespace mutex\n"); return; } acpi_ns_walk_namespace(typ, start_handle, max_depth, ACPI_NS_WALK_NO_UNLOCK | ACPI_NS_WALK_TEMP_NODES, Some(acpi_ns_dump_one_object), core::ptr::null_mut(), &mut info as *mut _ as *mut _, core::ptr::null_mut()); acpi_os_printf!("\nNamespace node count: %u\n\n", info.count); acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); }

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_debugger"))]
pub unsafe fn acpi_ns_dump_entry(handle: acpi_handle, debug_level: u32) { let mut info: acpi_walk_info = core::mem::zeroed(); info.debug_level = debug_level; info.owner_id = ACPI_OWNER_ID_MAX; info.display_type = ACPI_DISPLAY_SUMMARY; acpi_ns_dump_one_object(handle, 1, &mut info as *mut _ as *mut _, core::ptr::null_mut()); }

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_debugger"))]
unsafe extern "C" fn acpi_ns_get_max_depth(_handle: acpi_handle, level: u32, context: *mut core::ffi::c_void, _ret: *mut *mut core::ffi::c_void) -> acpi_status { let max = &mut *(context as *mut u32); if level > *max { *max = level; } AE_OK }

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_debugger"))]
unsafe extern "C" fn acpi_ns_dump_one_object_path(handle: acpi_handle, level: u32, context: *mut core::ffi::c_void, _ret: *mut *mut core::ffi::c_void) -> acpi_status {
    if handle.is_null() { return AE_OK; } let node = acpi_ns_validate_handle(handle); if node.is_null() { return AE_OK; }
    let max = *(context as *mut u32); let indent = if level <= max { max - level + 1 } else { 1 };
    let path = acpi_ns_get_normalized_pathname(node, 1); acpi_os_printf!("%2d%*s%-12s%*s", level, level, " ", acpi_ut_get_type_name((*node).type_), indent, " "); acpi_os_printf!("%s\n", path.add(1)); ACPI_FREE(path); AE_OK
}

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_debugger"))]
pub unsafe fn acpi_ns_dump_object_paths(typ: acpi_object_type, _display_type: u8, max_depth: u32, _owner_id: acpi_owner_id, start_handle: acpi_handle) { let mut max = 0u32; let status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); if ACPI_FAILURE(status) { acpi_os_printf!("Could not acquire namespace mutex\n"); return; } acpi_ns_walk_namespace(typ, start_handle, max_depth, ACPI_NS_WALK_NO_UNLOCK | ACPI_NS_WALK_TEMP_NODES, Some(acpi_ns_get_max_depth), core::ptr::null_mut(), &mut max as *mut _ as *mut _, core::ptr::null_mut()); acpi_ns_walk_namespace(typ, start_handle, max_depth, ACPI_NS_WALK_NO_UNLOCK | ACPI_NS_WALK_TEMP_NODES, Some(acpi_ns_dump_one_object_path), core::ptr::null_mut(), &mut max as *mut _ as *mut _, core::ptr::null_mut()); acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); }

#[cfg(feature = "acpi_obsolete_functions")]
pub unsafe fn acpi_ns_dump_pathname(handle: acpi_handle, msg: *const i8, level: u32, component: u32) { if !ACPI_IS_DEBUG_ENABLED(level, component) { return; } acpi_ns_print_node_pathname(handle, msg); acpi_os_printf!("\n"); }

#[cfg(feature = "acpi_asl_compiler")]
pub unsafe fn acpi_ns_dump_tables(search_base: acpi_handle, max_depth: u32) { if acpi_gbl_root_node.is_null() { ACPI_DEBUG_PRINT((ACPI_DB_TABLES, "namespace not initialized!\n")); return; } let search = if ACPI_NS_ALL == search_base { ACPI_DEBUG_PRINT((ACPI_DB_TABLES, "\\\n")); acpi_gbl_root_node as acpi_handle } else { search_base }; acpi_ns_dump_objects(ACPI_TYPE_ANY, ACPI_DISPLAY_OBJECTS, max_depth, ACPI_OWNER_ID_MAX, search); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
