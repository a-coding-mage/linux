// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  fan_attr.c - Create extra attributes for ACPI Fan driver
 *
 *  Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
 *  Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 *  Copyright (C) 2022 Intel Corporation. All rights reserved.
 */

// Linux kernel dependencies and "fan.h" are supplied by other translation units.

const MODULE_LICENSE: &str = "GPL";

unsafe extern "C" {
    fn sysfs_emit(buf: *mut core::ffi::c_char, fmt: *const core::ffi::c_char, ...) -> isize;
    fn sysfs_emit_at(
        buf: *mut core::ffi::c_char,
        at: isize,
        fmt: *const core::ffi::c_char,
        ...,
    ) -> isize;
    fn acpi_fan_get_fst(handle: *mut core::ffi::c_void, fst: *mut acpi_fan_fst) -> i32;
    fn sysfs_attr_init(attr: *mut attribute);
    fn sysfs_create_file(kobj: *mut kobject, attr: *mut attribute) -> i32;
    fn sysfs_remove_file(kobj: *mut kobject, attr: *mut attribute);
    fn acpi_driver_data(device: *mut acpi_device) -> *mut acpi_fan;
}

// Types and structure layouts are provided by the corresponding kernel headers.
#[repr(C)]
struct device;
#[repr(C)]
struct acpi_device {
    dev: device,
    handle: *mut core::ffi::c_void,
}
#[repr(C)]
struct device_attribute {
    attr: attribute,
    show: Option<unsafe extern "C" fn(*mut device, *mut device_attribute, *mut core::ffi::c_char) -> isize>,
    store: Option<unsafe extern "C" fn()>,
}
#[repr(C)]
struct attribute {
    name: *const core::ffi::c_char,
    mode: u16,
}
#[repr(C)]
struct kobject;
#[repr(C)]
struct acpi_fan_fst {
    speed: i64,
}
#[repr(C)]
struct acpi_fan_fps {
    control: u64,
    trip_point: u64,
    speed: u64,
    noise_level: u64,
    power: u64,
    name: [core::ffi::c_char; ACPI_FPS_NAME_LEN],
    dev_attr: device_attribute,
}
#[repr(C)]
struct acpi_fan_fif {
    fine_grain_ctrl: i32,
}
#[repr(C)]
struct acpi_fan {
    fst_speed: device_attribute,
    fine_grain_control: device_attribute,
    acpi4: bool,
    fps_count: i32,
    fps: *mut acpi_fan_fps,
    fif: acpi_fan_fif,
}

const ACPI_FPS_NAME_LEN: usize = 16;

unsafe extern "C" fn show_state(
    _dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let fps = (attr as *mut u8).sub(core::mem::offset_of!(acpi_fan_fps, dev_attr)) as *mut acpi_fan_fps;
    let mut count: isize;

    if (*fps).control == 0xFFFF_FFFF || (*fps).control > 100 {
        count = sysfs_emit(buf, c"not-defined:".as_ptr());
    } else {
        count = sysfs_emit(buf, c"%lld:".as_ptr(), (*fps).control as i64);
    }

    if (*fps).trip_point == 0xFFFF_FFFF || (*fps).trip_point > 9 {
        count += sysfs_emit_at(buf, count, c"not-defined:".as_ptr());
    } else {
        count += sysfs_emit_at(buf, count, c"%lld:".as_ptr(), (*fps).trip_point as i64);
    }

    if (*fps).speed == 0xFFFF_FFFF {
        count += sysfs_emit_at(buf, count, c"not-defined:".as_ptr());
    } else {
        count += sysfs_emit_at(buf, count, c"%lld:".as_ptr(), (*fps).speed as i64);
    }

    if (*fps).noise_level == 0xFFFF_FFFF {
        count += sysfs_emit_at(buf, count, c"not-defined:".as_ptr());
    } else {
        count += sysfs_emit_at(buf, count, c"%lld:".as_ptr(), (*fps).noise_level.wrapping_mul(100) as i64);
    }

    if (*fps).power == 0xFFFF_FFFF {
        count += sysfs_emit_at(buf, count, c"not-defined\n".as_ptr());
    } else {
        count += sysfs_emit_at(buf, count, c"%lld\n".as_ptr(), (*fps).power as i64);
    }

    count
}

