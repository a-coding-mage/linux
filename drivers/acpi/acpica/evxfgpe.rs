// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// External Interfaces for General Purpose Events (GPEs)

// This module is excluded when ACPI_REDUCED_HARDWARE is enabled.

pub unsafe fn acpi_update_all_gpes() -> acpi_status {
    let mut status: acpi_status;
    let mut is_polling_needed: u8 = FALSE;
    ACPI_FUNCTION_TRACE!(acpi_update_all_gpes);
    status = acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);
    if ACPI_FAILURE(status) { return_ACPI_STATUS!(status); }
    if acpi_gbl_all_gpes_initialized { goto_unlock_and_exit!(); }
    status = acpi_ev_walk_gpe_list(acpi_ev_initialize_gpe_block, &mut is_polling_needed as *mut _ as *mut _);
    if ACPI_SUCCESS(status) { acpi_gbl_all_gpes_initialized = TRUE; }
    unlock_and_exit:
    let _ = acpi_ut_release_mutex(ACPI_MTX_EVENTS);
    if is_polling_needed != FALSE && acpi_gbl_all_gpes_initialized {
        acpi_ev_gpe_detect(acpi_gbl_gpe_xrupt_list_head);
    }
    return_ACPI_STATUS!(status)
}

pub unsafe fn acpi_enable_gpe_cond(gpe_device: acpi_handle, gpe_number: u32, mut dispatch_type: u8) -> acpi_status {
    let mut status = AE_BAD_PARAMETER;
    let mut flags: acpi_cpu_flags;
    ACPI_FUNCTION_TRACE!(acpi_enable_gpe);
    flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
    let gpe_event_info = acpi_ev_get_gpe_event_info(gpe_device, gpe_number);
    if !gpe_event_info.is_null() {
        if dispatch_type == ACPI_GPE_DISPATCH_MASK { dispatch_type = ACPI_GPE_DISPATCH_TYPE((*gpe_event_info).flags); }
        else if dispatch_type != ACPI_GPE_DISPATCH_TYPE((*gpe_event_info).flags) { dispatch_type = ACPI_GPE_DISPATCH_NONE; }
        if dispatch_type != ACPI_GPE_DISPATCH_NONE {
            status = acpi_ev_add_gpe_reference(gpe_event_info, TRUE);
            if ACPI_SUCCESS(status) && ACPI_GPE_IS_POLLING_NEEDED!(gpe_event_info) {
                acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
                let _ = acpi_ev_detect_gpe(gpe_device, gpe_event_info, gpe_number);
                flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
            }
        } else { status = AE_NO_HANDLER; }
    }
    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    return_ACPI_STATUS!(status)
}

pub unsafe fn acpi_enable_gpe(gpe_device: acpi_handle, gpe_number: u32) -> acpi_status {
    acpi_enable_gpe_cond(gpe_device, gpe_number, ACPI_GPE_DISPATCH_MASK)
}

pub unsafe fn acpi_disable_gpe(gpe_device: acpi_handle, gpe_number: u32) -> acpi_status {
    let mut status = AE_BAD_PARAMETER;
    let flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
    let info = acpi_ev_get_gpe_event_info(gpe_device, gpe_number);
    if !info.is_null() { status = acpi_ev_remove_gpe_reference(info); }
    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    return_ACPI_STATUS!(status)
}

pub unsafe fn acpi_set_gpe(gpe_device: acpi_handle, gpe_number: u32, action: u8) -> acpi_status {
    let flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
    let info = acpi_ev_get_gpe_event_info(gpe_device, gpe_number);
    let status;
    if info.is_null() { status = AE_BAD_PARAMETER; }
    else { status = match action { ACPI_GPE_ENABLE => { let s=acpi_hw_low_set_gpe(info, ACPI_GPE_ENABLE); (*info).disable_for_dispatch=FALSE; s }, ACPI_GPE_DISABLE => { let s=acpi_hw_low_set_gpe(info, ACPI_GPE_DISABLE); (*info).disable_for_dispatch=TRUE; s }, _ => AE_BAD_PARAMETER }; }
    acpi_os_release_lock(acpi_gbl_gpe_lock, flags); return_ACPI_STATUS!(status)
}

pub unsafe fn acpi_mask_gpe(gpe_device: acpi_handle, gpe_number: u32, is_masked: u8) -> acpi_status {
    let flags=acpi_os_acquire_lock(acpi_gbl_gpe_lock); let info=acpi_ev_get_gpe_event_info(gpe_device,gpe_number);
    let status=if info.is_null(){AE_BAD_PARAMETER}else{acpi_ev_mask_gpe(info,is_masked)}; acpi_os_release_lock(acpi_gbl_gpe_lock,flags); return_ACPI_STATUS!(status)
}

