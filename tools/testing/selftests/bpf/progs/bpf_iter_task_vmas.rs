// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
/* Dependencies in the original C source:
 * #include <vmlinux.h>
 * #include <bpf/bpf_helpers.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;
type __u64 = u64;
type pid_t = i32;

#[repr(C)]
pub struct bpf_iter__task_vma {
    pub meta: *mut bpf_iter_meta,
    pub task: *mut task_struct,
    pub vma: *mut vm_area_struct,
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
}

#[repr(C)]
pub struct seq_file {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub tgid: pid_t,
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_start: __u64,
    pub vm_end: __u64,
    pub vm_flags: __u64,
    pub vm_file: *mut file,
    pub vm_pgoff: __u64,
}

#[repr(C)]
pub struct file {
    pub f_path: path,
    pub f_inode: *mut inode,
}

#[repr(C)]
pub struct path {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct inode {
    pub i_ino: __u64,
    pub i_sb: *mut super_block,
}

#[repr(C)]
pub struct super_block {
    pub s_dev: __u32,
}

unsafe extern "C" {
    fn bpf_d_path(path: *const path, buf: *mut i8, sz: __u32) -> i64;
    fn BPF_SEQ_PRINTF(seq: *mut seq_file, fmt: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [i8; 4] = [b'G' as i8, b'P' as i8, b'L' as i8, 0];

/* Copied from mm.h */
const VM_READ: __u64 = 0x00000001;
const VM_WRITE: __u64 = 0x00000002;
const VM_EXEC: __u64 = 0x00000004;
const VM_MAYSHARE: __u64 = 0x00000080;

/* Copied from kdev_t.h */
const MINORBITS: __u32 = 20;
const MINORMASK: __u32 = (1u32 << MINORBITS) - 1;

#[inline(always)]
fn MAJOR(dev: __u32) -> u32 {
    (dev >> MINORBITS) as u32
}

#[inline(always)]
fn MINOR(dev: __u32) -> u32 {
    (dev & MINORMASK) as u32
}

const D_PATH_BUF_SIZE: usize = 1024;

#[unsafe(no_mangle)]
pub static mut d_path_buf: [i8; D_PATH_BUF_SIZE] = [0; D_PATH_BUF_SIZE];
#[unsafe(no_mangle)]
pub static mut pid: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut one_task: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut one_task_error: __u32 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "iter/task_vma")]
pub unsafe extern "C" fn proc_maps(ctx: *mut bpf_iter__task_vma) -> i32 {
    let vma: *mut vm_area_struct = unsafe { (*ctx).vma };
    let seq: *mut seq_file = unsafe { (*(*ctx).meta).seq };
    let task: *mut task_struct = unsafe { (*ctx).task };
    let file: *mut file;
    let mut perm_str: [i8; 5] = [b'-' as i8, b'-' as i8, b'-' as i8, b'-' as i8, 0];

    if task == core::ptr::null_mut() || vma == core::ptr::null_mut() {
        return 0;
    }

    file = unsafe { (*vma).vm_file };
    if unsafe { (*task).tgid } != unsafe { pid as pid_t } {
        if unsafe { one_task } != 0 {
            unsafe {
                one_task_error = 1;
            }
        }
        return 0;
    }
    perm_str[0] = if unsafe { (*vma).vm_flags } & VM_READ != 0 {
        b'r' as i8
    } else {
        b'-' as i8
    };
    perm_str[1] = if unsafe { (*vma).vm_flags } & VM_WRITE != 0 {
        b'w' as i8
    } else {
        b'-' as i8
    };
    perm_str[2] = if unsafe { (*vma).vm_flags } & VM_EXEC != 0 {
        b'x' as i8
    } else {
        b'-' as i8
    };
    perm_str[3] = if unsafe { (*vma).vm_flags } & VM_MAYSHARE != 0 {
        b's' as i8
    } else {
        b'p' as i8
    };
    unsafe {
        BPF_SEQ_PRINTF(
            seq,
            c"%08llx-%08llx %s ".as_ptr(),
            (*vma).vm_start,
            (*vma).vm_end,
            perm_str.as_ptr(),
        );
    }

    if !file.is_null() {
        let dev: __u32 = unsafe { (*(*(*file).f_inode).i_sb).s_dev };

        unsafe {
            bpf_d_path(
                &(*file).f_path as *const path,
                core::ptr::addr_of_mut!(d_path_buf) as *mut i8,
                D_PATH_BUF_SIZE as __u32,
            );

            BPF_SEQ_PRINTF(seq, c"%08llx ".as_ptr(), (*vma).vm_pgoff << 12);
            BPF_SEQ_PRINTF(
                seq,
                c"%02x:%02x %llu".as_ptr(),
                MAJOR(dev),
                MINOR(dev),
                (*(*file).f_inode).i_ino,
            );
            BPF_SEQ_PRINTF(
                seq,
                c"\t%s\n".as_ptr(),
                core::ptr::addr_of!(d_path_buf) as *const i8,
            );
        }
    } else {
        unsafe {
            BPF_SEQ_PRINTF(seq, c"%08llx 00:00 0\n".as_ptr(), 0u64);
        }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
