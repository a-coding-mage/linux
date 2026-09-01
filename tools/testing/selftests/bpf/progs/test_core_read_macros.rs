// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C source dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_core_read.h>

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* shuffled layout for relocatable (CO-RE) reads */
#[repr(C)]
pub struct callback_head___shuffled {
    pub func: Option<unsafe extern "C" fn(head: *mut callback_head___shuffled)>,
    pub next: *mut callback_head___shuffled,
}

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

#[no_mangle]
pub static mut k_probe_in: callback_head = unsafe { core::mem::zeroed() };

#[no_mangle]
pub static mut k_core_in: callback_head___shuffled = callback_head___shuffled {
    func: None,
    next: core::ptr::null_mut(),
};

#[no_mangle]
pub static mut u_probe_in: *mut callback_head = core::ptr::null_mut();

#[no_mangle]
pub static mut u_core_in: *mut callback_head___shuffled = core::ptr::null_mut();

#[no_mangle]
pub static mut k_probe_out: i64 = 0;

#[no_mangle]
pub static mut u_probe_out: i64 = 0;

#[no_mangle]
pub static mut k_core_out: i64 = 0;

#[no_mangle]
pub static mut u_core_out: i64 = 0;

#[no_mangle]
pub static mut my_pid: i32 = 0;

#[link_section = "raw_tracepoint/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn handler(ctx: *mut core::ffi::c_void) -> i32 {
    let pid: i32 = (bpf_get_current_pid_tgid() >> 32) as i32;

    if my_pid != pid {
        return 0;
    }

    /* next pointers for kernel address space have to be initialized from
     * BPF side, user-space mmaped addresses are still user-space addresses
     */
    k_probe_in.next = &raw mut k_probe_in;
    // __builtin_preserve_access_index(({k_core_in.next = &k_core_in;}));
    k_core_in.next = &raw mut k_core_in;

    k_probe_out = BPF_PROBE_READ!(&raw mut k_probe_in, next, next, func) as i64;
    k_core_out = BPF_CORE_READ!(&raw mut k_core_in, next, next, func) as i64;
    u_probe_out = BPF_PROBE_READ_USER!(u_probe_in, next, next, func) as i64;
    u_core_out = BPF_CORE_READ_USER!(u_core_in, next, next, func) as i64;

    let _ = ctx;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
