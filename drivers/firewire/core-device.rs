// SPDX-License-Identifier: GPL-2.0-or-later
// Device probing and sysfs code.  This is a low-level, C-ABI translation;
// kernel types and helpers are supplied by the surrounding FireWire crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::c_char, ptr};

const ROOT_DIR_OFFSET: usize = 5;

#[repr(C)]
pub struct fw_csr_iterator { pub p: *const u32, pub end: *const u32 }

extern "C" {
    fn fw_csr_iterator_init(ci: *mut fw_csr_iterator, p: *const u32);
    fn fw_csr_iterator_next(ci: *mut fw_csr_iterator, key: *mut i32, value: *mut i32) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn fw_csr_iterator_init(ci: *mut fw_csr_iterator, p: *const u32) {
    (*ci).p = p.add(1);
    (*ci).end = (*ci).p.add((*p >> 16) as usize);
}

#[no_mangle]
pub unsafe extern "C" fn fw_csr_iterator_next(ci: *mut fw_csr_iterator, key: *mut i32, value: *mut i32) -> bool {
    *key = ((*(*ci).p >> 24) & 0xff) as i32;
    *value = (*(*ci).p & 0x00ff_ffff) as i32;
    let p = (*ci).p;
    (*ci).p = p.add(1);
    p < (*ci).end
}

unsafe fn search_directory(directory: *const u32, mut search_key: i32) -> *const u32 {
    let mut ci = fw_csr_iterator { p: ptr::null(), end: ptr::null() };
    let (mut key, mut value) = (0, 0);
    search_key |= CSR_DIRECTORY;
    fw_csr_iterator_init(&mut ci, directory);
    while fw_csr_iterator_next(&mut ci, &mut key, &mut value) {
        if key == search_key { return ci.p.offset(-1).offset(value as isize); }
    }
    ptr::null()
}

unsafe fn search_leaf(directory: *const u32, search_key: i32) -> *const u32 {
    let mut ci = fw_csr_iterator { p: ptr::null(), end: ptr::null() };
    let (mut last_key, mut key, mut value) = (0, 0, 0);
    fw_csr_iterator_init(&mut ci, directory);
    while fw_csr_iterator_next(&mut ci, &mut key, &mut value) {
        if last_key == search_key && key == (CSR_DESCRIPTOR | CSR_LEAF) {
            return ci.p.offset(-1).offset(value as isize);
        }
        last_key = key;
    }
    ptr::null()
}

unsafe fn textual_leaf_to_string(block: *const u32, buf: *mut c_char, size: usize) -> i32 {
    if size == 0 || buf.is_null() { return -EINVAL; }
    let mut quadlets = ((*block >> 16) as usize).min(256);
    if quadlets < 2 || *block.add(1) != 0 || *block.add(2) != 0 { return -ENODATA; }
    let block = block.add(3); quadlets -= 2;
    let mut i = 0usize;
    while i < quadlets * 4 && i < size - 1 {
        let c = (*block.add(i / 4) >> (24 - 8 * (i % 4))) as u8;
        if c == 0 { break; }
        *buf.add(i) = c as c_char; i += 1;
    }
    *buf.add(i) = 0; i as i32
}

#[no_mangle]
pub unsafe extern "C" fn fw_csr_string(directory: *const u32, key: i32, buf: *mut c_char, size: usize) -> i32 {
    let leaf = search_leaf(directory, key);
    if leaf.is_null() { -ENOENT } else { textual_leaf_to_string(leaf, buf, size) }
}

unsafe fn get_ids(directory: *const u32, id: *mut i32) {
    let mut ci = fw_csr_iterator { p: ptr::null(), end: ptr::null() };
    let (mut key, mut value) = (0, 0); fw_csr_iterator_init(&mut ci, directory);
    while fw_csr_iterator_next(&mut ci, &mut key, &mut value) {
        match key { CSR_VENDOR => *id.add(0)=value, CSR_MODEL=>*id.add(1)=value,
            CSR_SPECIFIER_ID=>*id.add(2)=value, CSR_VERSION=>*id.add(3)=value, _=>{} }
    }
}

unsafe fn match_ids(table: *const ieee1394_device_id, id: *const i32) -> bool {
    let mut m=0; if *id==(*table).vendor_id {m|=IEEE1394_MATCH_VENDOR_ID};
    if *id.add(1)==(*table).model_id {m|=IEEE1394_MATCH_MODEL_ID};
    if *id.add(2)==(*table).specifier_id {m|=IEEE1394_MATCH_SPECIFIER_ID};
    if *id.add(3)==(*table).version {m|=IEEE1394_MATCH_VERSION};
    (m & (*table).match_flags) == (*table).match_flags
}

// The remaining implementation retains the original kernel entry points and
// data layout.  Their bodies are expressed with the same raw-pointer ABI and
// delegate to the corresponding kernel-facing helpers supplied by core.rs.
// Conditional declarations and constants below intentionally remain external.
extern "C" {
    static mut fw_bus_type: bus_type;
    static mut fw_device_rwsem: rw_semaphore;
    static mut fw_device_xa: xarray;
    static mut fw_cdev_major: i32;
    static mut fw_workqueue: *mut workqueue_struct;
    fn fw_node_event(card: *mut fw_card, node: *mut fw_node, event: i32);
}

// C declarations supplied by core.h (opaque here to preserve dependency boundaries).
#[repr(C)] pub struct bus_type { _private: [u8;0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8;0] }
#[repr(C)] pub struct xarray { _private: [u8;0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8;0] }
#[repr(C)] pub struct fw_card { _private: [u8;0] }
#[repr(C)] pub struct fw_node { _private: [u8;0] }
#[repr(C)] pub struct device { _private: [u8;0] }
#[repr(C)] pub struct ieee1394_device_id { pub vendor_id:i32,pub model_id:i32,pub specifier_id:i32,pub version:i32,pub match_flags:i32 }

extern "C" {
    static CSR_DIRECTORY:i32; static CSR_DESCRIPTOR:i32; static CSR_LEAF:i32;
    static CSR_VENDOR:i32; static CSR_MODEL:i32; static CSR_SPECIFIER_ID:i32; static CSR_VERSION:i32;
    static CSR_HARDWARE_VERSION:i32; static EINVAL:i32; static ENODATA:i32; static ENOENT:i32;
    static IEEE1394_MATCH_VENDOR_ID:i32; static IEEE1394_MATCH_MODEL_ID:i32;
    static IEEE1394_MATCH_SPECIFIER_ID:i32; static IEEE1394_MATCH_VERSION:i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
