// SPDX-License-Identifier: GPL-2.0-only

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_helpers.h>

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

pub const EPERM: i32 = 1; /* Operation not permitted */

/* From include/linux/mm.h. */
pub const FMODE_WRITE: i32 = 0x2;

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

// Constants normally supplied by the included BPF/Linux headers.
pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_MAP_TYPE_ARRAY: u32 = 2;
pub const BPF_ANY: u64 = 0;

// Original C uses libbpf map-definition macros:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __type(key, __u32);
//     __type(value, __u32);
//     __uint(max_entries, 1);
// } prot_status_map SEC(".maps");
#[repr(C)]
pub struct prot_status_map_def {
    pub type_: u32,
    pub key: u32,
    pub value: u32,
    pub max_entries: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut prot_status_map: prot_status_map_def = prot_status_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    key: 0,
    value: 0,
    max_entries: 1,
};

// Original C uses libbpf map-definition macros:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __type(key, __u32);
//     __type(value, __u32);
//     __uint(max_entries, 3);
// } prot_map SEC(".maps");
#[repr(C)]
pub struct prot_map_def {
    pub type_: u32,
    pub key: u32,
    pub value: u32,
    pub max_entries: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut prot_map: prot_map_def = prot_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key: 0,
    value: 0,
    max_entries: 3,
};

// Original C uses libbpf map-definition macros:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __type(key, __u32);
//     __type(value, __u32);
//     __uint(max_entries, 3);
// } not_prot_map SEC(".maps");
#[repr(C)]
pub struct not_prot_map_def {
    pub type_: u32,
    pub key: u32,
    pub value: u32,
    pub max_entries: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut not_prot_map: not_prot_map_def = not_prot_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key: 0,
    value: 0,
    max_entries: 3,
};

extern "C" {
    pub fn bpf_map_lookup_elem(map: *const core::ffi::c_void, key: *const core::ffi::c_void) -> *mut u32;
    pub fn bpf_map_update_elem(
        map: *const core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

#[no_mangle]
#[link_section = "fmod_ret/security_bpf_map"]
pub unsafe extern "C" fn fmod_bpf_map(map: *mut bpf_map, fmode: i32) -> i32 {
    let key: u32 = 0;
    let status_ptr: *mut u32 = bpf_map_lookup_elem(
        &raw const prot_status_map as *const core::ffi::c_void,
        &key as *const u32 as *const core::ffi::c_void,
    );

    if status_ptr.is_null() || *status_ptr == 0 {
        return 0;
    }

    if map == (&raw mut prot_map as *mut bpf_map) {
        /* Allow read-only access */
        if (fmode & FMODE_WRITE) != 0 {
            return -EPERM;
        }
    }

    return 0;
}

/*
 * This program keeps references to maps. This is needed to prevent
 * optimizing them out.
 */
#[no_mangle]
#[link_section = "fentry/bpf_fentry_test1"]
pub unsafe extern "C" fn fentry_dummy1(a: i32) -> i32 {
    let key: u32 = 0;
    let val1: u32 = a as u32;
    let val2: u32 = a.wrapping_add(1) as u32;

    bpf_map_update_elem(
        &raw const prot_map as *const core::ffi::c_void,
        &key as *const u32 as *const core::ffi::c_void,
        &val1 as *const u32 as *const core::ffi::c_void,
        BPF_ANY,
    );
    bpf_map_update_elem(
        &raw const not_prot_map as *const core::ffi::c_void,
        &key as *const u32 as *const core::ffi::c_void,
        &val2 as *const u32 as *const core::ffi::c_void,
        BPF_ANY,
    );
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