pub unsafe fn acpi_mark_gpe_for_wake(gpe_device: acpi_handle, gpe_number: u32) -> acpi_status {
    let flags=acpi_os_acquire_lock(acpi_gbl_gpe_lock); let info=acpi_ev_get_gpe_event_info(gpe_device,gpe_number); let mut status=AE_BAD_PARAMETER;
    if !info.is_null(){(*info).flags|=ACPI_GPE_CAN_WAKE;status=AE_OK;} acpi_os_release_lock(acpi_gbl_gpe_lock,flags); return_ACPI_STATUS!(status)
}

pub unsafe fn acpi_setup_gpe_for_wake(wake_device: acpi_handle,gpe_device: acpi_handle,gpe_number:u32)->acpi_status{
    if wake_device.is_null(){return AE_BAD_PARAMETER;} let device_node=if wake_device==ACPI_ROOT_OBJECT{acpi_gbl_root_node}else{wake_device as *mut acpi_namespace_node};
    if (*device_node).type_!=ACPI_TYPE_DEVICE{return AE_BAD_PARAMETER;} let mut new_notify=ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_gpe_notify_info>()) as *mut acpi_gpe_notify_info; if new_notify.is_null(){return AE_NO_MEMORY;}
    let flags=acpi_os_acquire_lock(acpi_gbl_gpe_lock); let info=acpi_ev_get_gpe_event_info(gpe_device,gpe_number); let mut status;
    if info.is_null(){status=AE_BAD_PARAMETER;}else{if ACPI_GPE_DISPATCH_TYPE((*info).flags)==ACPI_GPE_DISPATCH_NONE{(*info).flags=ACPI_GPE_DISPATCH_NOTIFY|ACPI_GPE_LEVEL_TRIGGERED;}else if (*info).flags&ACPI_GPE_AUTO_ENABLED!=0{let _=acpi_ev_remove_gpe_reference(info);(*info).flags&=!ACPI_GPE_AUTO_ENABLED;}
        if ACPI_GPE_DISPATCH_TYPE((*info).flags)==ACPI_GPE_DISPATCH_NOTIFY{let mut n=(*info).dispatch.notify_list;while !n.is_null(){if (*n).device_node==device_node{status=AE_ALREADY_EXISTS;goto done;}n=(*n).next;}(*new_notify).device_node=device_node;(*new_notify).next=(*info).dispatch.notify_list;(*info).dispatch.notify_list=new_notify;new_notify=core::ptr::null_mut();}(*info).flags|=ACPI_GPE_CAN_WAKE;status=AE_OK;}
    done: acpi_os_release_lock(acpi_gbl_gpe_lock,flags); if !new_notify.is_null(){ACPI_FREE(new_notify as *mut _);} return_ACPI_STATUS!(status)
}

pub unsafe fn acpi_set_gpe_wake_mask(gpe_device:acpi_handle,gpe_number:u32,action:u8)->acpi_status{let flags=acpi_os_acquire_lock(acpi_gbl_gpe_lock);let i=acpi_ev_get_gpe_event_info(gpe_device,gpe_number);let mut s=AE_OK;if i.is_null(){s=AE_BAD_PARAMETER}else if (*i).flags&ACPI_GPE_CAN_WAKE==0{s=AE_TYPE}else{let r=(*i).register_info;if r.is_null(){s=AE_NOT_EXIST}else{let b=acpi_hw_get_gpe_register_bit(i) as u8;match action{ACPI_GPE_ENABLE=>ACPI_SET_BIT!((*r).enable_for_wake,b),ACPI_GPE_DISABLE=>ACPI_CLEAR_BIT!((*r).enable_for_wake,b),_=>s=AE_BAD_PARAMETER}}}acpi_os_release_lock(acpi_gbl_gpe_lock,flags);return_ACPI_STATUS!(s)}

