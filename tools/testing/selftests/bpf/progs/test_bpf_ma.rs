// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */
// Dependencies from C includes: vmlinux.h, bpf/bpf_tracing.h,
// bpf/bpf_helpers.h, bpf_experimental.h, bpf_misc.h.

use core::ffi::c_void;

type u32 = u32;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct generic_map_value {
    pub data: *mut c_void,
}

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut bpf_map, key: *const c_void) -> *mut generic_map_value;
    fn bpf_obj_new_impl(local_type_id__k: u64, meta: *mut c_void) -> *mut c_void;
    fn bpf_percpu_obj_new_impl(local_type_id__k: u64, meta: *mut c_void) -> *mut c_void;
    fn bpf_kptr_xchg(kptr: *mut *mut c_void, ptr: *mut c_void) -> *mut c_void;
    fn bpf_obj_drop(kptr: *mut c_void);
    fn bpf_percpu_obj_drop(kptr: *mut c_void);
    fn bpf_get_current_pid_tgid() -> u64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

pub const data_sizes: [u32; 11] = [16, 32, 64, 96, 128, 192, 256, 512, 1024, 2048, 4096];
pub static mut data_btf_ids: [u32; 11] = [0; 11];

pub const percpu_data_sizes: [u32; 9] = [8, 16, 32, 64, 96, 128, 192, 256, 512];
pub static mut percpu_data_btf_ids: [u32; 11] = [0; 11];

pub static mut err: i32 = 0;
pub static mut pid: u32 = 0;

macro_rules! define_array_with_kptr {
    ($bin:ident, $bin_ptr:ident, $value:ident, $array:ident, $size:expr) => {
        #[repr(C)]
        pub struct $bin {
            pub data: [i8; $size - core::mem::size_of::<*mut c_void>()],
        }

        /* See Commit 5d8d6634ccc, force btf generation for type bin_data_##_size */
        pub static mut $bin_ptr: *mut $bin = core::ptr::null_mut();

        #[repr(C)]
        pub struct $value {
            pub data: *mut $bin,
        }

        #[repr(C)]
        pub struct $array {
            _private: [u8; 0],
        }

        // Original C map definition used SEC(".maps") with:
        // __uint(type, BPF_MAP_TYPE_ARRAY);
        // __type(key, int);
        // __type(value, struct map_value_##_size);
        // __uint(max_entries, 128);
        #[unsafe(link_section = ".maps")]
        pub static mut $array: $array = $array { _private: [] };
    };
}

macro_rules! define_array_with_percpu_kptr {
    ($bin:ident, $bin_ptr:ident, $value:ident, $array:ident, $size:expr) => {
        #[repr(C)]
        pub struct $bin {
            pub data: [i8; $size],
        }

        pub static mut $bin_ptr: *mut $bin = core::ptr::null_mut();

        #[repr(C)]
        pub struct $value {
            pub data: *mut $bin,
        }

        #[repr(C)]
        pub struct $array {
            _private: [u8; 0],
        }

        // Original C map definition used SEC(".maps") with:
        // __uint(type, BPF_MAP_TYPE_ARRAY);
        // __type(key, int);
        // __type(value, struct map_value_percpu_##_size);
        // __uint(max_entries, 128);
        #[unsafe(link_section = ".maps")]
        pub static mut $array: $array = $array { _private: [] };
    };
}

#[inline(always)]
unsafe fn batch_alloc(map: *mut bpf_map, batch: u32, idx: u32) {
    let mut value: *mut generic_map_value;
    let mut i: u32;
    let mut key: u32;
    let mut old: *mut c_void;
    let mut new: *mut c_void;

    i = 0;
    while i < batch {
        key = i;
        value = unsafe { bpf_map_lookup_elem(map, (&mut key as *mut u32).cast::<c_void>()) };
        if value.is_null() {
            unsafe { err = 1 };
            return;
        }
        new = unsafe { bpf_obj_new_impl(data_btf_ids[idx as usize] as u64, core::ptr::null_mut()) };
        if new.is_null() {
            unsafe { err = 2 };
            return;
        }
        old = unsafe { bpf_kptr_xchg(&mut (*value).data, new) };
        if !old.is_null() {
            unsafe { bpf_obj_drop(old) };
            unsafe { err = 3 };
            return;
        }
        i = i.wrapping_add(1);
    }
}

