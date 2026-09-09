// SPDX-License-Identifier: GPL-2.0-only
/*
 * acpi_lpat.c - LPAT table processing functions
 *
 * Copyright (C) 2015 Intel Corporation. All rights reserved.
 */

use core::ffi::c_void;

pub const ENOENT: i32 = 2;
pub const ACPI_ALLOCATE_BUFFER: usize = usize::MAX;
pub const ACPI_TYPE_INTEGER: u32 = 1;
pub const ACPI_TYPE_PACKAGE: u32 = 2;

#[repr(C)]
pub struct acpi_lpat {
    pub raw: i32,
    pub temp: i32,
}

#[repr(C)]
pub struct acpi_lpat_conversion_table {
    pub lpat: *mut acpi_lpat,
    pub lpat_count: i32,
}

#[repr(C)]
pub struct acpi_buffer {
    pub length: usize,
    pub pointer: *mut c_void,
}

#[repr(C)]
pub struct acpi_object_integer {
    pub value: u64,
}

#[repr(C)]
pub struct acpi_object_package {
    pub count: usize,
    pub elements: *mut acpi_object,
}

#[repr(C)]
pub union acpi_object {
    pub type_: u32,
    pub integer: acpi_object_integer,
    pub package: acpi_object_package,
}

pub type acpi_handle = *mut c_void;
pub type acpi_status = u64;

unsafe extern "C" {
    fn acpi_evaluate_object(
        handle: acpi_handle,
        pathname: *const core::ffi::c_char,
        parameter_objects: *mut c_void,
        return_object_buffer: *mut acpi_buffer,
    ) -> acpi_status;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

#[inline]
unsafe fn acpi_failure(status: acpi_status) -> bool {
    status != 0
}

pub unsafe fn acpi_lpat_raw_to_temp(
    lpat_table: *mut acpi_lpat_conversion_table,
    raw: i32,
) -> i32 {
    let mut i: i32 = 0;
    let table = &*lpat_table;
    let lpat = table.lpat;

    while i < table.lpat_count - 1 {
        let current = &*lpat.add(i as usize);
        let next = &*lpat.add((i + 1) as usize);
        if (raw >= current.raw && raw <= next.raw)
            || (raw <= current.raw && raw >= next.raw)
        {
            break;
        }
        i += 1;
    }

    if i == table.lpat_count - 1 {
        return -ENOENT;
    }

    let current = &*lpat.add(i as usize);
    let next = &*lpat.add((i + 1) as usize);
    let delta_temp = next.temp - current.temp;
    let delta_raw = next.raw - current.raw;
    current.temp + (raw - current.raw) * delta_temp / delta_raw
}

pub unsafe fn acpi_lpat_temp_to_raw(
    lpat_table: *mut acpi_lpat_conversion_table,
    temp: i32,
) -> i32 {
    let mut i: i32 = 0;
    let table = &*lpat_table;
    let lpat = table.lpat;

    while i < table.lpat_count - 1 {
        let current = &*lpat.add(i as usize);
        let next = &*lpat.add((i + 1) as usize);
        if temp >= current.temp && temp <= next.temp {
            break;
        }
        i += 1;
    }

    if i == table.lpat_count - 1 {
        return -ENOENT;
    }

    let current = &*lpat.add(i as usize);
    let next = &*lpat.add((i + 1) as usize);
    let delta_temp = next.temp - current.temp;
    let delta_raw = next.raw - current.raw;
    current.raw + (temp - current.temp) * delta_raw / delta_temp
}

pub unsafe fn acpi_lpat_get_conversion_table(
    handle: acpi_handle,
) -> *mut acpi_lpat_conversion_table {
    let mut lpat_table: *mut acpi_lpat_conversion_table = core::ptr::null_mut();
    let mut buffer = acpi_buffer {
        length: ACPI_ALLOCATE_BUFFER,
        pointer: core::ptr::null_mut(),
    };
    let status = acpi_evaluate_object(
        handle,
        b"LPAT\0".as_ptr() as *const core::ffi::c_char,
        core::ptr::null_mut(),
        &mut buffer,
    );
    if acpi_failure(status) {
        return core::ptr::null_mut();
    }

    let obj_p = buffer.pointer as *mut acpi_object;
    if obj_p.is_null() {
        kfree(buffer.pointer);
        return lpat_table;
    }
    let obj_type = unsafe { (*obj_p).type_ };
    let package = unsafe { (*obj_p).package };
    if obj_type != ACPI_TYPE_PACKAGE || package.count % 2 != 0 || package.count < 4 {
        kfree(buffer.pointer);
        return lpat_table;
    }

    let lpat = kzalloc(package.count * core::mem::size_of::<i32>(), 0) as *mut i32;
    if lpat.is_null() {
        kfree(buffer.pointer);
        return lpat_table;
    }

    for i in 0..package.count {
        let obj_e = &*package.elements.add(i);
        if obj_e.type_ != ACPI_TYPE_INTEGER {
            kfree(lpat as *mut c_void);
            kfree(buffer.pointer);
            return lpat_table;
        }
        *lpat.add(i) = obj_e.integer.value as i64 as i32;
    }

    lpat_table = kzalloc(core::mem::size_of::<acpi_lpat_conversion_table>(), 0)
        as *mut acpi_lpat_conversion_table;
    if lpat_table.is_null() {
        kfree(lpat as *mut c_void);
        kfree(buffer.pointer);
        return lpat_table;
    }

    (*lpat_table).lpat = lpat as *mut acpi_lpat;
    (*lpat_table).lpat_count = (package.count / 2) as i32;
    kfree(buffer.pointer);
    lpat_table
}

pub unsafe fn acpi_lpat_free_conversion_table(
    lpat_table: *mut acpi_lpat_conversion_table,
) {
    if !lpat_table.is_null() {
        kfree((*lpat_table).lpat as *mut c_void);
        kfree(lpat_table as *mut c_void);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
