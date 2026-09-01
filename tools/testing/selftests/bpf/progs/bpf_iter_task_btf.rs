// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020, Oracle and/or its affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_core_read.h>
// #include <errno.h>

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut tasks: i64 = 0;
#[no_mangle]
pub static mut seq_err: i64 = 0;
#[no_mangle]
pub static mut skip: bool = false;

#[repr(C)]
pub struct bpf_iter__task {
    pub meta: *mut bpf_iter_meta,
    pub task: *mut task_struct,
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
    pub seq_num: u64,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_ptr {
    pub ptr: *mut core::ffi::c_void,
    pub type_id: u32,
    pub flags: u32,
}

extern "C" {
    fn bpf_seq_printf_btf(
        seq: *mut seq_file,
        ptr: *mut btf_ptr,
        ptr_size: u32,
        flags: u64,
    ) -> i64;
}

const ERANGE: i64 = 34;
const E2BIG: i64 = 7;

#[link_section = "iter/task"]
#[no_mangle]
pub unsafe extern "C" fn dump_task_struct(ctx: *mut bpf_iter__task) -> i32 {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let task: *mut task_struct = (*ctx).task;
    static mut ptr: btf_ptr = btf_ptr {
        ptr: core::ptr::null_mut(),
        type_id: 0,
        flags: 0,
    };
    let ret: i64;

    // C conditional: #if __has_builtin(__builtin_btf_type_id)
    #[cfg(has_builtin_btf_type_id)]
    {
        ptr.type_id = bpf_core_type_id_kernel!(task_struct);
        ptr.ptr = task as *mut core::ffi::c_void;

        if (*(*ctx).meta).seq_num == 0 {
            BPF_SEQ_PRINTF!(seq, "Raw BTF task\n");
        }

        ret = bpf_seq_printf_btf(seq, &mut ptr, core::mem::size_of::<btf_ptr>() as u32, 0);
        match ret {
            0 => {
                tasks += 1;
            }
            -ERANGE => {
                /* NULL task or task->fs, don't count it as an error. */
            }
            -E2BIG => {
                return 1;
            }
            _ => {
                seq_err = ret;
            }
        }
    }
    #[cfg(not(has_builtin_btf_type_id))]
    {
        skip = true;
    }

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
