// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2020 Google LLC.
 */

// C dependencies in the original source:
// "vmlinux.h", <errno.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

type bool_ = bool;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type c_int = core::ffi::c_int;

const EPERM: c_int = 1;
const BPF_MAP_TYPE_RINGBUF: u32 = 27;
const READING_POLICY: kernel_read_file_id = 1;

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub f_inode: *mut inode,
}

#[repr(C)]
pub struct linux_binprm {
    pub file: *mut file,
}

pub type kernel_read_file_id = c_int;

#[repr(C)]
pub struct ringbuf_def {
    pub type_: u32,
    pub max_entries: u32,
}

#[no_mangle]
pub static mut monitored_pid: u32 = 0;

#[no_mangle]
#[link_section = ".maps"]
pub static mut ringbuf: ringbuf_def = ringbuf_def {
    type_: BPF_MAP_TYPE_RINGBUF,
    max_entries: 1u32 << 12,
};

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut use_ima_file_hash: bool_ = false;
#[no_mangle]
pub static mut enable_bprm_creds_for_exec: bool_ = false;
#[no_mangle]
pub static mut enable_kernel_read_file: bool_ = false;
#[no_mangle]
pub static mut test_deny: bool_ = false;

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_ima_inode_hash(inode: *mut inode, dst: *mut u64, size: u32) -> c_int;
    fn bpf_ima_file_hash(file: *mut file, dst: *mut u64, size: u32) -> c_int;
    fn bpf_ringbuf_reserve(ringbuf: *mut ringbuf_def, size: u64, flags: u64) -> *mut u64;
    fn bpf_ringbuf_submit(data: *mut u64, flags: u64);
}

unsafe fn ima_test_common(file: *mut file) {
    let mut ima_hash: u64 = 0;
    let sample: *mut u64;
    let ret: c_int;
    let pid: u32;

    pid = (bpf_get_current_pid_tgid() >> 32) as u32;
    if pid == monitored_pid {
        if !use_ima_file_hash {
            ret = bpf_ima_inode_hash(
                (*file).f_inode,
                &mut ima_hash,
                core::mem::size_of_val(&ima_hash) as u32,
            );
        } else {
            ret = bpf_ima_file_hash(
                file,
                &mut ima_hash,
                core::mem::size_of_val(&ima_hash) as u32,
            );
        }
        if ret < 0 || ima_hash == 0 {
            return;
        }

        sample = bpf_ringbuf_reserve(
            &raw mut ringbuf,
            core::mem::size_of::<u64>() as u64,
            0,
        );
        if sample.is_null() {
            return;
        }

        *sample = ima_hash;
        bpf_ringbuf_submit(sample, 0);
    }

    return;
}

unsafe fn ima_test_deny() -> c_int {
    let pid: u32;

    pid = (bpf_get_current_pid_tgid() >> 32) as u32;
    if pid == monitored_pid && test_deny {
        return -EPERM;
    }

    return 0;
}

#[no_mangle]
#[link_section = "lsm.s/bprm_committed_creds"]
pub unsafe extern "C" fn bprm_committed_creds(bprm: *mut linux_binprm) {
    ima_test_common((*bprm).file);
}

#[no_mangle]
#[link_section = "lsm.s/bprm_creds_for_exec"]
pub unsafe extern "C" fn bprm_creds_for_exec(bprm: *mut linux_binprm) -> c_int {
    if !enable_bprm_creds_for_exec {
        return 0;
    }

    ima_test_common((*bprm).file);
    return 0;
}

#[no_mangle]
#[link_section = "lsm.s/kernel_read_file"]
pub unsafe extern "C" fn kernel_read_file(
    file: *mut file,
    id: kernel_read_file_id,
    contents: bool_,
) -> c_int {
    let ret: c_int;

    if !enable_kernel_read_file {
        return 0;
    }

    if !contents {
        return 0;
    }

    if id != READING_POLICY {
        return 0;
    }

    ret = ima_test_deny();
    if ret < 0 {
        return ret;
    }

    ima_test_common(file);
    return 0;
}
