// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Table manager data structure functions. Direct translation of tbdata.c.

// Dependencies supplied by the ACPI translation unit are intentionally left external.

unsafe fn acpi_tb_compare_tables(table_desc: *mut acpi_table_desc, table_index: u32) -> u8 {
    let mut table: *mut acpi_table_header = core::ptr::null_mut();
    let mut table_length: u32 = 0;
    let mut table_flags: u8 = 0;
    let status = acpi_tb_acquire_table(&mut (*acpi_gbl_root_table_list.tables.add(table_index as usize)), &mut table, &mut table_length, &mut table_flags);
    if ACPI_FAILURE(status) { return FALSE; }
    let identical = if (*table_desc).length != table_length || libc_memcmp((*table_desc).pointer as *const _, table as *const _, table_length as usize) != 0 { FALSE } else { TRUE };
    acpi_tb_release_table(table, table_length, table_flags);
    identical
}

pub unsafe fn acpi_tb_init_table_descriptor(table_desc: *mut acpi_table_desc, address: acpi_physical_address, flags: u8, table: *mut acpi_table_header) {
    libc_memset(table_desc as *mut _, 0, core::mem::size_of::<acpi_table_desc>());
    (*table_desc).address = address;
    (*table_desc).length = (*table).length;
    (*table_desc).flags = flags;
    ACPI_MOVE_32_TO_32((*table_desc).signature.ascii.as_mut_ptr(), (*table).signature.as_ptr());
    match flags & ACPI_TABLE_ORIGIN_MASK {
        ACPI_TABLE_ORIGIN_INTERNAL_VIRTUAL | ACPI_TABLE_ORIGIN_EXTERNAL_VIRTUAL => (*table_desc).pointer = table,
        ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL | _ => {}
    }
}

pub unsafe fn acpi_tb_acquire_table(table_desc: *mut acpi_table_desc, table_ptr: *mut *mut acpi_table_header, table_length: *mut u32, table_flags: *mut u8) -> acpi_status {
    let table = match (*table_desc).flags & ACPI_TABLE_ORIGIN_MASK {
        ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL => acpi_os_map_memory((*table_desc).address, (*table_desc).length),
        ACPI_TABLE_ORIGIN_INTERNAL_VIRTUAL | ACPI_TABLE_ORIGIN_EXTERNAL_VIRTUAL => (*table_desc).pointer,
        _ => core::ptr::null_mut()
    };
    if table.is_null() { return AE_NO_MEMORY; }
    *table_ptr = table; *table_length = (*table_desc).length; *table_flags = (*table_desc).flags; AE_OK
}

pub unsafe fn acpi_tb_release_table(table: *mut acpi_table_header, table_length: u32, table_flags: u8) {
    if table_flags & ACPI_TABLE_ORIGIN_MASK == ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL { acpi_os_unmap_memory(table, table_length); }
}

pub unsafe fn acpi_tb_acquire_temp_table(table_desc: *mut acpi_table_desc, address: acpi_physical_address, flags: u8, mut table: *mut acpi_table_header) -> acpi_status {
    let mut mapped = FALSE;
    match flags & ACPI_TABLE_ORIGIN_MASK {
        ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL => { if table.is_null() { table = acpi_os_map_memory(address, core::mem::size_of::<acpi_table_header>() as u32); if table.is_null() { return AE_NO_MEMORY; } mapped = TRUE; } }
        ACPI_TABLE_ORIGIN_INTERNAL_VIRTUAL | ACPI_TABLE_ORIGIN_EXTERNAL_VIRTUAL => { if table.is_null() { return AE_BAD_PARAMETER; } }
        _ => return AE_NO_MEMORY
    }
    acpi_tb_init_table_descriptor(table_desc, address, flags, table);
    if mapped != FALSE { acpi_os_unmap_memory(table, core::mem::size_of::<acpi_table_header>() as u32); }
    AE_OK
}

