// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// C includes translated as external dependency expectations:
// <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

pub type __u32 = u32;
pub type __s32 = i32;

pub const BPF_MAP_TYPE_PERCPU_HASH: u32 = 5;

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct key_t {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

#[repr(C)]
pub struct bpf_iter__bpf_map_elem {
    pub meta: *mut core::ffi::c_void,
    pub map: *mut core::ffi::c_void,
    pub key: *mut key_t,
    pub value: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct hashmap1_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut hashmap1: hashmap1_def = hashmap1_def {
    type_: BPF_MAP_TYPE_PERCPU_HASH,
    max_entries: 3,
    key_size: core::mem::size_of::<key_t>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

/* will set before prog run */
#[unsafe(no_mangle)]
pub static mut num_cpus: __s32 = 0;

/* will collect results during prog run */
#[unsafe(no_mangle)]
pub static mut key_sum_a: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut key_sum_b: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut key_sum_c: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut val_sum: __u32 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "iter/bpf_map_elem")]
pub unsafe extern "C" fn dump_bpf_percpu_hash_map(ctx: *mut bpf_iter__bpf_map_elem) -> i32 {
    let key: *mut key_t = unsafe { (*ctx).key };
    let mut pptr: *mut core::ffi::c_void = unsafe { (*ctx).value };
    let step: __u32;
    let mut i: i32;

    if key == core::ptr::null_mut()
        || pptr == core::ptr::null_mut()
    {
        return 0;
    }

    unsafe {
        key_sum_a = key_sum_a.wrapping_add((*key).a as __u32);
        key_sum_b = key_sum_b.wrapping_add((*key).b as __u32);
        key_sum_c = key_sum_c.wrapping_add((*key).c as __u32);
    }

    step = 8;
    i = 0;
    while unsafe { i < num_cpus } {
        unsafe {
            val_sum = val_sum.wrapping_add(*(pptr as *mut __u32));
            pptr = (pptr as *mut u8).add(step as usize) as *mut core::ffi::c_void;
        }
        i += 1;
    }
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