#[inline(always)]
unsafe fn batch_free(map: *mut bpf_map, batch: u32, idx: u32) {
    let mut value: *mut generic_map_value;
    let mut i: u32;
    let mut key: u32;
    let mut old: *mut c_void;

    i = 0;
    while i < batch {
        key = i;
        value = unsafe { bpf_map_lookup_elem(map, (&mut key as *mut u32).cast::<c_void>()) };
        if value.is_null() {
            unsafe { err = 4 };
            return;
        }
        old = unsafe { bpf_kptr_xchg(&mut (*value).data, core::ptr::null_mut()) };
        if old.is_null() {
            unsafe { err = 5 };
            return;
        }
        unsafe { bpf_obj_drop(old) };
        i = i.wrapping_add(1);
    }

    let _ = idx;
}

#[inline(always)]
unsafe fn batch_percpu_alloc(map: *mut bpf_map, batch: u32, idx: u32) {
    let mut value: *mut generic_map_value;
    let mut i: u32;
    let mut key: u32;
    let mut old: *mut c_void;
    let mut new: *mut c_void;

    i = 0;
    while i < batch {
        key = i;
        value = unsafe { bpf_map_lookup_elem(map, (&mut key as *mut u32).cast::<c_void>()) };
        if value.is_null() {
            unsafe { err = 1 };
            return;
        }
        /* per-cpu allocator may not be able to refill in time */
        new = unsafe {
            bpf_percpu_obj_new_impl(percpu_data_btf_ids[idx as usize] as u64, core::ptr::null_mut())
        };
        if new.is_null() {
            i = i.wrapping_add(1);
            continue;
        }

        old = unsafe { bpf_kptr_xchg(&mut (*value).data, new) };
        if !old.is_null() {
            unsafe { bpf_percpu_obj_drop(old) };
            unsafe { err = 2 };
            return;
        }
        i = i.wrapping_add(1);
    }
}

#[inline(always)]
unsafe fn batch_percpu_free(map: *mut bpf_map, batch: u32, idx: u32) {
    let mut value: *mut generic_map_value;
    let mut i: u32;
    let mut key: u32;
    let mut old: *mut c_void;

    i = 0;
    while i < batch {
        key = i;
        value = unsafe { bpf_map_lookup_elem(map, (&mut key as *mut u32).cast::<c_void>()) };
        if value.is_null() {
            unsafe { err = 3 };
            return;
        }
        old = unsafe { bpf_kptr_xchg(&mut (*value).data, core::ptr::null_mut()) };
        if old.is_null() {
            i = i.wrapping_add(1);
            continue;
        }
        unsafe { bpf_percpu_obj_drop(old) };
        i = i.wrapping_add(1);
    }

    let _ = idx;
}

/* kptr doesn't support bin_data_8 which is a zero-sized array */
define_array_with_kptr!(bin_data_16, __bin_data_16, map_value_16, array_16, 16);
define_array_with_kptr!(bin_data_32, __bin_data_32, map_value_32, array_32, 32);
define_array_with_kptr!(bin_data_64, __bin_data_64, map_value_64, array_64, 64);
define_array_with_kptr!(bin_data_96, __bin_data_96, map_value_96, array_96, 96);
define_array_with_kptr!(bin_data_128, __bin_data_128, map_value_128, array_128, 128);
define_array_with_kptr!(bin_data_192, __bin_data_192, map_value_192, array_192, 192);
define_array_with_kptr!(bin_data_256, __bin_data_256, map_value_256, array_256, 256);
define_array_with_kptr!(bin_data_512, __bin_data_512, map_value_512, array_512, 512);
define_array_with_kptr!(bin_data_1024, __bin_data_1024, map_value_1024, array_1024, 1024);
define_array_with_kptr!(bin_data_2048, __bin_data_2048, map_value_2048, array_2048, 2048);
define_array_with_kptr!(bin_data_4096, __bin_data_4096, map_value_4096, array_4096, 4096);

