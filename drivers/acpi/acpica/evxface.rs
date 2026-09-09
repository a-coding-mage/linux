// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// External interfaces for ACPI events. This is a direct low-level translation
// of evxface.c; symbols supplied by the ACPI headers are intentionally external.

// #define EXPORT_ACPI_INTERFACES
// Includes: acpi/acpi.h, accommon.h, acnamesp.h, acevents.h, acinterp.h

#[cfg(not(feature = "acpi_reduced_hardware"))]
unsafe fn acpi_ev_install_gpe_handler(gpe_device: acpi_handle, gpe_number: u32,
    ty: u32, is_raw_handler: u8, address: acpi_gpe_handler, context: *mut core::ffi::c_void) -> acpi_status {
    let mut gpe_event_info: *mut acpi_gpe_event_info;
    let handler: *mut acpi_gpe_handler_info;
    let mut status: acpi_status;
    let flags: acpi_cpu_flags;
    if address.is_null() || (ty & !ACPI_GPE_XRUPT_TYPE_MASK) != 0 { return AE_BAD_PARAMETER; }
    status = acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);
    if ACPI_FAILURE(status) { return status; }
    handler = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_gpe_handler_info>()) as *mut acpi_gpe_handler_info;
    if handler.is_null() { status = AE_NO_MEMORY; acpi_ut_release_mutex(ACPI_MTX_EVENTS); return status; }
    flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
    gpe_event_info = acpi_ev_get_gpe_event_info(gpe_device, gpe_number);
    if gpe_event_info.is_null() { status = AE_BAD_PARAMETER; acpi_os_release_lock(acpi_gbl_gpe_lock, flags); ACPI_FREE(handler); acpi_ut_release_mutex(ACPI_MTX_EVENTS); return status; }
    let dispatch = ACPI_GPE_DISPATCH_TYPE((*gpe_event_info).flags);
    if dispatch == ACPI_GPE_DISPATCH_HANDLER || dispatch == ACPI_GPE_DISPATCH_RAW_HANDLER {
        status = AE_ALREADY_EXISTS; acpi_os_release_lock(acpi_gbl_gpe_lock, flags); ACPI_FREE(handler); acpi_ut_release_mutex(ACPI_MTX_EVENTS); return status;
    }
    (*handler).address = address; (*handler).context = context;
    (*handler).method_node = (*gpe_event_info).dispatch.method_node;
    (*handler).original_flags = ((*gpe_event_info).flags & (ACPI_GPE_XRUPT_TYPE_MASK | ACPI_GPE_DISPATCH_MASK)) as u8;
    let old = ACPI_GPE_DISPATCH_TYPE((*handler).original_flags);
    if (old == ACPI_GPE_DISPATCH_METHOD || old == ACPI_GPE_DISPATCH_NOTIFY) && (*gpe_event_info).runtime_count != 0 {
        (*handler).originally_enabled = TRUE;
        acpi_ev_remove_gpe_reference(gpe_event_info);
        if ty != ((*gpe_event_info).flags & ACPI_GPE_XRUPT_TYPE_MASK) { ACPI_WARNING!((AE_INFO, "GPE type mismatch (level/edge)")); }
    }
    (*gpe_event_info).dispatch.handler = handler;
    (*gpe_event_info).flags &= !(ACPI_GPE_XRUPT_TYPE_MASK | ACPI_GPE_DISPATCH_MASK);
    (*gpe_event_info).flags |= (ty | if is_raw_handler != 0 { ACPI_GPE_DISPATCH_RAW_HANDLER } else { ACPI_GPE_DISPATCH_HANDLER }) as u8;
    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    acpi_ut_release_mutex(ACPI_MTX_EVENTS); status = AE_OK; status
}

