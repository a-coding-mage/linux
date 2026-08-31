// SPDX-License-Identifier: GPL-2.0
// Translated from C includes:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>

pub type __u32 = u32;
pub type __u64 = u64;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

// Original map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, __u32);
//     __type(value, __u64);
// } array_map SEC(".maps");
#[repr(C)]
pub struct array_map_def {
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __uint(max_entries, 1);
    // __type(key, __u32);
    // __type(value, __u64);
    _private: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut array_map: array_map_def = array_map_def { _private: [] };

extern "C" {
    fn bpf_get_current_comm(buf: *mut core::ffi::c_void, size_of_buf: __u32) -> i64;

    fn bpf_for_each_map_elem(
        map: *mut core::ffi::c_void,
        callback: Option<
            unsafe extern "C" fn(
                map: *mut bpf_map,
                key: *mut __u32,
                val: *mut __u64,
                data: *mut core::ffi::c_void,
            ) -> __u64,
        >,
        callback_ctx: *mut core::ffi::c_void,
        flags: __u64,
    ) -> i64;
}

static unsafe extern "C" fn check_array_elem(
    _map: *mut bpf_map,
    key: *mut __u32,
    _val: *mut __u64,
    _data: *mut core::ffi::c_void,
) -> __u64 {
    unsafe {
        bpf_get_current_comm(
            key as *mut core::ffi::c_void,
            core::mem::size_of::<__u32>() as __u32,
        );
    }
    0
}

#[link_section = "raw_tp/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn test_map_key_write(_ctx: *const core::ffi::c_void) -> i32 {
    unsafe {
        bpf_for_each_map_elem(
            &raw mut array_map as *mut array_map_def as *mut core::ffi::c_void,
            Some(check_array_elem),
            core::ptr::null_mut(),
            0,
        );
    }
    0
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
