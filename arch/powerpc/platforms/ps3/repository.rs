// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PS3 repository routines.
 *
 *  Copyright (C) 2006 Sony Computer Entertainment Inc.
 *  Copyright 2006 Sony Corp.
 */

#[allow(non_camel_case_types)]
type u64 = std::os::raw::c_ulonglong;

#[repr(C)]
pub enum ps3_vendor_id {
    PS3_VENDOR_ID_NONE = 0,
    PS3_VENDOR_ID_SONY = 0x8000000000000000u64,
}

#[repr(C)]
pub enum ps3_lpar_id {
    PS3_LPAR_ID_CURRENT = 0,
    PS3_LPAR_ID_PME = 1,
}

extern "C" {
    fn lv1_get_logical_partition_id(id: *mut u64);
    fn lv1_read_repository_node(
        lpar_id: u64,
        n1: u64,
        n2: u64,
        n3: u64,
        n4: u64,
        v1: *mut u64,
        v2: *mut u64,
    ) -> i32;
    fn ps3_result(result: i32) -> *const std::os::raw::c_char;
}

extern "C" {
    fn memcpy(dst: *mut std::os::raw::c_void, src: *const std::os::raw::c_void, n: usize) -> *mut std::os::raw::c_void;
    fn strnlen(s: *const std::os::raw::c_char, maxlen: usize) -> usize;
}

#[repr(C)]
pub struct ps3_repository_device {
    pub bus_type: ps3_bus_type,
    pub bus_index: std::os::raw::c_uint,
    pub bus_id: u64,
    pub dev_type: ps3_dev_type,
    pub dev_index: std::os::raw::c_uint,
    pub dev_id: u64,
}

pub type ps3_bus_type = u32;
pub type ps3_dev_type = u32;
pub type ps3_interrupt_type = u32;
pub type ps3_reg_type = u32;

extern "C" {
    fn pr_devel(fmt: *const std::os::raw::c_char, ...);
}

#[inline]
unsafe fn make_first_field(text: *const std::os::raw::c_char, index: u64) -> u64 {
    let mut n = 0u64;
    memcpy((&mut n as *mut u64).cast(), text.cast(), strnlen(text, std::mem::size_of::<u64>()));
    n.wrapping_shr(32).wrapping_add(index)
}

#[inline]
unsafe fn make_field(text: *const std::os::raw::c_char, index: u64) -> u64 {
    let mut n = 0u64;
    memcpy((&mut n as *mut u64).cast(), text.cast(), strnlen(text, std::mem::size_of::<u64>()));
    n.wrapping_add(index)
}

unsafe fn _dump_field(_hdr: *const std::os::raw::c_char, _n: u64, _func: *const std::os::raw::c_char, _line: i32) {}

unsafe fn _dump_node_name(lpar_id: u32, n1: u64, n2: u64, n3: u64, n4: u64, func: *const std::os::raw::c_char, line: i32) {
    _dump_field(b"n1: \0".as_ptr().cast(), n1, func, line);
    _dump_field(b"n2: \0".as_ptr().cast(), n2, func, line);
    _dump_field(b"n3: \0".as_ptr().cast(), n3, func, line);
    _dump_field(b"n4: \0".as_ptr().cast(), n4, func, line);
    let _ = lpar_id;
}

unsafe fn _dump_node(lpar_id: u32, n1: u64, n2: u64, n3: u64, n4: u64, v1: u64, v2: u64, func: *const std::os::raw::c_char, line: i32) {
    _dump_node_name(lpar_id, n1, n2, n3, n4, func, line);
    let _ = (v1, v2);
}

unsafe fn read_node(lpar_id: u32, n1: u64, n2: u64, n3: u64, n4: u64, v1_out: *mut u64, v2_out: *mut u64) -> i32 {
    let mut lpar_id = lpar_id as u64;
    let mut v1 = 0u64;
    let mut v2 = 0u64;
    if lpar_id == ps3_lpar_id::PS3_LPAR_ID_CURRENT as u64 {
        lv1_get_logical_partition_id(&mut lpar_id);
    }
    let result = lv1_read_repository_node(lpar_id, n1, n2, n3, n4, &mut v1, &mut v2);
    if result != 0 {
        _dump_node_name(lpar_id as u32, n1, n2, n3, n4, std::ptr::null(), 0);
        return -2;
    }
    _dump_node(lpar_id as u32, n1, n2, n3, n4, v1, v2, std::ptr::null(), 0);
    if !v1_out.is_null() { *v1_out = v1; }
    if !v2_out.is_null() { *v2_out = v2; }
    0
}