unsafe extern "C" fn show_fan_speed(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let acpi_dev = (dev as *mut u8).sub(core::mem::offset_of!(acpi_device, dev)) as *mut acpi_device;
    let mut fst = core::mem::zeroed::<acpi_fan_fst>();
    let status = acpi_fan_get_fst((*acpi_dev).handle, &mut fst);
    if status != 0 {
        return status as isize;
    }
    sysfs_emit(buf, c"%lld\n".as_ptr(), fst.speed)
}

unsafe extern "C" fn show_fine_grain_control(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let acpi_dev = (dev as *mut u8).sub(core::mem::offset_of!(acpi_device, dev)) as *mut acpi_device;
    let fan = acpi_driver_data(acpi_dev);
    sysfs_emit(buf, c"%d\n".as_ptr(), (*fan).fif.fine_grain_ctrl)
}

unsafe extern "C" fn acpi_fan_create_attributes(device: *mut acpi_device) -> i32 {
    let fan = acpi_driver_data(device);
    let mut status: i32;

    sysfs_attr_init(&mut (*fan).fst_speed.attr);
    (*fan).fst_speed.show = Some(show_fan_speed);
    (*fan).fst_speed.store = None;
    (*fan).fst_speed.attr.name = c"fan_speed_rpm".as_ptr();
    (*fan).fst_speed.attr.mode = 0o444;
    status = sysfs_create_file(&mut (*device).dev as *mut device as *mut kobject, &mut (*fan).fst_speed.attr);
    if status != 0 { return status; }
    if !(*fan).acpi4 { return 0; }

    sysfs_attr_init(&mut (*fan).fine_grain_control.attr);
    (*fan).fine_grain_control.show = Some(show_fine_grain_control);
    (*fan).fine_grain_control.store = None;
    (*fan).fine_grain_control.attr.name = c"fine_grain_control".as_ptr();
    (*fan).fine_grain_control.attr.mode = 0o444;
    status = sysfs_create_file(&mut (*device).dev as *mut device as *mut kobject, &mut (*fan).fine_grain_control.attr);
    if status != 0 {
        sysfs_remove_file(&mut (*device).dev as *mut device as *mut kobject, &mut (*fan).fst_speed.attr);
        return status;
    }

    for i in 0..(*fan).fps_count {
        let fps = &mut *(*fan).fps.offset(i as isize);
        // snprintf(fps->name, ACPI_FPS_NAME_LEN, "state%d", i)
        fps.name = [0; ACPI_FPS_NAME_LEN];
        let name = format!("state{}", i);
        for (n, byte) in name.bytes().enumerate().take(ACPI_FPS_NAME_LEN - 1) {
            fps.name[n] = byte as core::ffi::c_char;
        }
        sysfs_attr_init(&mut fps.dev_attr.attr);
        fps.dev_attr.show = Some(show_state);
        fps.dev_attr.store = None;
        fps.dev_attr.attr.name = fps.name.as_ptr();
        fps.dev_attr.attr.mode = 0o444;
        status = sysfs_create_file(&mut (*device).dev as *mut device as *mut kobject, &mut fps.dev_attr.attr);
        if status != 0 {
            for j in 0..i {
                sysfs_remove_file(&mut (*device).dev as *mut device as *mut kobject, &mut (*(*fan).fps.offset(j as isize)).dev_attr.attr);
            }
            sysfs_remove_file(&mut (*device).dev as *mut device as *mut kobject, &mut (*fan).fine_grain_control.attr);
            sysfs_remove_file(&mut (*device).dev as *mut device as *mut kobject, &mut (*fan).fst_speed.attr);
            return status;
        }
    }
    0
}

unsafe extern "C" fn acpi_fan_delete_attributes(device: *mut acpi_device) {
    let fan = acpi_driver_data(device);
    sysfs_remove_file(&mut (*device).dev as *mut device as *mut kobject, &mut (*fan).fst_speed.attr);
    if !(*fan).acpi4 { return; }
    for i in 0..(*fan).fps_count {
        sysfs_remove_file(&mut (*device).dev as *mut device as *mut kobject, &mut (*(*fan).fps.offset(i as isize)).dev_attr.attr);
    }
    sysfs_remove_file(&mut (*device).dev as *mut device as *mut kobject, &mut (*fan).fine_grain_control.attr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
