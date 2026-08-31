// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/* Dependencies from C headers:
 * #include <vmlinux.h>
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 */

pub type __u32 = u32;

pub const BPF_MAP_TYPE_PERCPU_ARRAY: u32 = 6;

#[repr(C)]
pub struct bpf_iter__bpf_map_elem {
    pub meta: *mut core::ffi::c_void,
    pub map: *mut core::ffi::c_void,
    pub key: *mut core::ffi::c_void,
    pub value: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct arraymap1 {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key: __u32,
    pub value: __u32,
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut arraymap1: arraymap1 = arraymap1 {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: 3,
    key: 0,
    value: 0,
};

/* will set before prog run */
#[unsafe(no_mangle)]
pub static mut num_cpus: __u32 = 0;

#[unsafe(no_mangle)]
pub static mut key_sum: __u32 = 0;

#[unsafe(no_mangle)]
pub static mut val_sum: __u32 = 0;

#[unsafe(link_section = "iter/bpf_map_elem")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_bpf_percpu_array_map(ctx: *mut bpf_iter__bpf_map_elem) -> i32 {
    let key: *mut __u32 = unsafe { (*ctx).key as *mut __u32 };
    let mut pptr: *mut core::ffi::c_void = unsafe { (*ctx).value };
    let mut step: __u32;
    let mut i: i32;

    if key == core::ptr::null_mut() || pptr == core::ptr::null_mut() {
        return 0;
    }

    unsafe {
        key_sum = key_sum.wrapping_add(*key);
    }

    step = 8;
    i = 0;
    while (i as __u32) < unsafe { core::ptr::read_volatile(core::ptr::addr_of!(num_cpus)) } {
        unsafe {
            val_sum = val_sum.wrapping_add(*(pptr as *mut __u32));
            pptr = (pptr as *mut u8).add(step as usize) as *mut core::ffi::c_void;
        }
        i += 1;
    }
    return 0;
}
