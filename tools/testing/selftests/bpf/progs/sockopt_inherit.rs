// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependencies:
// <linux/bpf.h>
// <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u8 = u8;
pub type __s32 = i32;

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

pub const SOL_CUSTOM: u32 = 0xdeadbeef;
pub const CUSTOM_INHERIT1: i32 = 0;
pub const CUSTOM_INHERIT2: i32 = 1;
pub const CUSTOM_LISTENER: i32 = 2;

#[no_mangle]
pub static mut page_size: __s32 = 0;

#[repr(C)]
pub struct sockopt_inherit {
    pub val: __u8,
}

#[repr(C)]
pub struct bpf_sockopt {
    pub sk: *mut core::ffi::c_void,
    pub optval: *mut __u8,
    pub optval_end: *mut __u8,
    pub level: i32,
    pub optname: i32,
    pub optlen: i32,
    pub retval: i32,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

pub const BPF_MAP_TYPE_SK_STORAGE: u32 = 24;
pub const BPF_F_NO_PREALLOC: u32 = 1;
pub const BPF_F_CLONE: u32 = 8192;
pub const BPF_SK_STORAGE_GET_F_CREATE: u64 = 1;

// SEC(".maps")
#[no_mangle]
pub static mut cloned1_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC | BPF_F_CLONE,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<sockopt_inherit>() as u32,
};

// SEC(".maps")
#[no_mangle]
pub static mut cloned2_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC | BPF_F_CLONE,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<sockopt_inherit>() as u32,
};

// SEC(".maps")
#[no_mangle]
pub static mut listener_only_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<sockopt_inherit>() as u32,
};

extern "C" {
    pub fn bpf_sk_storage_get(
        map: *mut core::ffi::c_void,
        sk: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut core::ffi::c_void;
}

#[inline(always)]
unsafe fn get_storage(ctx: *mut bpf_sockopt) -> *mut sockopt_inherit {
    if (*ctx).optname == CUSTOM_INHERIT1 {
        bpf_sk_storage_get(
            core::ptr::addr_of_mut!(cloned1_map).cast::<core::ffi::c_void>(),
            (*ctx).sk,
            core::ptr::null_mut(),
            BPF_SK_STORAGE_GET_F_CREATE,
        )
        .cast::<sockopt_inherit>()
    } else if (*ctx).optname == CUSTOM_INHERIT2 {
        bpf_sk_storage_get(
            core::ptr::addr_of_mut!(cloned2_map).cast::<core::ffi::c_void>(),
            (*ctx).sk,
            core::ptr::null_mut(),
            BPF_SK_STORAGE_GET_F_CREATE,
        )
        .cast::<sockopt_inherit>()
    } else {
        bpf_sk_storage_get(
            core::ptr::addr_of_mut!(listener_only_map).cast::<core::ffi::c_void>(),
            (*ctx).sk,
            core::ptr::null_mut(),
            BPF_SK_STORAGE_GET_F_CREATE,
        )
        .cast::<sockopt_inherit>()
    }
}

// SEC("cgroup/getsockopt")
#[no_mangle]
pub unsafe extern "C" fn _getsockopt(ctx: *mut bpf_sockopt) -> i32 {
    let optval_end: *mut __u8 = (*ctx).optval_end;
    let mut storage: *mut sockopt_inherit;
    let optval: *mut __u8 = (*ctx).optval;

    if (*ctx).level != SOL_CUSTOM as i32 {
        // only interested in SOL_CUSTOM
        // optval larger than PAGE_SIZE use kernel's buffer.
        if (*ctx).optlen > page_size {
            (*ctx).optlen = 0;
        }
        return 1;
    }

    if optval.add(1) > optval_end {
        return 0; // EPERM, bounds check
    }

    storage = get_storage(ctx);
    if storage.is_null() {
        return 0; // EPERM, couldn't get sk storage
    }

    (*ctx).retval = 0; // Reset system call return value to zero

    *optval.add(0) = (*storage).val;
    (*ctx).optlen = 1;

    1
}

// SEC("cgroup/setsockopt")
#[no_mangle]
pub unsafe extern "C" fn _setsockopt(ctx: *mut bpf_sockopt) -> i32 {
    let optval_end: *mut __u8 = (*ctx).optval_end;
    let mut storage: *mut sockopt_inherit;
    let optval: *mut __u8 = (*ctx).optval;

    if (*ctx).level != SOL_CUSTOM as i32 {
        // only interested in SOL_CUSTOM
        // optval larger than PAGE_SIZE use kernel's buffer.
        if (*ctx).optlen > page_size {
            (*ctx).optlen = 0;
        }
        return 1;
    }

    if optval.add(1) > optval_end {
        return 0; // EPERM, bounds check
    }

    storage = get_storage(ctx);
    if storage.is_null() {
        return 0; // EPERM, couldn't get sk storage
    }

    (*storage).val = *optval.add(0);
    (*ctx).optlen = -1;

    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
