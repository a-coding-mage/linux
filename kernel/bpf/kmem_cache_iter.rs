// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2024 Google */

// Dependencies supplied by the surrounding kernel/BPF build are intentionally
// referenced here rather than reimplemented.

#[repr(C, align(8))]
pub struct bpf_iter_kmem_cache {
    pub __opaque: [u64; 1],
}

#[repr(C, align(8))]
pub struct bpf_iter_kmem_cache_kern {
    pub pos: *mut kmem_cache,
}

pub const KMEM_CACHE_POS_START: *mut kmem_cache = 1 as *mut kmem_cache;

extern "C" {
    static mut slab_mutex: mutex;
    static mut slab_caches: list_head;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn list_empty(head: *const list_head) -> bool;
    fn list_first_entry(head: *mut list_head) -> *mut kmem_cache;
    fn list_last_entry(head: *mut list_head) -> *mut kmem_cache;
    fn list_next_entry(entry: *mut kmem_cache) -> *mut kmem_cache;
    fn kmem_cache_destroy(cache: *mut kmem_cache);
    fn bpf_iter_get_info(meta: *mut bpf_iter_meta, in_stop: bool) -> *mut bpf_prog;
    fn bpf_iter_run_prog(prog: *mut bpf_prog, ctx: *mut bpf_iter__kmem_cache) -> i32;
    fn seq_puts(seq: *mut seq_file, s: *const core::ffi::c_char);
    fn bpf_iter_reg_target(info: *mut bpf_iter_reg) -> i32;
}

// External kernel types and registration declarations.
#[repr(C)] pub struct kmem_cache { pub list: list_head, pub refcount: i32 }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct seq_file { pub private: *mut core::ffi::c_void }
#[repr(C)] pub struct bpf_prog { _private: [u8; 0] }
#[repr(C)] pub struct bpf_iter_meta { pub seq: *mut seq_file }
#[repr(C)] pub struct bpf_iter_aux_info { _private: [u8; 0] }
#[repr(C)] pub struct bpf_iter_seq_info { pub seq_ops: *const seq_operations, pub seq_priv_size: usize }
#[repr(C)] pub struct seq_operations {
    pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut i64) -> *mut core::ffi::c_void>,
    pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void, *mut i64) -> *mut core::ffi::c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void)>,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>,
}
#[repr(C)] pub struct bpf_iter_reg { pub target: *const core::ffi::c_char, pub feature: u32, pub show_fdinfo: Option<unsafe extern "C" fn(*const bpf_iter_aux_info, *mut seq_file)>, pub ctx_arg_info_size: u32, pub ctx_arg_info: [bpf_ctx_arg_info; 1], pub seq_info: *const bpf_iter_seq_info }
#[repr(C)] pub struct bpf_ctx_arg_info { pub offset: usize, pub type_: u32, pub btf_id: u32 }

#[repr(C)]
pub struct bpf_iter__kmem_cache {
    pub meta: *mut bpf_iter_meta,
    pub s: *mut kmem_cache,
}

#[repr(C)]
pub union kmem_cache_iter_priv {
    pub it: bpf_iter_kmem_cache,
    pub kit: bpf_iter_kmem_cache_kern,
}

