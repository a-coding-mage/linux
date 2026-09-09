// SPDX-License-Identifier: GPL-2.0
/*
 * This code provides functions to handle gcc's profiling data format
 * introduced with gcc 4.7.
 *
 * This file is based heavily on gcc_3_4.c file.
 *
 * Uses gcc-internal data definitions.
 */

// Kernel and gcov definitions supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_void};

const GCOV_COUNTERS: usize = 9;
const GCOV_TAG_FUNCTION_LENGTH: u32 = 3;
const GCOV_UNIT_SIZE: usize = 1;

type gcov_type = i64;

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gcov_link {
    pub link: u32,
    pub name: *const c_char,
}

#[repr(C)]
pub struct gcov_ctr_info {
    pub num: u32,
    pub values: *mut gcov_type,
}

#[repr(C)]
pub struct gcov_fn_info {
    pub key: *const gcov_info,
    pub ident: u32,
    pub lineno_checksum: u32,
    pub cfg_checksum: u32,
    pub ctrs: [gcov_ctr_info; 0],
}

#[repr(C)]
pub struct gcov_info {
    pub version: u32,
    pub next: *mut gcov_info,
    pub stamp: u32,
    pub filename: *const c_char,
    pub merge: [Option<unsafe extern "C" fn(*mut gcov_type, u32)>; GCOV_COUNTERS],
    pub n_functions: u32,
    pub functions: *mut *mut gcov_fn_info,
}

