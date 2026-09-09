/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/ipc/util.h
 * Copyright (C) 1999 Christoph Rohland
 *
 * ipc helper functions (c) 1999 Manfred Spraul <manfred@colorfullife.com>
 * namespaces support.      2006 OpenVZ, SWsoft Inc.
 *                               Pavel Emelianov <xemul@openvz.org>
 */

// C header dependencies: linux/unistd.h, linux/err.h,
// linux/ipc_namespace.h, linux/pid.h

pub const IPCMNI_SHIFT: u32 = 15;
pub const IPCMNI_EXTEND_SHIFT: u32 = 24;
pub const IPCMNI_EXTEND_MIN_CYCLE: usize = RADIX_TREE_MAP_SIZE * RADIX_TREE_MAP_SIZE;
pub const IPCMNI: i32 = 1 << IPCMNI_SHIFT;
pub const IPCMNI_EXTEND: i32 = 1 << IPCMNI_EXTEND_SHIFT;

#[cfg(CONFIG_SYSVIPC_SYSCTL)]
extern "C" {
    pub static mut ipc_mni: ::core::ffi::c_int;
    pub static mut ipc_mni_shift: ::core::ffi::c_int;
    pub static mut ipc_min_cycle: ::core::ffi::c_int;
}

#[cfg(CONFIG_SYSVIPC_SYSCTL)]
#[inline]
pub unsafe fn ipcmni_seq_shift() -> i32 { ipc_mni_shift }
#[cfg(CONFIG_SYSVIPC_SYSCTL)]
#[inline]
pub unsafe fn IPCMNI_IDX_MASK() -> i32 { (1 << ipc_mni_shift) - 1 }

#[cfg(not(CONFIG_SYSVIPC_SYSCTL))]
pub const ipc_mni: i32 = IPCMNI;
#[cfg(not(CONFIG_SYSVIPC_SYSCTL))]
pub const ipc_min_cycle: i32 = RADIX_TREE_MAP_SIZE as i32;
#[cfg(not(CONFIG_SYSVIPC_SYSCTL))]
#[inline]
pub const fn ipcmni_seq_shift() -> i32 { IPCMNI_SHIFT as i32 }
#[cfg(not(CONFIG_SYSVIPC_SYSCTL))]
pub const IPCMNI_IDX_MASK: i32 = (1 << IPCMNI_SHIFT) - 1;

extern "C" {
    pub fn sem_init();
    pub fn msg_init();
    pub fn shm_init();
}

#[repr(C)] pub struct ipc_namespace { _private: [u8; 0] }
#[repr(C)] pub struct pid_namespace { _private: [u8; 0] }

#[cfg(CONFIG_POSIX_MQUEUE)]
extern "C" { pub fn mq_clear_sbinfo(ns: *mut ipc_namespace); }
#[cfg(not(CONFIG_POSIX_MQUEUE))]
#[inline] pub unsafe fn mq_clear_sbinfo(_ns: *mut ipc_namespace) {}

#[cfg(CONFIG_SYSVIPC)]
extern "C" {
    pub fn sem_init_ns(ns: *mut ipc_namespace);
    pub fn msg_init_ns(ns: *mut ipc_namespace) -> i32;
    pub fn shm_init_ns(ns: *mut ipc_namespace);
    pub fn sem_exit_ns(ns: *mut ipc_namespace);
    pub fn msg_exit_ns(ns: *mut ipc_namespace);
    pub fn shm_exit_ns(ns: *mut ipc_namespace);
}
#[cfg(not(CONFIG_SYSVIPC))]
#[inline] pub unsafe fn sem_init_ns(_ns: *mut ipc_namespace) {}
#[cfg(not(CONFIG_SYSVIPC))]
#[inline] pub unsafe fn msg_init_ns(_ns: *mut ipc_namespace) -> i32 { 0 }
#[cfg(not(CONFIG_SYSVIPC))]
#[inline] pub unsafe fn shm_init_ns(_ns: *mut ipc_namespace) {}
#[cfg(not(CONFIG_SYSVIPC))]
#[inline] pub unsafe fn sem_exit_ns(_ns: *mut ipc_namespace) {}
#[cfg(not(CONFIG_SYSVIPC))]
#[inline] pub unsafe fn msg_exit_ns(_ns: *mut ipc_namespace) {}
#[cfg(not(CONFIG_SYSVIPC))]
#[inline] pub unsafe fn shm_exit_ns(_ns: *mut ipc_namespace) {}