#[no_mangle]
pub unsafe extern "C" fn bpf_iter_kmem_cache_new(it: *mut bpf_iter_kmem_cache) -> i32 {
    let kit = it as *mut bpf_iter_kmem_cache_kern;
    (*kit).pos = KMEM_CACHE_POS_START;
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_iter_kmem_cache_next(it: *mut bpf_iter_kmem_cache) -> *mut kmem_cache {
    let kit = it as *mut bpf_iter_kmem_cache_kern;
    let prev = (*kit).pos;
    let mut next: *mut kmem_cache;
    let mut destroy = false;
    if prev.is_null() { return core::ptr::null_mut(); }
    mutex_lock(&raw mut slab_mutex);
    if list_empty(&raw const slab_caches) { mutex_unlock(&raw mut slab_mutex); return core::ptr::null_mut(); }
    if prev == KMEM_CACHE_POS_START { next = list_first_entry(&raw mut slab_caches); }
    else if list_last_entry(&raw mut slab_caches) == prev { next = core::ptr::null_mut(); }
    else { next = list_next_entry(prev); }
    if !next.is_null() && (*next).refcount > 0 { (*next).refcount += 1; }
    if !prev.is_null() && prev != KMEM_CACHE_POS_START {
        if (*prev).refcount > 1 { (*prev).refcount -= 1; }
        else if (*prev).refcount == 1 { destroy = true; }
    }
    mutex_unlock(&raw mut slab_mutex);
    if destroy { kmem_cache_destroy(prev); }
    (*kit).pos = next;
    next
}

#[no_mangle]
pub unsafe extern "C" fn bpf_iter_kmem_cache_destroy(it: *mut bpf_iter_kmem_cache) {
    let kit = it as *mut bpf_iter_kmem_cache_kern;
    let s = (*kit).pos;
    let mut destroy = false;
    if s.is_null() || s == KMEM_CACHE_POS_START { return; }
    mutex_lock(&raw mut slab_mutex);
    if (*s).refcount > 1 { (*s).refcount -= 1; }
    else if (*s).refcount == 1 { destroy = true; }
    mutex_unlock(&raw mut slab_mutex);
    if destroy { kmem_cache_destroy(s); }
}

#[no_mangle]
pub unsafe extern "C" fn kmem_cache_iter_seq_start(seq: *mut seq_file, pos: *mut i64) -> *mut core::ffi::c_void {
    let mut cnt = 0i64;
    let mut found = false;
    let p = (*seq).private as *mut kmem_cache_iter_priv;
    mutex_lock(&raw mut slab_mutex);
    // list_for_each_entry: locate the entry at the requested position.
    let mut s = list_first_entry(&raw mut slab_caches);
    while !s.is_null() {
        if cnt == *pos { if (*s).refcount > 0 { (*s).refcount += 1; } found = true; break; }
        cnt += 1;
        s = list_next_entry(s);
    }
    mutex_unlock(&raw mut slab_mutex);
    if !found { s = core::ptr::null_mut(); }
    (*p).kit.pos = s;
    s as *mut core::ffi::c_void
}

#[no_mangle]
pub unsafe extern "C" fn kmem_cache_iter_seq_stop(seq: *mut seq_file, v: *mut core::ffi::c_void) {
    let mut meta = bpf_iter_meta { seq };
    let mut ctx = bpf_iter__kmem_cache { meta: &mut meta, s: v as *mut kmem_cache };
    let prog = bpf_iter_get_info(&mut meta, true);
    if !prog.is_null() && ctx.s.is_null() { bpf_iter_run_prog(prog, &mut ctx); }
    let p = (*seq).private as *mut kmem_cache_iter_priv;
    bpf_iter_kmem_cache_destroy(&mut (*p).it);
}

#[no_mangle]
pub unsafe extern "C" fn kmem_cache_iter_seq_next(seq: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut i64) -> *mut core::ffi::c_void {
    let p = (*seq).private as *mut kmem_cache_iter_priv;
    *pos += 1;
    bpf_iter_kmem_cache_next(&mut (*p).it) as *mut core::ffi::c_void
}

#[no_mangle]
pub unsafe extern "C" fn kmem_cache_iter_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let mut meta = bpf_iter_meta { seq };
    let mut ctx = bpf_iter__kmem_cache { meta: &mut meta, s: v as *mut kmem_cache };
    let prog = bpf_iter_get_info(&mut meta, false);
    if prog.is_null() { 0 } else { bpf_iter_run_prog(prog, &mut ctx) }
}

pub unsafe extern "C" fn bpf_iter_kmem_cache_show_fdinfo(_aux: *const bpf_iter_aux_info, seq: *mut seq_file) {
    seq_puts(seq, b"kmem_cache iter\n\0".as_ptr() as *const core::ffi::c_char);
}

pub unsafe extern "C" fn bpf_iter_kmem_cache_init() -> i32 {
    bpf_iter_reg_target(&raw mut bpf_kmem_cache_reg_info)
}

// The remaining BPF registration metadata is supplied by the kernel's macro
// environment; this declaration preserves the source-level registration hook.
extern "C" {
    static mut bpf_kmem_cache_reg_info: bpf_iter_reg;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
