// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// C dependencies: <vmlinux.h> and <bpf/bpf_helpers.h>

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub tgid: i32,
    pub pid: i32,
}

#[repr(C)]
pub struct file {
    pub f_op: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
    pub seq_num: u64,
}

#[repr(C)]
pub struct bpf_iter__task_file {
    pub meta: *mut bpf_iter_meta,
    pub task: *mut task_struct,
    pub file: *mut file,
    pub fd: u32,
}

unsafe extern "C" {
    fn BPF_SEQ_PRINTF(seq: *mut seq_file, fmt: *const u8, ...) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut count: i32 = 0;
#[unsafe(no_mangle)]
pub static mut tgid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut last_tgid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut unique_tgid_count: i32 = 0;

#[unsafe(link_section = "iter/task_file")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_task_file(ctx: *mut bpf_iter__task_file) -> i32 {
    let seq: *mut seq_file = unsafe { (*(*ctx).meta).seq };
    let task: *mut task_struct = unsafe { (*ctx).task };
    let file: *mut file = unsafe { (*ctx).file };
    let fd: u32 = unsafe { (*ctx).fd };

    if task == core::ptr::null_mut() || file == core::ptr::null_mut() {
        return 0;
    }

    if unsafe { (*(*ctx).meta).seq_num } == 0 {
        unsafe {
            count = 0;
            BPF_SEQ_PRINTF(seq, b"    tgid      gid       fd      file\n\0".as_ptr());
        }
    }

    if unsafe { tgid == (*task).tgid && (*task).tgid != (*task).pid } {
        unsafe {
            count += 1;
        }
    }

    if unsafe { last_tgid != (*task).tgid } {
        unsafe {
            last_tgid = (*task).tgid;
            unique_tgid_count += 1;
        }
    }

    unsafe {
        BPF_SEQ_PRINTF(
            seq,
            b"%8d %8d %8d %lx\n\0".as_ptr(),
            (*task).tgid,
            (*task).pid,
            fd,
            (*file).f_op as isize,
        );
    }
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