extern "C" {
    static mut gcov_info_head: *mut gcov_info;
    static gcov_link: [gcov_link; 2];
    fn within_module(addr: usize, module: *mut module) -> bool;
    fn memset(dst: *mut c_void, value: c_int, count: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
    fn store_gcov_u32(buffer: *mut c_char, pos: usize, value: u32) -> usize;
    fn store_gcov_u64(buffer: *mut c_char, pos: usize, value: gcov_type) -> usize;
    fn kmemdup(src: *const c_void, size: usize, flags: u32) -> *mut gcov_info;
    fn kstrdup(src: *const c_char, flags: u32) -> *mut c_char;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kzalloc_objs(size: usize, count: u32) -> *mut *mut gcov_fn_info;
    fn kvmalloc(size: usize, flags: u32) -> *mut gcov_type;
    fn kvfree(ptr: *mut gcov_type);
    fn kfree(ptr: *mut c_void);
}

const GCOV_DATA_MAGIC: u32 = 0;
const GCOV_TAG_FUNCTION: u32 = 0;

#[inline]
unsafe fn gcov_tag_for_counter(counter: u32) -> u32 {
    GCOV_TAG_FUNCTION.wrapping_add(counter)
}

pub unsafe fn gcov_info_filename(info: *mut gcov_info) -> *const c_char { (*info).filename }

pub unsafe fn gcov_info_version(info: *mut gcov_info) -> u32 { (*info).version }

pub unsafe fn gcov_info_next(info: *mut gcov_info) -> *mut gcov_info {
    if info.is_null() { gcov_info_head } else { (*info).next }
}

pub unsafe fn gcov_info_link(info: *mut gcov_info) {
    (*info).next = gcov_info_head;
    gcov_info_head = info;
}

pub unsafe fn gcov_info_unlink(prev: *mut gcov_info, info: *mut gcov_info) {
    if !prev.is_null() { (*prev).next = (*info).next; }
    else { gcov_info_head = (*info).next; }
}

pub unsafe fn gcov_info_within_module(info: *mut gcov_info, module: *mut module) -> bool {
    within_module(info as usize, module)
}

unsafe fn counter_active(info: *mut gcov_info, ty: usize) -> c_int {
    if (*info).merge[ty].is_some() { 1 } else { 0 }
}

unsafe fn num_counter_active(info: *mut gcov_info) -> u32 {
    let mut result = 0;
    for i in 0..GCOV_COUNTERS { if counter_active(info, i) != 0 { result += 1; } }
    result
}

pub unsafe fn gcov_info_reset(info: *mut gcov_info) {
    for fi_idx in 0..(*info).n_functions as usize {
        let mut ci_ptr = (*(*info).functions.add(fi_idx)).ctrs.as_mut_ptr();
        for ct_idx in 0..GCOV_COUNTERS {
            if counter_active(info, ct_idx) == 0 { continue; }
            memset((*ci_ptr).values as *mut c_void, 0,
                   core::mem::size_of::<gcov_type>() * (*ci_ptr).num as usize);
            ci_ptr = ci_ptr.add(1);
        }
    }
}

pub unsafe fn gcov_info_is_compatible(info1: *mut gcov_info, info2: *mut gcov_info) -> c_int {
    if (*info1).stamp == (*info2).stamp { 1 } else { 0 }
}

pub unsafe fn gcov_info_add(dst: *mut gcov_info, src: *mut gcov_info) {
    for fi_idx in 0..(*src).n_functions as usize {
        let mut dci_ptr = (*(*dst).functions.add(fi_idx)).ctrs.as_mut_ptr();
        let mut sci_ptr = (*(*src).functions.add(fi_idx)).ctrs.as_mut_ptr();
        for ct_idx in 0..GCOV_COUNTERS {
            if counter_active(src, ct_idx) == 0 { continue; }
            for val_idx in 0..(*sci_ptr).num as usize {
                *(*dci_ptr).values.add(val_idx) = (*(*dci_ptr).values.add(val_idx)).wrapping_add(*(*sci_ptr).values.add(val_idx));
            }
            dci_ptr = dci_ptr.add(1); sci_ptr = sci_ptr.add(1);
        }
    }
}

pub unsafe fn gcov_info_dup(info: *mut gcov_info) -> *mut gcov_info {
    let dup = kmemdup(info as *const c_void, core::mem::size_of::<gcov_info>(), 0);
    if dup.is_null() { return core::ptr::null_mut(); }
    (*dup).next = core::ptr::null_mut(); (*dup).filename = core::ptr::null(); (*dup).functions = core::ptr::null_mut();
    (*dup).filename = kstrdup((*info).filename, 0);
    if (*dup).filename.is_null() { gcov_info_free(dup); return core::ptr::null_mut(); }
    let active = num_counter_active(info);
    (*dup).functions = kzalloc_objs(core::mem::size_of::<*mut gcov_fn_info>(), (*info).n_functions);
    if (*dup).functions.is_null() { gcov_info_free(dup); return core::ptr::null_mut(); }
    let fi_size = core::mem::size_of::<gcov_fn_info>() + core::mem::size_of::<gcov_ctr_info>() * active as usize;
    for fi_idx in 0..(*info).n_functions as usize {
        *(*dup).functions.add(fi_idx) = kzalloc(fi_size, 0) as *mut gcov_fn_info;
        if (*(*dup).functions.add(fi_idx)).is_null() { gcov_info_free(dup); return core::ptr::null_mut(); }
        *(*(*dup).functions.add(fi_idx)) = *(*(*info).functions.add(fi_idx));
        let mut sci = (*(*info).functions.add(fi_idx)).ctrs.as_ptr();
        let mut dci = (*(*dup).functions.add(fi_idx)).ctrs.as_mut_ptr();
        for _ in 0..active {
            let size = core::mem::size_of::<gcov_type>() * (*sci).num as usize;
            (*dci).values = kvmalloc(size, 0); if (*dci).values.is_null() { gcov_info_free(dup); return core::ptr::null_mut(); }
            (*dci).num = (*sci).num; memcpy((*dci).values as *mut c_void, (*sci).values as *const c_void, size);
            sci = sci.add(1); dci = dci.add(1);
        }
    }
    dup
}

pub unsafe fn gcov_info_free(info: *mut gcov_info) {
    if !(*info).functions.is_null() {
        let active = num_counter_active(info);
        for fi_idx in 0..(*info).n_functions as usize {
            let function = *(*info).functions.add(fi_idx); if function.is_null() { continue; }
            let mut ci = (*function).ctrs.as_mut_ptr();
            for _ in 0..active { kvfree((*ci).values); ci = ci.add(1); }
            kfree(function as *mut c_void);
        }
    }
    kfree((*info).functions as *mut c_void); kfree((*info).filename as *mut c_void); kfree(info as *mut c_void);
}

pub unsafe fn convert_to_gcda(buffer: *mut c_char, info: *mut gcov_info) -> usize {
    let mut pos = 0;
    pos += store_gcov_u32(buffer, pos, GCOV_DATA_MAGIC); pos += store_gcov_u32(buffer, pos, (*info).version); pos += store_gcov_u32(buffer, pos, (*info).stamp);
    for fi_idx in 0..(*info).n_functions as usize {
        let fi = *(*info).functions.add(fi_idx);
        pos += store_gcov_u32(buffer, pos, GCOV_TAG_FUNCTION); pos += store_gcov_u32(buffer, pos, GCOV_TAG_FUNCTION_LENGTH * GCOV_UNIT_SIZE as u32);
        pos += store_gcov_u32(buffer, pos, (*fi).ident); pos += store_gcov_u32(buffer, pos, (*fi).lineno_checksum); pos += store_gcov_u32(buffer, pos, (*fi).cfg_checksum);
        let mut ci = (*fi).ctrs.as_mut_ptr();
        for ct_idx in 0..GCOV_COUNTERS {
            if counter_active(info, ct_idx) == 0 { continue; }
            pos += store_gcov_u32(buffer, pos, gcov_tag_for_counter(ct_idx as u32)); pos += store_gcov_u32(buffer, pos, (*ci).num * 2 * GCOV_UNIT_SIZE as u32);
            for cv_idx in 0..(*ci).num as usize { pos += store_gcov_u64(buffer, pos, *(*ci).values.add(cv_idx)); }
            ci = ci.add(1);
        }
    }
    pos
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
