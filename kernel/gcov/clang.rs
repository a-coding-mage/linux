// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 Google, Inc.
 * modified from kernel/gcov/gcc_4_7.c
 *
 * This software is licensed under the terms of the GNU General Public
 * License version 2, as published by the Free Software Foundation, and
 * may be copied, distributed, and modified under those terms.
 */

// LLVM profiling data export implementation, translated from clang.c.

use core::ffi::{c_char, c_int, c_void};

type u32_t = u32;
type u64_t = u64;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gcov_link {
    pub link: u32_t,
    pub name: *const c_char,
}

pub type llvm_gcov_callback = unsafe extern "C" fn();

#[repr(C)]
pub struct gcov_info {
    pub head: list_head,
    pub filename: *const c_char,
    pub version: u32_t,
    pub checksum: u32_t,
    pub functions: list_head,
}

#[repr(C)]
pub struct gcov_fn_info {
    pub head: list_head,
    pub ident: u32_t,
    pub checksum: u32_t,
    pub cfg_checksum: u32_t,
    pub num_counters: u32_t,
    pub counters: *mut u64_t,
}

extern "C" {
    static mut gcov_lock: c_void;
    static mut gcov_events_enabled: bool;
    static mut GCOV_DATA_MAGIC: u32_t;
    static mut GCOV_TAG_FUNCTION: u32_t;
    static mut GCOV_TAG_COUNTER_BASE: u32_t;
    static mut OBJ_TREE: u32_t;

    fn kzalloc(size: usize, flags: u32_t) -> *mut c_void;
    fn kmemdup(src: *const c_void, size: usize, flags: u32_t) -> *mut c_void;
    fn kvmalloc(size: usize, flags: u32_t) -> *mut c_void;
    fn kstrdup(src: *const c_char, flags: u32_t) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn kvfree(ptr: *mut c_void);
    fn memset(dst: *mut c_void, value: c_int, size: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn gcov_event(event: u32_t, info: *mut gcov_info);
    fn within_module(addr: usize, module: *mut module) -> bool;
    fn store_gcov_u32(buffer: *mut c_char, pos: usize, value: u32_t) -> usize;
    fn store_gcov_u64(buffer: *mut c_char, pos: usize, value: u64_t) -> usize;
    fn __list_del_entry(entry: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn gcov_info_free(info: *mut gcov_info);
}

const GFP_KERNEL: u32_t = 0;
const GCOV_ADD: u32_t = 0;

static mut current_info: *mut gcov_info = core::ptr::null_mut();
static mut clang_gcov_list: list_head = list_head {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};

unsafe fn init_list_head(head: *mut list_head) {
    (*head).next = head;
    (*head).prev = head;
}

#[no_mangle]
pub unsafe extern "C" fn llvm_gcov_init(writeout: llvm_gcov_callback, _flush: llvm_gcov_callback) {
    let info = kzalloc(core::mem::size_of::<gcov_info>(), GFP_KERNEL) as *mut gcov_info;
    if info.is_null() { return; }
    init_list_head(&mut (*info).head);
    init_list_head(&mut (*info).functions);
    mutex_lock(&raw mut gcov_lock);
    list_add_tail(&mut (*info).head, &raw mut clang_gcov_list);
    current_info = info;
    writeout();
    current_info = core::ptr::null_mut();
    if gcov_events_enabled { gcov_event(GCOV_ADD, info); }
    mutex_unlock(&raw mut gcov_lock);
}

#[no_mangle]
pub unsafe extern "C" fn llvm_gcda_start_file(orig_filename: *const c_char, version: u32_t, checksum: u32_t) {
    (*current_info).filename = orig_filename;
    (*current_info).version = version;
    (*current_info).checksum = checksum;
}

#[no_mangle]
pub unsafe extern "C" fn llvm_gcda_emit_function(ident: u32_t, func_checksum: u32_t, cfg_checksum: u32_t) {
    let info = kzalloc(core::mem::size_of::<gcov_fn_info>(), GFP_KERNEL) as *mut gcov_fn_info;
    if info.is_null() { return; }
    init_list_head(&mut (*info).head);
    (*info).ident = ident;
    (*info).checksum = func_checksum;
    (*info).cfg_checksum = cfg_checksum;
    list_add_tail(&mut (*info).head, &mut (*current_info).functions);
}

#[no_mangle]
pub unsafe extern "C" fn llvm_gcda_emit_arcs(num_counters: u32_t, counters: *mut u64_t) {
    let head = (*current_info).functions.prev;
    let info = (head as *mut u8).sub(core::mem::offset_of!(gcov_fn_info, head)) as *mut gcov_fn_info;
    (*info).num_counters = num_counters;
    (*info).counters = counters;
}

#[no_mangle]
pub unsafe extern "C" fn llvm_gcda_summary_info() {}

#[no_mangle]
pub unsafe extern "C" fn llvm_gcda_end_file() {}

pub unsafe extern "C" fn gcov_info_filename(info: *mut gcov_info) -> *const c_char { (*info).filename }
pub unsafe extern "C" fn gcov_info_version(info: *mut gcov_info) -> u32_t { (*info).version }

pub unsafe extern "C" fn gcov_info_next(info: *mut gcov_info) -> *mut gcov_info {
    if info.is_null() { return core::ptr::null_mut(); }
    let head = (*info).head.next;
    if head == &raw mut clang_gcov_list { return core::ptr::null_mut(); }
    (head as *mut u8).sub(core::mem::offset_of!(gcov_info, head)) as *mut gcov_info
}

pub unsafe extern "C" fn gcov_info_link(info: *mut gcov_info) { list_add_tail(&mut (*info).head, &raw mut clang_gcov_list); }
pub unsafe extern "C" fn gcov_info_unlink(_prev: *mut gcov_info, info: *mut gcov_info) { __list_del_entry(&mut (*info).head); }
pub unsafe extern "C" fn gcov_info_within_module(info: *mut gcov_info, module: *mut module) -> bool { within_module((*info).filename as usize, module) }

pub static gcov_link: [gcov_link; 2] = [
    gcov_link { link: OBJ_TREE, name: b"gcno\0".as_ptr() as *const c_char },
    gcov_link { link: 0, name: core::ptr::null() },
];

pub unsafe extern "C" fn gcov_info_reset(info: *mut gcov_info) {
    let mut node = (*info).functions.next;
    while node != &mut (*info).functions {
        let fn_info = (node as *mut u8).sub(core::mem::offset_of!(gcov_fn_info, head)) as *mut gcov_fn_info;
        memset((*fn_info).counters as *mut c_void, 0, core::mem::size_of::<u64_t>() * (*fn_info).num_counters as usize);
        node = (*node).next;
    }
}

pub unsafe extern "C" fn gcov_info_is_compatible(info1: *mut gcov_info, info2: *mut gcov_info) -> c_int {
    if (*info1).checksum != (*info2).checksum { return 0; }
    let mut a = (*info1).functions.next;
    let mut b = (*info2).functions.next;
    loop {
        let ae = a == &mut (*info1).functions;
        let be = b == &mut (*info2).functions;
        if ae || be { return if ae && be { 1 } else { 0 }; }
        let af = (a as *mut u8).sub(core::mem::offset_of!(gcov_fn_info, head)) as *mut gcov_fn_info;
        let bf = (b as *mut u8).sub(core::mem::offset_of!(gcov_fn_info, head)) as *mut gcov_fn_info;
        if (*af).checksum != (*bf).checksum || (*af).cfg_checksum != (*bf).cfg_checksum { return 0; }
        a = (*a).next; b = (*b).next;
    }
}

pub unsafe extern "C" fn gcov_info_add(dst: *mut gcov_info, src: *mut gcov_info) {
    let mut d = (*dst).functions.next;
    let mut s = (*src).functions.next;
    while d != &mut (*dst).functions {
        let df = (d as *mut u8).sub(core::mem::offset_of!(gcov_fn_info, head)) as *mut gcov_fn_info;
        let sf = (s as *mut u8).sub(core::mem::offset_of!(gcov_fn_info, head)) as *mut gcov_fn_info;
        for i in 0..(*sf).num_counters as usize { *(*df).counters.add(i) = (*(*df).counters.add(i)).wrapping_add(*(*sf).counters.add(i)); }
        d = (*d).next; s = (*s).next;
    }
}

unsafe fn gcov_fn_info_dup(fn_info: *mut gcov_fn_info) -> *mut gcov_fn_info {
    let dup = kmemdup(fn_info as *const c_void, core::mem::size_of::<gcov_fn_info>(), GFP_KERNEL) as *mut gcov_fn_info;
    if dup.is_null() { return core::ptr::null_mut(); }
    init_list_head(&mut (*dup).head);
    let size = (*fn_info).num_counters as usize * core::mem::size_of::<u64_t>();
    (*dup).counters = kvmalloc(size, GFP_KERNEL) as *mut u64_t;
    if (*dup).counters.is_null() { kfree(dup as *mut c_void); return core::ptr::null_mut(); }
    memcpy((*dup).counters as *mut c_void, (*fn_info).counters as *const c_void, size);
    dup
}

pub unsafe extern "C" fn gcov_info_dup(info: *mut gcov_info) -> *mut gcov_info {
    let dup = kmemdup(info as *const c_void, core::mem::size_of::<gcov_info>(), GFP_KERNEL) as *mut gcov_info;
    if dup.is_null() { return core::ptr::null_mut(); }
    init_list_head(&mut (*dup).head); init_list_head(&mut (*dup).functions);
    (*dup).filename = kstrdup((*info).filename, GFP_KERNEL);
    if (*dup).filename.is_null() { gcov_info_free(dup); return core::ptr::null_mut(); }
    let mut node = (*info).functions.next;
    while node != &mut (*info).functions {
        let fn_info = (node as *mut u8).sub(core::mem::offset_of!(gcov_fn_info, head)) as *mut gcov_fn_info;
        let fn_dup = gcov_fn_info_dup(fn_info);
        if fn_dup.is_null() { gcov_info_free(dup); return core::ptr::null_mut(); }
        list_add_tail(&mut (*fn_dup).head, &mut (*dup).functions); node = (*node).next;
    }
    dup
}

pub unsafe extern "C" fn gcov_info_free(info: *mut gcov_info) {
    let mut node = (*info).functions.next;
    while node != &mut (*info).functions {
        let next = (*node).next;
        let fn_info = (node as *mut u8).sub(core::mem::offset_of!(gcov_fn_info, head)) as *mut gcov_fn_info;
        kvfree((*fn_info).counters as *mut c_void); list_del(&mut (*fn_info).head); kfree(fn_info as *mut c_void); node = next;
    }
    kfree((*info).filename as *mut c_void); kfree(info as *mut c_void);
}

pub unsafe extern "C" fn convert_to_gcda(buffer: *mut c_char, info: *mut gcov_info) -> usize {
    let mut pos = 0;
    pos += store_gcov_u32(buffer, pos, GCOV_DATA_MAGIC); pos += store_gcov_u32(buffer, pos, (*info).version); pos += store_gcov_u32(buffer, pos, (*info).checksum);
    let mut node = (*info).functions.next;
    while node != &mut (*info).functions {
        let fi = (node as *mut u8).sub(core::mem::offset_of!(gcov_fn_info, head)) as *mut gcov_fn_info;
        pos += store_gcov_u32(buffer, pos, GCOV_TAG_FUNCTION); pos += store_gcov_u32(buffer, pos, 3); pos += store_gcov_u32(buffer, pos, (*fi).ident); pos += store_gcov_u32(buffer, pos, (*fi).checksum); pos += store_gcov_u32(buffer, pos, (*fi).cfg_checksum); pos += store_gcov_u32(buffer, pos, GCOV_TAG_COUNTER_BASE); pos += store_gcov_u32(buffer, pos, (*fi).num_counters * 2);
        for i in 0..(*fi).num_counters as usize { pos += store_gcov_u64(buffer, pos, *(*fi).counters.add(i)); }
        node = (*node).next;
    }
    pos
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
