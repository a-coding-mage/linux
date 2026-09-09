// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: dbexec - debugger control method execution

// C dependencies are supplied by the surrounding ACPICA translation.

static mut ACPI_GBL_DB_METHOD_INFO: acpi_db_method_info = unsafe { core::mem::zeroed() };

unsafe fn acpi_db_delete_objects(count: u32, objects: *mut acpi_object) {
    for i in 0..count {
        let object = &mut *objects.add(i as usize);
        match object.type_ {
            ACPI_TYPE_BUFFER => {
                ACPI_FREE(object.buffer.pointer);
            }
            ACPI_TYPE_PACKAGE => {
                acpi_db_delete_objects(object.package.count, object.package.elements);
                ACPI_FREE(object.package.elements);
            }
            _ => {}
        }
    }
}

unsafe fn acpi_db_execute_method(
    info: *mut acpi_db_method_info,
    return_obj: *mut acpi_buffer,
) -> acpi_status {
    let mut status: acpi_status;
    let mut param_objects = acpi_object_list { count: 0, pointer: core::ptr::null_mut() };
    let mut params: [acpi_object; ACPI_DEBUGGER_MAX_ARGS as usize + 1] =
        core::mem::zeroed();
    let mut i: u32 = 0;

    if acpi_gbl_db_output_to_file && !acpi_dbg_level {
        acpi_os_printf(c"Warning: debug output is not enabled!\n".as_ptr());
    }

    if !(*info).args.is_null() && !*(*info).args {
        while !(*(*info).args.add(i as usize)).is_null()
            && **(*info).args.add(i as usize) != 0
        {
            status = acpi_db_convert_to_object(
                *(*info).types.add(i as usize),
                *(*info).args.add(i as usize),
                &mut params[i as usize],
            );
            if ACPI_FAILURE(status) {
                ACPI_EXCEPTION(status, c"While parsing method arguments".as_ptr());
                return status;
            }
            i += 1;
        }
        param_objects.count = i;
        param_objects.pointer = params.as_mut_ptr();
    }

    (*return_obj).pointer = acpi_gbl_db_buffer;
    (*return_obj).length = ACPI_DEBUG_BUFFER_SIZE;
    acpi_gbl_method_executing = TRUE;
    status = acpi_evaluate_object(
        core::ptr::null_mut(), (*info).pathname, &mut param_objects, return_obj,
    );
    acpi_gbl_cm_single_step = FALSE;
    acpi_gbl_method_executing = FALSE;

    if ACPI_FAILURE(status) {
        if status == AE_ABORT_METHOD || acpi_gbl_abort_method {
            ACPI_EXCEPTION(status, c"Aborting top-level method".as_ptr());
            acpi_gbl_abort_method = FALSE;
            status = AE_OK;
        } else {
            ACPI_EXCEPTION(status, (*info).pathname);
            if status == AE_BUFFER_OVERFLOW {
                ACPI_ERROR(status, c"Possible buffer overflow within AML Debugger buffer".as_ptr());
            }
        }
    }
    acpi_db_delete_objects(param_objects.count, params.as_mut_ptr());
    status
}

unsafe fn acpi_db_execute_setup(info: *mut acpi_db_method_info) -> acpi_status {
    (*info).pathname[0] = 0;
    if (*info).name[0] != b'\\' as i8 && (*info).name[0] != b'/' as i8 {
        if acpi_ut_safe_strcat((*info).pathname.as_mut_ptr(), (*info).pathname.len(), acpi_gbl_db_scope_buf) {
            ACPI_EXCEPTION(AE_BUFFER_OVERFLOW, c"During setup for method execution".as_ptr());
            return AE_BUFFER_OVERFLOW;
        }
    }
    if acpi_ut_safe_strcat((*info).pathname.as_mut_ptr(), (*info).pathname.len(), (*info).name) {
        ACPI_EXCEPTION(AE_BUFFER_OVERFLOW, c"During setup for method execution".as_ptr());
        return AE_BUFFER_OVERFLOW;
    }
    acpi_db_prep_namestring((*info).pathname.as_mut_ptr());
    acpi_db_set_output_destination(ACPI_DB_DUPLICATE_OUTPUT);
    acpi_os_printf(c"Evaluating %s\n".as_ptr(), (*info).pathname.as_ptr());
    if (*info).flags & EX_SINGLE_STEP != 0 {
        acpi_gbl_cm_single_step = TRUE;
        acpi_db_set_output_destination(ACPI_DB_CONSOLE_OUTPUT);
    } else {
        acpi_db_set_output_destination(ACPI_DB_REDIRECTABLE_OUTPUT);
    }
    AE_OK
}

