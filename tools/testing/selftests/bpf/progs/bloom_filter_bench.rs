// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/*
 * Translated from C. Original dependencies:
 * <errno.h>, <linux/bpf.h>, <stdbool.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"
 */

pub const ENOENT: i32 = 2;
pub const BPF_MAP_TYPE_ARRAY: u32 = 2;
pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_MAP_TYPE_BLOOM_FILTER: u32 = 30;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut rand_vals: [u8; 2500000] = [0; 2500000];
#[unsafe(no_mangle)]
pub static nr_rand_bytes: u32 = 2500000;

/*
 * Map definition metadata from the original C SEC(".maps") declarations:
 *
 * array_map:
 *   __uint(type, BPF_MAP_TYPE_ARRAY);
 *   __uint(key_size, sizeof(__u32));
 *   max entries and value_size will be set programmatically.
 *   They are configurable from the userspace bench program.
 *
 * bloom_map:
 *   __uint(type, BPF_MAP_TYPE_BLOOM_FILTER);
 *   max entries, value_size, and # of hash functions will be set
 *   programmatically. They are configurable from the userspace
 *   bench program.
 *   __uint(map_extra, 3);
 *
 * hashmap:
 *   __uint(type, BPF_MAP_TYPE_HASH);
 *   max entries, key_size, and value_size, will be set
 *   programmatically. They are configurable from the userspace
 *   bench program.
 */
#[repr(C)]
pub struct array_map_def {
    pub type_: u32,
    pub key_size: usize,
}

#[repr(C)]
pub struct bloom_map_def {
    pub type_: u32,
    pub map_extra: u32,
}

#[repr(C)]
pub struct hashmap_def {
    pub type_: u32,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut array_map: array_map_def = array_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    key_size: core::mem::size_of::<u32>(),
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut bloom_map: bloom_map_def = bloom_map_def {
    type_: BPF_MAP_TYPE_BLOOM_FILTER,
    map_extra: 3,
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut hashmap: hashmap_def = hashmap_def {
    type_: BPF_MAP_TYPE_HASH,
};

#[repr(C)]
pub struct callback_ctx {
    pub map: *mut bpf_map,
    pub update: bool,
}

/* Tracks the number of hits, drops, and false hits */
#[repr(C, align(256))]
#[derive(Copy, Clone)]
pub struct percpu_stats_t {
    pub stats: [u32; 3],
}

#[unsafe(no_mangle)]
pub static mut percpu_stats: [percpu_stats_t; 256] = [percpu_stats_t { stats: [0; 3] }; 256];

#[unsafe(no_mangle)]
pub static hit_key: u32 = 0;
#[unsafe(no_mangle)]
pub static drop_key: u32 = 1;
#[unsafe(no_mangle)]
pub static false_hit_key: u32 = 2;

#[unsafe(no_mangle)]
pub static mut value_size: u8 = 0;

#[unsafe(no_mangle)]
pub static hashmap_use_bloom: bool = false;
#[unsafe(no_mangle)]
pub static count_false_hits: bool = false;

#[unsafe(no_mangle)]
pub static mut error: i32 = 0;

unsafe extern "C" {
    fn bpf_get_smp_processor_id() -> u32;
    fn bpf_map_push_elem(map: *mut bpf_map, value: *const core::ffi::c_void, flags: u64) -> i32;
    fn bpf_map_peek_elem(map: *mut bpf_map, value: *const core::ffi::c_void) -> i32;
    fn bpf_map_lookup_elem(map: *mut hashmap_def, key: *const core::ffi::c_void) -> *mut u64;
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_for_each_map_elem(
        map: *mut array_map_def,
        callback: unsafe extern "C" fn(
            *mut bpf_map,
            *mut u32,
            *mut core::ffi::c_void,
            *mut callback_ctx,
        ) -> u64,
        callback_ctx: *mut callback_ctx,
        flags: u64,
    ) -> i32;
}

#[inline(always)]
unsafe fn log_result(key: u32) {
    let cpu: u32 = unsafe { bpf_get_smp_processor_id() };

    unsafe {
        percpu_stats[(cpu & 255) as usize].stats[key as usize] =
            percpu_stats[(cpu & 255) as usize].stats[key as usize].wrapping_add(1);
    }
}

unsafe extern "C" fn bloom_callback(
    _map: *mut bpf_map,
    _key: *mut u32,
    val: *mut core::ffi::c_void,
    data: *mut callback_ctx,
) -> u64 {
    let err: i32;

    if unsafe { (*data).update } {
        err = unsafe { bpf_map_push_elem((*data).map, val, 0) };
    } else {
        err = unsafe { bpf_map_peek_elem((*data).map, val) };
    }

    if err != 0 {
        unsafe {
            error |= 1;
        }
        return 1; /* stop the iteration */
    }

    unsafe {
        log_result(hit_key);
    }

    0
}

/* SEC("fentry/" SYS_PREFIX "sys_getpgid") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bloom_lookup(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut data: callback_ctx = callback_ctx {
        map: core::ptr::null_mut(),
        update: false,
    };

    data.map = unsafe { &raw mut bloom_map as *mut bpf_map };
    data.update = false;

    unsafe {
        bpf_for_each_map_elem(&raw mut array_map, bloom_callback, &mut data, 0);
    }

    0
}

/* SEC("fentry/" SYS_PREFIX "sys_getpgid") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bloom_update(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut data: callback_ctx = callback_ctx {
        map: core::ptr::null_mut(),
        update: false,
    };

    data.map = unsafe { &raw mut bloom_map as *mut bpf_map };
    data.update = true;

    unsafe {
        bpf_for_each_map_elem(&raw mut array_map, bloom_callback, &mut data, 0);
    }

    0
}

/* SEC("fentry/" SYS_PREFIX "sys_getpgid") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bloom_hashmap_lookup(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut result: *mut u64;
    let mut i: i32;
    let mut err: i32;

    let mut index: u32 = unsafe { bpf_get_prandom_u32() };
    let bitmask: u32 = ((1u64 << 21) - 1) as u32;

    i = 0;
    while i < 1024 {
        index = index & bitmask;

        if hashmap_use_bloom {
            err = unsafe {
                bpf_map_peek_elem(
                    &raw mut bloom_map as *mut bpf_map,
                    rand_vals.as_mut_ptr().add(index as usize) as *const core::ffi::c_void,
                )
            };
            if err != 0 {
                if err != -ENOENT {
                    unsafe {
                        error |= 2;
                    }
                    return 0;
                }
                unsafe {
                    log_result(hit_key);
                }
                i += 1;
                index = index.wrapping_add(unsafe { value_size } as u32);
                continue;
            }
        }

        result = unsafe {
            bpf_map_lookup_elem(
                &raw mut hashmap,
                rand_vals.as_mut_ptr().add(index as usize) as *const core::ffi::c_void,
            )
        };
        if !result.is_null() {
            unsafe {
                log_result(hit_key);
            }
        } else {
            if hashmap_use_bloom && count_false_hits {
                unsafe {
                    log_result(false_hit_key);
                }
            }
            unsafe {
                log_result(drop_key);
            }
        }

        i += 1;
        index = index.wrapping_add(unsafe { value_size } as u32);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
