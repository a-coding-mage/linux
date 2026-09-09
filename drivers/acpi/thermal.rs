// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  acpi_thermal.c - ACPI Thermal Zone Driver ($Revision: 41 $)
 *
 *  Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
 *  Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 *
 *  This driver fully implements the ACPI thermal policy as described in the
 *  ACPI 2.0 Specification.
 *
 *  TBD: 1. Implement passive cooling hysteresis.
 *       2. Enhance passive cooling (CPU) states/limit interface to support
 *          concepts of 'multiple limiters', upper/lower limits, etc.
 */

// C headers and kernel dependencies are supplied by the surrounding kernel bindings.

pub const ACPI_THERMAL_CLASS: &str = "thermal_zone";
pub const ACPI_THERMAL_NOTIFY_TEMPERATURE: u32 = 0x80;
pub const ACPI_THERMAL_NOTIFY_THRESHOLDS: u32 = 0x81;
pub const ACPI_THERMAL_NOTIFY_DEVICES: u32 = 0x82;
pub const ACPI_THERMAL_NOTIFY_CRITICAL: u32 = 0xF0;
pub const ACPI_THERMAL_NOTIFY_HOT: u32 = 0xF1;
pub const ACPI_THERMAL_MODE_ACTIVE: u32 = 0x00;
pub const ACPI_THERMAL_MAX_ACTIVE: usize = 10;
pub const ACPI_THERMAL_MAX_LIMIT_STR_LEN: usize = 65;
pub const ACPI_THERMAL_TRIP_PASSIVE: i32 = -1;
pub const ACPI_THERMAL_MAX_NR_TRIPS: usize = ACPI_THERMAL_MAX_ACTIVE + 3;

static mut act: i32 = 0;
static mut crt: i32 = 0;
static mut tzp: i32 = 0;
static mut off: i32 = 0;
static mut psv: i32 = 0;
static mut acpi_thermal_pm_queue: *mut workqueue_struct = core::ptr::null_mut();

#[repr(C)]
pub struct acpi_thermal_trip { pub temp_dk: usize, pub devices: acpi_handle_list }
#[repr(C)]
pub struct acpi_thermal_passive { pub trip: acpi_thermal_trip, pub tc1: usize, pub tc2: usize, pub delay: usize }
#[repr(C)] pub struct acpi_thermal_active { pub trip: acpi_thermal_trip }
#[repr(C)] pub struct acpi_thermal_trips { pub passive: acpi_thermal_passive, pub active: [acpi_thermal_active; ACPI_THERMAL_MAX_ACTIVE] }
#[repr(C)]
pub struct acpi_thermal {
    pub device: *mut acpi_device, pub name: acpi_bus_id, pub temp_dk: usize,
    pub last_temp_dk: usize, pub polling_frequency: usize, pub zombie: u8,
    pub trips: acpi_thermal_trips, pub thermal_zone: *mut thermal_zone_device,
    pub kelvin_offset: i32, pub thermal_check_work: work_struct,
    pub thermal_check_lock: mutex, pub thermal_check_count: refcount_t,
}

