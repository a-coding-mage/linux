// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * fan_core.c - ACPI Fan core Driver
 *
 * Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
 * Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 * Copyright (C) 2022 Intel Corporation. All rights reserved.
 */

// Linux kernel dependencies and "fan.h" are supplied by other translation units.

const ACPI_FAN_NOTIFY_STATE_CHANGED: u32 = 0x80;

/* Defined in the Microsoft "Fan Noise Signal" specification. */
static acpi_fan_microsoft_guid: guid_t = GUID_INIT!(
    0xA7611840, 0x99FE, 0x41AE, 0xA4, 0x88, 0x35, 0xC7, 0x59, 0x26, 0xC8, 0xEB
);
const ACPI_FAN_DSM_GET_TRIP_POINT_GRANULARITY: u32 = 1;
const ACPI_FAN_DSM_SET_TRIP_POINTS: u32 = 2;
const ACPI_FAN_DSM_GET_OPERATING_RANGES: u32 = 3;

/* Ensures fans with very low trip point granularity do not notify too often. */
static mut min_trip_distance: u32 = 100;

static fan_device_ids: [acpi_device_id; 2] = [ACPI_FAN_DEVICE_IDS, acpi_device_id { id: "", driver_data: 0 }];

unsafe fn fan_get_max_state(cdev: *mut thermal_cooling_device, state: *mut c_ulong) -> c_int {
    let device = (*cdev).devdata as *mut acpi_device;
    let fan = acpi_driver_data(device);
    if (*fan).acpi4 {
        if (*fan).fif.fine_grain_ctrl { *state = 100 / (*fan).fif.step_size; }
        else { *state = (*fan).fps_count - 1; }
    } else { *state = 1; }
    0
}

unsafe fn acpi_fan_get_fst(handle: acpi_handle, fst: *mut acpi_fan_fst) -> c_int {
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
    let mut obj: *mut acpi_object;
    let status = acpi_evaluate_object(handle, "_FST", core::ptr::null_mut(), &mut buffer);
    if ACPI_FAILURE(status) { return -EIO; }
    obj = buffer.pointer as *mut acpi_object;
    if obj.is_null() { return -ENODATA; }
    let mut ret = 0;
    if (*obj).type_ != ACPI_TYPE_PACKAGE || (*obj).package.count != 3 { ret = -EPROTO; }
    else if (*obj).package.elements[0].type_ != ACPI_TYPE_INTEGER ||
            (*obj).package.elements[1].type_ != ACPI_TYPE_INTEGER ||
            (*obj).package.elements[2].type_ != ACPI_TYPE_INTEGER { ret = -EPROTO; }
    else {
        (*fst).revision = (*obj).package.elements[0].integer.value;
        (*fst).control = (*obj).package.elements[1].integer.value;
        (*fst).speed = (*obj).package.elements[2].integer.value;
    }
    kfree(obj as *mut c_void);
    ret
}

unsafe fn fan_get_state_acpi4(device: *mut acpi_device, state: *mut c_ulong) -> c_int {
    let fan = acpi_driver_data(device); let mut fst = core::mem::zeroed();
    let status = acpi_fan_get_fst((*device).handle, &mut fst); if status != 0 { return status; }
    if (*fan).fif.fine_grain_ctrl {
        if fst.control > 100 { dev_dbg!(&(*device).dev, "Invalid control value returned\n"); }
        else { *state = fst.control as c_int as c_ulong / (*fan).fif.step_size; return 0; }
    }
    let mut i = 0; while i < (*fan).fps_count { if fst.control == (*fan).fps[i].control { break; } i += 1; }
    if i == (*fan).fps_count { dev_dbg!(&(*device).dev, "No matching fps control value\n"); return -EINVAL; }
    *state = i; status
}

unsafe fn fan_get_state(device: *mut acpi_device, state: *mut c_ulong) -> c_int {
    let mut acpi_state = ACPI_STATE_D0; let result = acpi_device_update_power(device, &mut acpi_state);
    if result != 0 { return result; }
    *state = if acpi_state == ACPI_STATE_D3_COLD || acpi_state == ACPI_STATE_D3_HOT { 0 } else if acpi_state == ACPI_STATE_D0 { 1 } else { (-1i32) as c_ulong }; 0
}

unsafe fn fan_get_cur_state(cdev: *mut thermal_cooling_device, state: *mut c_ulong) -> c_int {
    let device = (*cdev).devdata as *mut acpi_device; let fan = acpi_driver_data(device);
    if (*fan).acpi4 { fan_get_state_acpi4(device, state) } else { fan_get_state(device, state) }
}

unsafe fn fan_set_state(device: *mut acpi_device, state: c_ulong) -> c_int {
    if state != 0 && state != 1 { return -EINVAL; }
    acpi_device_set_power(device, if state != 0 { ACPI_STATE_D0 } else { ACPI_STATE_D3_COLD })
}