define_array_with_percpu_kptr!(
    percpu_bin_data_8,
    __percpu_bin_data_8,
    map_value_percpu_8,
    array_percpu_8,
    8
);
define_array_with_percpu_kptr!(
    percpu_bin_data_16,
    __percpu_bin_data_16,
    map_value_percpu_16,
    array_percpu_16,
    16
);
define_array_with_percpu_kptr!(
    percpu_bin_data_32,
    __percpu_bin_data_32,
    map_value_percpu_32,
    array_percpu_32,
    32
);
define_array_with_percpu_kptr!(
    percpu_bin_data_64,
    __percpu_bin_data_64,
    map_value_percpu_64,
    array_percpu_64,
    64
);
define_array_with_percpu_kptr!(
    percpu_bin_data_96,
    __percpu_bin_data_96,
    map_value_percpu_96,
    array_percpu_96,
    96
);
define_array_with_percpu_kptr!(
    percpu_bin_data_128,
    __percpu_bin_data_128,
    map_value_percpu_128,
    array_percpu_128,
    128
);
define_array_with_percpu_kptr!(
    percpu_bin_data_192,
    __percpu_bin_data_192,
    map_value_percpu_192,
    array_percpu_192,
    192
);
define_array_with_percpu_kptr!(
    percpu_bin_data_256,
    __percpu_bin_data_256,
    map_value_percpu_256,
    array_percpu_256,
    256
);
define_array_with_percpu_kptr!(
    percpu_bin_data_512,
    __percpu_bin_data_512,
    map_value_percpu_512,
    array_percpu_512,
    512
);

// SEC("?fentry/" SYS_PREFIX "sys_nanosleep")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_batch_alloc_free(ctx: *mut c_void) -> i32 {
    if unsafe { bpf_get_current_pid_tgid() as u32 } != unsafe { pid } {
        return 0;
    }

    /* Alloc 128 16-bytes objects in batch to trigger refilling,
     * then free 128 16-bytes objects in batch to trigger freeing.
     */
    unsafe {
        batch_alloc((&mut array_16 as *mut array_16).cast::<bpf_map>(), 128, 0);
        batch_free((&mut array_16 as *mut array_16).cast::<bpf_map>(), 128, 0);
        batch_alloc((&mut array_32 as *mut array_32).cast::<bpf_map>(), 128, 1);
        batch_free((&mut array_32 as *mut array_32).cast::<bpf_map>(), 128, 1);
        batch_alloc((&mut array_64 as *mut array_64).cast::<bpf_map>(), 128, 2);
        batch_free((&mut array_64 as *mut array_64).cast::<bpf_map>(), 128, 2);
        batch_alloc((&mut array_96 as *mut array_96).cast::<bpf_map>(), 128, 3);
        batch_free((&mut array_96 as *mut array_96).cast::<bpf_map>(), 128, 3);
        batch_alloc((&mut array_128 as *mut array_128).cast::<bpf_map>(), 128, 4);
        batch_free((&mut array_128 as *mut array_128).cast::<bpf_map>(), 128, 4);
        batch_alloc((&mut array_192 as *mut array_192).cast::<bpf_map>(), 128, 5);
        batch_free((&mut array_192 as *mut array_192).cast::<bpf_map>(), 128, 5);
        batch_alloc((&mut array_256 as *mut array_256).cast::<bpf_map>(), 128, 6);
        batch_free((&mut array_256 as *mut array_256).cast::<bpf_map>(), 128, 6);
        batch_alloc((&mut array_512 as *mut array_512).cast::<bpf_map>(), 64, 7);
        batch_free((&mut array_512 as *mut array_512).cast::<bpf_map>(), 64, 7);
        batch_alloc((&mut array_1024 as *mut array_1024).cast::<bpf_map>(), 32, 8);
        batch_free((&mut array_1024 as *mut array_1024).cast::<bpf_map>(), 32, 8);
        batch_alloc((&mut array_2048 as *mut array_2048).cast::<bpf_map>(), 16, 9);
        batch_free((&mut array_2048 as *mut array_2048).cast::<bpf_map>(), 16, 9);
        batch_alloc((&mut array_4096 as *mut array_4096).cast::<bpf_map>(), 8, 10);
        batch_free((&mut array_4096 as *mut array_4096).cast::<bpf_map>(), 8, 10);
    }

    let _ = ctx;
    0
}

