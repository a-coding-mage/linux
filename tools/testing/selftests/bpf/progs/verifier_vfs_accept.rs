// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Google LLC. */

/*
 * Translated from C source that included:
 * <vmlinux.h>, <errno.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
 * "bpf_misc.h", and "bpf_experimental.h".
 *
 * Kernel/BPF types, helper declarations, and verifier annotation macros are
 * expected to be supplied by the surrounding BPF Rust build environment.
 */

const EACCES: i32 = 13;

static mut buf: [u8; 64] = [0; 64];

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_get_task_exe_file(task: *mut task_struct) -> *mut file;
    fn bpf_put_file(file: *mut file);
    fn bpf_path_d_path(path: *const path, buf: *mut u8, sz: usize) -> i32;
    fn __sink<T>(value: T);
}

/* SEC("lsm.s/file_open") */
/* __success */
#[no_mangle]
#[link_section = "lsm.s/file_open"]
pub unsafe extern "C" fn get_task_exe_file_and_put_kfunc_from_current_sleepable() -> i32 {
    let acquired: *mut file;

    acquired = bpf_get_task_exe_file(bpf_get_current_task_btf());
    if acquired.is_null() {
        return 0;
    }

    bpf_put_file(acquired);
    0
}

/* SEC("lsm/file_open") */
/* __success */
#[no_mangle]
#[link_section = "lsm/file_open"]
pub unsafe extern "C" fn get_task_exe_file_and_put_kfunc_from_current_non_sleepable(
    file: *mut file,
) -> i32 {
    let acquired: *mut file;

    acquired = bpf_get_task_exe_file(bpf_get_current_task_btf());
    if acquired.is_null() {
        return 0;
    }

    bpf_put_file(acquired);
    0
}

/* SEC("lsm.s/task_alloc") */
/* __success */
#[no_mangle]
#[link_section = "lsm.s/task_alloc"]
pub unsafe extern "C" fn get_task_exe_file_and_put_kfunc_from_argument(
    task: *mut task_struct,
) -> i32 {
    let acquired: *mut file;

    acquired = bpf_get_task_exe_file(task);
    if acquired.is_null() {
        return 0;
    }

    bpf_put_file(acquired);
    0
}

/* SEC("lsm.s/inode_getattr") */
/* __success */
#[no_mangle]
#[link_section = "lsm.s/inode_getattr"]
pub unsafe extern "C" fn path_d_path_from_path_argument(path: *mut path) -> i32 {
    let ret: i32;

    ret = bpf_path_d_path(path, buf.as_mut_ptr(), core::mem::size_of_val(&buf));
    __sink(ret);
    0
}

/* SEC("lsm.s/file_open") */
/* __success */
#[no_mangle]
#[link_section = "lsm.s/file_open"]
pub unsafe extern "C" fn path_d_path_from_file_argument(file: *mut file) -> i32 {
    let ret: i32;
    let path: *const path;

    /* The f_path member is a path which is embedded directly within a
     * file. Therefore, a pointer to such embedded members are still
     * recognized by the BPF verifier as being PTR_TRUSTED as it's
     * essentially PTR_TRUSTED w/ a non-zero fixed offset.
     */
    path = &(*file).f_path;
    ret = bpf_path_d_path(path, buf.as_mut_ptr(), core::mem::size_of_val(&buf));
    __sink(ret);
    0
}

/* SEC("lsm.s/inode_rename") */
/* __success */
#[no_mangle]
#[link_section = "lsm.s/inode_rename"]
pub unsafe extern "C" fn inode_rename(
    old_dir: *mut inode,
    old_dentry: *mut dentry,
    new_dir: *mut inode,
    new_dentry: *mut dentry,
    flags: u32,
) -> i32 {
    let inode: *mut inode = (*new_dentry).d_inode;
    let ino: ino_t;

    if inode.is_null() {
        return 0;
    }
    ino = (*inode).i_ino;
    if ino == 0 {
        return -EACCES;
    }
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
