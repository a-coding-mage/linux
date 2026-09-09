// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Translation of acpi/acpica/dbcmds.c. External ACPICA definitions are supplied by other units.

use core::{ffi::c_void, ptr, slice};

static mut ACPI_DB_TRACE_METHOD_NAME: *mut i8 = ptr::null_mut();

unsafe extern "C" {
    fn strtoul(*const i8, *mut *mut i8, i32) -> usize;
    fn strlen(*const i8) -> usize;
    fn strcmp(*const i8, *const i8) -> i32;
    fn strstr(*const i8, *const i8) -> *mut i8;
    fn memcpy(*mut c_void, *const c_void, usize) -> *mut c_void;
}

// ACPICA types, constants, globals, and functions referenced below are external dependencies.
// Their names and C-compatible layouts are intentionally preserved.

#[allow(non_snake_case, non_camel_case_types, dead_code)]
pub unsafe fn acpi_db_convert_to_node(in_string: *mut i8) -> *mut acpi_namespace_node {
    let mut node: *mut acpi_namespace_node;
    let address: usize;
    if *in_string >= 0x30 && *in_string <= 0x39 {
        address = strtoul(in_string, ptr::null_mut(), 16);
        node = address as *mut acpi_namespace_node;
        if acpi_os_readable(node as *const c_void, core::mem::size_of::<acpi_namespace_node>()) == 0 {
            acpi_os_printf(b"Address %p is invalid\0".as_ptr() as *const i8, node);
            return ptr::null_mut();
        }
        if ACPI_GET_DESCRIPTOR_TYPE(node) != ACPI_DESC_TYPE_NAMED {
            acpi_os_printf(b"Address %p is not a valid namespace node [%s]\n\0".as_ptr() as *const i8, node, acpi_ut_get_descriptor_name(node));
            return ptr::null_mut();
        }
    } else {
        node = acpi_db_local_ns_lookup(in_string);
        if node.is_null() {
            acpi_os_printf(b"Could not find [%s] in namespace, defaulting to root node\n\0".as_ptr() as *const i8, in_string);
            node = acpi_gbl_root_node;
        }
    }
    node
}

pub unsafe fn acpi_db_sleep(object_arg: *mut i8) -> acpi_status {
    if object_arg.is_null() {
        acpi_os_printf(b"Invoking all possible sleep states, 0-%d\n\0".as_ptr() as *const i8, ACPI_S_STATES_MAX);
        for i in 0..=ACPI_S_STATES_MAX { acpi_db_do_one_sleep_state(i as u8); }
    } else { acpi_db_do_one_sleep_state(strtoul(object_arg, ptr::null_mut(), 0) as u8); }
    AE_OK
}

unsafe fn acpi_db_do_one_sleep_state(sleep_state: u8) {
    if sleep_state > ACPI_S_STATES_MAX { acpi_os_printf(b"Sleep state %d out of range (%d max)\n\0".as_ptr() as *const i8, sleep_state, ACPI_S_STATES_MAX); return; }
    acpi_os_printf(b"\n---- Invoking sleep state S%d (%s):\n\0".as_ptr() as *const i8, sleep_state, acpi_gbl_sleep_state_names[sleep_state as usize]);
    let mut a = 0u8; let mut b = 0u8;
    let mut status = acpi_get_sleep_type_data(sleep_state, &mut a, &mut b);
    if ACPI_FAILURE(status) != 0 { acpi_os_printf(b"Could not evaluate [%s] method, %s\n\0".as_ptr() as *const i8, acpi_gbl_sleep_state_names[sleep_state as usize], acpi_format_exception(status)); return; }
    acpi_os_printf(b"Register values for sleep state S%d: Sleep-A: %.2X, Sleep-B: %.2X\n\0".as_ptr() as *const i8, sleep_state, a, b);
    acpi_os_printf(b"**** Sleep: Prepare to sleep (S%d) ****\n\0".as_ptr() as *const i8, sleep_state); status = acpi_enter_sleep_state_prep(sleep_state); if ACPI_FAILURE(status) != 0 { acpi_exception(status, b"During invocation of sleep state S%d\0".as_ptr() as *const i8, sleep_state); return; }
    acpi_os_printf(b"**** Sleep: Going to sleep (S%d) ****\n\0".as_ptr() as *const i8, sleep_state); status = acpi_enter_sleep_state(sleep_state); if ACPI_FAILURE(status) != 0 { acpi_exception(status, b"During invocation of sleep state S%d\0".as_ptr() as *const i8, sleep_state); return; }
    acpi_os_printf(b"**** Wake: Prepare to return from sleep (S%d) ****\n\0".as_ptr() as *const i8, sleep_state); status = acpi_leave_sleep_state_prep(sleep_state); if ACPI_FAILURE(status) != 0 { acpi_exception(status, b"During invocation of sleep state S%d\0".as_ptr() as *const i8, sleep_state); return; }
    acpi_os_printf(b"**** Wake: Return from sleep (S%d) ****\n\0".as_ptr() as *const i8, sleep_state); status = acpi_leave_sleep_state(sleep_state); if ACPI_FAILURE(status) != 0 { acpi_exception(status, b"During invocation of sleep state S%d\0".as_ptr() as *const i8, sleep_state); }
}