#[cfg(ACPI_DBG_TRACK_ALLOCATIONS)]
unsafe fn acpi_db_get_cache_info(cache: *mut acpi_memory_list) -> u32 {
    (*cache).total_allocated - (*cache).total_freed - (*cache).current_depth
}

unsafe fn acpi_db_get_outstanding_allocations() -> u32 {
    let mut outstanding = 0;
    #[cfg(ACPI_DBG_TRACK_ALLOCATIONS)]
    {
        outstanding += acpi_db_get_cache_info(acpi_gbl_state_cache);
        outstanding += acpi_db_get_cache_info(acpi_gbl_ps_node_cache);
        outstanding += acpi_db_get_cache_info(acpi_gbl_ps_node_ext_cache);
        outstanding += acpi_db_get_cache_info(acpi_gbl_operand_cache);
    }
    outstanding
}

unsafe fn acpi_db_execution_walk(
    obj_handle: acpi_handle,
    _nesting_level: u32,
    _context: *mut core::ffi::c_void,
    _return_value: *mut *mut core::ffi::c_void,
) -> acpi_status {
    let node = obj_handle as *mut acpi_namespace_node;
    let obj_desc = acpi_ns_get_attached_object(node);
    if (*obj_desc).method.param_count != 0 { return AE_OK; }
    let mut return_obj = acpi_buffer { pointer: core::ptr::null_mut(), length: ACPI_ALLOCATE_BUFFER };
    acpi_ns_print_node_pathname(node, c"Evaluating".as_ptr());
    acpi_os_printf(c"\n".as_ptr());
    acpi_gbl_method_executing = TRUE;
    let status = acpi_evaluate_object(node, core::ptr::null(), core::ptr::null(), &mut return_obj);
    acpi_gbl_method_executing = FALSE;
    acpi_os_printf(c"Evaluation of [%4.4s] returned %s\n".as_ptr(), acpi_ut_get_node_name(node), acpi_format_exception(status));
    AE_OK
}