#[repr(C)] pub union ipc_params_u { pub size: usize, pub nsems: i32 }
#[repr(C)] pub struct ipc_params { pub key: key_t, pub flg: i32, pub u: ipc_params_u }

#[repr(C)] pub struct ipc_ops {
    pub getnew: Option<unsafe extern "C" fn(*mut ipc_namespace, *mut ipc_params) -> i32>,
    pub associate: Option<unsafe extern "C" fn(*mut kern_ipc_perm, i32) -> i32>,
    pub more_checks: Option<unsafe extern "C" fn(*mut kern_ipc_perm, *mut ipc_params) -> i32>,
}

#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct ipc_ids { pub in_use: i32, pub max_idx: i32, _private: [u8; 0] }

extern "C" {
    pub fn ipc_init_ids(ids: *mut ipc_ids);
    pub fn ipc_init_proc_interface(path: *const i8, header: *const i8, ids: i32, show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>);
    pub fn ipc_seq_pid_ns(file: *mut seq_file) -> *mut pid_namespace;
}

pub const IPC_SEM_IDS: i32 = 0;
pub const IPC_MSG_IDS: i32 = 1;
pub const IPC_SHM_IDS: i32 = 2;

#[inline] pub fn ipcid_to_idx(id: i32) -> i32 { id & IPCMNI_IDX_MASK }
#[inline] pub unsafe fn ipcid_to_seqx(id: i32) -> i32 { id >> ipcmni_seq_shift() }
#[inline] pub unsafe fn ipcid_seq_max() -> i32 { INT_MAX >> ipcmni_seq_shift() }

#[repr(C)] pub struct kern_ipc_perm { pub seq: i32, pub deleted: bool, _private: [u8; 0] }
#[repr(C)] pub struct ipc64_perm { _private: [u8; 0] }
#[repr(C)] pub struct ipc_perm { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct msg_msg { _private: [u8; 0] }

extern "C" {
    pub fn ipc_addid(ids: *mut ipc_ids, out: *mut kern_ipc_perm, idx: i32) -> i32;
    pub fn ipc_rmid(ids: *mut ipc_ids, out: *mut kern_ipc_perm);
    pub fn ipc_set_key_private(ids: *mut ipc_ids, out: *mut kern_ipc_perm);
    pub fn ipcperms(ns: *mut ipc_namespace, ipcp: *mut kern_ipc_perm, flg: i16) -> i32;
    pub fn ipc_rcu_getref(ptr: *mut kern_ipc_perm) -> bool;
    pub fn ipc_rcu_putref(ptr: *mut kern_ipc_perm, func: Option<unsafe extern "C" fn(*mut rcu_head)>);
    pub fn ipc_obtain_object_idr(ids: *mut ipc_ids, id: i32) -> *mut kern_ipc_perm;
    pub fn kernel_to_ipc64_perm(input: *mut kern_ipc_perm, output: *mut ipc64_perm);
    pub fn ipc64_perm_to_ipc_perm(input: *mut ipc64_perm, output: *mut ipc_perm);
    pub fn ipc_update_perm(input: *mut ipc64_perm, output: *mut kern_ipc_perm) -> i32;
    pub fn ipcctl_obtain_check(ns: *mut ipc_namespace, ids: *mut ipc_ids, id: i32, cmd: i32, perm: *mut ipc64_perm, extra_perm: i32) -> *mut kern_ipc_perm;
    pub fn ipc_obtain_object_check(ids: *mut ipc_ids, id: i32) -> *mut kern_ipc_perm;
    pub fn ipcget(ns: *mut ipc_namespace, ids: *mut ipc_ids, ops: *const ipc_ops, params: *mut ipc_params) -> i32;
    pub fn free_ipcs(ns: *mut ipc_namespace, ids: *mut ipc_ids, free: Option<unsafe extern "C" fn(*mut ipc_namespace, *mut kern_ipc_perm)>);
    pub fn free_msg(msg: *mut msg_msg);
    pub fn load_msg(src: *const core::ffi::c_void, len: usize) -> *mut msg_msg;
    pub fn copy_msg(src: *mut msg_msg, dst: *mut msg_msg) -> *mut msg_msg;
    pub fn store_msg(dest: *mut core::ffi::c_void, msg: *mut msg_msg, len: usize) -> i32;
}

