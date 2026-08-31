// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/addr_location.c.
// Dependencies from addr_location.h, map.h, maps.h, and thread.h are external.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulonglong};

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_location {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
    pub srcline: *mut c_char,
    pub addr: c_ulonglong,
    pub level: c_int,
    pub filtered: c_int,
    pub cpumode: c_int,
    pub cpu: c_int,
    pub socket: c_int,
    pub parallelism: c_int,
}

unsafe extern "C" {
    fn map__zput(map: *mut map);
    fn thread__zput(thread: *mut thread);
    fn thread__put(thread: *mut thread);
    fn map__put(map: *mut map);
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn map__get(map: *mut map) -> *mut map;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn addr_location__init(al: *mut addr_location) {
    unsafe {
        (*al).thread = core::ptr::null_mut();
        (*al).map = core::ptr::null_mut();
        (*al).sym = core::ptr::null_mut();
        (*al).srcline = core::ptr::null_mut();
        (*al).addr = 0;
        (*al).level = 0;
        (*al).filtered = 0;
        (*al).cpumode = 0;
        (*al).cpu = 0;
        (*al).socket = 0;
        (*al).parallelism = 1;
    }
}

/*
 * The preprocess_sample method will return with reference counts for the
 * in it, when done using (and perhaps getting ref counts if needing to
 * keep a pointer to one of those entries) it must be paired with
 * addr_location__exit(), so that the refcounts can be decremented.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn addr_location__exit(al: *mut addr_location) {
    unsafe {
        map__zput((*al).map);
        thread__zput((*al).thread);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn addr_location__copy(
    dst: *mut addr_location,
    src: *mut addr_location,
) {
    unsafe {
        thread__put((*dst).thread);
        map__put((*dst).map);
        *dst = *src;
        (*dst).thread = thread__get((*src).thread);
        (*dst).map = map__get((*src).map);
    }
}
