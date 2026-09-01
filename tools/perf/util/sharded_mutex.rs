// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/sharded_mutex.c.
// Depends on declarations normally supplied by "sharded_mutex.h".

use core::ffi::{c_uint, c_void};
use core::mem::size_of;
use core::ptr;

pub type size_t = usize;

#[repr(C)]
pub struct mutex {
    // External type supplied by the original C headers.
    _private: [u8; 0],
}

#[repr(C)]
pub struct sharded_mutex {
    pub cap_bits: c_uint,
    pub mutexes: [mutex; 0],
}

unsafe extern "C" {
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn mutex_init(mutex: *mut mutex);
    fn mutex_destroy(mutex: *mut mutex);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sharded_mutex__new(num_shards: size_t) -> *mut sharded_mutex {
    let mut result: *mut sharded_mutex;
    let size: size_t;
    let mut bits: c_uint;

    bits = 0;
    while ((1 as size_t) << bits) < num_shards {
        bits += 1;
    }

    size = size_of::<sharded_mutex>() + size_of::<mutex>() * ((1 as size_t) << bits);
    result = unsafe { malloc(size) as *mut sharded_mutex };
    if result.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        (*result).cap_bits = bits;
    }
    let mut i: size_t = 0;
    while i < ((1 as size_t) << bits) {
        unsafe {
            mutex_init((*result).mutexes.as_mut_ptr().add(i));
        }
        i += 1;
    }

    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sharded_mutex__delete(sm: *mut sharded_mutex) {
    let mut i: size_t = 0;
    while i < ((1 as size_t) << unsafe { (*sm).cap_bits }) {
        unsafe {
            mutex_destroy((*sm).mutexes.as_mut_ptr().add(i));
        }
        i += 1;
    }

    unsafe {
        free(sm as *mut c_void);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