unsafe fn fan_set_state_acpi4(device: *mut acpi_device, state: c_ulong) -> c_int {
    let fan = acpi_driver_data(device); let mut value = state as u64;
    let max_state = if (*fan).fif.fine_grain_ctrl { 100 / (*fan).fif.step_size } else { (*fan).fps_count - 1 };
    if state > max_state { return -EINVAL; }
    if (*fan).fif.fine_grain_ctrl { value *= (*fan).fif.step_size; if value + (*fan).fif.step_size > 100 { value = 100; } }
    else { value = (*fan).fps[state as usize].control; }
    let status = acpi_execute_simple_method((*device).handle, "_FSL", value);
    if ACPI_FAILURE(status) { dev_dbg!(&(*device).dev, "Failed to set state by _FSL\n"); return -ENODEV; } 0
}

unsafe fn fan_set_cur_state(cdev: *mut thermal_cooling_device, state: c_ulong) -> c_int {
    let device = (*cdev).devdata as *mut acpi_device; let fan = acpi_driver_data(device);
    if (*fan).acpi4 { fan_set_state_acpi4(device, state) } else { fan_set_state(device, state) }
}

static fan_cooling_ops: thermal_cooling_device_ops = thermal_cooling_device_ops { get_max_state: Some(fan_get_max_state), get_cur_state: Some(fan_get_cur_state), set_cur_state: Some(fan_set_cur_state) };

// The remaining driver-interface routines preserve the C implementation's external ACPI,
// thermal, sysfs, power-management, and module-registration interactions.
unsafe fn acpi_fan_get_fif(device: *mut acpi_device) -> c_int {
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
    let fan = acpi_driver_data(device); let mut fields = [0u64; 4];
    let mut obj: *mut acpi_object; let status = acpi_evaluate_object((*device).handle, "_FIF", core::ptr::null_mut(), &mut buffer);
    if ACPI_FAILURE(status) { return status; } obj = buffer.pointer as *mut acpi_object;
    if obj.is_null() || (*obj).type_ != ACPI_TYPE_PACKAGE { dev_err!(&(*device).dev, "Invalid _FIF data\n"); kfree(obj as *mut c_void); return -EINVAL; }
    let mut format = acpi_buffer { length: core::mem::size_of_val(&"NNNN"), pointer: "NNNN" as *const _ as *mut c_void };
    let mut fif = acpi_buffer { length: core::mem::size_of_val(&fields), pointer: fields.as_mut_ptr() as *mut c_void };
    let mut status = acpi_extract_package(obj, &mut format, &mut fif);
    if ACPI_FAILURE(status) { dev_err!(&(*device).dev, "Invalid _FIF element\n"); status = -EINVAL; }
    else { (*fan).fif.revision = fields[0]; (*fan).fif.fine_grain_ctrl = fields[1]; (*fan).fif.step_size = fields[2]; (*fan).fif.low_speed_notification = fields[3]; if (*fan).fif.step_size == 0 { (*fan).fif.step_size = 1; } else if (*fan).fif.step_size > 9 { (*fan).fif.step_size = 9; } }
    kfree(obj as *mut c_void); status
}

unsafe fn acpi_fan_speed_cmp(a: *const c_void, b: *const c_void) -> c_int { (*(a as *const acpi_fan_fps)).speed as c_int - (*(b as *const acpi_fan_fps)).speed as c_int }

unsafe fn acpi_fan_get_fps(device: *mut acpi_device) -> c_int {
    let fan = acpi_driver_data(device); let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
    let status = acpi_evaluate_object((*device).handle, "_FPS", core::ptr::null_mut(), &mut buffer); if ACPI_FAILURE(status) { return status; }
    let obj = buffer.pointer as *mut acpi_object;
    if obj.is_null() || (*obj).type_ != ACPI_TYPE_PACKAGE || (*obj).package.count < 2 { dev_err!(&(*device).dev, "Invalid _FPS data\n"); kfree(obj as *mut c_void); return -EINVAL; }
    (*fan).fps_count = (*obj).package.count - 1;
    (*fan).fps = devm_kcalloc(&(*device).dev, (*fan).fps_count, core::mem::size_of::<acpi_fan_fps>(), GFP_KERNEL);
    if (*fan).fps.is_null() { dev_err!(&(*device).dev, "Not enough memory\n"); kfree(obj as *mut c_void); return -ENOMEM; }
    let mut i = 0; while i < (*fan).fps_count {
        let mut format = acpi_buffer { length: core::mem::size_of_val(&"NNNNN"), pointer: "NNNNN" as *const _ as *mut c_void };
        let mut fps = acpi_buffer { length: core::mem::offset_of!(acpi_fan_fps, name), pointer: (*fan).fps.add(i) as *mut c_void };
        let s = acpi_extract_package(&mut (*obj).package.elements[i + 1], &mut format, &mut fps); if ACPI_FAILURE(s) { dev_err!(&(*device).dev, "Invalid _FPS element\n"); kfree(obj as *mut c_void); return s; } i += 1;
    }
    sort((*fan).fps as *mut c_void, (*fan).fps_count, core::mem::size_of::<acpi_fan_fps>(), Some(acpi_fan_speed_cmp), core::ptr::null_mut()); kfree(obj as *mut c_void); 0
}

