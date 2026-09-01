// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

pub type __u32 = u32;
pub type __u64 = u64;

#[repr(C)]
pub struct task_struct {
    _unused: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_smp_processor_id() -> __u32;
}

#[unsafe(no_mangle)]
pub static mut count: __u32 = 0;

#[unsafe(no_mangle)]
pub static mut on_cpu: __u32 = 0xffffffff;

// SEC("raw_tp/task_rename")
// BPF_PROG(rename, struct task_struct *task, char *comm)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rename(task: *mut task_struct, comm: *mut ::core::ffi::c_char) -> i32 {
    unsafe {
        count = count.wrapping_add(1);
        if task as __u64 == 0x1234_u64 && comm as __u64 == 0x5678_u64 {
            on_cpu = bpf_get_smp_processor_id();
            return (task as isize).wrapping_add(comm as isize) as i32;
        }

        0
    }
}

// SEC("license")
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
