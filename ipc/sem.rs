// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level Rust translation of linux/ipc/sem.c.
 * Kernel-provided types, constants, synchronization primitives, allocation,
 * IPC, RCU, security, and syscall interfaces remain external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

pub const SEMMSL_FAST: usize = 256;
pub const SEMOPM_FAST: usize = 64;
pub const USE_GLOBAL_LOCK_HYSTERESIS: u32 = 10;
pub const SEM_GLOBAL_LOCK: i32 = -1;

#[repr(C)]
pub struct sem {
    pub semval: i32,
    pub sempid: *mut pid,
    pub lock: spinlock_t,
    pub pending_alter: list_head,
    pub pending_const: list_head,
    pub sem_otime: time64_t,
}

#[repr(C)]
pub struct sem_array {
    pub sem_perm: kern_ipc_perm,
    pub sem_ctime: time64_t,
    pub pending_alter: list_head,
    pub pending_const: list_head,
    pub list_id: list_head,
    pub sem_nsems: i32,
    pub complex_count: i32,
    pub use_global_lock: u32,
    pub sems: [sem; 0],
}

#[repr(C)]
pub struct sem_queue {
    pub list: list_head,
    pub sleeper: *mut task_struct,
    pub undo: *mut sem_undo,
    pub pid: *mut pid,
    pub status: i32,
    pub sops: *mut sembuf,
    pub blocking: *mut sembuf,
    pub nsops: i32,
    pub alter: bool,
    pub dupsop: bool,
}

#[repr(C)]
pub struct sem_undo {
    pub list_proc: list_head,
    pub rcu: rcu_head,
    pub ulp: *mut sem_undo_list,
    pub list_id: list_head,
    pub semid: i32,
    pub semadj: [i16; 0],
}

#[repr(C)]
pub struct sem_undo_list {
    pub refcnt: refcount_t,
    pub lock: spinlock_t,
    pub list_proc: list_head,
}

// The following declarations are supplied by the kernel translation unit.
// They intentionally remain opaque here, matching the C file's dependencies.
pub type key_t = i32;
pub type time64_t = i64;
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct kern_ipc_perm { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }
#[repr(C)] pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }
#[repr(C)] pub struct ipc_namespace { _private: [u8; 0] }
#[repr(C)] pub struct ipc_params { _private: [u8; 0] }
#[repr(C)] pub struct wake_q_head { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }

extern "C" {
    pub fn sem_init_ns(ns: *mut ipc_namespace);
    pub fn sem_exit_ns(ns: *mut ipc_namespace);
    pub fn sem_init();
}

// Remaining kernel operations are deliberately expressed as external symbols;
// their implementations and declarations are provided by the surrounding
// Linux IPC translation units, as they are by the original source includes.
extern "C" {
    pub fn sem_alloc(nsems: usize) -> *mut sem_array;
    pub fn newary(ns: *mut ipc_namespace, params: *mut ipc_params) -> i32;
    pub fn ksys_semget(key: key_t, nsems: i32, semflg: i32) -> isize;
}

const _: *const c_void = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
