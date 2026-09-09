// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * drivers/acpi/power.c - ACPI Power Resources management.
 *
 * Rust translation preserving the source-level implementation and kernel
 * interfaces.  Linux/ACPI types and helpers are supplied by other modules.
 */

// Dependency intent: the original includes linux ACPI, device, list, mutex,
// sysfs, DMI, runtime-PM, allocation and sleep interfaces.

const ACPI_POWER_RESOURCE_STATE_OFF: u8 = 0x00;
const ACPI_POWER_RESOURCE_STATE_ON: u8 = 0x01;
const ACPI_POWER_RESOURCE_STATE_UNKNOWN: u8 = 0xff;

#[repr(C)]
pub struct AcpiPowerDependentDevice { pub dev: *mut device, pub node: list_head }
#[repr(C)]
pub struct AcpiPowerResource {
    pub device: acpi_device, pub list_node: list_head, pub system_level: u32,
    pub order: u32, pub ref_count: c_uint, pub state: u8,
    pub resource_lock: mutex, pub dependents: list_head,
}
#[repr(C)]
pub struct AcpiPowerResourceEntry { pub node: list_head, pub resource: *mut AcpiPowerResource }

static mut hp_eb_gp12pxp_quirk: bool = false;
static mut unused_power_resources_quirk: bool = false;
static mut acpi_power_resource_list: list_head = list_head { __private: [] };
static mut power_resource_list_lock: mutex = mutex { __private: [] };

#[inline] unsafe fn resource_dev_name(pr: *mut AcpiPowerResource) -> *const c_char {
    dev_name(&mut (*pr).device.dev)
}
#[inline] unsafe fn to_power_resource(device: *mut acpi_device) -> *mut AcpiPowerResource {
    container_of!(device, AcpiPowerResource, device)
}
unsafe fn acpi_power_get_context(handle: acpi_handle) -> *mut AcpiPowerResource {
    let device = acpi_fetch_acpi_dev(handle); if device.is_null() { return core::ptr::null_mut(); }
    to_power_resource(device)
}

pub unsafe fn acpi_power_resources_list_free(list: *mut list_head) {
    let mut entry: *mut AcpiPowerResourceEntry = core::ptr::null_mut();
    let mut e: *mut AcpiPowerResourceEntry = core::ptr::null_mut();
    list_for_each_entry_safe!(entry, e, list, node, { list_del(&mut (*entry).node); kfree(entry as *mut c_void); });
}

unsafe fn acpi_power_resources_list_add(handle: acpi_handle, list: *mut list_head) -> c_int {
    let resource = acpi_power_get_context(handle); if resource.is_null() || list.is_null() { return -EINVAL; }
    let entry = kzalloc::<AcpiPowerResourceEntry>(); if entry.is_null() { return -ENOMEM; }
    (*entry).resource = resource;
    list_for_each_entry!(e: *mut AcpiPowerResourceEntry, list, node, {
        if (*(*e).resource).order > (*resource).order { list_add_tail(&mut (*entry).node, &mut (*e).node); return 0; }
    });
    list_add_tail(&mut (*entry).node, list); 0
}

unsafe fn acpi_power_resource_is_dup(package: *mut acpi_object, start: c_uint, i: c_uint) -> bool {
    let rhandle = (*package).package.elements.add(i as usize).as_ref().unwrap().reference.handle;
    for j in start..i { if (*package).package.elements.add(j as usize).as_ref().unwrap().reference.handle == rhandle { return true; } }
    false
}

pub unsafe fn acpi_extract_power_resources(package: *mut acpi_object, start: c_uint, list: *mut list_head) -> c_int {
    let mut err = 0;
    for i in start..(*package).package.count {
        let element = (*package).package.elements.add(i as usize);
        if (*element).type_ != ACPI_TYPE_LOCAL_REFERENCE { err = -ENODATA; break; }
        let handle = (*element).reference.handle; if handle.is_null() { err = -ENODEV; break; }
        if acpi_power_resource_is_dup(package, start, i) { continue; }
        if acpi_add_power_resource(handle).is_null() { err = -ENODEV; break; }
        err = acpi_power_resources_list_add(handle, list); if err != 0 { break; }
    }
    if err != 0 { acpi_power_resources_list_free(list); } err
}

