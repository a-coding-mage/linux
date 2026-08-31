// SPDX-License-Identifier: GPL-2.0
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, "../test_kmods/bpf_testmod_kfunc.h"

use core::ffi::c_void;

#[no_mangle]
pub static mut done: i32 = 0;

extern "C" {
    fn bpf_kfunc_call_test_call_rcu_tasks_trace(arg1: *mut i32) -> i32;
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn call_rcu_tasks_trace(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    bpf_kfunc_call_test_call_rcu_tasks_trace(core::ptr::addr_of_mut!(done))
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
