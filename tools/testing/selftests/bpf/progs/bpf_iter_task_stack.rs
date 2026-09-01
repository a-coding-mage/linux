// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

pub const MAX_STACK_TRACE_DEPTH: usize = 64;
#[no_mangle]
pub static mut entries: [core::ffi::c_ulong; MAX_STACK_TRACE_DEPTH] = [0; MAX_STACK_TRACE_DEPTH];
pub const SIZE_OF_ULONG: usize = core::mem::size_of::<core::ffi::c_ulong>();

#[repr(C)]
pub struct bpf_iter__task {
    pub meta: *mut bpf_iter_meta,
    pub task: *mut task_struct,
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub pid: u32,
}

pub const BPF_F_USER_STACK: u64 = 1 << 8;

extern "C" {
    pub fn bpf_get_task_stack(
        task: *mut task_struct,
        buf: *mut core::ffi::c_void,
        size: u32,
        flags: u64,
    ) -> i64;
    pub fn bpf_seq_write(seq: *mut seq_file, data: *const core::ffi::c_void, len: u64) -> i64;
}

// External BPF helper macro equivalent supplied by bpf_helpers.h in C.
macro_rules! BPF_SEQ_PRINTF {
    ($seq:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        let _ = ($seq, $fmt $(, $arg)*);
    }};
}

#[no_mangle]
#[link_section = "iter/task"]
pub unsafe extern "C" fn dump_task_stack(ctx: *mut bpf_iter__task) -> i32 {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let task: *mut task_struct = (*ctx).task;
    let mut i: isize;
    let retlen: isize;

    if task == core::ptr::null_mut() {
        return 0;
    }

    retlen = bpf_get_task_stack(
        task,
        entries.as_mut_ptr() as *mut core::ffi::c_void,
        (MAX_STACK_TRACE_DEPTH * SIZE_OF_ULONG) as u32,
        0,
    ) as isize;
    if retlen < 0 {
        return 0;
    }

    BPF_SEQ_PRINTF!(
        seq,
        "pid: %8u num_entries: %8u\n",
        (*task).pid,
        retlen / SIZE_OF_ULONG as isize
    );
    i = 0;
    while i < MAX_STACK_TRACE_DEPTH as isize {
        if retlen > i * SIZE_OF_ULONG as isize {
            BPF_SEQ_PRINTF!(
                seq,
                "[<0>] %pB\n",
                entries[i as usize] as *mut core::ffi::c_void
            );
        }
        i += 1;
    }
    BPF_SEQ_PRINTF!(seq, "\n");

    0
}

#[no_mangle]
pub static mut num_user_stacks: i32 = 0;

#[no_mangle]
#[link_section = "iter/task"]
pub unsafe extern "C" fn get_task_user_stacks(ctx: *mut bpf_iter__task) -> i32 {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let task: *mut task_struct = (*ctx).task;
    let mut buf_sz: u64 = 0;
    let res: i64;

    if task == core::ptr::null_mut() {
        return 0;
    }

    res = bpf_get_task_stack(
        task,
        entries.as_mut_ptr() as *mut core::ffi::c_void,
        (MAX_STACK_TRACE_DEPTH * SIZE_OF_ULONG) as u32,
        BPF_F_USER_STACK,
    );
    if res <= 0 {
        return 0;
    }

    /* Only one task, the current one, should succeed */
    num_user_stacks += 1;

    buf_sz += res as u64;

    /* If the verifier doesn't refine bpf_get_task_stack res, and instead
     * assumes res is entirely unknown, this program will fail to load as
     * the verifier will believe that max buf_sz value allows reading
     * past the end of entries in bpf_seq_write call
     */
    bpf_seq_write(
        seq,
        &entries as *const _ as *const core::ffi::c_void,
        buf_sz,
    );
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
