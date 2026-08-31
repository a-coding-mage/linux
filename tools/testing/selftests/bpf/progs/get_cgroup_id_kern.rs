// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook

// Dependencies from the original C includes:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

pub type __u32 = u32;
pub type __u64 = u64;

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_get_current_cgroup_id() -> __u64;
}

#[unsafe(no_mangle)]
pub static mut cg_id: __u64 = 0;

#[unsafe(no_mangle)]
pub static mut expected_pid: __u64 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "tracepoint/syscalls/sys_enter_nanosleep")]
pub unsafe extern "C" fn trace(ctx: *mut core::ffi::c_void) -> i32 {
    let pid: __u32 = unsafe { bpf_get_current_pid_tgid() as __u32 };

    if unsafe { expected_pid } == pid as __u64 {
        unsafe {
            cg_id = bpf_get_current_cgroup_id();
        }
    }

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";
