// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2022 Huawei Technologies Duesseldorf GmbH
 *
 * Author: Roberto Sassu <roberto.sassu@huawei.com>
 */

// C dependencies from the original source:
// "vmlinux.h", <errno.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
// "bpf_misc.h", and "bpf_kfuncs.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u32 = u32;
pub type __u64 = u64;

pub const BPF_MAP_TYPE_RINGBUF: u32 = 27;
pub const BPF_MAP_TYPE_ARRAY: u32 = 2;
pub const EINVAL: i32 = 22;

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_key {
    _private: [u8; 0],
}

#[repr(C)]
pub union bpf_attr {
    _private: [u8; 0],
}

// Original:
// struct {
//      __uint(type, BPF_MAP_TYPE_RINGBUF);
//      __uint(max_entries, 4096);
// } ringbuf SEC(".maps");
#[repr(C)]
pub struct ringbuf_map_def {
    pub type_: u32,
    pub max_entries: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut ringbuf: ringbuf_map_def = ringbuf_map_def {
    type_: BPF_MAP_TYPE_RINGBUF,
    max_entries: 4096,
};

// Original:
// struct {
//      __uint(type, BPF_MAP_TYPE_ARRAY);
//      __uint(max_entries, 1);
//      __type(key, __u32);
//      __type(value, __u32);
// } array_map SEC(".maps");
#[repr(C)]
pub struct array_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: usize,
    pub value_size: usize,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut array_map: array_map_def = array_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>(),
    value_size: core::mem::size_of::<__u32>(),
};

#[no_mangle]
pub static mut err: i32 = 0;

#[no_mangle]
pub static mut pid: i32 = 0;

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    fn bpf_verify_pkcs7_signature(
        data: *mut bpf_dynptr,
        sig: *mut bpf_dynptr,
        trusted_keyring: *mut bpf_key,
    ) -> i32;
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_map_lookup_elem(map: *mut array_map_def, key: *const i32) -> *mut __u32;
    fn bpf_dynptr_from_mem(
        data: *mut __u32,
        size: usize,
        flags: __u64,
        ptr: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_lookup_system_key(id: u64) -> *mut bpf_key;
    fn bpf_key_put(key: *mut bpf_key);
}

// SEC("?lsm.s/bpf")
// __failure __msg("cannot pass in dynptr at an offset=-8")
#[no_mangle]
pub unsafe extern "C" fn not_valid_dynptr(
    cmd: i32,
    attr: *mut bpf_attr,
    size: u32,
    kernel: bool,
) -> i32 {
    let mut val: core::ffi::c_ulong = 0;

    let _ = (cmd, attr, size, kernel);
    return bpf_verify_pkcs7_signature(
        &mut val as *mut core::ffi::c_ulong as *mut bpf_dynptr,
        &mut val as *mut core::ffi::c_ulong as *mut bpf_dynptr,
        core::ptr::null_mut(),
    );
}

// SEC("?lsm.s/bpf")
// __failure __msg("R1 expected pointer to stack or const struct bpf_dynptr")
#[no_mangle]
pub unsafe extern "C" fn not_ptr_to_stack(
    cmd: i32,
    attr: *mut bpf_attr,
    size: u32,
    kernel: bool,
) -> i32 {
    static mut val: bpf_dynptr = bpf_dynptr { _private: [] };

    let _ = (cmd, attr, size, kernel);
    return bpf_verify_pkcs7_signature(&mut val, &mut val, core::ptr::null_mut());
}

// SEC("lsm.s/bpf")
#[no_mangle]
pub unsafe extern "C" fn dynptr_data_null(
    cmd: i32,
    attr: *mut bpf_attr,
    size: u32,
    kernel: bool,
) -> i32 {
    let mut trusted_keyring: *mut bpf_key;
    let mut ptr: bpf_dynptr = core::mem::zeroed();
    let mut value: *mut __u32;
    let mut ret: i32;
    let mut zero: i32 = 0;

    let _ = (cmd, attr, size, kernel);
    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return 0;
    }

    value = bpf_map_lookup_elem(&mut array_map, &zero);
    if value.is_null() {
        return 0;
    }

    /* Pass invalid flags. */
    ret = bpf_dynptr_from_mem(
        value,
        core::mem::size_of_val(&*value),
        !0_u64 as __u64,
        &mut ptr,
    );
    if ret != -EINVAL {
        return 0;
    }

    trusted_keyring = bpf_lookup_system_key(0);
    if trusted_keyring.is_null() {
        return 0;
    }

    err = bpf_verify_pkcs7_signature(&mut ptr, &mut ptr, trusted_keyring);

    bpf_key_put(trusted_keyring);

    return 0;
}