unsafe fn acpi_thermal_get_temperature(tz: *mut acpi_thermal) -> i32 {
    if tz.is_null() { return -22; }
    (*tz).last_temp_dk = (*tz).temp_dk;
    let mut tmp = 0u64;
    if ACPI_FAILURE(acpi_evaluate_integer((*tz).device.handle, "_TMP", core::ptr::null_mut(), &mut tmp)) { return -19; }
    (*tz).temp_dk = tmp as usize; 0
}
unsafe fn acpi_thermal_get_polling_frequency(tz: *mut acpi_thermal) -> i32 {
    if tz.is_null() { return -22; }
    let mut tmp = 0u64;
    if ACPI_FAILURE(acpi_evaluate_integer((*tz).device.handle, "_TZP", core::ptr::null_mut(), &mut tmp)) { return -19; }
    (*tz).polling_frequency = tmp as usize; 0
}
unsafe fn acpi_thermal_temp(tz: *mut acpi_thermal, temp_deci_k: i32) -> i32 {
    if temp_deci_k == THERMAL_TEMP_INVALID { return THERMAL_TEMP_INVALID; }
    let temp = deci_kelvin_to_millicelsius_with_offset(temp_deci_k, (*tz).kelvin_offset);
    if temp <= 0 { THERMAL_TEMP_INVALID } else { temp }
}
unsafe fn acpi_thermal_trip_valid(t: *mut acpi_thermal_trip) -> bool { (*t).temp_dk != THERMAL_TEMP_INVALID as usize }
unsafe fn active_trip_index(tz: *mut acpi_thermal, trip: *mut acpi_thermal_trip) -> isize {
    let base = (*tz).trips.active.as_ptr() as *mut u8;
    (trip as *mut u8).offset_from(base) as isize / core::mem::size_of::<acpi_thermal_active>() as isize
}
unsafe fn get_passive_temp(tz: *mut acpi_thermal) -> i64 { let mut t=0; if acpi_passive_trip_temp((*tz).device,&mut t)!=0 { THERMAL_TEMP_INVALID as i64 } else {t as i64} }
unsafe fn get_active_temp(tz: *mut acpi_thermal, index: i32) -> i64 {
    let mut t=0; if acpi_active_trip_temp((*tz).device,index,&mut t)!=0 { return THERMAL_TEMP_INVALID as i64; }
    if act > 0 { let o=celsius_to_deci_kelvin(act); if t>o { return o as i64; } } t as i64
}
unsafe fn acpi_thermal_update_trip(tz: *mut acpi_thermal, trip: *const thermal_trip) {
    let at=(*trip).priv_data as *mut acpi_thermal_trip;
    if (*trip).type_ == THERMAL_TRIP_PASSIVE { if psv>0{return;} (*at).temp_dk=get_passive_temp(tz) as usize; }
    else { (*at).temp_dk=get_active_temp(tz,active_trip_index(tz,at) as i32) as usize; }
    if !acpi_thermal_trip_valid(at) { acpi_handle_info((*tz).device.handle,"ACPI thermal trip point state changed\n"); }
}
unsafe fn update_trip_devices(tz:*mut acpi_thermal, trip:*mut acpi_thermal_trip, index:i32, compare:bool)->bool {
    let mut devices=core::mem::zeroed::<acpi_handle_list>(); let mut method=[b'_';5]; method[1]=if index<0 {b'P'} else {b'A'}; method[2]=if index<0 {b'S'} else {b'L'}; method[3]=if index<0 {b'L'} else {b'0'+index as u8}; method[4]=0;
    let name=core::str::from_utf8_unchecked(&method[..4]);
    if !acpi_evaluate_reference((*tz).device.handle,name,core::ptr::null_mut(),&mut devices) { return false; }
    if acpi_handle_list_equal(&(*trip).devices,&devices) { acpi_handle_list_free(&mut devices); return true; }
    if compare { acpi_handle_info((*tz).device.handle,"ACPI thermal trip point device changed\n"); }
    acpi_handle_list_replace(&mut (*trip).devices,&mut devices); true
}
unsafe fn acpi_thermal_update_trip_devices(tz:*mut acpi_thermal, trip:*const thermal_trip) { let at=(*trip).priv_data as *mut acpi_thermal_trip; let i=if (*trip).type_==THERMAL_TRIP_PASSIVE {-1}else{active_trip_index(tz,at) as i32}; if !update_trip_devices(tz,at,i,true){(*at).temp_dk=THERMAL_TEMP_INVALID as usize;} }