pub unsafe fn acpi_tb_release_temp_table(table_desc: *mut acpi_table_desc) { acpi_tb_invalidate_table(table_desc); }

pub unsafe fn acpi_tb_validate_table(table_desc: *mut acpi_table_desc) -> acpi_status {
    let mut status = AE_OK;
    ACPI_FUNCTION_TRACE!(tb_validate_table);
    if (*table_desc).pointer.is_null() { status = acpi_tb_acquire_table(table_desc, &mut (*table_desc).pointer, &mut (*table_desc).length, &mut (*table_desc).flags); if (*table_desc).pointer.is_null() { status = AE_NO_MEMORY; } }
    status
}

pub unsafe fn acpi_tb_invalidate_table(table_desc: *mut acpi_table_desc) {
    ACPI_FUNCTION_TRACE!(tb_invalidate_table);
    if (*table_desc).pointer.is_null() { return; }
    acpi_tb_release_table((*table_desc).pointer, (*table_desc).length, (*table_desc).flags);
    if (*table_desc).flags & ACPI_TABLE_ORIGIN_MASK == ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL { (*table_desc).pointer = core::ptr::null_mut(); }
}

pub unsafe fn acpi_tb_validate_temp_table(table_desc: *mut acpi_table_desc) -> acpi_status {
    if (*table_desc).pointer.is_null() && !acpi_gbl_enable_table_validation { (*table_desc).length = core::mem::size_of::<acpi_table_header>() as u32; }
    acpi_tb_validate_table(table_desc)
}

unsafe fn acpi_tb_check_duplication(table_desc: *mut acpi_table_desc, table_index: *mut u32) -> acpi_status {
    ACPI_FUNCTION_TRACE!(tb_check_duplication);
    for i in 0..acpi_gbl_root_table_list.current_table_count { let d = &*acpi_gbl_root_table_list.tables.add(i as usize); if d.flags & ACPI_TABLE_IS_VERIFIED == 0 || acpi_tb_compare_tables(table_desc, i) == FALSE { continue; } if d.flags & ACPI_TABLE_IS_LOADED != 0 { return AE_ALREADY_EXISTS; } *table_index = i; return AE_CTRL_TERMINATE; }
    AE_OK
}

pub unsafe fn acpi_tb_verify_temp_table(table_desc: *mut acpi_table_desc, signature: *mut i8, table_index: *mut u32) -> acpi_status {
    ACPI_FUNCTION_TRACE!(tb_verify_temp_table);
    let mut status = acpi_tb_validate_temp_table(table_desc); if ACPI_FAILURE(status) { return AE_NO_MEMORY; }
    if !signature.is_null() && !ACPI_COMPARE_NAMESEG!(&(*table_desc).signature, signature) { status = AE_BAD_SIGNATURE; acpi_tb_invalidate_table(table_desc); return status; }
    if acpi_gbl_enable_table_validation { status = acpi_ut_verify_checksum((*table_desc).pointer, (*table_desc).length); if ACPI_FAILURE(status) { acpi_tb_invalidate_table(table_desc); return status; } if !table_index.is_null() { status = acpi_tb_check_duplication(table_desc, table_index); if ACPI_FAILURE(status) { acpi_tb_invalidate_table(table_desc); return status; } } (*table_desc).flags |= ACPI_TABLE_IS_VERIFIED; }
    status
}

