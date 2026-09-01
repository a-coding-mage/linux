// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2020 Google LLC.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

/* Dependencies from vmlinux.h, errno.h, bpf_helpers.h, and bpf_tracing.h. */
pub type __u32 = u32;

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    pub d_inode: *mut inode,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct socket {
    pub sk: *mut sock,
}

#[repr(C)]
pub struct sockaddr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub f_inode: *mut inode,
}

#[repr(C)]
pub struct linux_binprm {
    pub file: *mut file,
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_task_storage_get(
        map: *mut core::ffi::c_void,
        task: *mut task_struct,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut local_storage;
    fn bpf_task_storage_delete(
        map: *mut core::ffi::c_void,
        task: *mut task_struct,
    ) -> i32;
    fn bpf_inode_storage_get(
        map: *mut core::ffi::c_void,
        inode: *mut inode,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut local_storage;
    fn bpf_inode_storage_delete(
        map: *mut core::ffi::c_void,
        inode: *mut inode,
    ) -> i32;
    fn bpf_sk_storage_get(
        map: *mut core::ffi::c_void,
        sk: *mut sock,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut local_storage;
    fn bpf_sk_storage_delete(map: *mut core::ffi::c_void, sk: *mut sock) -> i32;
}

const EPERM: i32 = 1;
const DUMMY_STORAGE_VALUE: __u32 = 0xdeadbeef;
const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 1;

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut monitored_pid: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut inode_storage_result: i32 = -1;
#[unsafe(no_mangle)]
pub static mut sk_storage_result: i32 = -1;
#[unsafe(no_mangle)]
pub static mut task_storage_result: i32 = -1;

#[repr(C)]
pub struct local_storage {
    pub exec_inode: *mut inode,
    pub value: __u32,
}

#[repr(C)]
pub struct bpf_local_storage_map {
    _private: [u8; 0],
}

/* BPF map declarations translated from:
 * __uint(type, BPF_MAP_TYPE_*_STORAGE);
 * __uint(map_flags, BPF_F_NO_PREALLOC[, BPF_F_CLONE]);
 * __type(key, int);
 * __type(value, struct local_storage);
 */
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut inode_storage_map: bpf_local_storage_map = bpf_local_storage_map { _private: [] };

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sk_storage_map: bpf_local_storage_map = bpf_local_storage_map { _private: [] };

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sk_storage_map2: bpf_local_storage_map = bpf_local_storage_map { _private: [] };

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut task_storage_map: bpf_local_storage_map = bpf_local_storage_map { _private: [] };

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut task_storage_map2: bpf_local_storage_map = bpf_local_storage_map { _private: [] };

#[unsafe(link_section = "lsm/inode_unlink")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unlink_hook(_dir: *mut inode, victim: *mut dentry) -> i32 {
    let pid: __u32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as __u32;
    let mut storage: *mut local_storage;
    let task: *mut task_struct;
    let is_self_unlink: bool;

    if pid != unsafe { monitored_pid } {
        return 0;
    }

    task = unsafe { bpf_get_current_task_btf() };
    if task.is_null() {
        return 0;
    }

    unsafe {
        task_storage_result = -1;
    }

    storage = unsafe {
        bpf_task_storage_get(
            core::ptr::addr_of_mut!(task_storage_map).cast(),
            task,
            core::ptr::null_mut(),
            0,
        )
    };
    if storage.is_null() {
        return 0;
    }

    /* Don't let an executable delete itself */
    is_self_unlink = unsafe { (*storage).exec_inode == (*victim).d_inode };

    storage = unsafe {
        bpf_task_storage_get(
            core::ptr::addr_of_mut!(task_storage_map2).cast(),
            task,
            core::ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        )
    };
    if storage.is_null() || unsafe { (*storage).value != 0 } {
        return 0;
    }

    if unsafe { bpf_task_storage_delete(core::ptr::addr_of_mut!(task_storage_map2).cast(), task) }
        != 0
    {
        return 0;
    }

    if unsafe { bpf_task_storage_delete(core::ptr::addr_of_mut!(task_storage_map).cast(), task) }
        != 0
    {
        return 0;
    }

    unsafe {
        task_storage_result = 0;
    }

    if is_self_unlink { -EPERM } else { 0 }
}

#[unsafe(link_section = "lsm.s/inode_rename")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn inode_rename(
    _old_dir: *mut inode,
    old_dentry: *mut dentry,
    _new_dir: *mut inode,
    new_dentry: *mut dentry,
    _flags: u32,
) -> i32 {
    let storage: *mut local_storage;
    let err: i32;

    /* new_dentry->d_inode can be NULL when the inode is renamed to a file
     * that did not exist before. The helper should be able to handle this
     * NULL pointer.
     */
    unsafe {
        bpf_inode_storage_get(
            core::ptr::addr_of_mut!(inode_storage_map).cast(),
            (*new_dentry).d_inode,
            core::ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        );
    }

    storage = unsafe {
        bpf_inode_storage_get(
            core::ptr::addr_of_mut!(inode_storage_map).cast(),
            (*old_dentry).d_inode,
            core::ptr::null_mut(),
            0,
        )
    };
    if storage.is_null() {
        return 0;
    }

    if unsafe { (*storage).value != DUMMY_STORAGE_VALUE } {
        unsafe {
            inode_storage_result = -1;
        }
    }

    err = unsafe {
        bpf_inode_storage_delete(
            core::ptr::addr_of_mut!(inode_storage_map).cast(),
            (*old_dentry).d_inode,
        )
    };
    if err == 0 {
        unsafe {
            inode_storage_result = err;
        }
    }

    0
}

#[unsafe(link_section = "lsm.s/socket_bind")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn socket_bind(
    sock: *mut socket,
    _address: *mut sockaddr,
    _addrlen: i32,
) -> i32 {
    let pid: __u32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as __u32;
    let mut storage: *mut local_storage;
    let sk: *mut sock = unsafe { (*sock).sk };

    if pid != unsafe { monitored_pid } || sk.is_null() {
        return 0;
    }

    storage = unsafe {
        bpf_sk_storage_get(
            core::ptr::addr_of_mut!(sk_storage_map).cast(),
            sk,
            core::ptr::null_mut(),
            0,
        )
    };
    if storage.is_null() {
        return 0;
    }

    unsafe {
        sk_storage_result = -1;
    }
    if unsafe { (*storage).value != DUMMY_STORAGE_VALUE } {
        return 0;
    }

    /* This tests that we can associate multiple elements
     * with the local storage.
     */
    storage = unsafe {
        bpf_sk_storage_get(
            core::ptr::addr_of_mut!(sk_storage_map2).cast(),
            sk,
            core::ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        )
    };
    if storage.is_null() {
        return 0;
    }

    if unsafe { bpf_sk_storage_delete(core::ptr::addr_of_mut!(sk_storage_map2).cast(), sk) } != 0 {
        return 0;
    }

    if unsafe { bpf_sk_storage_delete(core::ptr::addr_of_mut!(sk_storage_map).cast(), sk) } != 0 {
        return 0;
    }

    unsafe {
        sk_storage_result = 0;
    }
    0
}

