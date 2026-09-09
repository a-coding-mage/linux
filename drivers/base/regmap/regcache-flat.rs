// SPDX-License-Identifier: GPL-2.0
//
// Register cache access API - flat caching support
//
// Copyright 2012 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Linux headers and "internal.h" provide these declarations.
extern "C" {
    fn regcache_get_index_by_order(map: *const Regmap, reg: c_uint) -> c_uint;
    fn kzalloc_flex(size: usize, flags: c_uint) -> *mut RegcacheFlatData;
    fn bitmap_zalloc(nbits: c_uint, flags: c_uint) -> *mut c_ulong;
    fn bitmap_free(bitmap: *mut c_ulong);
    fn kfree(ptr: *mut c_void);
    fn __set_bit(nr: c_uint, addr: *mut c_ulong);
    fn test_bit(nr: c_uint, addr: *const c_ulong) -> c_int;
    fn bitmap_clear(addr: *mut c_ulong, start: c_uint, nbits: c_uint);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_warn_once(dev: *mut c_void, fmt: *const c_char, ...);
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;

#[repr(C)]
pub struct Regmap {
    pub reg_stride_order: c_int,
    pub max_register_is_set: bool,
    pub max_register: c_uint,
    pub alloc_flags: c_uint,
    pub cache: *mut c_void,
    pub num_reg_defaults: usize,
    pub reg_defaults: *mut RegDefault,
    pub reg_default_cb: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    pub dev: *mut c_void,
    pub reg_stride: c_uint,
}

#[repr(C)]
pub struct RegDefault {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct RegcacheFlatData {
    pub valid: *mut c_ulong,
    pub data: [c_uint; 0],
}

#[repr(C)]
pub struct RegcacheOps {
    pub type_: c_int,
    pub name: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut Regmap) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut Regmap)>,
    pub populate: Option<unsafe extern "C" fn(*mut Regmap) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut Regmap, c_uint, *mut c_uint) -> c_int>,
    pub write: Option<unsafe extern "C" fn(*mut Regmap, c_uint, c_uint) -> c_int>,
    pub drop: Option<unsafe extern "C" fn(*mut Regmap, c_uint, c_uint) -> c_int>,
}

const REGCACHE_FLAT: c_int = 0;
const REGCACHE_FLAT_S: c_int = 1;

#[inline]
unsafe fn regcache_flat_get_index(map: *const Regmap, reg: c_uint) -> c_uint {
    regcache_get_index_by_order(map, reg)
}

unsafe extern "C" fn regcache_flat_init(map: *mut Regmap) -> c_int {
    if map.is_null() || (*map).reg_stride_order < 0 || !(*map).max_register_is_set {
        return -EINVAL;
    }

    let cache_size = regcache_flat_get_index(map, (*map).max_register) + 1;
    let cache = kzalloc_flex(
        core::mem::size_of::<RegcacheFlatData>() + cache_size as usize * core::mem::size_of::<c_uint>(),
        (*map).alloc_flags,
    );
    if cache.is_null() {
        return -ENOMEM;
    }

    (*cache).valid = bitmap_zalloc(cache_size, (*map).alloc_flags);
    if (*cache).valid.is_null() {
        kfree(cache.cast());
        return -ENOMEM;
    }

    (*map).cache = cache.cast();
    0
}

unsafe extern "C" fn regcache_flat_exit(map: *mut Regmap) {
    let cache = (*map).cache.cast::<RegcacheFlatData>();
    if !cache.is_null() {
        bitmap_free((*cache).valid);
    }
    kfree(cache.cast());
    (*map).cache = core::ptr::null_mut();
}

unsafe extern "C" fn regcache_flat_populate(map: *mut Regmap) -> c_int {
    let cache = (*map).cache.cast::<RegcacheFlatData>();
    for i in 0..(*map).num_reg_defaults {
        let d = &*(*map).reg_defaults.add(i);
        let index = regcache_flat_get_index(map, d.reg);
        *(*cache).data.as_mut_ptr().add(index as usize) = d.def;
        __set_bit(index, (*cache).valid);
    }

    if let Some(cb) = (*map).reg_default_cb {
        dev_dbg((*map).dev, b"Populating regcache_flat using reg_default_cb callback\n\0".as_ptr().cast());
        let mut i = 0;
        while i <= (*map).max_register {
            let index = regcache_flat_get_index(map, i);
            let mut value = 0;
            if test_bit(index, (*cache).valid) != 0 {
                i += (*map).reg_stride;
                continue;
            }
            if cb((*map).dev, i, &mut value) != 0 {
                i += (*map).reg_stride;
                continue;
            }
            *(*cache).data.as_mut_ptr().add(index as usize) = value;
            __set_bit(index, (*cache).valid);
            i += (*map).reg_stride;
        }
    }
    0
}

unsafe extern "C" fn regcache_flat_read(map: *mut Regmap, reg: c_uint, value: *mut c_uint) -> c_int {
    let cache = (*map).cache.cast::<RegcacheFlatData>();
    let index = regcache_flat_get_index(map, reg);
    if test_bit(index, (*cache).valid) == 0 {
        dev_warn_once((*map).dev, b"using zero-initialized flat cache, this may cause unexpected behavior\0".as_ptr().cast());
    }
    *value = *(*cache).data.as_ptr().add(index as usize);
    0
}

unsafe extern "C" fn regcache_flat_sparse_read(map: *mut Regmap, reg: c_uint, value: *mut c_uint) -> c_int {
    let cache = (*map).cache.cast::<RegcacheFlatData>();
    let index = regcache_flat_get_index(map, reg);
    if test_bit(index, (*cache).valid) == 0 { return -ENOENT; }
    *value = *(*cache).data.as_ptr().add(index as usize);
    0
}

unsafe extern "C" fn regcache_flat_write(map: *mut Regmap, reg: c_uint, value: c_uint) -> c_int {
    let cache = (*map).cache.cast::<RegcacheFlatData>();
    let index = regcache_flat_get_index(map, reg);
    *(*cache).data.as_mut_ptr().add(index as usize) = value;
    __set_bit(index, (*cache).valid);
    0
}

unsafe extern "C" fn regcache_flat_drop(map: *mut Regmap, min: c_uint, max: c_uint) -> c_int {
    let cache = (*map).cache.cast::<RegcacheFlatData>();
    let bitmap_min = regcache_flat_get_index(map, min);
    let bitmap_max = regcache_flat_get_index(map, max);
    bitmap_clear((*cache).valid, bitmap_min, bitmap_max + 1 - bitmap_min);
    0
}

#[no_mangle]
pub static mut regcache_flat_ops: RegcacheOps = RegcacheOps {
    type_: REGCACHE_FLAT,
    name: b"flat\0".as_ptr().cast(),
    init: Some(regcache_flat_init), exit: Some(regcache_flat_exit), populate: Some(regcache_flat_populate),
    read: Some(regcache_flat_read), write: Some(regcache_flat_write), drop: None,
};

#[no_mangle]
pub static mut regcache_flat_sparse_ops: RegcacheOps = RegcacheOps {
    type_: REGCACHE_FLAT_S,
    name: b"flat-sparse\0".as_ptr().cast(),
    init: Some(regcache_flat_init), exit: Some(regcache_flat_exit), populate: Some(regcache_flat_populate),
    read: Some(regcache_flat_sparse_read), write: Some(regcache_flat_write), drop: Some(regcache_flat_drop),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