pub unsafe fn ps3_repository_read_bus_id(bus_index: u32, bus_id: *mut u64) -> i32 {
    read_node(ps3_lpar_id::PS3_LPAR_ID_PME as u32, make_first_field(b"bus\0".as_ptr().cast(), bus_index as u64), make_field(b"id\0".as_ptr().cast(), 0), 0, 0, bus_id, std::ptr::null_mut())
}

pub unsafe fn ps3_repository_read_bus_str(bus_index: u32, bus_str: *const std::os::raw::c_char, value: *mut u64) -> i32 {
    read_node(1, make_first_field(b"bus\0".as_ptr().cast(), bus_index as u64), make_field(bus_str, 0), 0, 0, value, std::ptr::null_mut())
}

pub unsafe fn ps3_repository_read_bus_type(bus_index: u32, bus_type: *mut ps3_bus_type) -> i32 {
    let mut v1 = 0u64;
    let result = read_node(1, make_first_field(b"bus\0".as_ptr().cast(), bus_index as u64), make_field(b"type\0".as_ptr().cast(), 0), 0, 0, &mut v1, std::ptr::null_mut());
    *bus_type = v1 as ps3_bus_type;
    result
}

pub unsafe fn ps3_repository_read_bus_num_dev(bus_index: u32, num_dev: *mut u32) -> i32 {
    let mut v1 = 0u64;
    let result = read_node(1, make_first_field(b"bus\0".as_ptr().cast(), bus_index as u64), make_field(b"num_dev\0".as_ptr().cast(), 0), 0, 0, &mut v1, std::ptr::null_mut());
    *num_dev = v1 as u32;
    result
}

pub unsafe fn ps3_repository_read_dev_id(bus_index: u32, dev_index: u32, dev_id: *mut u64) -> i32 {
    read_node(1, make_first_field(b"bus\0".as_ptr().cast(), bus_index as u64), make_field(b"dev\0".as_ptr().cast(), dev_index as u64), make_field(b"id\0".as_ptr().cast(), 0), 0, dev_id, std::ptr::null_mut())
}

pub unsafe fn ps3_repository_read_dev_str(bus_index: u32, dev_index: u32, dev_str: *const std::os::raw::c_char, value: *mut u64) -> i32 {
    read_node(1, make_first_field(b"bus\0".as_ptr().cast(), bus_index as u64), make_field(b"dev\0".as_ptr().cast(), dev_index as u64), make_field(dev_str, 0), 0, value, std::ptr::null_mut())
}

pub unsafe fn ps3_repository_read_dev_type(bus_index: u32, dev_index: u32, dev_type: *mut ps3_dev_type) -> i32 {
    let mut v1 = 0u64;
    let result = read_node(1, make_first_field(b"bus\0".as_ptr().cast(), bus_index as u64), make_field(b"dev\0".as_ptr().cast(), dev_index as u64), make_field(b"type\0".as_ptr().cast(), 0), 0, &mut v1, std::ptr::null_mut());
    *dev_type = v1 as ps3_dev_type;
    result
}

pub unsafe fn ps3_repository_read_dev_intr(bus_index: u32, dev_index: u32, intr_index: u32, intr_type: *mut ps3_interrupt_type, interrupt_id: *mut u32) -> i32 {
    let mut v1 = 0u64; let mut v2 = 0u64;
    let result = read_node(1, make_first_field(b"bus\0".as_ptr().cast(), bus_index as u64), make_field(b"dev\0".as_ptr().cast(), dev_index as u64), make_field(b"intr\0".as_ptr().cast(), intr_index as u64), 0, &mut v1, &mut v2);
    *intr_type = v1 as ps3_interrupt_type; *interrupt_id = v2 as u32; result
}

pub unsafe fn ps3_repository_read_dev_reg_type(bus_index: u32, dev_index: u32, reg_index: u32, reg_type: *mut ps3_reg_type) -> i32 {
    let mut v1 = 0u64;
    let result = read_node(1, make_first_field(b"bus\0".as_ptr().cast(), bus_index as u64), make_field(b"dev\0".as_ptr().cast(), dev_index as u64), make_field(b"reg\0".as_ptr().cast(), reg_index as u64), make_field(b"type\0".as_ptr().cast(), 0), &mut v1, std::ptr::null_mut());
    *reg_type = v1 as ps3_reg_type; result
}

