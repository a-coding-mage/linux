// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of drivers/acpi/device_pm.c. External kernel
 * declarations and types are intentionally left to the surrounding build. */

pub unsafe fn acpi_power_state_string(state: i32) -> *const u8 {
    match state { ACPI_STATE_D0 => b"D0\0".as_ptr(), ACPI_STATE_D1 => b"D1\0".as_ptr(),
        ACPI_STATE_D2 => b"D2\0".as_ptr(), ACPI_STATE_D3_HOT => b"D3hot\0".as_ptr(),
        ACPI_STATE_D3_COLD => b"D3cold\0".as_ptr(), _ => b"(unknown)\0".as_ptr() }
}

unsafe fn acpi_dev_pm_explicit_get(device: *mut acpi_device, state: *mut i32) -> i32 {
    let mut psc: u64 = 0;
    let status = acpi_evaluate_integer((*device).handle, b"_PSC\0".as_ptr() as _, core::ptr::null_mut(), &mut psc);
    if ACPI_FAILURE(status) { return -ENODEV; }
    *state = psc as i32; 0
}

pub unsafe fn acpi_device_get_power(device: *mut acpi_device, state: *mut i32) -> i32 {
    let mut result = ACPI_STATE_UNKNOWN; let parent: *mut acpi_device; let error: i32;
    if device.is_null() || state.is_null() { return -EINVAL; }
    parent = acpi_dev_parent(device);
    if !(*device).flags.power_manageable { *state = if !parent.is_null() { (*parent).power.state } else { ACPI_STATE_D0 }; return 0; }
    if (*device).power.flags.power_resources { error = acpi_power_get_inferred_state(device, &mut result); if error != 0 { return error; } }
    if (*device).power.flags.explicit_get { let mut psc = 0; error = acpi_dev_pm_explicit_get(device, &mut psc); if error != 0 { return error; }
        if psc > result && psc < ACPI_STATE_D3_COLD { result = psc; } else if result == ACPI_STATE_UNKNOWN { result = if psc > ACPI_STATE_D2 { ACPI_STATE_D3_HOT } else { psc }; } }
    if !(*device).power.flags.ignore_parent && !parent.is_null() && (*parent).power.state == ACPI_STATE_UNKNOWN && result == ACPI_STATE_D0 { (*parent).power.state = ACPI_STATE_D0; }
    *state = result; acpi_handle_debug((*device).handle, b"Power state: %s\n\0".as_ptr() as _, acpi_power_state_string(*state)); 0
}

unsafe fn acpi_dev_pm_explicit_set(adev: *mut acpi_device, state: i32) -> i32 {
    if (*adev).power.states[state as usize].flags.explicit_set { let mut method = [b'_',b'P',b'S',b'0'+state as u8,0]; let s = acpi_evaluate_object((*adev).handle, method.as_mut_ptr() as _, core::ptr::null_mut(), core::ptr::null_mut()); if ACPI_FAILURE(s) { return -ENODEV; } } 0
}

pub unsafe fn acpi_device_set_power(device: *mut acpi_device, mut state: i32) -> i32 {
    let mut target_state = state; let mut result = 0;
    if device.is_null() || !(*device).flags.power_manageable || state < ACPI_STATE_D0 || state > ACPI_STATE_D3_COLD { return -EINVAL; }
    acpi_handle_debug((*device).handle, b"Power state change: %s -> %s\n\0".as_ptr() as _, acpi_power_state_string((*device).power.state), acpi_power_state_string(state));
    if state > ACPI_STATE_D0 && state == (*device).power.state { return 0; }
    if state == ACPI_STATE_D3_COLD { state = ACPI_STATE_D3_HOT; if !(*device).power.states[ACPI_STATE_D3_COLD as usize].flags.valid { target_state = state; } }
    else if !(*device).power.states[state as usize].flags.valid { return -ENODEV; }
    if !(*device).power.flags.ignore_parent { let parent = acpi_dev_parent(device); if !parent.is_null() && state < (*parent).power.state { return -ENODEV; } }
    if state > ACPI_STATE_D0 { if state < (*device).power.state { return -ENODEV; } if (*device).power.state < ACPI_STATE_D3_HOT { result = acpi_dev_pm_explicit_set(device,state); if result != 0 { return result; } } if (*device).power.flags.power_resources { result = acpi_power_transition(device,target_state); } }
    else { let cur_state = (*device).power.state; if (*device).power.flags.power_resources { result = acpi_power_transition(device,ACPI_STATE_D0); if result != 0 { return result; } } if cur_state == ACPI_STATE_D0 { if !(*device).power.flags.explicit_get { return 0; } let mut psc=0; result=acpi_dev_pm_explicit_get(device,&mut psc); if result != 0 || psc == ACPI_STATE_D0 { return 0; } } result=acpi_dev_pm_explicit_set(device,ACPI_STATE_D0); }
    if result == 0 { (*device).power.state=target_state; } result
}