pub unsafe fn acpi_clear_gpe(d:acpi_handle,n:u32)->acpi_status{let f=acpi_os_acquire_lock(acpi_gbl_gpe_lock);let i=acpi_ev_get_gpe_event_info(d,n);let s=if i.is_null(){AE_BAD_PARAMETER}else{acpi_hw_clear_gpe(i)};acpi_os_release_lock(acpi_gbl_gpe_lock,f);return_ACPI_STATUS!(s)}
pub unsafe fn acpi_get_gpe_status(d:acpi_handle,n:u32,e:*mut acpi_event_status)->acpi_status{let f=acpi_os_acquire_lock(acpi_gbl_gpe_lock);let i=acpi_ev_get_gpe_event_info(d,n);let s=if i.is_null(){AE_BAD_PARAMETER}else{acpi_hw_get_gpe_status(i,e)};acpi_os_release_lock(acpi_gbl_gpe_lock,f);return_ACPI_STATUS!(s)}
pub unsafe fn acpi_dispatch_gpe(d:acpi_handle,n:u32)->u32{acpi_ev_detect_gpe(d,core::ptr::null_mut(),n)}
pub unsafe fn acpi_finish_gpe(d:acpi_handle,n:u32)->acpi_status{let f=acpi_os_acquire_lock(acpi_gbl_gpe_lock);let i=acpi_ev_get_gpe_event_info(d,n);let s=if i.is_null(){AE_BAD_PARAMETER}else{acpi_ev_finish_gpe(i)};acpi_os_release_lock(acpi_gbl_gpe_lock,f);return_ACPI_STATUS!(s)}
pub unsafe fn acpi_disable_all_gpes()->acpi_status{let s=acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);if ACPI_FAILURE(s){return s;}let r=acpi_hw_disable_all_gpes();let _=acpi_ut_release_mutex(ACPI_MTX_EVENTS);r}
pub unsafe fn acpi_enable_all_runtime_gpes()->acpi_status{let s=acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);if ACPI_FAILURE(s){return s;}let r=acpi_hw_enable_all_runtime_gpes();let _=acpi_ut_release_mutex(ACPI_MTX_EVENTS);r}
pub unsafe fn acpi_enable_all_wakeup_gpes()->acpi_status{let s=acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);if ACPI_FAILURE(s){return s;}let r=acpi_hw_enable_all_wakeup_gpes();let _=acpi_ut_release_mutex(ACPI_MTX_EVENTS);r}
pub unsafe fn acpi_any_gpe_status_set(skip:u32)->u32{let s=acpi_ut_acquire_mutex(ACPI_MTX_EVENTS);if ACPI_FAILURE(s){return FALSE;}let mut d=core::ptr::null_mut();let x=acpi_get_gpe_device(skip,&mut d);if ACPI_FAILURE(x){d=core::ptr::null_mut();}let r=acpi_hw_check_all_gpes(d,skip);let _=acpi_ut_release_mutex(ACPI_MTX_EVENTS);r}

pub unsafe fn acpi_install_gpe_block(d:acpi_handle,a:*mut acpi_generic_address,count:u32,irq:u32)->acpi_status{if d.is_null()||a.is_null()||count==0{return AE_BAD_PARAMETER;}let s=acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);if ACPI_FAILURE(s){return s;}let n=acpi_ns_validate_handle(d);let mut status;if n.is_null(){status=AE_BAD_PARAMETER;}else if (*n).type_!=ACPI_TYPE_DEVICE{status=AE_TYPE;}else if !(*n).object.is_null(){status=AE_ALREADY_EXISTS;}else{let mut block=core::ptr::null_mut();status=acpi_ev_create_gpe_block(n,(*a).address,(*a).space_id,count,0,irq,&mut block);if ACPI_SUCCESS(status){let mut obj=acpi_ns_get_attached_object(n);if obj.is_null(){obj=acpi_ut_create_internal_object(ACPI_TYPE_DEVICE);if obj.is_null(){status=AE_NO_MEMORY;}else{status=acpi_ns_attach_object(n,obj,ACPI_TYPE_DEVICE);acpi_ut_remove_reference(obj);}}if ACPI_SUCCESS(status){(*obj).device.gpe_block=block;}}}let _=acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);status}

pub unsafe fn acpi_remove_gpe_block(d:acpi_handle)->acpi_status{if d.is_null(){return AE_BAD_PARAMETER;}let s=acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);if ACPI_FAILURE(s){return s;}let n=acpi_ns_validate_handle(d);let status;if n.is_null(){status=AE_BAD_PARAMETER;}else if (*n).type_!=ACPI_TYPE_DEVICE{status=AE_TYPE;}else{let o=acpi_ns_get_attached_object(n);if o.is_null()||(*o).device.gpe_block.is_null(){return AE_NULL_OBJECT;}status=acpi_ev_delete_gpe_block((*o).device.gpe_block);if ACPI_SUCCESS(status){(*o).device.gpe_block=core::ptr::null_mut();}}let _=acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);status}

pub unsafe fn acpi_get_gpe_device(index:u32,out:*mut acpi_handle)->acpi_status{if out.is_null(){return AE_BAD_PARAMETER;}if index>=acpi_current_gpe_count{return AE_NOT_EXIST;}let mut i=acpi_gpe_device_info{index,status:AE_NOT_EXIST,gpe_device:core::ptr::null_mut(),next_block_base_index:0};let s=acpi_ev_walk_gpe_list(acpi_ev_get_gpe_device,&mut i as *mut _ as *mut _);if ACPI_FAILURE(s){return s;}*out=i.gpe_device as acpi_handle;i.status}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