pub unsafe fn ps3_repository_read_dev_reg_addr(bus_index: u32, dev_index: u32, reg_index: u32, bus_addr: *mut u64, len: *mut u64) -> i32 {
    read_node(1, make_first_field(b"bus\0".as_ptr().cast(), bus_index as u64), make_field(b"dev\0".as_ptr().cast(), dev_index as u64), make_field(b"reg\0".as_ptr().cast(), reg_index as u64), make_field(b"data\0".as_ptr().cast(), 0), bus_addr, len)
}

pub unsafe fn ps3_repository_read_dev_reg(bus_index: u32, dev_index: u32, reg_index: u32, reg_type: *mut ps3_reg_type, bus_addr: *mut u64, len: *mut u64) -> i32 {
    let result = ps3_repository_read_dev_reg_type(bus_index, dev_index, reg_index, reg_type);
    if result != 0 { result } else { ps3_repository_read_dev_reg_addr(bus_index, dev_index, reg_index, bus_addr, len) }
}

pub unsafe fn ps3_repository_find_device(repo: *mut ps3_repository_device) -> i32 {
    let mut tmp = *repo;
    let mut num_dev = 0u32;
    if tmp.bus_index > 10 || tmp.dev_index > 10 { std::process::abort(); }
    let mut result = ps3_repository_read_bus_num_dev(tmp.bus_index, &mut num_dev);
    if result != 0 { return result; }
    if tmp.dev_index >= num_dev { return -19; }
    result = ps3_repository_read_dev_type(tmp.bus_index, tmp.dev_index, &mut tmp.dev_type);
    if result != 0 { return result; }
    result = ps3_repository_read_dev_id(tmp.bus_index, tmp.dev_index, &mut tmp.dev_id);
    if result != 0 { return result; }
    *repo = tmp;
    0
}

pub unsafe fn ps3_repository_find_device_by_id(repo: *mut ps3_repository_device, bus_id: u64, dev_id: u64) -> i32 {
    let mut tmp = std::mem::zeroed::<ps3_repository_device>();
    let mut result = -19;
    let mut num_dev = 0u32;
    for i in 0..10 { tmp.bus_index = i; result = ps3_repository_read_bus_id(i, &mut tmp.bus_id); if result != 0 { return result; } if tmp.bus_id == bus_id { break; } }
    if tmp.bus_id != bus_id { return result; }
    result = ps3_repository_read_bus_type(tmp.bus_index, &mut tmp.bus_type); if result != 0 { return result; }
    result = ps3_repository_read_bus_num_dev(tmp.bus_index, &mut num_dev); if result != 0 { return result; }
    for i in 0..num_dev { tmp.dev_index = i; result = ps3_repository_read_dev_id(tmp.bus_index, i, &mut tmp.dev_id); if result != 0 { return result; } if tmp.dev_id == dev_id { break; } }
    if tmp.dev_id != dev_id { return result; }
    result = ps3_repository_read_dev_type(tmp.bus_index, tmp.dev_index, &mut tmp.dev_type); if result != 0 { return result; }
    *repo = tmp; 0
}

pub unsafe fn ps3_repository_find_bus(bus_type: ps3_bus_type, from: u32, bus_index: *mut u32) -> i32 {
    let mut ty = 0;
    for i in from..10 { let result = ps3_repository_read_bus_type(i, &mut ty); if result != 0 { *bus_index = u32::MAX; return result; } if ty == bus_type { *bus_index = i; return 0; } }
    *bus_index = u32::MAX; -19
}

pub unsafe fn ps3_repository_find_devices(bus_type: ps3_bus_type, callback: Option<unsafe extern "C" fn(*const ps3_repository_device) -> i32>) -> i32 {
    let mut repo = std::mem::zeroed::<ps3_repository_device>();
    repo.bus_type = bus_type;
    let mut result = ps3_repository_find_bus(repo.bus_type, 0, &mut repo.bus_index); if result != 0 { return result; }
    result = ps3_repository_read_bus_id(repo.bus_index, &mut repo.bus_id); if result != 0 { return result; }
    repo.dev_index = 0;
    loop { result = ps3_repository_find_device(&mut repo); if result == -19 { return 0; } if result != 0 { return result; } result = callback.unwrap()(&repo); if result != 0 { return result; } repo.dev_index = repo.dev_index.wrapping_add(1); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