// SEC("?fentry/" SYS_PREFIX "sys_nanosleep")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_free_through_map_free(ctx: *mut c_void) -> i32 {
    if unsafe { bpf_get_current_pid_tgid() as u32 } != unsafe { pid } {
        return 0;
    }

    /* Alloc 128 16-bytes objects in batch to trigger refilling,
     * then free these objects through map free.
     */
    unsafe {
        batch_alloc((&mut array_16 as *mut array_16).cast::<bpf_map>(), 128, 0);
        batch_alloc((&mut array_32 as *mut array_32).cast::<bpf_map>(), 128, 1);
        batch_alloc((&mut array_64 as *mut array_64).cast::<bpf_map>(), 128, 2);
        batch_alloc((&mut array_96 as *mut array_96).cast::<bpf_map>(), 128, 3);
        batch_alloc((&mut array_128 as *mut array_128).cast::<bpf_map>(), 128, 4);
        batch_alloc((&mut array_192 as *mut array_192).cast::<bpf_map>(), 128, 5);
        batch_alloc((&mut array_256 as *mut array_256).cast::<bpf_map>(), 128, 6);
        batch_alloc((&mut array_512 as *mut array_512).cast::<bpf_map>(), 64, 7);
        batch_alloc((&mut array_1024 as *mut array_1024).cast::<bpf_map>(), 32, 8);
        batch_alloc((&mut array_2048 as *mut array_2048).cast::<bpf_map>(), 16, 9);
        batch_alloc((&mut array_4096 as *mut array_4096).cast::<bpf_map>(), 8, 10);
    }

    let _ = ctx;
    0
}

