// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/mem-info.c.
// Dependencies originally included: <linux/zalloc.h>, "mem-info.h".

use core::ffi::c_void;

#[repr(C)]
pub struct mem_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct addr_map_symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mem_info_data_src {
    pub val: u64,
}

type rc_mem_info = mem_info;

unsafe extern "C" {
    fn zalloc(size: usize) -> *mut c_void;

    fn mem_info__refcnt(mi: *mut mem_info) -> *mut refcount_t;
    fn mem_info__iaddr(mi: *mut mem_info) -> *mut addr_map_symbol;
    fn mem_info__daddr(mi: *mut mem_info) -> *mut addr_map_symbol;
    fn mem_info__data_src(mi: *mut mem_info) -> *mut mem_info_data_src;

    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    fn refcount_set(r: *mut refcount_t, n: i32);

    fn addr_map_symbol__exit(ams: *mut addr_map_symbol);
    fn addr_map_symbol__copy(dst: *mut addr_map_symbol, src: *mut addr_map_symbol);

    fn RC_CHK_GET(result: *mut *mut mem_info, mi: *mut mem_info) -> bool;
    fn RC_CHK_PUT(mi: *mut mem_info);
    fn RC_CHK_FREE(mi: *mut mem_info);
    fn ADD_RC_CHK(result: *mut *mut mem_info, mi: *mut rc_mem_info) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mem_info__get(mi: *mut mem_info) -> *mut mem_info {
    let mut result: *mut mem_info = core::ptr::null_mut();

    if unsafe { RC_CHK_GET(&mut result, mi) } {
        unsafe { refcount_inc(mem_info__refcnt(mi)) };
    }

    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mem_info__put(mi: *mut mem_info) {
    if !mi.is_null() && unsafe { refcount_dec_and_test(mem_info__refcnt(mi)) } {
        unsafe { addr_map_symbol__exit(mem_info__iaddr(mi)) };
        unsafe { addr_map_symbol__exit(mem_info__daddr(mi)) };
        unsafe { RC_CHK_FREE(mi) };
    } else {
        unsafe { RC_CHK_PUT(mi) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mem_info__new() -> *mut mem_info {
    let mut result: *mut mem_info = core::ptr::null_mut();
    let mi: *mut rc_mem_info =
        unsafe { zalloc(core::mem::size_of::<rc_mem_info>()) as *mut rc_mem_info };

    if unsafe { ADD_RC_CHK(&mut result, mi) } {
        unsafe { refcount_set(mem_info__refcnt(result), 1) };
    }

    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mem_info__clone(mi: *mut mem_info) -> *mut mem_info {
    let result: *mut mem_info = unsafe { mem_info__new() };

    if !result.is_null() {
        unsafe { addr_map_symbol__copy(mem_info__iaddr(result), mem_info__iaddr(mi)) };
        unsafe { addr_map_symbol__copy(mem_info__daddr(result), mem_info__daddr(mi)) };
        unsafe {
            (*mem_info__data_src(result)).val = (*mem_info__data_src(mi)).val;
        }
    }

    result
}