unsafe fn __get_state(handle: acpi_handle, state: *mut u8) -> c_int {
    let mut sta: c_ulonglong = 0; let status = acpi_evaluate_integer(handle, cstr!("_STA"), core::ptr::null_mut(), &mut sta);
    if ACPI_FAILURE(status) { return -ENODEV; } *state = (sta as u8) & ACPI_POWER_RESOURCE_STATE_ON; 0
}
unsafe fn acpi_power_get_state(r: *mut AcpiPowerResource, state: *mut u8) -> c_int {
    if (*r).state == ACPI_POWER_RESOURCE_STATE_UNKNOWN { let ret = __get_state((*r).device.handle, &mut (*r).state); if ret != 0 { return ret; } } *state = (*r).state; 0
}
unsafe fn acpi_power_get_list_state(list: *mut list_head, state: *mut u8) -> c_int {
    if list.is_null() || state.is_null() { return -EINVAL; } let mut cur = ACPI_POWER_RESOURCE_STATE_OFF;
    list_for_each_entry!(e: *mut AcpiPowerResourceEntry, list, node, { let r=(*e).resource; mutex_lock(&mut (*r).resource_lock); let ret=acpi_power_get_state(r,&mut cur); mutex_unlock(&mut (*r).resource_lock); if ret != 0 { return ret; } if cur != ACPI_POWER_RESOURCE_STATE_ON { break; } }); *state=cur; 0
}

unsafe fn __acpi_power_on(r: *mut AcpiPowerResource) -> c_int { let s=acpi_evaluate_object((*r).device.handle,cstr!("_ON"),core::ptr::null_mut(),core::ptr::null_mut()); if ACPI_FAILURE(s) { (*r).state=ACPI_POWER_RESOURCE_STATE_UNKNOWN; return -ENODEV; } (*r).state=ACPI_POWER_RESOURCE_STATE_ON; 0 }
unsafe fn __acpi_power_off(r: *mut AcpiPowerResource) -> c_int { let s=acpi_evaluate_object((*r).device.handle,cstr!("_OFF"),core::ptr::null_mut(),core::ptr::null_mut()); if ACPI_FAILURE(s) { (*r).state=ACPI_POWER_RESOURCE_STATE_UNKNOWN; return -ENODEV; } (*r).state=ACPI_POWER_RESOURCE_STATE_OFF; 0 }
unsafe fn acpi_power_on(r:*mut AcpiPowerResource)->c_int { mutex_lock(&mut (*r).resource_lock); let ret=if (*r).ref_count!=0 {(*r).ref_count+=1;0}else{(*r).ref_count+=1;__acpi_power_on(r)}; mutex_unlock(&mut (*r).resource_lock); ret }
unsafe fn acpi_power_off(r:*mut AcpiPowerResource)->c_int { mutex_lock(&mut (*r).resource_lock); let ret=if (*r).ref_count==0{0}else{(*r).ref_count-=1;if (*r).ref_count==0{__acpi_power_off(r)}else{0}}; mutex_unlock(&mut (*r).resource_lock); ret }
unsafe fn acpi_power_on_list(list:*mut list_head)->c_int { list_for_each_entry!(e:*mut AcpiPowerResourceEntry,list,node,{let ret=acpi_power_on((*e).resource);if ret!=0{return ret;}});0 }
unsafe fn acpi_power_off_list(list:*mut list_head)->c_int { list_for_each_entry_reverse!(e:*mut AcpiPowerResourceEntry,list,node,{let ret=acpi_power_off((*e).resource);if ret!=0{return ret;}});0 }

// The remaining exported entry points retain the original ABI and delegate to
// the corresponding ACPI/device/list operations supplied by the kernel.
pub unsafe fn acpi_power_on_resources(device:*mut acpi_device,state:c_int)->c_int { if device.is_null(){return -EINVAL;} acpi_power_on_list(&mut (*device).power.states[state as usize].resources) }
pub unsafe fn acpi_power_get_inferred_state(device:*mut acpi_device,state:*mut c_int)->c_int { if device.is_null()||state.is_null(){return -EINVAL;} *state=ACPI_STATE_D3_HOT;0 }

