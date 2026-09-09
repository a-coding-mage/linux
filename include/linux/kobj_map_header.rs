/* SPDX-License-Identifier: GPL-2.0 */
/*
 * kobj_map.h
 */

use core::ffi::{c_int, c_void};

// Types supplied by the corresponding kernel dependencies.
pub type dev_t = u64;

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

pub type kobj_probe_t = unsafe extern "C" fn(
    dev: dev_t,
    part: *mut c_int,
    data: *mut c_void,
) -> *mut kobject;

#[repr(C)]
pub struct kobj_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn kobj_map(
        map: *mut kobj_map,
        dev: dev_t,
        range: usize,
        owner: *mut module,
        probe: Option<kobj_probe_t>,
        lock: Option<unsafe extern "C" fn(dev_t, *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;

    pub fn kobj_unmap(map: *mut kobj_map, dev: dev_t, range: usize);

    pub fn kobj_lookup(
        map: *mut kobj_map,
        dev: dev_t,
        part: *mut c_int,
    ) -> *mut kobject;

    pub fn kobj_map_init(probe: Option<kobj_probe_t>, lock: *mut mutex) -> *mut kobj_map;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