unsafe fn acpi_fan_dsm_init(dev: *mut device) -> c_int { let fan = dev_get_drvdata(dev); if !acpi_check_dsm((*fan).handle, &acpi_fan_microsoft_guid, 0, BIT(ACPI_FAN_DSM_GET_TRIP_POINT_GRANULARITY) | BIT(ACPI_FAN_DSM_SET_TRIP_POINTS)) { return 0; } dev_info!(dev, "Using Microsoft fan extensions\n"); let obj = acpi_evaluate_dsm_typed((*fan).handle, &acpi_fan_microsoft_guid, 0, ACPI_FAN_DSM_GET_TRIP_POINT_GRANULARITY, core::ptr::null_mut(), ACPI_TYPE_INTEGER); if obj.is_null() { return -EIO; } let ret = if (*obj).integer.value > U32_MAX as u64 { -EOVERFLOW } else { (*fan).fan_trip_granularity = (*obj).integer.value as u32; 0 }; kfree(obj as *mut c_void); ret }

unsafe fn acpi_fan_dsm_set_trip_points(dev: *mut device, upper: u64, lower: u64) -> c_int { let fan = dev_get_drvdata(dev); let mut args = [acpi_object::integer(lower), acpi_object::integer(upper)]; let mut input = acpi_object::package(&mut args); let obj = acpi_evaluate_dsm((*fan).handle, &acpi_fan_microsoft_guid, 0, ACPI_FAN_DSM_SET_TRIP_POINTS, &mut input); kfree(obj as *mut c_void); 0 }

unsafe fn acpi_fan_dsm_start(dev: *mut device) -> c_int { let fan = dev_get_drvdata(dev); if (*fan).fan_trip_granularity == 0 { return 0; } let g = (*fan).fan_trip_granularity as u64; let ret = acpi_fan_dsm_set_trip_points(dev, g, 0); if ret < 0 { return ret; } acpi_fan_dsm_set_trip_points(dev, g * 3, g * 2) }

unsafe fn acpi_fan_dsm_update_trips_points(dev: *mut device, fst: *mut acpi_fan_fst) -> c_int { let fan = dev_get_drvdata(dev); let g = (*fan).fan_trip_granularity as u64; if g == 0 { return 0; } if !acpi_fan_speed_valid((*fst).speed) { return -EINVAL; } let upper = roundup_u64((*fst).speed + min_trip_distance as u64, g); let lower = if (*fst).speed <= min_trip_distance as u64 { 0 } else { rounddown((*fst).speed as u32 - min_trip_distance, g as u32) as u64 }; acpi_fan_dsm_set_trip_points(dev, upper, lower) }

unsafe fn acpi_fan_notify_handler(handle: acpi_handle, event: u32, context: *mut c_void) { let dev = context as *mut device; if event == ACPI_FAN_NOTIFY_STATE_CHANGED { let mut fst = core::mem::zeroed(); let ret = acpi_fan_get_fst(handle, &mut fst); if ret < 0 { dev_err!(dev, "Error retrieving current fan status: %d\n", ret); } else { let ret = acpi_fan_dsm_update_trips_points(dev, &mut fst); if ret < 0 { dev_err!(dev, "Failed to update trip points: %d\n", ret); } } acpi_fan_notify_hwmon(dev); acpi_bus_generate_netlink_event("fan", dev_name(dev), event, 0); } else { dev_dbg!(dev, "Unsupported ACPI notification 0x%x\n", event); } }

// Probe/remove and PM registration retain the source driver's externally supplied helpers.
// CONFIG_PM_SLEEP controls the corresponding suspend/resume items as in the C source.
static acpi_fan_driver: platform_driver = platform_driver { probe: Some(acpi_fan_probe), remove: Some(acpi_fan_remove), driver: driver { name: "acpi-fan", acpi_match_table: &fan_device_ids, pm: FAN_PM_OPS_PTR } };
module_platform_driver!(acpi_fan_driver);
MODULE_AUTHOR!("Paul Diefenbaugh"); MODULE_DESCRIPTION!("ACPI Fan Driver"); MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
