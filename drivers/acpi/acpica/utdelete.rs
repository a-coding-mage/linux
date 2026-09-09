// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Object deletion and reference count utilities.

// C dependencies and build-time configuration are supplied by the surrounding ACPICA translation.

unsafe fn acpi_ut_delete_internal_obj(object: *mut acpi_operand_object) {
    let mut obj_pointer: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut handler_desc: *mut acpi_operand_object;
    let mut second_desc: *mut acpi_operand_object;
    let mut next_desc: *mut acpi_operand_object;
    let mut start_desc: *mut acpi_operand_object;
    let mut last_obj_ptr: *mut *mut acpi_operand_object;

    acpi_function_trace_ptr!(ut_delete_internal_obj, object);
    if object.is_null() { return; }

    match (*object).common.type_ {
        ACPI_TYPE_STRING => {
            acpi_debug_print_alloc!("**** String {:p}, ptr {:p}\n", object, (*object).string.pointer);
            if (*object).common.flags & AOPOBJ_STATIC_POINTER == 0 { obj_pointer = (*object).string.pointer as *mut _; }
        }
        ACPI_TYPE_BUFFER => {
            acpi_debug_print_alloc!("**** Buffer {:p}, ptr {:p}\n", object, (*object).buffer.pointer);
            if (*object).common.flags & AOPOBJ_STATIC_POINTER == 0 { obj_pointer = (*object).buffer.pointer as *mut _; }
        }
        ACPI_TYPE_PACKAGE => {
            acpi_debug_print_alloc!(" **** Package of count {:X}\n", (*object).package.count);
            obj_pointer = (*object).package.elements as *mut _;
        }
        ACPI_TYPE_DEVICE => {
            if !(*object).device.gpe_block.is_null() { let _ = acpi_ev_delete_gpe_block((*object).device.gpe_block); }
            handler_desc = (*object).common_notify.handler;
            while !handler_desc.is_null() { next_desc = (*handler_desc).address_space.next; acpi_ut_remove_reference(handler_desc); handler_desc = next_desc; }
        }
        ACPI_TYPE_PROCESSOR | ACPI_TYPE_THERMAL => {
            handler_desc = (*object).common_notify.handler;
            while !handler_desc.is_null() { next_desc = (*handler_desc).address_space.next; acpi_ut_remove_reference(handler_desc); handler_desc = next_desc; }
        }
        ACPI_TYPE_MUTEX => {
            acpi_debug_print_alloc!("***** Mutex {:p}, OS Mutex {:p}\n", object, (*object).mutex.os_mutex);
            if object == acpi_gbl_global_lock_mutex { let _ = acpi_os_delete_semaphore(acpi_gbl_global_lock_semaphore); acpi_gbl_global_lock_semaphore = ACPI_SEMAPHORE_NULL; acpi_os_delete_mutex((*object).mutex.os_mutex); acpi_gbl_global_lock_mutex = core::ptr::null_mut(); }
            else { acpi_ex_unlink_mutex(object); acpi_os_delete_mutex((*object).mutex.os_mutex); }
        }
        ACPI_TYPE_EVENT => { acpi_debug_print_alloc!("***** Event {:p}, OS Semaphore {:p}\n", object, (*object).event.os_semaphore); let _ = acpi_os_delete_semaphore((*object).event.os_semaphore); (*object).event.os_semaphore = ACPI_SEMAPHORE_NULL; }
        ACPI_TYPE_METHOD => {
            acpi_debug_print_alloc!("***** Method {:p}\n", object);
            if !(*object).method.mutex.is_null() { acpi_os_delete_mutex((*object).method.mutex.as_ref().unwrap().mutex.os_mutex); acpi_ut_delete_object_desc((*object).method.mutex); (*object).method.mutex = core::ptr::null_mut(); }
            if !(*object).method.node.is_null() { (*object).method.node = core::ptr::null_mut(); }
        }
        ACPI_TYPE_REGION => {
            acpi_debug_print_alloc!("***** Region {:p}\n", object);
            if (*object).region.node.flags & ANOBJ_TEMPORARY == 0 { acpi_ut_remove_address_range((*object).region.space_id, (*object).region.node); }
            second_desc = acpi_ns_get_secondary_object(object);
            if !second_desc.is_null() {
                handler_desc = (*object).region.handler;
                if !handler_desc.is_null() {
                    next_desc = (*handler_desc).address_space.region_list; start_desc = next_desc; last_obj_ptr = &mut (*handler_desc).address_space.region_list;
                    while !next_desc.is_null() { if next_desc == object { *last_obj_ptr = (*next_desc).region.next; break; } last_obj_ptr = &mut (*next_desc).region.next; next_desc = (*next_desc).region.next; if next_desc == start_desc { acpi_error!(AE_INFO, "Circular region list in address handler object {:p}", handler_desc); return; } }
                    if (*handler_desc).address_space.handler_flags & ACPI_ADDR_HANDLER_DEFAULT_INSTALLED != 0 { if let Some(setup) = (*handler_desc).address_space.setup { let _ = setup(object, ACPI_REGION_DEACTIVATE, (*handler_desc).address_space.context, &mut (*second_desc).extra.region_context); } }
                    acpi_ut_remove_reference(handler_desc);
                }
                acpi_ut_delete_object_desc(second_desc);
            }
            if !(*object).field.internal_pcc_buffer.is_null() { acpi_free!((*object).field.internal_pcc_buffer); }
        }
        ACPI_TYPE_BUFFER_FIELD | ACPI_TYPE_LOCAL_BANK_FIELD => { second_desc = acpi_ns_get_secondary_object(object); if !second_desc.is_null() { acpi_ut_delete_object_desc(second_desc); } }
        ACPI_TYPE_LOCAL_ADDRESS_HANDLER => { acpi_debug_print_alloc!("***** Address handler {:p}\n", object); acpi_os_delete_mutex((*object).address_space.context_mutex); }
        _ => {}
    }
    if !obj_pointer.is_null() { acpi_debug_print_alloc!("Deleting Object Subptr {:p}\n", obj_pointer); acpi_free!(obj_pointer); }
    acpi_debug_print_raw!("Deleting Object {:p}\n", object);
    acpi_ut_delete_object_desc(object);
}

