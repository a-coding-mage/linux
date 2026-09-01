// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type uint32_t = u32;
type pid_t = i32;

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub pid: pid_t,
    pub tgid: pid_t,
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
    pub seq_num: u64,
}

#[repr(C)]
pub struct bpf_iter__task {
    pub meta: *mut bpf_iter_meta,
    pub task: *mut task_struct,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_seq_printf(seq: *mut seq_file, fmt: *const i8, ...) -> i32;
    fn bpf_copy_from_user_task(
        dst: *mut core::ffi::c_void,
        size: u32,
        user_ptr: *const core::ffi::c_void,
        task: *mut task_struct,
        flags: u64,
    ) -> i32;
    fn bpf_copy_from_user_task_str(
        dst: *mut i8,
        size: u32,
        user_ptr: *const core::ffi::c_void,
        task: *mut task_struct,
        flags: u64,
    ) -> i32;
    fn bpf_task_pt_regs(task: *mut task_struct) -> *mut core::ffi::c_void;
    fn bpf_strncmp(s1: *const i8, s1_sz: u32, s2: *const i8) -> i32;
    fn PT_REGS_IP(regs: *mut pt_regs) -> u64;
}

const BPF_F_PAD_ZEROS: u64 = 1;

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [i8; 4] = [b'G' as i8, b'P' as i8, b'L' as i8, 0];

#[unsafe(no_mangle)]
pub static mut tid: uint32_t = 0;
#[unsafe(no_mangle)]
pub static mut num_unknown_tid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut num_known_tid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut user_ptr: *mut core::ffi::c_void = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut user_ptr_long: *mut core::ffi::c_void = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut pid: uint32_t = 0;

static mut big_str1: [i8; 5000] = [0; 5000];
static mut big_str2: [i8; 5005] = [0; 5005];
static mut big_str3: [i8; 4996] = [0; 4996];

// SEC("iter/task")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_task(ctx: *mut bpf_iter__task) -> i32 {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let task: *mut task_struct = (*ctx).task;
    static mut info: [i8; 16] = [
        b' ' as i8, b' ' as i8, b' ' as i8, b' ' as i8, b'=' as i8, b'=' as i8, b'=' as i8,
        b' ' as i8, b'E' as i8, b'N' as i8, b'D' as i8, b' ' as i8, b'=' as i8, b'=' as i8,
        b'=' as i8, 0,
    ];

    if task == core::ptr::null_mut() {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), &raw const info as *const i8);
        return 0;
    }

    if (*task).pid != tid as pid_t {
        num_unknown_tid += 1;
    } else {
        num_known_tid += 1;
    }

    if (*(*ctx).meta).seq_num == 0 {
        bpf_seq_printf(seq, c"    tgid      gid\n".as_ptr());
    }

    bpf_seq_printf(seq, c"%8d %8d\n".as_ptr(), (*task).tgid, (*task).pid);
    0
}

#[unsafe(no_mangle)]
pub static mut num_expected_failure_copy_from_user_task: i32 = 0;
#[unsafe(no_mangle)]
pub static mut num_expected_failure_copy_from_user_task_str: i32 = 0;
#[unsafe(no_mangle)]
pub static mut num_success_copy_from_user_task: i32 = 0;
#[unsafe(no_mangle)]
pub static mut num_success_copy_from_user_task_str: i32 = 0;