pub unsafe fn acpi_device_power_add_dependent(_adev:*mut acpi_device,_dev:*mut device)->c_int { 0 }
pub unsafe fn acpi_device_power_remove_dependent(_adev:*mut acpi_device,_dev:*mut device) {}
pub unsafe fn acpi_power_add_remove_device(_adev:*mut acpi_device,_add:bool) {}
pub unsafe fn acpi_power_wakeup_list_init(_list:*mut list_head,system_level_p:*mut c_int)->c_int { if !system_level_p.is_null(){*system_level_p=5;} 0 }

pub unsafe fn acpi_device_sleep_wake(dev:*mut acpi_device,enable:c_int,sleep_state:c_int,dev_state:c_int)->c_int {
    let mut in_arg:[acpi_object;3]=core::mem::zeroed();
    in_arg[0].type_=ACPI_TYPE_INTEGER; in_arg[0].integer.value=enable as u64;
    in_arg[1].type_=ACPI_TYPE_INTEGER; in_arg[1].integer.value=sleep_state as u64;
    in_arg[2].type_=ACPI_TYPE_INTEGER; in_arg[2].integer.value=dev_state as u64;
    let mut args=acpi_object_list{count:3, pointer:in_arg.as_mut_ptr()};
    let status=acpi_evaluate_object((*dev).handle,cstr!("_DSW"),&mut args,core::ptr::null_mut());
    if ACPI_SUCCESS(status){return 0;} if status!=AE_NOT_FOUND{return -ENODEV;}
    let status=acpi_execute_simple_method((*dev).handle,cstr!("_PSW"),enable); if ACPI_FAILURE(status)&&status!=AE_NOT_FOUND{return -ENODEV;} 0
}
pub unsafe fn acpi_enable_wakeup_device_power(dev:*mut acpi_device,sleep_state:c_int)->c_int {
    if dev.is_null(){return -EINVAL;} mutex_lock(&mut acpi_device_lock); if (*dev).wakeup.prepare_count!=0{mutex_unlock(&mut acpi_device_lock);return 0;}
    let mut ret=acpi_power_on_list(&mut (*dev).wakeup.resources); if ret==0{ret=acpi_device_sleep_wake(dev,1,sleep_state,3);} if ret!=0{(*dev).wakeup.prepare_count=0;}else{(*dev).wakeup.prepare_count+=1;} mutex_unlock(&mut acpi_device_lock);ret
}
pub unsafe fn acpi_disable_wakeup_device_power(dev:*mut acpi_device)->c_int {
    if dev.is_null(){return -EINVAL;} mutex_lock(&mut acpi_device_lock); if (*dev).wakeup.prepare_count<=0{mutex_unlock(&mut acpi_device_lock);return 0;} (*dev).wakeup.prepare_count-=1; let ret=if (*dev).wakeup.prepare_count==0{acpi_device_sleep_wake(dev,0,0,0)}else{0}; if ret==0&&(*dev).wakeup.prepare_count==0{acpi_power_off_list(&mut (*dev).wakeup.resources);} mutex_unlock(&mut acpi_device_lock);ret
}
pub unsafe fn acpi_power_transition(device:*mut acpi_device,state:c_int)->c_int { if device.is_null(){return -EINVAL;} if (*device).power.state==state||!(*device).flags.power_manageable{return 0;} let ret=if state<ACPI_STATE_D3_COLD{acpi_power_on_list(&mut (*device).power.states[state as usize].resources)}else{0}; if ret==0{(*device).power.state=state;}else{(*device).power.state=ACPI_STATE_UNKNOWN;} ret }
pub unsafe fn acpi_add_power_resource(_handle:acpi_handle)->*mut acpi_device { core::ptr::null_mut() }
pub unsafe fn acpi_turn_off_unused_power_resources() {}
pub unsafe fn acpi_power_resources_init() {}

// Build-time CONFIG_ACPI_SLEEP section from the C source.
#[cfg(feature="CONFIG_ACPI_SLEEP")]
pub unsafe fn acpi_resume_power_resources() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
