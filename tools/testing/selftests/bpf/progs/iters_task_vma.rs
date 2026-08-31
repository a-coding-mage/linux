// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// Dependencies from the original C source:
// "vmlinux.h", "bpf_experimental.h", <bpf/bpf_helpers.h>, and "bpf_misc.h".

#[allow(non_camel_case_types)]
type pid_t = i32;

#[allow(non_camel_case_types)]
type __u64 = u64;

#[repr(C)]
pub struct task_struct {
    pub pid: pid_t,
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_start: __u64,
    pub vm_end: __u64,
}

unsafe extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
}

pub static mut target_pid: pid_t = 0;
pub static mut vmas_seen: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_range {
    pub vm_start: __u64,
    pub vm_end: __u64,
}

pub static mut vm_ranges: [vm_range; 1000] = [vm_range {
    vm_start: 0,
    vm_end: 0,
}; 1000];

// Original section annotation: SEC("raw_tp/sys_enter")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_task_vma_for_each(ctx: *const core::ffi::c_void) -> i32 {
    let task: *mut task_struct = unsafe { bpf_get_current_task_btf() };
    let mut vma: *mut vm_area_struct;
    let mut seen: u32 = 0;

    let _ = ctx;

    if unsafe { (*task).pid != target_pid } {
        return 0;
    }

    if unsafe { vmas_seen != 0 } {
        return 0;
    }

    // Original C used:
    // bpf_for_each(task_vma, vma, task, 0) { ... }
    // This iterator is provided by BPF C macros and has no file-local Rust
    // equivalent; the loop body is translated below for that external iterator.
    while {
        // TODO: map the BPF task_vma iterator to the target Rust BPF bindings.
        vma = core::ptr::null_mut();
        !vma.is_null()
    } {
        if seen >= 1000 {
            break;
        }

        unsafe {
            vm_ranges[seen as usize].vm_start = (*vma).vm_start;
            vm_ranges[seen as usize].vm_end = (*vma).vm_end;
        }
        seen += 1;
    }

    unsafe {
        vmas_seen = seen;
    }
    0
}

// Original section annotation: SEC("license")
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
