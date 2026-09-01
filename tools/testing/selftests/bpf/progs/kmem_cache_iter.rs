// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Google */

/* Original C dependencies:
 * #include <vmlinux.h>
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 * #include "bpf_experimental.h"
 */

pub const SLAB_NAME_MAX: usize = 32;
pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_MAP_TYPE_ARRAY: u32 = 2;
pub const BPF_NOEXIST: u64 = 1;

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct kmem_cache_result {
    pub name: [::core::ffi::c_char; SLAB_NAME_MAX],
    pub obj_size: ::core::ffi::c_long,
}

#[repr(C)]
pub struct kmem_cache {
    pub name: *const ::core::ffi::c_char,
    pub size: u32,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
}

#[repr(C)]
pub struct bpf_iter__kmem_cache {
    pub meta: *mut bpf_iter_meta,
    pub s: *mut kmem_cache,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut slab_hash: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: ::core::mem::size_of::<*mut ::core::ffi::c_void>() as u32,
    value_size: SLAB_NAME_MAX as u32,
    max_entries: 1,
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut slab_result: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    key_size: ::core::mem::size_of::<::core::ffi::c_int>() as u32,
    value_size: ::core::mem::size_of::<kmem_cache_result>() as u32,
    max_entries: 1024,
};

unsafe extern "C" {
    #[link_name = "bpf_get_kmem_cache"]
    pub fn bpf_get_kmem_cache(addr: u64) -> *mut kmem_cache;
    pub fn bpf_get_current_task() -> u64;
    pub fn bpf_map_lookup_elem(
        map: *mut ::core::ffi::c_void,
        key: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    pub fn bpf_map_update_elem(
        map: *mut ::core::ffi::c_void,
        key: *const ::core::ffi::c_void,
        value: *const ::core::ffi::c_void,
        flags: u64,
    ) -> i64;
    pub fn bpf_probe_read_kernel_str(
        dst: *mut ::core::ffi::c_void,
        size: u32,
        unsafe_ptr: *const ::core::ffi::c_void,
    ) -> i64;
    pub fn bpf_strncmp(
        s1: *const ::core::ffi::c_char,
        s1_sz: u32,
        s2: *const ::core::ffi::c_char,
    ) -> i32;
    pub fn bpf_seq_printf(
        seq: *mut seq_file,
        fmt: *const ::core::ffi::c_char,
        fmt_size: u32,
        data: *const ::core::ffi::c_void,
        data_len: u32,
    ) -> i64;
}

/* Result, will be checked by userspace */
#[unsafe(no_mangle)]
pub static mut task_struct_found: ::core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut kmem_cache_seen: ::core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut open_coded_seen: ::core::ffi::c_int = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "iter/kmem_cache")]
pub unsafe extern "C" fn slab_info_collector(ctx: *mut bpf_iter__kmem_cache) -> ::core::ffi::c_int {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let s: *mut kmem_cache = (*ctx).s;
    let mut r: *mut kmem_cache_result;
    let mut idx: ::core::ffi::c_int;

    if !s.is_null() {
        /* To make sure if the slab_iter implements the seq interface
         * properly and it's also useful for debugging.
         */
        let fmt = b"%s: %u\n\0";
        let mut seq_data = [(*s).name as u64, (*s).size as u64];
        bpf_seq_printf(
            seq,
            fmt.as_ptr() as *const ::core::ffi::c_char,
            fmt.len() as u32,
            seq_data.as_mut_ptr() as *const ::core::ffi::c_void,
            ::core::mem::size_of_val(&seq_data) as u32,
        );

        idx = kmem_cache_seen;
        r = bpf_map_lookup_elem(
            &raw mut slab_result as *mut ::core::ffi::c_void,
            &raw const idx as *const ::core::ffi::c_void,
        ) as *mut kmem_cache_result;
        if r.is_null() {
            return 0;
        }

        kmem_cache_seen += 1;

        /* Save name and size to match /proc/slabinfo */
        bpf_probe_read_kernel_str(
            (*r).name.as_mut_ptr() as *mut ::core::ffi::c_void,
            ::core::mem::size_of_val(&(*r).name) as u32,
            (*s).name as *const ::core::ffi::c_void,
        );
        (*r).obj_size = (*s).size as ::core::ffi::c_long;

        if bpf_strncmp((*r).name.as_ptr(), 11, c"task_struct".as_ptr()) == 0 {
            bpf_map_update_elem(
                &raw mut slab_hash as *mut ::core::ffi::c_void,
                &raw const s as *const ::core::ffi::c_void,
                (*r).name.as_ptr() as *const ::core::ffi::c_void,
                BPF_NOEXIST,
            );
        }
    }

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/bpf_test_finish")]
pub unsafe extern "C" fn check_task_struct() -> ::core::ffi::c_int {
    let curr: u64 = bpf_get_current_task();
    let mut s: *mut kmem_cache;
    let name: *mut ::core::ffi::c_char;

    s = bpf_get_kmem_cache(curr);
    if s.is_null() {
        task_struct_found = -1;
        return 0;
    }
    name = bpf_map_lookup_elem(
        &raw mut slab_hash as *mut ::core::ffi::c_void,
        &raw const s as *const ::core::ffi::c_void,
    ) as *mut ::core::ffi::c_char;
    if !name.is_null() && bpf_strncmp(name, 11, c"task_struct".as_ptr()) == 0 {
        task_struct_found = 1;
    } else {
        task_struct_found = -2;
    }
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
pub unsafe extern "C" fn open_coded_iter(_ctx: *const ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut s: *mut kmem_cache;

    /* Original C uses bpf_for_each(kmem_cache, s), supplied by bpf_experimental.h.
     * The iterator mechanism is an external BPF macro facility; its loop body is
     * translated below as the per-element behavior.
     */
    loop {
        let r: *mut kmem_cache_result;

        // TODO: bind the next kmem_cache element provided by bpf_for_each(kmem_cache, s).
        s = ::core::ptr::null_mut();
        if s.is_null() {
            break;
        }

        r = bpf_map_lookup_elem(
            &raw mut slab_result as *mut ::core::ffi::c_void,
            &raw const open_coded_seen as *const ::core::ffi::c_void,
        ) as *mut kmem_cache_result;
        if r.is_null() {
            break;
        }

        if (*r).obj_size != (*s).size as ::core::ffi::c_long {
            break;
        }

        open_coded_seen += 1;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