pub unsafe fn acpi_tb_resize_root_table_list() -> acpi_status {
    ACPI_FUNCTION_TRACE!(tb_resize_root_table_list);
    if acpi_gbl_root_table_list.flags & ACPI_ROOT_ALLOW_RESIZE == 0 { return AE_SUPPORT; }
    let count = if acpi_gbl_root_table_list.flags & ACPI_ROOT_ORIGIN_ALLOCATED != 0 { acpi_gbl_root_table_list.max_table_count } else { acpi_gbl_root_table_list.current_table_count };
    let max = count + ACPI_ROOT_TABLE_SIZE_INCREMENT;
    let tables = ACPI_ALLOCATE_ZEROED!((max as usize) * core::mem::size_of::<acpi_table_desc>()) as *mut acpi_table_desc; if tables.is_null() { return AE_NO_MEMORY; }
    let mut current = 0; if !acpi_gbl_root_table_list.tables.is_null() { for i in 0..count { if (*acpi_gbl_root_table_list.tables.add(i as usize)).address != 0 { core::ptr::copy_nonoverlapping(acpi_gbl_root_table_list.tables.add(i as usize), tables.add(current as usize), 1); current += 1; } } if acpi_gbl_root_table_list.flags & ACPI_ROOT_ORIGIN_ALLOCATED != 0 { ACPI_FREE!(acpi_gbl_root_table_list.tables); } }
    acpi_gbl_root_table_list.tables = tables; acpi_gbl_root_table_list.max_table_count = max; acpi_gbl_root_table_list.current_table_count = current; acpi_gbl_root_table_list.flags |= ACPI_ROOT_ORIGIN_ALLOCATED; AE_OK
}

pub unsafe fn acpi_tb_get_next_table_descriptor(table_index: *mut u32, table_desc: *mut *mut acpi_table_desc) -> acpi_status {
    if acpi_gbl_root_table_list.current_table_count >= acpi_gbl_root_table_list.max_table_count { let s = acpi_tb_resize_root_table_list(); if ACPI_FAILURE(s) { return s; } }
    let i = acpi_gbl_root_table_list.current_table_count; acpi_gbl_root_table_list.current_table_count += 1; if !table_index.is_null() { *table_index = i; } if !table_desc.is_null() { *table_desc = acpi_gbl_root_table_list.tables.add(i as usize); } AE_OK
}

pub unsafe fn acpi_tb_terminate() { ACPI_FUNCTION_TRACE!(tb_terminate); acpi_ut_acquire_mutex(ACPI_MTX_TABLES); for i in 0..acpi_gbl_root_table_list.current_table_count { acpi_tb_uninstall_table(acpi_gbl_root_table_list.tables.add(i as usize)); } if acpi_gbl_root_table_list.flags & ACPI_ROOT_ORIGIN_ALLOCATED != 0 { ACPI_FREE!(acpi_gbl_root_table_list.tables); } acpi_gbl_root_table_list.tables = core::ptr::null_mut(); acpi_gbl_root_table_list.flags = 0; acpi_gbl_root_table_list.current_table_count = 0; acpi_ut_release_mutex(ACPI_MTX_TABLES); }

pub unsafe fn acpi_tb_delete_namespace_by_owner(table_index: u32) -> acpi_status { let s = acpi_ut_acquire_mutex(ACPI_MTX_TABLES); if ACPI_FAILURE(s) { return s; } if table_index >= acpi_gbl_root_table_list.current_table_count { acpi_ut_release_mutex(ACPI_MTX_TABLES); return AE_NOT_EXIST; } let owner = (*acpi_gbl_root_table_list.tables.add(table_index as usize)).owner_id; acpi_ut_release_mutex(ACPI_MTX_TABLES); let s = acpi_ut_acquire_write_lock(&mut acpi_gbl_namespace_rw_lock); if ACPI_FAILURE(s) { return s; } acpi_ns_delete_namespace_by_owner(owner); acpi_ut_release_write_lock(&mut acpi_gbl_namespace_rw_lock); s }