#[inline] pub unsafe fn ipc_get_maxidx(ids: *mut ipc_ids) -> i32 {
    if (*ids).in_use == 0 { -1 } else if (*ids).in_use == ipc_mni { ipc_mni - 1 } else { (*ids).max_idx }
}

#[inline] pub unsafe fn ipc_update_pid(pos: *mut *mut pid, new_pid: *mut pid) {
    let old = *pos;
    if old != new_pid { *pos = get_pid(new_pid); put_pid(old); }
}

#[cfg(CONFIG_ARCH_WANT_IPC_PARSE_VERSION)]
extern "C" { pub fn ipc_parse_version(cmd: *mut i32) -> i32; }

#[inline] pub unsafe fn ipc_checkid(ipcp: *mut kern_ipc_perm, id: i32) -> bool { ipcid_to_seqx(id) != (*ipcp).seq }
#[inline] pub unsafe fn ipc_lock_object(perm: *mut kern_ipc_perm) { spin_lock(perm); }
#[inline] pub unsafe fn ipc_unlock_object(perm: *mut kern_ipc_perm) { spin_unlock(perm); }
#[inline] pub unsafe fn ipc_assert_locked_object(perm: *mut kern_ipc_perm) { assert_spin_locked(perm); }
#[inline] pub unsafe fn ipc_unlock(perm: *mut kern_ipc_perm) { ipc_unlock_object(perm); rcu_read_unlock(); }
#[inline] pub unsafe fn ipc_valid_object(perm: *mut kern_ipc_perm) -> bool { !(*perm).deleted }

extern "C" {
    pub fn get_pid(pid: *mut pid) -> *mut pid;
    pub fn put_pid(pid: *mut pid);
    pub fn spin_lock(lock: *mut kern_ipc_perm);
    pub fn spin_unlock(lock: *mut kern_ipc_perm);
    pub fn assert_spin_locked(lock: *mut kern_ipc_perm);
    pub fn rcu_read_unlock();
}

#[inline] pub unsafe fn sem_check_semmni(ns: *mut ipc_namespace) -> i32 {
    // Check semmni range [0, ipc_mni]. semmni is the last element of sem_ctls[4].
    let value = *(ns.cast::<i32>().add(3));
    if value < 0 || value > ipc_mni { -ERANGE } else { 0 }
}

#[cfg(CONFIG_COMPAT)]
#[repr(C)] pub struct compat_ipc_perm { pub key: key_t, pub uid: __compat_uid_t, pub gid: __compat_gid_t, pub cuid: __compat_uid_t, pub cgid: __compat_gid_t, pub mode: compat_mode_t, pub seq: u16 }
#[cfg(CONFIG_COMPAT)]
extern "C" {
    pub fn to_compat_ipc_perm(out: *mut compat_ipc_perm, input: *mut ipc64_perm);
    pub fn to_compat_ipc64_perm(out: *mut compat_ipc64_perm, input: *mut ipc64_perm);
    pub fn get_compat_ipc_perm(out: *mut ipc64_perm, input: *mut compat_ipc_perm) -> i32;
    pub fn get_compat_ipc64_perm(out: *mut ipc64_perm, input: *mut compat_ipc64_perm) -> i32;
}
#[cfg(CONFIG_COMPAT)]
#[inline] pub unsafe fn compat_ipc_parse_version(cmd: *mut i32) -> i32 { let version = *cmd & IPC_64; *cmd &= !IPC_64; version }

#[cfg(CONFIG_COMPAT)]
extern "C" {
    pub fn compat_ksys_old_semctl(semid: i32, semnum: i32, cmd: i32, arg: i32) -> i64;
    pub fn compat_ksys_old_msgctl(msqid: i32, cmd: i32, uptr: *mut core::ffi::c_void) -> i64;
    pub fn compat_ksys_msgrcv(msqid: i32, msgp: compat_uptr_t, msgsz: compat_ssize_t, msgtyp: compat_long_t, msgflg: i32) -> i64;
    pub fn compat_ksys_msgsnd(msqid: i32, msgp: compat_uptr_t, msgsz: compat_ssize_t, msgflg: i32) -> i64;
    pub fn compat_ksys_old_shmctl(shmid: i32, cmd: i32, uptr: *mut core::ffi::c_void) -> i64;
}

// External compatibility declarations from linux/compat.h and related headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
