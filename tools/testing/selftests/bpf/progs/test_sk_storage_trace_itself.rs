// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// C includes translated as dependency intent:
// <vmlinux.h>
// <bpf/bpf_tracing.h>
// <bpf/bpf_helpers.h>

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

// Original C map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_SK_STORAGE);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __type(key, int);
//     __type(value, int);
// } sk_stg_map SEC(".maps");
#[repr(C)]
pub struct sk_stg_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key: i32,
    pub value: i32,
}

unsafe extern "C" {
    static mut sk_stg_map: sk_stg_map_def;

    fn bpf_sk_storage_get(
        map: *mut sk_stg_map_def,
        sk: *mut sock,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut i32;
}

pub const BPF_MAP_TYPE_SK_STORAGE: u32 = 24;
pub const BPF_F_NO_PREALLOC: u32 = 1;
pub const BPF_SK_STORAGE_GET_F_CREATE: u64 = 1;

#[unsafe(no_mangle)]
#[unsafe(link_section = "fentry/bpf_sk_storage_free")]
pub unsafe extern "C" fn trace_bpf_sk_storage_free(sk: *mut sock) -> i32 {
    let value: *mut i32;

    value = unsafe {
        bpf_sk_storage_get(
            &raw mut sk_stg_map,
            sk,
            core::ptr::null_mut(),
            BPF_SK_STORAGE_GET_F_CREATE,
        )
    };

    if !value.is_null() {
        unsafe {
            *value = 1;
        }
    }

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