pub unsafe fn acpi_db_display_locks() { for i in 0..ACPI_MAX_MUTEX { acpi_os_printf(b"%26s : %s\n\0".as_ptr() as *const i8, acpi_ut_get_mutex_name(i), if acpi_gbl_mutex_info[i as usize].thread_id == ACPI_MUTEX_NOT_ACQUIRED { b"Locked\0" } else { b"Unlocked\0" }); } }

pub unsafe fn acpi_db_display_table_info(_table_arg: *mut i8) {
    acpi_os_printf(b"Idx ID  Status Type                    TableHeader (Sig, Address, Length, Misc)\n\0".as_ptr() as *const i8);
    for i in 0..acpi_gbl_root_table_list.current_table_count { let t = &mut acpi_gbl_root_table_list.tables[i as usize]; acpi_os_printf(b"%3u %.2u \0".as_ptr() as *const i8, i, t.owner_id); if t.flags & ACPI_TABLE_IS_LOADED == 0 { acpi_os_printf(b"NotLoaded \0".as_ptr() as *const i8); } else { acpi_os_printf(b" Loaded \0".as_ptr() as *const i8); } match t.flags & ACPI_TABLE_ORIGIN_MASK { ACPI_TABLE_ORIGIN_EXTERNAL_VIRTUAL => acpi_os_printf(b"External/virtual \0".as_ptr() as *const i8), ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL => acpi_os_printf(b"Internal/physical \0".as_ptr() as *const i8), ACPI_TABLE_ORIGIN_INTERNAL_VIRTUAL => acpi_os_printf(b"Internal/virtual \0".as_ptr() as *const i8), _ => acpi_os_printf(b"INVALID TYPE    \0".as_ptr() as *const i8) }; if ACPI_FAILURE(acpi_tb_validate_table(t)) != 0 { return; } if !t.pointer.is_null() { acpi_tb_print_table_header(t.address, t.pointer); } else { ACPI_INFO(t.signature.ascii); } }
}

pub unsafe fn acpi_db_unload_acpi_table(object_name: *mut i8) { let n=acpi_db_convert_to_node(object_name); if n.is_null(){return;} let s=acpi_unload_parent_table(n as acpi_handle); if ACPI_SUCCESS(s)!=0 { acpi_os_printf(b"Parent of [%s] (%p) unloaded and uninstalled\n\0".as_ptr() as *const i8,object_name,n); } else { acpi_os_printf(b"%s, while unloading parent table of [%s]\n\0".as_ptr() as *const i8,acpi_format_exception(s),object_name); } }

pub unsafe fn acpi_db_send_notify(name:*mut i8,value:u32){let n=acpi_db_convert_to_node(name);if n.is_null(){return;}if acpi_ev_is_notify_object(n)!=0{if ACPI_FAILURE(acpi_ev_queue_notify_request(n,value))!=0{acpi_os_printf(b"Could not queue notify\n\0".as_ptr()as*const i8);}}else{acpi_os_printf(b"Named object [%4.4s] Type %s, must be Device/Thermal/Processor type\n\0".as_ptr()as*const i8,acpi_ut_get_node_name(n),acpi_ut_get_type_name((*n).type));}}