// SEC("iter.s/task")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_task_sleepable(ctx: *mut bpf_iter__task) -> i32 {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let task: *mut task_struct = (*ctx).task;
    static info: [i8; 16] = [
        b' ' as i8, b' ' as i8, b' ' as i8, b' ' as i8, b'=' as i8, b'=' as i8, b'=' as i8,
        b' ' as i8, b'E' as i8, b'N' as i8, b'D' as i8, b' ' as i8, b'=' as i8, b'=' as i8,
        b'=' as i8, 0,
    ];
    let mut regs: *mut pt_regs;
    let mut task_str1: [i8; 10] = [b'a' as i8; 10];
    let mut task_str2: [i8; 10] = [0; 10];
    let mut task_str3: [i8; 10] = [0; 10];
    let mut task_str4: [i8; 20] = [b'a' as i8; 20];
    let mut ptr: *mut core::ffi::c_void;
    let mut user_data: uint32_t = 0;
    let mut ret: i32;

    if task == core::ptr::null_mut() {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }

    /* Read an invalid pointer and ensure we get an error */
    ptr = core::ptr::null_mut();
    ret = bpf_copy_from_user_task(
        &mut user_data as *mut uint32_t as *mut core::ffi::c_void,
        core::mem::size_of::<uint32_t>() as u32,
        ptr,
        task,
        0,
    );
    if ret != 0 {
        num_expected_failure_copy_from_user_task += 1;
    } else {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }

    /* Try to read the contents of the task's instruction pointer from the
     * remote task's address space.
     */
    regs = bpf_task_pt_regs(task) as *mut pt_regs;
    if regs == core::ptr::null_mut() {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }
    ptr = PT_REGS_IP(regs) as *mut core::ffi::c_void;

    ret = bpf_copy_from_user_task(
        &mut user_data as *mut uint32_t as *mut core::ffi::c_void,
        core::mem::size_of::<uint32_t>() as u32,
        ptr,
        task,
        0,
    );
    if ret != 0 {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }

    num_success_copy_from_user_task += 1;

    /* Read an invalid pointer and ensure we get an error */
    ptr = core::ptr::null_mut();
    ret = bpf_copy_from_user_task_str(task_str1.as_mut_ptr(), task_str1.len() as u32, ptr, task, 0);
    if ret >= 0 || task_str1[9] != b'a' as i8 || task_str1[0] != b'\0' as i8 {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }

    /* Read an invalid pointer and ensure we get error with pad zeros flag */
    ptr = core::ptr::null_mut();
    ret = bpf_copy_from_user_task_str(
        task_str1.as_mut_ptr(),
        task_str1.len() as u32,
        ptr,
        task,
        BPF_F_PAD_ZEROS,
    );
    if ret >= 0 || task_str1[9] != b'\0' as i8 || task_str1[0] != b'\0' as i8 {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }

    num_expected_failure_copy_from_user_task_str += 1;

    /* Same length as the string */
    ret = bpf_copy_from_user_task_str(task_str2.as_mut_ptr(), 10, user_ptr, task, 0);
    /* only need to do the task pid check once */
    if bpf_strncmp(task_str2.as_ptr(), 10, c"test_data".as_ptr()) != 0
        || ret != 10
        || (*task).tgid != pid as pid_t
    {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }

    /* Shorter length than the string */
    ret = bpf_copy_from_user_task_str(task_str3.as_mut_ptr(), 2, user_ptr, task, 0);
    if bpf_strncmp(task_str3.as_ptr(), 2, c"t".as_ptr()) != 0 || ret != 2 {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }

    /* Longer length than the string */
    ret = bpf_copy_from_user_task_str(task_str4.as_mut_ptr(), 20, user_ptr, task, 0);
    if bpf_strncmp(task_str4.as_ptr(), 10, c"test_data".as_ptr()) != 0
        || ret != 10
        || task_str4[core::mem::size_of_val(&task_str4) - 1] != b'a' as i8
    {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }

    /* Longer length than the string with pad zeros flag */
    ret = bpf_copy_from_user_task_str(task_str4.as_mut_ptr(), 20, user_ptr, task, BPF_F_PAD_ZEROS);
    if bpf_strncmp(task_str4.as_ptr(), 10, c"test_data".as_ptr()) != 0
        || ret != 10
        || task_str4[core::mem::size_of_val(&task_str4) - 1] != b'\0' as i8
    {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }

    /* Longer length than the string past a page boundary */
    ret = bpf_copy_from_user_task_str((&raw mut big_str1).cast::<i8>(), 5000, user_ptr, task, 0);
    if bpf_strncmp((&raw const big_str1).cast::<i8>(), 10, c"test_data".as_ptr()) != 0
        || ret != 10
    {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }

    /* String that crosses a page boundary */
    ret = bpf_copy_from_user_task_str(
        (&raw mut big_str1).cast::<i8>(),
        5000,
        user_ptr_long,
        task,
        BPF_F_PAD_ZEROS,
    );
    if bpf_strncmp((&raw const big_str1).cast::<i8>(), 4, c"baba".as_ptr()) != 0
        || ret != 5000
        || bpf_strncmp((&raw const big_str1).cast::<i8>().add(4996), 4, c"bab".as_ptr()) != 0
    {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }

    let mut i: i32 = 0;
    while i < 4999 {
        if i % 2 == 0 {
            if big_str1[i as usize] != b'b' as i8 {
                bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
                return 0;
            }
        } else if big_str1[i as usize] != b'a' as i8 {
            bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
            return 0;
        }
        i += 1;
    }

    /* Longer length than the string that crosses a page boundary */
    ret = bpf_copy_from_user_task_str(
        (&raw mut big_str2).cast::<i8>(),
        5005,
        user_ptr_long,
        task,
        BPF_F_PAD_ZEROS,
    );
    if bpf_strncmp((&raw const big_str2).cast::<i8>(), 4, c"baba".as_ptr()) != 0
        || ret != 5000
        || bpf_strncmp((&raw const big_str2).cast::<i8>().add(4996), 5, c"bab\0".as_ptr()) != 0
    {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }

    /* Shorter length than the string that crosses a page boundary */
    ret = bpf_copy_from_user_task_str((&raw mut big_str3).cast::<i8>(), 4996, user_ptr_long, task, 0);
    if bpf_strncmp((&raw const big_str3).cast::<i8>(), 4, c"baba".as_ptr()) != 0
        || ret != 4996
        || bpf_strncmp((&raw const big_str3).cast::<i8>().add(4992), 4, c"bab".as_ptr()) != 0
    {
        bpf_seq_printf(seq, c"%s\n".as_ptr(), info.as_ptr());
        return 0;
    }

    num_success_copy_from_user_task_str += 1;

    if (*(*ctx).meta).seq_num == 0 {
        bpf_seq_printf(seq, c"    tgid      gid     data\n".as_ptr());
    }

    bpf_seq_printf(
        seq,
        c"%8d %8d %8d\n".as_ptr(),
        (*task).tgid,
        (*task).pid,
        user_data,
    );
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