pub unsafe fn acpi_install_notify_handler(device: acpi_handle, handler_type: u32, handler: acpi_notify_handler, context: *mut core::ffi::c_void) -> acpi_status {
    let node = device as *mut acpi_namespace_node; let mut status; let mut obj_desc; let mut handler_obj; let mut i: u32;
    if device.is_null() || handler.is_null() || handler_type == 0 || handler_type > ACPI_MAX_NOTIFY_HANDLER_TYPE { return AE_BAD_PARAMETER; }
    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); if ACPI_FAILURE(status) { return status; }
    if device == ACPI_ROOT_OBJECT { for i in 0..ACPI_NUM_NOTIFY_TYPES { if handler_type & (i + 1) != 0 { if !acpi_gbl_global_notify[i as usize].handler.is_null() { status = AE_ALREADY_EXISTS; break; } acpi_gbl_global_notify[i as usize].handler = handler; acpi_gbl_global_notify[i as usize].context = context; } } acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); return status; }
    if !acpi_ev_is_notify_object(node) { acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); return AE_TYPE; }
    obj_desc = acpi_ns_get_attached_object(node);
    if obj_desc.is_null() { obj_desc = acpi_ut_create_internal_object((*node).type_); if obj_desc.is_null() { acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); return AE_NO_MEMORY; } status = acpi_ns_attach_object(device, obj_desc, (*node).type_); acpi_ut_remove_reference(obj_desc); if ACPI_FAILURE(status) { acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); return status; } }
    for i in 0..ACPI_NUM_NOTIFY_TYPES { if handler_type & (i + 1) != 0 { handler_obj = (*obj_desc).common_notify.notify_list[i as usize]; while !handler_obj.is_null() { if (*handler_obj).notify.handler == handler { acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); return AE_ALREADY_EXISTS; } handler_obj = (*handler_obj).notify.next[i as usize]; } } }
    handler_obj = acpi_ut_create_internal_object(ACPI_TYPE_LOCAL_NOTIFY); if handler_obj.is_null() { acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); return AE_NO_MEMORY; }
    (*handler_obj).notify.node = node; (*handler_obj).notify.handler_type = handler_type; (*handler_obj).notify.handler = handler; (*handler_obj).notify.context = context;
    for i in 0..ACPI_NUM_NOTIFY_TYPES { if handler_type & (i + 1) != 0 { (*handler_obj).notify.next[i as usize] = (*obj_desc).common_notify.notify_list[i as usize]; (*obj_desc).common_notify.notify_list[i as usize] = handler_obj; } }
    if handler_type == ACPI_ALL_NOTIFY { acpi_ut_add_reference(handler_obj); } acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); status
}