pub unsafe fn acpi_db_display_interfaces(action:*mut i8,name:*mut i8){if action.is_null(){let mut p=acpi_gbl_supported_interfaces;acpi_os_acquire_mutex(acpi_gbl_osi_mutex,ACPI_WAIT_FOREVER);while !p.is_null(){if (*p).flags&ACPI_OSI_INVALID==0{acpi_os_printf(b"%s\n\0".as_ptr()as*const i8,(*p).name);}p=(*p).next;}acpi_os_release_mutex(acpi_gbl_osi_mutex);return;}if name.is_null(){acpi_os_printf(b"Missing Interface Name argument\n\0".as_ptr()as*const i8);return;}acpi_ut_strupr(action);let install=strstr(b"INSTALL\0".as_ptr()as*const i8,action);let remove=strstr(b"REMOVE\0".as_ptr()as*const i8,action);let s=if !install.is_null(){acpi_install_interface(name)}else if !remove.is_null(){acpi_remove_interface(name)}else{acpi_os_printf(b"Invalid action argument: %s\n\0".as_ptr()as*const i8,action);return;};if ACPI_FAILURE(s)!=0{acpi_os_printf(b"%s, while modifying \"%s\"\n\0".as_ptr()as*const i8,acpi_format_exception(s),name);}}

pub unsafe fn acpi_db_generate_interrupt(arg:*mut i8){let mut p=acpi_gbl_ged_handler_list;if p.is_null(){acpi_os_printf(b"No GED handling present\n\0".as_ptr()as*const i8);}let n=strtoul(arg,ptr::null_mut(),0)as u32;while !p.is_null(){if(*p).int_id==n{if(*p).evt_method.is_null(){acpi_os_printf(b"Undefined _EVT method\n\0".as_ptr()as*const i8);return;}let mut o=acpi_object{integer:acpi_object_integer{type_:ACPI_TYPE_INTEGER,value:n as u64}};let l=acpi_object_list{count:1,pointer:&mut o};if ACPI_FAILURE(acpi_evaluate_object((*p).evt_method,ptr::null(),&l,ptr::null_mut()))!=0{acpi_os_printf(b"Could not evaluate _EVT\n\0".as_ptr()as*const i8);return;}}p=(*p).next;}}

#[cfg(not(feature="acpi_reduced_hardware"))] pub unsafe fn acpi_db_generate_gpe(a:*mut i8,b:*mut i8){let mut block=0;if !b.is_null(){block=strtoul(b,ptr::null_mut(),0)as u32;if block==1{block=0;}}let e=acpi_ev_get_gpe_event_info(block as *mut c_void,strtoul(a,ptr::null_mut(),0)as u32);if e.is_null(){acpi_os_printf(b"Invalid GPE\n\0".as_ptr()as*const i8);return;}acpi_ev_gpe_dispatch(ptr::null_mut(),e,0);}
#[cfg(not(feature="acpi_reduced_hardware"))] pub unsafe fn acpi_db_generate_sci(){acpi_ev_sci_dispatch();}

pub unsafe fn acpi_db_trace(enable:*mut i8,method:*mut i8,once:*mut i8){let(mut level,mut layer,mut flags)=(0,0,0);acpi_ut_strupr(enable);if !once.is_null(){acpi_ut_strupr(once);}if !method.is_null(){if !ACPI_DB_TRACE_METHOD_NAME.is_null(){ACPI_FREE(ACPI_DB_TRACE_METHOD_NAME as *mut c_void);}ACPI_DB_TRACE_METHOD_NAME=ACPI_ALLOCATE(strlen(method)+1)as*mut i8;if ACPI_DB_TRACE_METHOD_NAME.is_null(){return;}memcpy(ACPI_DB_TRACE_METHOD_NAME as*mut c_void,method,strlen(method)+1);}if strcmp(enable,b"ENABLE\0".as_ptr()as*const i8)==0{level=acpi_gbl_db_console_debug_level;layer=acpi_dbg_layer;flags=ACPI_TRACE_ENABLED;}else if strcmp(enable,b"METHOD\0".as_ptr()as*const i8)==0||strcmp(enable,b"OPCODE\0".as_ptr()as*const i8)==0{level=ACPI_LV_TRACE_POINT;layer=ACPI_EXECUTER;flags=ACPI_TRACE_ENABLED;if strcmp(enable,b"OPCODE\0".as_ptr()as*const i8)==0{flags|=ACPI_TRACE_OPCODE;}if !once.is_null()&&strcmp(once,b"ONCE\0".as_ptr()as*const i8)==0{flags|=ACPI_TRACE_ONESHOT;}}acpi_debug_trace(ACPI_DB_TRACE_METHOD_NAME,level,layer,flags);}

// The remaining resource/template diagnostic routines retain their C ABI and are supplied by the ACPICA translation unit.
extern "C" { pub fn acpi_db_display_template(*mut i8); pub fn acpi_db_display_resources(*mut i8); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
