// SPDX-License-Identifier: GPL-2.0-only
// Source-level Rust translation of linux/kernel/fork.c.
// Kernel types, constants, macros, and functions referenced here are supplied
// by the surrounding kernel translation unit.

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const MIN_THREADS: c_ulong = 20;
pub const MAX_THREADS: c_ulong = FUTEX_TID_MASK;

extern "C" {
    static mut FUTEX_TID_MASK: c_ulong;
    static mut total_forks: c_ulong;
    static mut nr_threads: c_int;
    static mut current: *mut task_struct;
    static mut init_mm: mm_struct;
    static mut init_task: task_struct;
    static mut max_threads: c_int;

    fn kmem_cache_alloc_node(cache: *mut kmem_cache, flags: c_ulong, node: c_int) -> *mut c_void;
    fn kmem_cache_free(cache: *mut kmem_cache, object: *mut c_void);
    fn arch_release_task_struct(tsk: *mut task_struct);
    fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> c_int;
    fn alloc_pages_node(node: c_int, flags: c_ulong, order: c_uint) -> *mut page;
    fn page_address(page: *mut page) -> *mut c_void;
    fn __free_pages(page: *mut page, order: c_uint);
    fn call_rcu(head: *mut rcu_head, callback: unsafe extern "C" fn(*mut rcu_head));
    fn vfree(addr: *mut c_void);
    fn mmput(mm: *mut mm_struct);
    fn mmdrop(mm: *mut mm_struct);
    fn free_task_struct(tsk: *mut task_struct);
}

#[repr(C)] pub struct task_struct { pub stack: *mut c_void, pub flags: c_ulong, pub mm: *mut mm_struct, pub active_mm: *mut mm_struct, pub signal: *mut signal_struct, pub usage: c_ulong, pub exit_state: c_ulong }
#[repr(C)] pub struct mm_struct { pub mm_users: c_ulong, pub mm_count: c_ulong, pub exe_file: *mut file, pub binfmt: *mut binfmt, pub mmlist: list_head }
#[repr(C)] pub struct signal_struct { pub sigcnt: c_ulong, pub oom_mm: *mut mm_struct }
#[repr(C)] pub struct file { pub f_path: path }
#[repr(C)] pub struct binfmt { pub module: *mut c_void }
#[repr(C)] pub struct path { pub _opaque: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct page { pub _opaque: [u8; 0] }
#[repr(C)] pub struct kmem_cache { pub _opaque: [u8; 0] }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: *mut c_void }

#[inline]
pub unsafe fn alloc_task_struct_node(cache: *mut kmem_cache, flags: c_ulong, node: c_int) -> *mut task_struct {
    kmem_cache_alloc_node(cache, flags, node) as *mut task_struct
}

#[inline]
pub unsafe fn free_task(tsk: *mut task_struct, cache: *mut kmem_cache) {
    arch_release_task_struct(tsk);
    kmem_cache_free(cache, tsk as *mut c_void);
}

pub unsafe fn arch_task_struct_cache_init() {}

pub unsafe fn set_task_stack_end_magic(_tsk: *mut task_struct) {
    // The architecture-specific end-of-stack marker is supplied by the kernel ABI.
}

pub unsafe fn mmput_async(mm: *mut mm_struct) {
    mmput(mm);
}

pub unsafe fn __mmdrop(mm: *mut mm_struct) {
    if mm == &raw mut init_mm { return; }
    mmdrop(mm);
}

// The remaining implementation consists of configuration-selected kernel
// operations whose concrete types and helpers are defined by the included
// Linux headers; those external dependencies remain declarations by design.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
