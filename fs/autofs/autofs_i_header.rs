/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright 1997-1998 Transmeta Corporation - All Rights Reserved
 *  Copyright 2005-2006 Ian Kent <raven@themaw.net>
 */

/* Internal header file for autofs. */
/* C dependencies: linux/auto_fs.h, linux/auto_dev-ioctl.h, and kernel VFS headers. */

pub const AUTOFS_IOC_FIRST: u32 = AUTOFS_IOC_READY;
pub const AUTOFS_IOC_COUNT: u32 = 32;
pub const AUTOFS_DEV_IOCTL_IOC_FIRST: u32 = AUTOFS_DEV_IOCTL_VERSION;
pub const AUTOFS_DEV_IOCTL_IOC_COUNT: u32 =
    AUTOFS_DEV_IOCTL_ISMOUNTPOINT_CMD - AUTOFS_DEV_IOCTL_VERSION_CMD;

extern "C" {
    pub static mut autofs_fs_type: file_system_type;
}

#[repr(C)]
pub struct autofs_info {
    pub dentry: *mut dentry,
    pub flags: ::core::ffi::c_int,
    pub expire_complete: completion,
    pub active: list_head,
    pub expiring: list_head,
    pub sbi: *mut autofs_sb_info,
    pub exp_timeout: ::core::ffi::c_ulong,
    pub last_used: ::core::ffi::c_ulong,
    pub count: ::core::ffi::c_int,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub rcu: rcu_head,
}

pub const AUTOFS_INF_EXPIRING: u32 = 1 << 0;
pub const AUTOFS_INF_WANT_EXPIRE: u32 = 1 << 1;
pub const AUTOFS_INF_PENDING: u32 = 1 << 2;
pub const AUTOFS_INF_EXPIRE_SET: u32 = 1 << 3;

#[repr(C)]
pub struct autofs_wait_queue {
    pub queue: wait_queue_head_t,
    pub next: *mut autofs_wait_queue,
    pub wait_queue_token: autofs_wqt_t,
    pub name: qstr,
    pub offset: u32,
    pub dev: u32,
    pub ino: u64,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub pid: pid_t,
    pub tgid: pid_t,
    pub status: ::core::ffi::c_int,
    pub wait_ctr: u32,
}

pub const AUTOFS_SBI_MAGIC: u32 = 0x6d4a556d;
pub const AUTOFS_SBI_CATATONIC: u32 = 0x0001;
pub const AUTOFS_SBI_STRICTEXPIRE: u32 = 0x0002;
pub const AUTOFS_SBI_IGNORE: u32 = 0x0004;

#[repr(C)]
pub struct autofs_sb_info {
    pub magic: u32,
    pub pipefd: ::core::ffi::c_int,
    pub pipe: *mut file,
    pub oz_pgrp: *mut pid,
    pub mnt_ns_id: u64,
    pub version: ::core::ffi::c_int,
    pub sub_version: ::core::ffi::c_int,
    pub min_proto: ::core::ffi::c_int,
    pub max_proto: ::core::ffi::c_int,
    pub flags: u32,
    pub exp_timeout: ::core::ffi::c_ulong,
    pub type_: u32,
    pub sb: *mut super_block,
    pub wq_mutex: mutex,
    pub pipe_mutex: mutex,
    pub fs_lock: spinlock_t,
    pub queues: *mut autofs_wait_queue,
    pub lookup_lock: spinlock_t,
    pub active_list: list_head,
    pub expiring_list: list_head,
    pub rcu: rcu_head,
}

#[inline]
pub unsafe fn autofs_sbi(sb: *mut super_block) -> *mut autofs_sb_info {
    (*sb).s_fs_info as *mut autofs_sb_info
}

#[inline]
pub unsafe fn autofs_dentry_ino(dentry: *mut dentry) -> *mut autofs_info {
    (*dentry).d_fsdata as *mut autofs_info
}

#[inline]
pub unsafe fn autofs_oz_mode(sbi: *mut autofs_sb_info) -> ::core::ffi::c_int {
    (( (*sbi).flags & AUTOFS_SBI_CATATONIC) != 0 || task_pgrp(current) == (*sbi).oz_pgrp) as _
}

#[inline]
pub unsafe fn autofs_empty(ino: *mut autofs_info) -> bool { (*ino).count < 2 }