pub unsafe fn acpi_remove_notify_handler(device: acpi_handle, handler_type: u32, handler: acpi_notify_handler) -> acpi_status {
    let node = device as *mut acpi_namespace_node; if device.is_null() || handler.is_null() || handler_type == 0 || handler_type > ACPI_MAX_NOTIFY_HANDLER_TYPE { return AE_BAD_PARAMETER; }
    let mut status = AE_OK; if device == ACPI_ROOT_OBJECT { for i in 0..ACPI_NUM_NOTIFY_TYPES { if handler_type & (i+1) != 0 { status=acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); if ACPI_FAILURE(status) { return status; } if acpi_gbl_global_notify[i as usize].handler != handler { acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); return AE_NOT_EXIST; } acpi_gbl_global_notify[i as usize].handler = None; acpi_gbl_global_notify[i as usize].context = core::ptr::null_mut(); acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); acpi_os_wait_events_complete(); } } return AE_OK; }
    if !acpi_ev_is_notify_object(node) { return AE_TYPE; } let obj_desc=acpi_ns_get_attached_object(node); if obj_desc.is_null() { return AE_NOT_EXIST; }
    for i in 0..ACPI_NUM_NOTIFY_TYPES { if handler_type & (i+1) != 0 { status=acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE); if ACPI_FAILURE(status) { return status; } let mut p=(*obj_desc).common_notify.notify_list[i as usize]; let mut prev=core::ptr::null_mut(); while !p.is_null() && (*p).notify.handler != handler { prev=p; p=(*p).notify.next[i as usize]; } if p.is_null() { acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); return AE_NOT_EXIST; } if prev.is_null() { (*obj_desc).common_notify.notify_list[i as usize]=(*p).notify.next[i as usize]; } else { (*prev).notify.next[i as usize]=(*p).notify.next[i as usize]; } acpi_ut_release_mutex(ACPI_MTX_NAMESPACE); acpi_os_wait_events_complete(); acpi_ut_remove_reference(p); } } status
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_install_sci_handler(address: acpi_sci_handler, context: *mut core::ffi::c_void) -> acpi_status { if address.is_null() { return AE_BAD_PARAMETER; } let n=ACPI_ALLOCATE(core::mem::size_of::<acpi_sci_handler_info>()) as *mut acpi_sci_handler_info; if n.is_null(){return AE_NO_MEMORY;} (*n).address=address;(*n).context=context; let mut s=acpi_ut_acquire_mutex(ACPI_MTX_EVENTS); if ACPI_FAILURE(s){ACPI_FREE(n);return s;} let f=acpi_os_acquire_lock(acpi_gbl_gpe_lock); let mut p=acpi_gbl_sci_handler_list; while !p.is_null(){if (*p).address==address{s=AE_ALREADY_EXISTS;break;}p=(*p).next;} if ACPI_SUCCESS(s){(*n).next=acpi_gbl_sci_handler_list;acpi_gbl_sci_handler_list=n;} acpi_os_release_lock(acpi_gbl_gpe_lock,f);acpi_ut_release_mutex(ACPI_MTX_EVENTS);if ACPI_FAILURE(s){ACPI_FREE(n);}s }

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_remove_sci_handler(address: acpi_sci_handler) -> acpi_status { if address.is_null(){return AE_BAD_PARAMETER;} let s=acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);if ACPI_FAILURE(s){return s;}let f=acpi_os_acquire_lock(acpi_gbl_gpe_lock);let mut p=acpi_gbl_sci_handler_list;let mut prev=core::ptr::null_mut();while !p.is_null(){if (*p).address==address{if prev.is_null(){acpi_gbl_sci_handler_list=(*p).next;}else{(*prev).next=(*p).next;}acpi_os_release_lock(acpi_gbl_gpe_lock,f);ACPI_FREE(p);acpi_ut_release_mutex(ACPI_MTX_EVENTS);return AE_OK;}prev=p;p=(*p).next;}acpi_os_release_lock(acpi_gbl_gpe_lock,f);acpi_ut_release_mutex(ACPI_MTX_EVENTS);AE_NOT_EXIST }

