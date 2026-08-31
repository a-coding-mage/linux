// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2020 Google LLC.
 */

// C dependencies:
// #include <linux/bpf.h>
// #include <errno.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type c_int = i32;
type c_void = core::ffi::c_void;

extern "C" {
    static BPF_MAP_TYPE_TASK_STORAGE: u32;
    static BPF_F_NO_PREALLOC: u32;
    static BPF_LOCAL_STORAGE_GET_F_CREATE: u64;
    static BPF_F_BPRM_SECUREEXEC: u64;

    fn bpf_task_storage_get(
        map: *mut c_void,
        task: *mut c_void,
        value: *mut c_void,
        flags: u64,
    ) -> *mut c_void;
    fn bpf_get_current_task_btf() -> *mut c_void;
    fn bpf_bprm_opts_set(bprm: *mut linux_binprm, flags: u64) -> c_int;
}

#[repr(C)]
pub struct linux_binprm {
    _private: [u8; 0],
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct secure_exec_task_map_t {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut secure_exec_task_map: secure_exec_task_map_t = secure_exec_task_map_t {
    type_: 0,
    map_flags: 0,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<c_int>() as u32,
};

#[link_section = "lsm/bprm_creds_for_exec"]
#[no_mangle]
pub unsafe extern "C" fn secure_exec(bprm: *mut linux_binprm) -> c_int {
    let secureexec: *mut c_int;

    secureexec = bpf_task_storage_get(
        &mut secure_exec_task_map as *mut secure_exec_task_map_t as *mut c_void,
        bpf_get_current_task_btf(),
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    ) as *mut c_int;

    if !secureexec.is_null() && *secureexec != 0 {
        bpf_bprm_opts_set(bprm, BPF_F_BPRM_SECUREEXEC);
    }

    0
}