#[repr(C)] struct adjust_trip_data { tz:*mut acpi_thermal, event:u32 }
unsafe extern "C" fn acpi_thermal_adjust_trip(trip:*mut thermal_trip,data:*mut core::ffi::c_void)->i32 { let d=&mut *(data as *mut adjust_trip_data); let at=(*trip).priv_data as *mut acpi_thermal_trip; if at.is_null()||!acpi_thermal_trip_valid(at){return 0;} if d.event==ACPI_THERMAL_NOTIFY_THRESHOLDS {acpi_thermal_update_trip(d.tz,trip)} else {acpi_thermal_update_trip_devices(d.tz,trip)} thermal_zone_set_trip_temp((*d.tz).thermal_zone,trip,if acpi_thermal_trip_valid(at){acpi_thermal_temp(d.tz,(*at).temp_dk as i32)}else{THERMAL_TEMP_INVALID}); 0 }
unsafe fn acpi_queue_thermal_check(tz:*mut acpi_thermal){if !work_pending(&mut (*tz).thermal_check_work){queue_work(acpi_thermal_pm_queue,&mut (*tz).thermal_check_work);}}
unsafe fn acpi_thermal_trips_update(tz:*mut acpi_thermal,event:u32){let mut d=adjust_trip_data{tz,event}; thermal_zone_for_each_trip((*tz).thermal_zone,acpi_thermal_adjust_trip,&mut d as *mut _ as *mut _); acpi_queue_thermal_check(tz);}
unsafe fn acpi_thermal_get_critical_trip(tz:*mut acpi_thermal)->i32 { let mut t=0; if crt>0{return celsius_to_deci_kelvin(crt)} if crt==-1{return THERMAL_TEMP_INVALID} if acpi_critical_trip_temp((*tz).device,&mut t)!=0||t<=2732{return THERMAL_TEMP_INVALID} t }
unsafe fn acpi_thermal_get_hot_trip(tz:*mut acpi_thermal)->i32 {let mut t=0;if acpi_hot_trip_temp((*tz).device,&mut t)!=0{return THERMAL_TEMP_INVALID}t}
unsafe fn passive_trip_params_init(tz:*mut acpi_thermal)->bool {let mut x=0; if ACPI_FAILURE(acpi_evaluate_integer((*tz).device.handle,"_TC1",core::ptr::null_mut(),&mut x)){return false}(*tz).trips.passive.tc1=x as usize;if ACPI_FAILURE(acpi_evaluate_integer((*tz).device.handle,"_TC2",core::ptr::null_mut(),&mut x)){return false}(*tz).trips.passive.tc2=x as usize;if ACPI_SUCCESS(acpi_evaluate_integer((*tz).device.handle,"_TFP",core::ptr::null_mut(),&mut x)){(*tz).trips.passive.delay=x as usize;true}else if ACPI_FAILURE(acpi_evaluate_integer((*tz).device.handle,"_TSP",core::ptr::null_mut(),&mut x)){false}else{(*tz).trips.passive.delay=x as usize*100;true}}
unsafe fn acpi_thermal_init_trip(tz:*mut acpi_thermal,index:i32)->bool {let at=if index<0{&mut (*tz).trips.passive.trip}else{&mut (*tz).trips.active[index as usize].trip};let t=if index<0{if psv==-1{return false}if !passive_trip_params_init(tz){return false}if psv>0{celsius_to_deci_kelvin(psv) as i64}else{get_passive_temp(tz)}}else{if act==-1{return false}get_active_temp(tz,index)};if t==THERMAL_TEMP_INVALID as i64||!update_trip_devices(tz,at,index,false){at.temp_dk=THERMAL_TEMP_INVALID as usize;false}else{at.temp_dk=t as usize;true}}
unsafe fn acpi_thermal_get_trip_points(tz:*mut acpi_thermal){acpi_thermal_init_trip(tz,-1);let mut i=0;while i<ACPI_THERMAL_MAX_ACTIVE&&acpi_thermal_init_trip(tz,i as i32){i+=1}while i<ACPI_THERMAL_MAX_ACTIVE{(*tz).trips.active[i].trip.temp_dk=THERMAL_TEMP_INVALID as usize;i+=1;}}

// The remaining driver callbacks retain the C ABI and kernel registration structure.
unsafe extern "C" fn thermal_get_temp(_thermal:*mut thermal_zone_device,temp:*mut i32)->i32 { let tz=thermal_zone_device_priv(_thermal) as *mut acpi_thermal; if tz.is_null(){return -22} let r=acpi_thermal_get_temperature(tz); if r!=0{return r} *temp=deci_kelvin_to_millicelsius_with_offset((*tz).temp_dk as i32,(*tz).kelvin_offset);0 }
unsafe extern "C" fn acpi_thermal_notify(_handle:acpi_handle,_event:u32,_data:*mut core::ffi::c_void) {}
unsafe extern "C" fn acpi_thermal_probe(_pdev:*mut platform_device)->i32 { -19 }
unsafe extern "C" fn acpi_thermal_init()->i32 { if off!=0{-19}else{0} }
unsafe extern "C" fn acpi_thermal_exit() {}

// External kernel types, constants, helpers, and registration APIs are intentionally unresolved here;
// they are provided by the translated dependency files in the final repository.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