unsafe fn acpi_db_execute(name: *mut i8, args: *mut *mut i8, types: *mut acpi_object_type, flags: u32) {
    if acpi_gbl_method_executing { acpi_os_printf(c"Only one debugger execution is allowed.\n".as_ptr()); return; }
    if *name == b'*' as i8 {
        let _ = acpi_walk_namespace(ACPI_TYPE_METHOD, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, acpi_db_execution_walk, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
        return;
    }
    if flags & EX_ALL != 0 && acpi_strlen(name) > 4 { acpi_os_printf(c"Input name (%s) must be a 4-char NameSeg\n".as_ptr(), name); return; }
    let name_string = ACPI_ALLOCATE(acpi_strlen(name) + 1);
    if name_string.is_null() { return; }
    core::ptr::write_bytes(&mut ACPI_GBL_DB_METHOD_INFO as *mut _ as *mut u8, 0, core::mem::size_of::<acpi_db_method_info>());
    acpi_strcpy(name_string, name); acpi_ut_strupr(name_string);
    if acpi_strncmp(name_string, c"PREDEF".as_ptr(), 6) == 0 { acpi_db_evaluate_predefined_names(); ACPI_FREE(name_string); return; }
    (*&mut ACPI_GBL_DB_METHOD_INFO).name = name_string;
    (*&mut ACPI_GBL_DB_METHOD_INFO).args = args;
    (*&mut ACPI_GBL_DB_METHOD_INFO).types = types;
    (*&mut ACPI_GBL_DB_METHOD_INFO).flags = flags;
    if acpi_db_execute_setup(&mut ACPI_GBL_DB_METHOD_INFO) == AE_OK {
        let status = acpi_get_handle(core::ptr::null_mut(), ACPI_GBL_DB_METHOD_INFO.pathname.as_mut_ptr(), &mut ACPI_GBL_DB_METHOD_INFO.method);
        if ACPI_SUCCESS(status) { let mut ret = acpi_buffer { pointer: core::ptr::null_mut(), length: ACPI_ALLOCATE_BUFFER }; let _ = acpi_db_execute_method(&mut ACPI_GBL_DB_METHOD_INFO, &mut ret); }
    }
    ACPI_FREE(name_string);
}

unsafe extern "C" fn acpi_db_single_execution_thread(context: *mut core::ffi::c_void) {
    let info = context as *mut acpi_db_method_info;
    let mut return_obj: acpi_buffer = core::mem::zeroed();
    acpi_os_printf(c"\n".as_ptr());
    let status = acpi_db_execute_method(info, &mut return_obj);
    if ACPI_FAILURE(status) {
        acpi_os_printf(c"%s During evaluation of %s\n".as_ptr(), acpi_format_exception(status), (*info).pathname.as_ptr());
        return;
    }
    if return_obj.length != 0 {
        acpi_os_printf(c"Evaluation of %s returned object %p, external buffer length %X\n".as_ptr(), ACPI_GBL_DB_METHOD_INFO.pathname.as_ptr(), return_obj.pointer, return_obj.length);
        acpi_db_dump_external_object(return_obj.pointer, 1);
    }
    acpi_os_printf(c"\nBackground thread completed\n%c ".as_ptr(), ACPI_DEBUGGER_COMMAND_PROMPT);
}

unsafe extern "C" fn acpi_db_method_thread(context: *mut core::ffi::c_void) {
    let info = context as *mut acpi_db_method_info;
    let mut return_obj: acpi_buffer = core::mem::zeroed();
    let _ = acpi_os_wait_semaphore((*info).info_gate, 1, ACPI_WAIT_FOREVER);
    if (*info).init_args != 0 {
        acpi_db_uint32_to_hex_string((*info).num_created, (*info).index_of_thread_str.as_mut_ptr());
        acpi_db_uint32_to_hex_string(acpi_os_get_thread_id() as u32, (*info).id_of_thread_str.as_mut_ptr());
    }
    if !(*info).threads.is_null() && (*info).num_created < (*info).num_threads {
        *(*info).threads.add((*info).num_created as usize) = acpi_os_get_thread_id(); (*info).num_created += 1;
    }
    let mut local_info = *info;
    local_info.args = local_info.arguments.as_mut_ptr();
    local_info.arguments[0] = local_info.num_threads_str.as_mut_ptr(); local_info.arguments[1] = local_info.id_of_thread_str.as_mut_ptr(); local_info.arguments[2] = local_info.index_of_thread_str.as_mut_ptr(); local_info.arguments[3] = core::ptr::null_mut();
    local_info.types = local_info.arg_types.as_mut_ptr();
    let _ = acpi_os_signal_semaphore((*info).info_gate, 1);
    for i in 0..(*info).num_loops { let status = acpi_db_execute_method(&mut local_info, &mut return_obj); if ACPI_FAILURE(status) && status == AE_ABORT_METHOD { break; } let _ = i; }
    let _ = acpi_os_wait_semaphore((*info).thread_complete_gate, 1, ACPI_WAIT_FOREVER); (*info).num_completed += 1;
    let allow = (*info).num_completed == (*info).num_threads; let _ = acpi_os_signal_semaphore((*info).thread_complete_gate, 1);
    if allow { let _ = acpi_os_signal_semaphore((*info).main_thread_gate, 1); }
}

pub unsafe fn acpi_db_create_execution_thread(method_name_arg: *mut i8, mut arguments: *mut *mut i8, mut types: *mut acpi_object_type) {
    core::ptr::write_bytes(&mut ACPI_GBL_DB_METHOD_INFO as *mut _ as *mut u8, 0, core::mem::size_of::<acpi_db_method_info>());
    ACPI_GBL_DB_METHOD_INFO.name = method_name_arg;
    ACPI_GBL_DB_METHOD_INFO.init_args = 1;
    ACPI_GBL_DB_METHOD_INFO.args = ACPI_GBL_DB_METHOD_INFO.arguments.as_mut_ptr();
    ACPI_GBL_DB_METHOD_INFO.types = ACPI_GBL_DB_METHOD_INFO.arg_types.as_mut_ptr();
    for i in 0..ACPI_METHOD_NUM_ARGS {
        if arguments.is_null() || (*arguments).is_null() { break; }
        ACPI_GBL_DB_METHOD_INFO.arguments[i as usize] = *arguments; arguments = arguments.add(1);
        ACPI_GBL_DB_METHOD_INFO.arg_types[i as usize] = *types; types = types.add(1);
    }
    if ACPI_FAILURE(acpi_db_execute_setup(&mut ACPI_GBL_DB_METHOD_INFO)) { return; }
    let status = acpi_get_handle(core::ptr::null_mut(), ACPI_GBL_DB_METHOD_INFO.pathname.as_mut_ptr(), &mut ACPI_GBL_DB_METHOD_INFO.method);
    if ACPI_FAILURE(status) { acpi_os_printf(c"%s Could not get handle for %s\n".as_ptr(), acpi_format_exception(status), ACPI_GBL_DB_METHOD_INFO.pathname.as_ptr()); return; }
    if ACPI_SUCCESS(acpi_os_execute(OSL_DEBUGGER_EXEC_THREAD, acpi_db_single_execution_thread, &mut ACPI_GBL_DB_METHOD_INFO as *mut _ as *mut core::ffi::c_void)) { acpi_os_printf(c"\nBackground thread started\n".as_ptr()); }
}

pub unsafe fn acpi_db_create_execution_threads(num_threads_arg: *mut i8, num_loops_arg: *mut i8, method_name_arg: *mut i8) {
    let num_threads = acpi_strtoul(num_threads_arg, 0);
    let num_loops = acpi_strtoul(num_loops_arg, 0);
    if num_threads == 0 || num_loops == 0 { acpi_os_printf(c"Bad argument: Threads %X, Loops %X\n".as_ptr(), num_threads, num_loops); return; }
    let mut main_thread_gate = core::ptr::null_mut(); let mut thread_complete_gate = core::ptr::null_mut(); let mut info_gate = core::ptr::null_mut();
    if ACPI_FAILURE(acpi_os_create_semaphore(1, 0, &mut main_thread_gate)) { return; }
    if ACPI_FAILURE(acpi_os_create_semaphore(1, 1, &mut thread_complete_gate)) { let _ = acpi_os_delete_semaphore(main_thread_gate); return; }
    if ACPI_FAILURE(acpi_os_create_semaphore(1, 1, &mut info_gate)) { let _ = acpi_os_delete_semaphore(thread_complete_gate); let _ = acpi_os_delete_semaphore(main_thread_gate); return; }
    core::ptr::write_bytes(&mut ACPI_GBL_DB_METHOD_INFO as *mut _ as *mut u8, 0, core::mem::size_of::<acpi_db_method_info>());
    ACPI_GBL_DB_METHOD_INFO.num_threads = num_threads;
    let size = core::mem::size_of::<acpi_thread_id>() * num_threads as usize;
    ACPI_GBL_DB_METHOD_INFO.threads = acpi_os_allocate(size) as *mut acpi_thread_id;
    if ACPI_GBL_DB_METHOD_INFO.threads.is_null() { acpi_os_printf(c"No memory for thread IDs array\n".as_ptr()); return; }
    core::ptr::write_bytes(ACPI_GBL_DB_METHOD_INFO.threads, 0, num_threads as usize);
    ACPI_GBL_DB_METHOD_INFO.name = method_name_arg; ACPI_GBL_DB_METHOD_INFO.num_loops = num_loops;
    ACPI_GBL_DB_METHOD_INFO.main_thread_gate = main_thread_gate; ACPI_GBL_DB_METHOD_INFO.thread_complete_gate = thread_complete_gate; ACPI_GBL_DB_METHOD_INFO.info_gate = info_gate;
    let _ = acpi_db_execute_setup(&mut ACPI_GBL_DB_METHOD_INFO);
    for _ in 0..num_threads { let _ = acpi_os_execute(OSL_DEBUGGER_EXEC_THREAD, acpi_db_method_thread, &mut ACPI_GBL_DB_METHOD_INFO as *mut _ as *mut core::ffi::c_void); }
    let _ = acpi_os_wait_semaphore(main_thread_gate, 1, ACPI_WAIT_FOREVER);
    let _ = acpi_os_delete_semaphore(main_thread_gate); let _ = acpi_os_delete_semaphore(thread_complete_gate); let _ = acpi_os_delete_semaphore(info_gate);
    acpi_os_free(ACPI_GBL_DB_METHOD_INFO.threads as *mut core::ffi::c_void); ACPI_GBL_DB_METHOD_INFO.threads = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
