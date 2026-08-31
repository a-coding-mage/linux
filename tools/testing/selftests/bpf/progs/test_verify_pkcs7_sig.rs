// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2022 Huawei Technologies Duesseldorf GmbH
 *
 * Author: Roberto Sassu <roberto.sassu@huawei.com>
 */

// C dependencies: "vmlinux.h", <errno.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_tracing.h>, "bpf_kfuncs.h", "err.h".

use core::ffi::c_void;

const MAX_DATA_SIZE: usize = 1024 * 1024;
const MAX_SIG_SIZE: usize = 1024;

const EINVAL: i32 = 22;
const ENOENT: i32 = 2;
const EFAULT: i32 = 14;

#[no_mangle]
pub static mut monitored_pid: u32 = 0;
#[no_mangle]
pub static mut user_keyring_serial: i32 = 0;
#[no_mangle]
pub static mut system_keyring_id: u64 = 0;

#[repr(C)]
pub struct data {
    pub data: [u8; MAX_DATA_SIZE],
    pub data_len: u32,
    pub sig: [u8; MAX_SIG_SIZE],
    pub sig_len: u32,
}

// Original C declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, __u32);
//     __type(value, struct data);
// } data_input SEC(".maps");
unsafe extern "C" {
    static mut data_input: c_void;
}

#[unsafe(link_section = "license")]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

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
    pub value: u64,
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_probe_read_kernel(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_copy_from_user(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_dynptr_from_mem(
        data: *mut c_void,
        size: u32,
        flags: u64,
        ptr: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_lookup_user_key(serial: i32, flags: u64) -> *mut bpf_key;
    fn bpf_lookup_system_key(id: u64) -> *mut bpf_key;
    fn bpf_verify_pkcs7_signature(
        data_ptr: *mut bpf_dynptr,
        sig_ptr: *mut bpf_dynptr,
        trusted_keyring: *mut bpf_key,
    ) -> i32;
    fn bpf_key_put(key: *mut bpf_key);
    fn set_if_not_errno_or_zero(ret: i32, new_errno: i32);
}

// SEC("lsm.s/bpf")
#[unsafe(link_section = "lsm.s/bpf")]
#[no_mangle]
pub unsafe extern "C" fn bpf(
    cmd: i32,
    attr: *mut bpf_attr,
    size: u32,
    kernel: bool,
) -> i32 {
    let mut data_ptr: bpf_dynptr = core::mem::zeroed();
    let mut sig_ptr: bpf_dynptr = core::mem::zeroed();
    let mut data_val: *mut data;
    let mut trusted_keyring: *mut bpf_key;
    let mut pid: u32;
    let mut value: u64 = 0;
    let mut ret: i32;
    let mut zero: i32 = 0;

    let _ = cmd;
    let _ = size;
    let _ = kernel;

    pid = (bpf_get_current_pid_tgid() >> 32) as u32;
    if pid != monitored_pid {
        return 0;
    }

    data_val = bpf_map_lookup_elem(
        &raw mut data_input,
        &mut zero as *mut i32 as *const c_void,
    ) as *mut data;
    if data_val.is_null() {
        return 0;
    }

    ret = bpf_probe_read_kernel(
        &mut value as *mut u64 as *mut c_void,
        core::mem::size_of_val(&value) as u32,
        &raw const (*attr).value as *const c_void,
    );
    if ret != 0 {
        set_if_not_errno_or_zero(ret, -EFAULT);
        return ret;
    }

    ret = bpf_copy_from_user(
        data_val as *mut c_void,
        core::mem::size_of::<data>() as u32,
        value as usize as *const c_void,
    );
    if ret != 0 {
        set_if_not_errno_or_zero(ret, -EFAULT);
        return ret;
    }

    if (*data_val).data_len as usize > core::mem::size_of_val(&(*data_val).data) {
        return -EINVAL;
    }

    bpf_dynptr_from_mem(
        (*data_val).data.as_mut_ptr() as *mut c_void,
        (*data_val).data_len,
        0,
        &mut data_ptr,
    );

    if (*data_val).sig_len as usize > core::mem::size_of_val(&(*data_val).sig) {
        return -EINVAL;
    }

    bpf_dynptr_from_mem(
        (*data_val).sig.as_mut_ptr() as *mut c_void,
        (*data_val).sig_len,
        0,
        &mut sig_ptr,
    );

    if user_keyring_serial != 0 {
        trusted_keyring = bpf_lookup_user_key(user_keyring_serial, 0);
    } else {
        trusted_keyring = bpf_lookup_system_key(system_keyring_id);
    }

    if trusted_keyring.is_null() {
        return -ENOENT;
    }

    ret = bpf_verify_pkcs7_signature(&mut data_ptr, &mut sig_ptr, trusted_keyring);

    bpf_key_put(trusted_keyring);

    set_if_not_errno_or_zero(ret, -EFAULT);

    ret
}