// SEC("?fentry/" SYS_PREFIX "sys_nanosleep")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_batch_percpu_alloc_free(ctx: *mut c_void) -> i32 {
    if unsafe { bpf_get_current_pid_tgid() as u32 } != unsafe { pid } {
        return 0;
    }

    /* Alloc 128 8-bytes per-cpu objects in batch to trigger refilling,
     * then free 128 8-bytes per-cpu objects in batch to trigger freeing.
     */
    unsafe {
        batch_percpu_alloc(
            (&mut array_percpu_8 as *mut array_percpu_8).cast::<bpf_map>(),
            128,
            0,
        );
        batch_percpu_free(
            (&mut array_percpu_8 as *mut array_percpu_8).cast::<bpf_map>(),
            128,
            0,
        );
        batch_percpu_alloc(
            (&mut array_percpu_16 as *mut array_percpu_16).cast::<bpf_map>(),
            128,
            1,
        );
        batch_percpu_free(
            (&mut array_percpu_16 as *mut array_percpu_16).cast::<bpf_map>(),
            128,
            1,
        );
        batch_percpu_alloc(
            (&mut array_percpu_32 as *mut array_percpu_32).cast::<bpf_map>(),
            128,
            2,
        );
        batch_percpu_free(
            (&mut array_percpu_32 as *mut array_percpu_32).cast::<bpf_map>(),
            128,
            2,
        );
        batch_percpu_alloc(
            (&mut array_percpu_64 as *mut array_percpu_64).cast::<bpf_map>(),
            128,
            3,
        );
        batch_percpu_free(
            (&mut array_percpu_64 as *mut array_percpu_64).cast::<bpf_map>(),
            128,
            3,
        );
        batch_percpu_alloc(
            (&mut array_percpu_96 as *mut array_percpu_96).cast::<bpf_map>(),
            128,
            4,
        );
        batch_percpu_free(
            (&mut array_percpu_96 as *mut array_percpu_96).cast::<bpf_map>(),
            128,
            4,
        );
        batch_percpu_alloc(
            (&mut array_percpu_128 as *mut array_percpu_128).cast::<bpf_map>(),
            128,
            5,
        );
        batch_percpu_free(
            (&mut array_percpu_128 as *mut array_percpu_128).cast::<bpf_map>(),
            128,
            5,
        );
        batch_percpu_alloc(
            (&mut array_percpu_192 as *mut array_percpu_192).cast::<bpf_map>(),
            128,
            6,
        );
        batch_percpu_free(
            (&mut array_percpu_192 as *mut array_percpu_192).cast::<bpf_map>(),
            128,
            6,
        );
        batch_percpu_alloc(
            (&mut array_percpu_256 as *mut array_percpu_256).cast::<bpf_map>(),
            128,
            7,
        );
        batch_percpu_free(
            (&mut array_percpu_256 as *mut array_percpu_256).cast::<bpf_map>(),
            128,
            7,
        );
        batch_percpu_alloc(
            (&mut array_percpu_512 as *mut array_percpu_512).cast::<bpf_map>(),
            64,
            8,
        );
        batch_percpu_free(
            (&mut array_percpu_512 as *mut array_percpu_512).cast::<bpf_map>(),
            64,
            8,
        );
    }

    let _ = ctx;
    0
}

// SEC("?fentry/" SYS_PREFIX "sys_nanosleep")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_percpu_free_through_map_free(ctx: *mut c_void) -> i32 {
    if unsafe { bpf_get_current_pid_tgid() as u32 } != unsafe { pid } {
        return 0;
    }

    /* Alloc 128 8-bytes per-cpu objects in batch to trigger refilling,
     * then free these object through map free.
     */
    unsafe {
        batch_percpu_alloc(
            (&mut array_percpu_8 as *mut array_percpu_8).cast::<bpf_map>(),
            128,
            0,
        );
        batch_percpu_alloc(
            (&mut array_percpu_16 as *mut array_percpu_16).cast::<bpf_map>(),
            128,
            1,
        );
        batch_percpu_alloc(
            (&mut array_percpu_32 as *mut array_percpu_32).cast::<bpf_map>(),
            128,
            2,
        );
        batch_percpu_alloc(
            (&mut array_percpu_64 as *mut array_percpu_64).cast::<bpf_map>(),
            128,
            3,
        );
        batch_percpu_alloc(
            (&mut array_percpu_96 as *mut array_percpu_96).cast::<bpf_map>(),
            128,
            4,
        );
        batch_percpu_alloc(
            (&mut array_percpu_128 as *mut array_percpu_128).cast::<bpf_map>(),
            128,
            5,
        );
        batch_percpu_alloc(
            (&mut array_percpu_192 as *mut array_percpu_192).cast::<bpf_map>(),
            128,
            6,
        );
        batch_percpu_alloc(
            (&mut array_percpu_256 as *mut array_percpu_256).cast::<bpf_map>(),
            128,
            7,
        );
        batch_percpu_alloc(
            (&mut array_percpu_512 as *mut array_percpu_512).cast::<bpf_map>(),
            64,
            8,
        );
    }

    let _ = ctx;
    0
}