#[unsafe(link_section = "lsm.s/socket_post_create")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn socket_post_create(
    sock: *mut socket,
    _family: i32,
    _type: i32,
    _protocol: i32,
    _kern: i32,
) -> i32 {
    let pid: __u32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as __u32;
    let storage: *mut local_storage;
    let sk: *mut sock = unsafe { (*sock).sk };

    if pid != unsafe { monitored_pid } || sk.is_null() {
        return 0;
    }

    storage = unsafe {
        bpf_sk_storage_get(
            core::ptr::addr_of_mut!(sk_storage_map).cast(),
            sk,
            core::ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        )
    };
    if storage.is_null() {
        return 0;
    }

    unsafe {
        (*storage).value = DUMMY_STORAGE_VALUE;
    }

    0
}

/* This uses the local storage to remember the inode of the binary that a
 * process was originally executing.
 */
#[unsafe(link_section = "lsm.s/bprm_committed_creds")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exec(bprm: *mut linux_binprm) {
    let pid: __u32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as __u32;
    let mut storage: *mut local_storage;

    if pid != unsafe { monitored_pid } {
        return;
    }

    storage = unsafe {
        bpf_task_storage_get(
            core::ptr::addr_of_mut!(task_storage_map).cast(),
            bpf_get_current_task_btf(),
            core::ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        )
    };
    if !storage.is_null() {
        unsafe {
            (*storage).exec_inode = (*(*bprm).file).f_inode;
        }
    }

    storage = unsafe {
        bpf_inode_storage_get(
            core::ptr::addr_of_mut!(inode_storage_map).cast(),
            (*(*bprm).file).f_inode,
            core::ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        )
    };
    if storage.is_null() {
        return;
    }

    unsafe {
        (*storage).value = DUMMY_STORAGE_VALUE;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
