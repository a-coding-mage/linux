// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2022 Huawei Technologies Duesseldorf GmbH
 *
 * Author: Roberto Sassu <roberto.sassu@huawei.com>
 */

// C dependencies: "vmlinux.h", <errno.h>, <bpf/bpf_helpers.h>,
// and <bpf/bpf_tracing.h>.

pub type __u32 = u32;
pub type __s32 = i32;
pub type __u64 = u64;

pub const ENOENT: i32 = 2;

#[repr(C)]
pub struct bpf_key {
    _private: [u8; 0],
}

#[repr(C)]
pub union bpf_attr {
    _private: [u8; 0],
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut monitored_pid: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut key_serial: __s32 = 0;
#[unsafe(no_mangle)]
pub static mut key_id: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut flags: __u64 = 0;

unsafe extern "C" {
    #[link_name = "bpf_lookup_user_key"]
    pub fn bpf_lookup_user_key(serial: __s32, flags: __u64) -> *mut bpf_key;
    #[link_name = "bpf_lookup_system_key"]
    pub fn bpf_lookup_system_key(id: __u64) -> *mut bpf_key;
    #[link_name = "bpf_key_put"]
    pub fn bpf_key_put(key: *mut bpf_key);
    pub fn bpf_get_current_pid_tgid() -> __u64;
}

// SEC("lsm.s/bpf")
#[unsafe(link_section = "lsm.s/bpf")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf(
    cmd: i32,
    attr: *mut bpf_attr,
    size: u32,
    kernel: bool,
) -> i32 {
    let bkey: *mut bpf_key;
    let pid: __u32;

    let _ = cmd;
    let _ = attr;
    let _ = size;
    let _ = kernel;

    pid = (unsafe { bpf_get_current_pid_tgid() } >> 32) as __u32;
    if pid != unsafe { monitored_pid } {
        return 0;
    }

    if unsafe { key_serial } != 0 {
        bkey = unsafe { bpf_lookup_user_key(key_serial, flags) };
    } else {
        bkey = unsafe { bpf_lookup_system_key(key_id as __u64) };
    }

    if bkey.is_null() {
        return -ENOENT;
    }

    unsafe { bpf_key_put(bkey) };

    0
}