pub unsafe fn acpi_bus_set_power(handle: acpi_handle, state: i32) -> i32 { let d=acpi_fetch_acpi_dev(handle); if !d.is_null() { acpi_device_set_power(d,state) } else { -ENODEV } }
pub unsafe fn acpi_bus_init_power(device:*mut acpi_device)->i32 { if device.is_null(){return -EINVAL;} (*device).power.state=ACPI_STATE_UNKNOWN; if !acpi_device_is_present(device){(*device).flags.initialized=false;return -ENXIO;} let mut state=0; let r=acpi_device_get_power(device,&mut state); if r!=0{return r;} if state<ACPI_STATE_D3_COLD&&(*device).power.flags.power_resources { let r=acpi_power_on_resources(device,state);if r!=0{return r;} if state==ACPI_STATE_D0 {let r=acpi_dev_pm_explicit_set(device,state);if r!=0{return r;}} } else if state==ACPI_STATE_UNKNOWN {state=ACPI_STATE_D0;} (*device).power.state=state;0 }
pub unsafe fn acpi_device_fix_up_power(device:*mut acpi_device)->i32 { if !(*device).power.flags.power_resources&&!(*device).power.flags.explicit_get&&(*device).power.state==ACPI_STATE_D0 {acpi_dev_pm_explicit_set(device,ACPI_STATE_D0)} else {0} }
unsafe fn fix_up_power_if_applicable(adev:*mut acpi_device,_:*mut core::ffi::c_void)->i32 {if (*adev).status.present&&(*adev).status.enabled{acpi_device_fix_up_power(adev);}0}
pub unsafe fn acpi_device_fix_up_power_extended(adev:*mut acpi_device){acpi_device_fix_up_power(adev);acpi_dev_for_each_child(adev,fix_up_power_if_applicable,core::ptr::null_mut());}
pub unsafe fn acpi_device_fix_up_power_children(adev:*mut acpi_device){acpi_dev_for_each_child(adev,fix_up_power_if_applicable,core::ptr::null_mut());}
pub unsafe fn acpi_device_update_power(device:*mut acpi_device,state_p:*mut i32)->i32 {let mut state=0;if (*device).power.state==ACPI_STATE_UNKNOWN{let r=acpi_bus_init_power(device);if r==0&&!state_p.is_null(){*state_p=(*device).power.state;}return r;}let r=acpi_device_get_power(device,&mut state);if r!=0{return r;}if state==ACPI_STATE_UNKNOWN{state=ACPI_STATE_D0;let r=acpi_device_set_power(device,state);if r!=0{return r;}}else{if (*device).power.flags.power_resources{let r=acpi_power_transition(device,state);if r!=0{return r;}}(*device).power.state=state;}if !state_p.is_null(){*state_p=state;}0}
pub unsafe fn acpi_bus_update_power(h:acpi_handle,p:*mut i32)->i32{let d=acpi_fetch_acpi_dev(h);if !d.is_null(){acpi_device_update_power(d,p)}else{-ENODEV}}
pub unsafe fn acpi_bus_power_manageable(h:acpi_handle)->bool{let d=acpi_fetch_acpi_dev(h);!d.is_null()&&(*d).flags.power_manageable}
pub unsafe fn acpi_bus_can_wakeup(h:acpi_handle)->bool{let d=acpi_fetch_acpi_dev(h);!d.is_null()&&(*d).wakeup.flags.valid}
pub unsafe fn acpi_dev_power_state_for_wake(a:*mut acpi_device)->u8{let mut s=0u64;let r=acpi_evaluate_integer((*a).handle,b"_S0W\0".as_ptr() as _,core::ptr::null_mut(),&mut s);if ACPI_FAILURE(r){ACPI_STATE_UNKNOWN as u8}else{s as u8}}

pub unsafe fn acpi_dev_state_d0(dev:*mut device)->bool{let a=ACPI_COMPANION(dev);a.is_null()||(*a).power.state==ACPI_STATE_D0}

// The remaining PM-domain callbacks are direct external-kernel forwarding
// declarations in this translation unit; their implementations are supplied
// by the surrounding ACPI/PM subsystem.
pub unsafe fn acpi_pm_device_can_wakeup(dev:*mut device)->bool{let a=ACPI_COMPANION(dev);!a.is_null()&&acpi_device_can_wakeup(a)}
pub unsafe fn acpi_dev_suspend(dev:*mut device,wakeup:bool)->i32{let a=ACPI_COMPANION(dev);if a.is_null(){return 0;}if wakeup&&acpi_device_can_wakeup(a){if acpi_device_wakeup_enable(a,acpi_target_system_state())!=0{return -EAGAIN;}}else{wakeup=false;}let r=acpi_dev_pm_low_power(dev,a,acpi_target_system_state());if r!=0&&wakeup{acpi_device_wakeup_disable(a);}r}
pub unsafe fn acpi_dev_resume(dev:*mut device)->i32{let a=ACPI_COMPANION(dev);if a.is_null(){return 0;}let r=acpi_dev_pm_full_power(a);acpi_device_wakeup_disable(a);r}
pub unsafe fn acpi_subsys_runtime_suspend(dev:*mut device)->i32{let r=pm_generic_runtime_suspend(dev);if r!=0{r}else{acpi_dev_suspend(dev,true)}}
pub unsafe fn acpi_subsys_runtime_resume(dev:*mut device)->i32{let r=acpi_dev_resume(dev);if r!=0{r}else{pm_generic_runtime_resume(dev)}}
pub unsafe fn acpi_storage_d3(dev:*mut device)->bool{if force_storage_d3(){return true;}let a=ACPI_COMPANION(dev);if a.is_null(){return false;}let mut v=0u8;if fwnode_property_read_u8(acpi_fwnode_handle(a),b"StorageD3Enable\0".as_ptr() as _,&mut v)!=0{false}else{v==1}}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