pub unsafe fn acpi_tb_allocate_owner_id(i: u32) -> acpi_status { let mut s = AE_BAD_PARAMETER; acpi_ut_acquire_mutex(ACPI_MTX_TABLES); if i < acpi_gbl_root_table_list.current_table_count { s = acpi_ut_allocate_owner_id(&mut (*acpi_gbl_root_table_list.tables.add(i as usize)).owner_id); } acpi_ut_release_mutex(ACPI_MTX_TABLES); s }
pub unsafe fn acpi_tb_release_owner_id(i: u32) -> acpi_status { let mut s = AE_BAD_PARAMETER; acpi_ut_acquire_mutex(ACPI_MTX_TABLES); if i < acpi_gbl_root_table_list.current_table_count { acpi_ut_release_owner_id(&mut (*acpi_gbl_root_table_list.tables.add(i as usize)).owner_id); s = AE_OK; } acpi_ut_release_mutex(ACPI_MTX_TABLES); s }
pub unsafe fn acpi_tb_get_owner_id(i: u32, owner: *mut acpi_owner_id) -> acpi_status { let mut s = AE_BAD_PARAMETER; acpi_ut_acquire_mutex(ACPI_MTX_TABLES); if i < acpi_gbl_root_table_list.current_table_count { *owner = (*acpi_gbl_root_table_list.tables.add(i as usize)).owner_id; s = AE_OK; } acpi_ut_release_mutex(ACPI_MTX_TABLES); s }
pub unsafe fn acpi_tb_is_table_loaded(i: u32) -> u8 { let mut v=FALSE; acpi_ut_acquire_mutex(ACPI_MTX_TABLES); if i < acpi_gbl_root_table_list.current_table_count { v = ((*acpi_gbl_root_table_list.tables.add(i as usize)).flags & ACPI_TABLE_IS_LOADED) as u8; } acpi_ut_release_mutex(ACPI_MTX_TABLES); v }
pub unsafe fn acpi_tb_set_table_loaded_flag(i: u32, loaded: u8) { acpi_ut_acquire_mutex(ACPI_MTX_TABLES); if i < acpi_gbl_root_table_list.current_table_count { if loaded != FALSE { (*acpi_gbl_root_table_list.tables.add(i as usize)).flags |= ACPI_TABLE_IS_LOADED; } else { (*acpi_gbl_root_table_list.tables.add(i as usize)).flags &= !ACPI_TABLE_IS_LOADED; } } acpi_ut_release_mutex(ACPI_MTX_TABLES); }

pub unsafe fn acpi_tb_load_table(i: u32, parent: *mut acpi_namespace_node) -> acpi_status { let mut table=core::ptr::null_mut(); let mut s=acpi_get_table_by_index(i,&mut table); if ACPI_FAILURE(s){return s;} s=acpi_ns_load_table(i,parent); if ACPI_FAILURE(s){return s;} let mut owner=0; if ACPI_SUCCESS(acpi_tb_get_owner_id(i,&mut owner)){acpi_ev_update_gpes(owner);} acpi_tb_notify_table(ACPI_TABLE_EVENT_LOAD,table); s }
pub unsafe fn acpi_tb_install_and_load_table(address: acpi_physical_address, flags:u8, table:*mut acpi_table_header, override_:u8, index:*mut u32)->acpi_status { let mut i=0; let mut s=acpi_tb_install_standard_table(address,flags,table,TRUE,override_,&mut i); if ACPI_SUCCESS(s){s=acpi_tb_load_table(i,acpi_gbl_root_node);} *index=i;s }
pub unsafe fn acpi_tb_unload_table(i:u32)->acpi_status { if acpi_tb_is_table_loaded(i)==FALSE{return AE_NOT_EXIST;} let mut table=core::ptr::null_mut(); let mut s=acpi_get_table_by_index(i,&mut table); if ACPI_SUCCESS(s){acpi_tb_notify_table(ACPI_TABLE_EVENT_UNLOAD,table);} s=acpi_tb_delete_namespace_by_owner(i); if ACPI_FAILURE(s){return s;} acpi_tb_release_owner_id(i);acpi_tb_set_table_loaded_flag(i,FALSE);s }
pub unsafe fn acpi_tb_notify_table(event:u32, table:*mut core::ffi::c_void) { if !acpi_gbl_table_handler.is_none() { ((*acpi_gbl_table_handler.unwrap())(event,table,acpi_gbl_table_handler_context)); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
