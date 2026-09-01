// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2021 Hengqi Chen

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

type pid_t = i32;
type __u32 = u32;

const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_tail_call(ctx: *mut core::ffi::c_void, map: *mut core::ffi::c_void, index: __u32);
}

#[no_mangle]
pub static mut my_pid: pid_t = 0;

#[no_mangle]
pub static mut value: i32 = 0;

// SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn tailcall_1(ctx: *mut core::ffi::c_void) -> i32 {
    unsafe {
        value = 42;
    }
    0
}

#[repr(C)]
pub struct ProgArrayInit {
    // __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    pub type_: u32,
    // __uint(max_entries, 2);
    pub max_entries: u32,
    // __uint(key_size, sizeof(__u32));
    pub key_size: u32,
    // __array(values, int (void *));
    pub values: [*mut core::ffi::c_void; 2],
}

// struct { ... } prog_array_init SEC(".maps") = { .values = { [1] = (void *)&tailcall_1, }, };
#[no_mangle]
pub static mut prog_array_init: ProgArrayInit = ProgArrayInit {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 2,
    key_size: core::mem::size_of::<__u32>() as u32,
    values: [
        core::ptr::null_mut(),
        tailcall_1 as *mut core::ffi::c_void,
    ],
};

// SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn entry(ctx: *mut core::ffi::c_void) -> i32 {
    let pid: pid_t = unsafe { (bpf_get_current_pid_tgid() >> 32) as pid_t };

    if unsafe { pid != my_pid } {
        return 0;
    }

    unsafe {
        bpf_tail_call(
            ctx,
            core::ptr::addr_of_mut!(prog_array_init) as *mut core::ffi::c_void,
            1,
        );
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