extern "C" {
    pub fn autofs_get_inode(sb: *mut super_block, mode: umode_t) -> *mut inode;
    pub fn autofs_free_ino(ino: *mut autofs_info);
    pub fn is_autofs_dentry(dentry: *mut dentry) -> ::core::ffi::c_int;
    pub fn autofs_expire_wait(path: *const path, rcu_walk: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn autofs_expire_run(sb: *mut super_block, mnt: *mut vfsmount, sbi: *mut autofs_sb_info, pkt: *mut autofs_packet_expire) -> ::core::ffi::c_int;
    pub fn autofs_do_expire_multi(sb: *mut super_block, mnt: *mut vfsmount, sbi: *mut autofs_sb_info, how: u32) -> ::core::ffi::c_int;
    pub fn autofs_expire_multi(sb: *mut super_block, mnt: *mut vfsmount, sbi: *mut autofs_sb_info, arg: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn autofs_dev_ioctl_init() -> ::core::ffi::c_int;
    pub fn autofs_dev_ioctl_exit();
    pub fn autofs_init_fs_context(fc: *mut fs_context) -> ::core::ffi::c_int;
    pub fn autofs_new_ino(sbi: *mut autofs_sb_info) -> *mut autofs_info;
    pub fn autofs_clean_ino(ino: *mut autofs_info);
    pub fn autofs_wait(sbi: *mut autofs_sb_info, path: *const path, notify: autofs_notify) -> ::core::ffi::c_int;
    pub fn autofs_wait_release(sbi: *mut autofs_sb_info, token: autofs_wqt_t, status: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn autofs_catatonic_mode(sbi: *mut autofs_sb_info);
    pub fn autofs_kill_sb(sb: *mut super_block);
}

extern "C" {
    pub static autofs_symlink_inode_operations: inode_operations;
    pub static autofs_dir_inode_operations: inode_operations;
    pub static autofs_dir_operations: file_operations;
    pub static autofs_root_operations: file_operations;
    pub static autofs_dentry_operations: dentry_operations;
    pub static autofs_param_specs: fs_parameter_spec;
}

#[inline]
pub unsafe fn __managed_dentry_set_managed(dentry: *mut dentry) {
    (*dentry).d_flags |= DCACHE_NEED_AUTOMOUNT | DCACHE_MANAGE_TRANSIT;
}

#[inline]
pub unsafe fn managed_dentry_set_managed(dentry: *mut dentry) {
    spin_lock(&mut (*dentry).d_lock);
    __managed_dentry_set_managed(dentry);
    spin_unlock(&mut (*dentry).d_lock);
}

#[inline]
pub unsafe fn __managed_dentry_clear_managed(dentry: *mut dentry) {
    (*dentry).d_flags &= !(DCACHE_NEED_AUTOMOUNT | DCACHE_MANAGE_TRANSIT);
}

#[inline]
pub unsafe fn managed_dentry_clear_managed(dentry: *mut dentry) {
    spin_lock(&mut (*dentry).d_lock);
    __managed_dentry_clear_managed(dentry);
    spin_unlock(&mut (*dentry).d_lock);
}

#[inline]
pub unsafe fn autofs_check_pipe(pipe: *mut file) -> ::core::ffi::c_int {
    if (*pipe).f_mode & FMODE_PATH != 0 { return -EINVAL; }
    if (*pipe).f_mode & FMODE_CAN_WRITE == 0 { return -EINVAL; }
    if !S_ISFIFO((*file_inode(pipe)).i_mode) { return -EINVAL; }
    0
}

#[inline]
pub unsafe fn autofs_set_packet_pipe_flags(pipe: *mut file) {
    (*pipe).f_flags |= O_DIRECT;
    (*pipe).f_flags &= !O_NONBLOCK;
}

#[inline]
pub unsafe fn autofs_prepare_pipe(pipe: *mut file) -> ::core::ffi::c_int {
    let ret = autofs_check_pipe(pipe);
    if ret < 0 { return ret; }
    autofs_set_packet_pipe_flags(pipe);
    0
}

#[inline]
pub unsafe fn autofs_get_dev(sbi: *mut autofs_sb_info) -> u32 {
    new_encode_dev((*(*sbi).sb).s_dev)
}

#[inline]
pub unsafe fn autofs_get_ino(sbi: *mut autofs_sb_info) -> u64 {
    (*d_inode((*(*sbi).sb).s_root)).i_ino
}

#[inline]
pub unsafe fn __autofs_add_expiring(dentry: *mut dentry) {
    let sbi = autofs_sbi((*dentry).d_sb);
    let ino = autofs_dentry_ino(dentry);
    if !ino.is_null() && list_empty(&mut (*ino).expiring) {
        list_add(&mut (*ino).expiring, &mut (*sbi).expiring_list);
    }
}

#[inline]
pub unsafe fn autofs_add_expiring(dentry: *mut dentry) {
    let sbi = autofs_sbi((*dentry).d_sb);
    let ino = autofs_dentry_ino(dentry);
    if !ino.is_null() {
        spin_lock(&mut (*sbi).lookup_lock);
        if list_empty(&mut (*ino).expiring) {
            list_add(&mut (*ino).expiring, &mut (*sbi).expiring_list);
        }
        spin_unlock(&mut (*sbi).lookup_lock);
    }
}

#[inline]
pub unsafe fn autofs_del_expiring(dentry: *mut dentry) {
    let sbi = autofs_sbi((*dentry).d_sb);
    let ino = autofs_dentry_ino(dentry);
    if !ino.is_null() {
        spin_lock(&mut (*sbi).lookup_lock);
        if !list_empty(&mut (*ino).expiring) { list_del_init(&mut (*ino).expiring); }
        spin_unlock(&mut (*sbi).lookup_lock);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