pub unsafe fn acpi_install_global_event_handler(handler: acpi_gbl_event_handler, context: *mut core::ffi::c_void) -> acpi_status { if handler.is_null(){return AE_BAD_PARAMETER;}let mut s=acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);if ACPI_FAILURE(s){return s;}if !acpi_gbl_global_event_handler.is_null(){s=AE_ALREADY_EXISTS;}else{acpi_gbl_global_event_handler=handler;acpi_gbl_global_event_handler_context=context;}acpi_ut_release_mutex(ACPI_MTX_EVENTS);s }
pub unsafe fn acpi_install_fixed_event_handler(event:u32, handler:acpi_event_handler, context:*mut core::ffi::c_void)->acpi_status{if event>ACPI_EVENT_MAX{return AE_BAD_PARAMETER;}let mut s=acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);if ACPI_FAILURE(s){return s;}if !acpi_gbl_fixed_event_handlers[event as usize].handler.is_null(){s=AE_ALREADY_EXISTS;}else{acpi_gbl_fixed_event_handlers[event as usize].handler=handler;acpi_gbl_fixed_event_handlers[event as usize].context=context;s=acpi_clear_event(event);if ACPI_SUCCESS(s){s=acpi_enable_event(event,0);}if ACPI_FAILURE(s){acpi_gbl_fixed_event_handlers[event as usize].handler=core::ptr::null_mut();acpi_gbl_fixed_event_handlers[event as usize].context=core::ptr::null_mut();}}acpi_ut_release_mutex(ACPI_MTX_EVENTS);s}
pub unsafe fn acpi_remove_fixed_event_handler(event:u32,_handler:acpi_event_handler)->acpi_status{if event>ACPI_EVENT_MAX{return AE_BAD_PARAMETER;}let s=acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);if ACPI_FAILURE(s){return s;}let mut r=acpi_disable_event(event,0);acpi_gbl_fixed_event_handlers[event as usize].handler=core::ptr::null_mut();acpi_gbl_fixed_event_handlers[event as usize].context=core::ptr::null_mut();acpi_ut_release_mutex(ACPI_MTX_EVENTS);r}
pub unsafe fn acpi_install_gpe_handler(d: acpi_handle,n:u32,t:u32,a:acpi_gpe_handler,c:*mut core::ffi::c_void)->acpi_status{acpi_ev_install_gpe_handler(d,n,t,FALSE,a,c)}
pub unsafe fn acpi_install_gpe_raw_handler(d: acpi_handle,n:u32,t:u32,a:acpi_gpe_handler,c:*mut core::ffi::c_void)->acpi_status{acpi_ev_install_gpe_handler(d,n,t,TRUE,a,c)}
pub unsafe fn acpi_remove_gpe_handler(d:acpi_handle,n:u32,a:acpi_gpe_handler)->acpi_status{if a.is_null(){return AE_BAD_PARAMETER;}let mut s=acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);if ACPI_FAILURE(s){return s;}let f=acpi_os_acquire_lock(acpi_gbl_gpe_lock);let e=acpi_ev_get_gpe_event_info(d,n);if e.is_null(){s=AE_BAD_PARAMETER;}else{let k=ACPI_GPE_DISPATCH_TYPE((*e).flags);if k!=ACPI_GPE_DISPATCH_HANDLER&&k!=ACPI_GPE_DISPATCH_RAW_HANDLER{s=AE_NOT_EXIST;}else if (*e).dispatch.handler.is_null()||(*(*e).dispatch.handler).address!=a{s=AE_BAD_PARAMETER;}else{let h=(*e).dispatch.handler;(*e).dispatch.handler=core::ptr::null_mut();(*e).dispatch.method_node=(*h).method_node;(*e).flags&=!(ACPI_GPE_XRUPT_TYPE_MASK|ACPI_GPE_DISPATCH_MASK);(*e).flags|=(*h).original_flags as u32;}}acpi_os_release_lock(acpi_gbl_gpe_lock,f);acpi_ut_release_mutex(ACPI_MTX_EVENTS);if ACPI_SUCCESS(s){acpi_os_wait_events_complete();}s}

// The remaining fixed-event, GPE-removal, and global-lock interfaces retain the
// same direct calls and ordering as the C implementation.
pub unsafe fn acpi_acquire_global_lock(timeout:u16, handle:*mut u32)->acpi_status{if handle.is_null(){return AE_BAD_PARAMETER;}acpi_ex_enter_interpreter();let s=acpi_ex_acquire_mutex_object(timeout,acpi_gbl_global_lock_mutex,acpi_os_get_thread_id());if ACPI_SUCCESS(s){*handle=acpi_gbl_global_lock_handle;}acpi_ex_exit_interpreter();s}
pub unsafe fn acpi_release_global_lock(handle:u32)->acpi_status{if handle==0||handle!=acpi_gbl_global_lock_handle{return AE_NOT_ACQUIRED;}acpi_ex_release_mutex_object(acpi_gbl_global_lock_mutex)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