pub unsafe fn acpi_ut_delete_internal_object_list(obj_list: *mut *mut acpi_operand_object) {
    acpi_function_entry!();
    let mut internal_obj = obj_list;
    while !(*internal_obj).is_null() { acpi_ut_remove_reference(*internal_obj); internal_obj = internal_obj.add(1); }
    acpi_free!(obj_list as *mut core::ffi::c_void);
}

unsafe fn acpi_ut_update_ref_count(object: *mut acpi_operand_object, action: u32) {
    if object.is_null() { return; }
    let flags = acpi_os_acquire_lock(acpi_gbl_reference_count_lock);
    let original_count = (*object).common.reference_count;
    let new_count = match action { REF_INCREMENT => { let n = original_count.wrapping_add(1); (*object).common.reference_count = n; acpi_os_release_lock(acpi_gbl_reference_count_lock, flags); n }, REF_DECREMENT => { let n = original_count.saturating_sub(1); if original_count != 0 { (*object).common.reference_count = n; } acpi_os_release_lock(acpi_gbl_reference_count_lock, flags); if original_count != 0 && n == 0 { acpi_ut_delete_internal_obj(object); } n }, _ => { acpi_os_release_lock(acpi_gbl_reference_count_lock, flags); acpi_error!(AE_INFO, "Unknown Reference Count action (0x{:X})", action); return; } };
    if new_count > ACPI_MAX_REFERENCE_COUNT { acpi_warning!(AE_INFO, "Large Reference Count (0x{:X}) in object {:p}", new_count, object); }
}

pub unsafe fn acpi_ut_update_object_reference(mut object: *mut acpi_operand_object, action: u16) -> acpi_status {
    while !object.is_null() { acpi_ut_update_ref_count(object, action as u32); object = core::ptr::null_mut(); }
    AE_OK
}

pub unsafe fn acpi_ut_add_reference(object: *mut acpi_operand_object) { if acpi_ut_valid_internal_object(object) { let _ = acpi_ut_update_object_reference(object, REF_INCREMENT as u16); } }
pub unsafe fn acpi_ut_remove_reference(object: *mut acpi_operand_object) { if !object.is_null() && acpi_ut_valid_internal_object(object) { let _ = acpi_ut_update_object_reference(object, REF_DECREMENT as u16); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
